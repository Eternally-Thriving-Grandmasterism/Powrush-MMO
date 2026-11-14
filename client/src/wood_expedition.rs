//! Inert wood document and off-thread publication.
//! Both siblings confirmed absent: offline local deletion resets. No tamper claim.
//! A crash may replay a checkpoint. No durable exactly-once. No one-flush backup bound.
//! One process. No cross-process exclusion.
//! Final exit wait is a 250 ms design ceiling, not a measured guarantee.

use bevy::mesh::{ConeAnchor, ConeMeshBuilder, CylinderAnchor, CylinderMeshBuilder};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use shared::heartwood_wards::{WARD_POST_CENTERS, WARD_USE_RADIUS};
use shared::hex_travel::PlaceId;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const SAVE_NAME: &str = "powrush_wood_expedition.json";
const SCHEMA: u32 = 1;
const TREE_ID: u64 = 0;
const EXIT_BUDGET: Duration = Duration::from_millis(250);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Stand {
    Standing,
    Depleted,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Charge {
    None,
    Ready,
    Spent,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Doc {
    schema: u32,
    revision: u64,
    generation: u64,
    session: u64,
    op: u64,
    tree: u64,
    stand: Stand,
    unit: u8,
    charge: Charge,
}
impl Doc {
    fn fresh(session: u64) -> Self {
        Self {
            schema: SCHEMA,
            revision: 0,
            generation: 0,
            session,
            op: 0,
            tree: TREE_ID,
            stand: Stand::Standing,
            unit: 0,
            charge: Charge::None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Ident {
    session: u64,
    generation: u64,
    op: u64,
    revision: u64,
    content: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Snap {
    doc: Doc,
    bytes: Vec<u8>,
    id: Ident,
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Raw {
    Bytes(Vec<u8>),
    NotFound,
    Err,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fail {
    #[cfg_attr(not(test), allow(dead_code))]
    PreRename,
    Ambiguous,
    Conflict,
}
#[derive(Clone, Debug, PartialEq, Eq)]
enum Recovered {
    Fresh(Doc),
    Primary(Doc),
    Backup(Doc),
    Inert,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Recovery {
    Fresh,
    Primary,
    Backup,
    Inert,
}
enum Cmd {
    Snap(Snap),
    Stop,
}
enum Note {
    Boot(Recovered),
    Done { id: Ident, result: Result<(), Fail> },
    Ended,
}

trait Io: Send + 'static {
    fn note_worker(&self) {}
    fn read_live(&self) -> Raw;
    fn read_backup(&self) -> Raw;
    fn publish(&self, bytes: &[u8]) -> Result<(), Fail>;
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|err| err.into_inner())
}
fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
fn canon(doc: &Doc) -> Option<Vec<u8>> {
    serde_json::to_vec(doc).ok()
}
fn domain_ok(doc: &Doc) -> bool {
    if doc.schema != SCHEMA || doc.tree != TREE_ID || doc.unit > 1 {
        return false;
    }
    if doc.stand == Stand::Standing && doc.unit != 0 {
        return false;
    }
    if doc.stand == Stand::Standing && doc.charge == Charge::Ready {
        return false;
    }
    doc.unit != 1 || doc.charge != Charge::Ready
}
fn parse_doc(bytes: &[u8]) -> Option<Doc> {
    let doc: Doc = serde_json::from_slice(bytes).ok()?;
    domain_ok(&doc).then_some(doc)
}
fn snap_from(doc: Doc) -> Option<Snap> {
    if !domain_ok(&doc) {
        return None;
    }
    let bytes = canon(&doc)?;
    let id = Ident {
        session: doc.session,
        generation: doc.generation,
        op: doc.op,
        revision: doc.revision,
        content: hash_bytes(&bytes),
    };
    Some(Snap { doc, bytes, id })
}
fn classify(raw: Raw) -> Result<Option<Doc>, ()> {
    match raw {
        Raw::NotFound => Ok(None),
        Raw::Err => Err(()),
        Raw::Bytes(bytes) => parse_doc(&bytes).map(Some).ok_or(()),
    }
}
fn recover(primary: Raw, backup: Raw, session: u64) -> Recovered {
    match classify(primary) {
        Ok(Some(doc)) => Recovered::Primary(doc),
        Err(()) => Recovered::Inert,
        Ok(None) => match classify(backup) {
            Ok(Some(doc)) => Recovered::Backup(doc), // whole snapshot, not a field merge
            Ok(None) => Recovered::Fresh(Doc::fresh(session)), // deletion resets; not a tamper claim
            Err(()) => Recovered::Inert,
        },
    }
}
fn mint_session() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(1);
    nanos.wrapping_mul(0x9E37_79B1_85EB_CA87) ^ u64::from(std::process::id())
}
/// Checkpoint replay is allowed. No durable exactly-once. No one-flush backup bound.
fn publish_one(io: &impl Io, snap: &Snap) -> Result<(), Fail> {
    match io.publish(&snap.bytes) {
        Ok(()) => Ok(()),
        Err(Fail::PreRename) => Err(Fail::PreRename),
        Err(Fail::Conflict) => Err(Fail::Conflict),
        Err(Fail::Ambiguous) => match io.read_live() {
            Raw::Bytes(found) if found == snap.bytes => Ok(()),
            Raw::Bytes(found) => match parse_doc(&found) {
                Some(doc) if doc.revision == snap.doc.revision && found != snap.bytes => {
                    Err(Fail::Conflict)
                }
                _ => Err(Fail::Ambiguous),
            },
            _ => Err(Fail::Ambiguous),
        },
    }
}
fn worker(io: impl Io, rx: Receiver<Cmd>, tx: Sender<Note>, acked: Arc<AtomicBool>) {
    io.note_worker();
    let _ = tx.send(Note::Boot(recover(
        io.read_live(),
        io.read_backup(),
        mint_session(),
    )));
    loop {
        match rx.recv() {
            Ok(Cmd::Snap(snap)) => {
                let result = publish_one(&io, &snap);
                let _ = tx.send(Note::Done {
                    id: snap.id,
                    result,
                });
            }
            Ok(Cmd::Stop) => {
                acked.store(true, Ordering::SeqCst);
                let _ = tx.send(Note::Ended);
                break;
            }
            Err(_) => break,
        }
    }
}

#[derive(Default)]
struct Body {
    memory: Option<Snap>,
    inflight: Option<Snap>,
    queued: Option<Snap>,
    confirmed: Option<Snap>,
    inert: bool,
    blocked: bool,
    conflict: bool,
    dirty: bool,
    booted: bool,
    recovery: Option<Recovery>,
}
impl Body {
    fn sync_dirty(&mut self) {
        self.dirty = self.memory.as_ref().map(|s| s.id) != self.confirmed.as_ref().map(|s| s.id);
    }
    fn boot(&mut self, recovered: Recovered) {
        self.booted = true;
        self.dirty = false;
        let (recovery, doc) = match recovered {
            Recovered::Inert => {
                self.inert = true;
                self.recovery = Some(Recovery::Inert);
                return;
            }
            Recovered::Fresh(doc) => (Recovery::Fresh, doc),
            Recovered::Primary(doc) => (Recovery::Primary, doc),
            Recovered::Backup(doc) => (Recovery::Backup, doc),
        };
        self.recovery = Some(recovery);
        match snap_from(doc) {
            Some(snap) => self.memory = Some(snap),
            None => {
                self.inert = true;
                self.recovery = Some(Recovery::Inert);
            }
        }
    }
    fn hand(&mut self, snap: Snap, tx: &Sender<Cmd>) {
        if self.inert || self.blocked {
            return;
        }
        self.inflight = Some(snap.clone());
        let _ = tx.send(Cmd::Snap(snap));
    }
    #[cfg_attr(not(test), allow(dead_code))]
    fn submit(&mut self, doc: Doc, tx: &Sender<Cmd>) {
        if self.inert || self.blocked || self.conflict || !self.booted {
            return;
        }
        let Some(snap) = snap_from(doc) else {
            return;
        };
        if let Some(memory) = &self.memory {
            if memory.doc.revision == snap.doc.revision && memory.bytes != snap.bytes {
                self.conflict = true;
                return;
            }
            if snap.doc.revision < memory.doc.revision || memory.bytes == snap.bytes {
                return;
            }
        }
        self.memory = Some(snap.clone());
        self.sync_dirty();
        if self.inflight.is_some() {
            self.queued = Some(snap);
        } else {
            self.hand(snap, tx);
        }
    }
    fn finish(&mut self, id: Ident, result: Result<(), Fail>, tx: &Sender<Cmd>) {
        let Some(flight) = self.inflight.clone() else {
            return;
        };
        if flight.id != id {
            return;
        }
        match result {
            Ok(()) => {
                self.confirmed = self.inflight.take();
                if let Some(next) = self.queued.take() {
                    self.hand(next, tx);
                }
            }
            Err(Fail::PreRename) => {
                // Previous published bytes and confirmed stay. The failed snap does not replace queued.
                self.inflight = None;
                if let Some(next) = self.queued.take() {
                    self.hand(next, tx);
                }
            }
            Err(fail) => {
                // An unresolved write blocks later saves. No retry and no backoff.
                self.inflight = None;
                self.blocked = true;
                self.inert = true;
                self.conflict = fail == Fail::Conflict;
            }
        }
        self.sync_dirty();
    }
    fn writable(&self) -> bool {
        self.booted && !self.inert && !self.blocked && !self.conflict && self.memory.is_some()
    }
    fn claim_snap(&self) -> Result<Snap, Reject> {
        if !self.writable() {
            return Err(Reject::Writable);
        }
        let snap = self.memory.clone().ok_or(Reject::Writable)?;
        let _ = successor(&snap.doc)?;
        Ok(snap)
    }
    /// `claimed` is the predecessor captured at the press, not a fresh clone of memory.
    fn commit_at(&mut self, claimed: &Snap, tx: &Sender<Cmd>) -> Result<(), Reject> {
        self.commit_by(claimed, tx, successor)
    }
    fn commit_by(
        &mut self,
        claimed: &Snap,
        tx: &Sender<Cmd>,
        step: fn(&Doc) -> Result<Doc, Reject>,
    ) -> Result<(), Reject> {
        if !self.writable() {
            return Err(Reject::Writable);
        }
        let prior = self.memory.clone().ok_or(Reject::Writable)?;
        if prior.id != claimed.id || prior.bytes != claimed.bytes {
            return Err(Reject::Identity);
        }
        let snap = snap_from(step(&prior.doc)?).ok_or(Reject::Preview)?;
        let (queued, dirty, inflight) = (self.queued.clone(), self.dirty, self.inflight.clone());
        self.memory = Some(snap.clone());
        self.sync_dirty();
        if inflight.is_some() {
            self.queued = Some(snap);
            return Ok(());
        }
        match tx.send(Cmd::Snap(snap.clone())) {
            Ok(()) => {
                self.inflight = Some(snap);
                Ok(())
            }
            Err(_) => {
                self.memory = Some(prior);
                self.queued = queued;
                self.dirty = dirty;
                self.inflight = inflight;
                Err(Reject::Publish)
            }
        }
    }
}

struct Disk;
impl Io for Disk {
    fn read_live(&self) -> Raw {
        map_raw(shared::user_persist::read_live_bytes(SAVE_NAME))
    }
    fn read_backup(&self) -> Raw {
        map_raw(shared::user_persist::read_backup_bytes(SAVE_NAME))
    }
    fn publish(&self, bytes: &[u8]) -> Result<(), Fail> {
        // One writer. `write_named` uses its fixed temp. `Err` may be post-rename.
        match shared::user_persist::write_named(SAVE_NAME, bytes) {
            Ok(()) => Ok(()),
            Err(_) => Err(Fail::Ambiguous),
        }
    }
}
fn map_raw(result: std::io::Result<shared::user_persist::RawPersist>) -> Raw {
    match result {
        Ok(shared::user_persist::RawPersist::Bytes(bytes)) => Raw::Bytes(bytes),
        Ok(shared::user_persist::RawPersist::NotFound) => Raw::NotFound,
        Err(_) => Raw::Err,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Reject {
    Writable,
    Identity,
    Ready,
    Overflow,
    Preview,
    Publish,
}

#[derive(Resource)]
pub(crate) struct WoodHub {
    tx: Sender<Cmd>,
    rx: Mutex<Receiver<Note>>,
    join: Mutex<Option<JoinHandle<()>>>,
    body: Body,
    budget: Duration,
    released: bool,
    stop_sent: bool,
    #[cfg_attr(not(test), allow(dead_code))]
    acked: Arc<AtomicBool>,
}
impl WoodHub {
    fn spawn(io: impl Io, budget: Duration) -> Self {
        let (tx, cmd_rx) = mpsc::channel();
        let (note_tx, note_rx) = mpsc::channel();
        let acked = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&acked);
        let join = thread::spawn(move || worker(io, cmd_rx, note_tx, flag));
        Self {
            tx,
            rx: Mutex::new(note_rx),
            join: Mutex::new(Some(join)),
            body: Body::default(),
            budget,
            released: false,
            stop_sent: false,
            acked,
        }
    }
    #[cfg_attr(not(test), allow(dead_code))]
    fn submit(&mut self, doc: Doc) {
        self.body.submit(doc, &self.tx);
    }
    fn poll(&mut self) {
        for note in self.try_notes() {
            self.apply(note);
        }
    }
    fn try_notes(&self) -> Vec<Note> {
        let rx = lock(&self.rx);
        let mut notes = Vec::new();
        loop {
            match rx.try_recv() {
                Ok(note) => notes.push(note),
                Err(_) => break,
            }
        }
        notes
    }
    fn apply(&mut self, note: Note) {
        match note {
            Note::Boot(recovered) => self.body.boot(recovered),
            Note::Done { id, result } => self.body.finish(id, result, &self.tx),
            Note::Ended => self.release_join(),
        }
    }
    pub(crate) fn shutdown(&mut self) {
        if self.released || self.stop_sent {
            return;
        }
        let deadline = Instant::now() + self.budget;
        let mut handed = false;
        loop {
            self.poll();
            if self.body.inflight.is_some() {
                if Instant::now() >= deadline {
                    self.seal_timeout();
                    return;
                }
                thread::sleep(Duration::from_millis(5));
                continue;
            }
            if !handed {
                if let Some(snap) = self.pending_intent() {
                    handed = true;
                    self.body.hand(snap, &self.tx);
                    continue;
                }
            }
            break;
        }
        if Instant::now() >= deadline {
            self.seal_timeout();
            return;
        }
        self.stop_sent = true;
        let _ = self.tx.send(Cmd::Stop);
        while Instant::now() < deadline {
            self.poll();
            if self.released {
                return;
            }
            thread::sleep(Duration::from_millis(5));
        }
    }
    fn seal_timeout(&mut self) {
        // The deadline passed. No second writer, no join, and no assumed outcome.
        self.stop_sent = true;
        let _ = self.tx.send(Cmd::Stop);
    }
    fn pending_intent(&mut self) -> Option<Snap> {
        if self.body.blocked || self.body.inert || !self.body.dirty {
            return None;
        }
        self.body.queued.take().or_else(|| self.body.memory.clone())
    }
    fn release_join(&mut self) {
        if let Some(handle) = lock(&self.join).take() {
            let _ = handle.join();
        }
        self.released = true;
    }
    fn claim_snap(&self) -> Result<Snap, Reject> {
        self.body.claim_snap()
    }
    fn commit_at(&mut self, claimed: &Snap) -> Result<(), Reject> {
        self.body.commit_at(claimed, &self.tx)
    }
    fn can_write(&self) -> bool {
        self.body.writable()
    }
    /// Readable accepted memory. Writer flags stay on the admit path.
    fn view_doc(&self) -> Option<Doc> {
        if !self.body.booted {
            return None;
        }
        match self.body.recovery {
            Some(Recovery::Fresh | Recovery::Primary | Recovery::Backup) => {
                self.body.memory.as_ref().map(|snap| snap.doc.clone())
            }
            _ => None,
        }
    }
    pub(crate) fn is_booted(&self) -> bool {
        self.body.booted
    }
    pub(crate) fn memory_unit(&self) -> Option<u8> {
        self.body.memory.as_ref().map(|snap| snap.doc.unit)
    }
    /// Refine leg: commits the carried unit out and returns it (1), or 0 when none can be taken.
    pub(crate) fn take_unit(&mut self) -> u8 {
        let Some(claimed) = self.body.memory.clone() else {
            return 0;
        };
        u8::from(self.body.commit_by(&claimed, &self.tx, consume).is_ok())
    }
    #[cfg(test)]
    fn join_after_stop(&mut self) {
        if self.released {
            return;
        }
        let deadline = Instant::now() + Duration::from_secs(2);
        while Instant::now() < deadline {
            let _ = self.try_notes();
            if self.acked.load(Ordering::SeqCst) {
                self.release_join();
                return;
            }
            thread::sleep(Duration::from_millis(2));
        }
    }
}

pub struct WoodExpeditionPlugin;
impl Plugin for WoodExpeditionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WoodHub::spawn(Disk, EXIT_BUDGET))
            .init_resource::<WoodClaim>()
            .init_resource::<WoodFacts>()
            .init_resource::<WoodView>();
        if let Some(site) = first_passing_site() {
            app.insert_resource(WoodClearance::reserve(site));
        }
        add_wood_systems(app);
        app.add_systems(Last, stop_on_exit);
    }
}
fn poll_wood(mut hub: ResMut<WoodHub>) {
    hub.poll();
}
fn stop_on_exit(mut exit: MessageReader<AppExit>, mut hub: ResMut<WoodHub>) {
    if exit.read().next().is_some() {
        hub.shutdown();
    }
}

pub(crate) const SITES: [Vec2; 4] = [
    Vec2::new(6.0, 14.0),
    Vec2::new(-6.0, 14.0),
    Vec2::new(6.0, 18.0),
    Vec2::new(-6.0, 18.0),
];
const TRUNK_R: f32 = 0.11;
const TRUNK_H: f32 = 1.35;
const CANOPY_R: f32 = 0.95;
const CANOPY_H: f32 = 2.05;
const CANOPY_SIT: f32 = 0.78;
const BODY_R: f32 = 0.3;
const CAM_MARGIN: f32 = 0.5;
const IN_TOL: f32 = 1.0e-4;
const OUT_TOL: f32 = 1.0e-3;

#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct WoodClearance {
    pub center: Vec2,
    pub radius: f32,
}
impl WoodClearance {
    pub(crate) fn reserve(site: Vec2) -> Self {
        Self {
            center: site,
            radius: enclosing_radius(),
        }
    }
}
#[derive(Resource, Default)]
pub(crate) struct WoodClaim {
    held: bool,
    prior: Option<Snap>,
}
impl WoodClaim {
    pub(crate) fn blocks(&self) -> bool {
        self.held
    }
    #[cfg(test)]
    pub(crate) fn set_held(&mut self, held: bool) {
        self.held = held;
    }
}
#[derive(Resource, Default)]
pub(crate) struct WoodFacts {
    mercy_blocks: bool,
    door_blocks: bool,
    in_reach: bool,
    live: bool,
}
#[derive(Resource, Default)]
pub(crate) struct WoodView {
    pub(crate) unit: u8,
    pub(crate) standing: bool,
}
#[derive(Component)]
pub(crate) struct OwnedTree;
#[derive(Component)]
struct BarkBit;

fn successor(doc: &Doc) -> Result<Doc, Reject> {
    if doc.schema != SCHEMA || doc.tree != TREE_ID || doc.stand != Stand::Standing || doc.unit != 0
    {
        return Err(Reject::Identity);
    }
    if doc.charge == Charge::Ready {
        return Err(Reject::Ready);
    }
    if doc.charge != Charge::None && doc.charge != Charge::Spent {
        return Err(Reject::Identity);
    }
    let next = Doc {
        schema: doc.schema,
        revision: doc.revision.checked_add(1).ok_or(Reject::Overflow)?,
        generation: doc.generation.checked_add(1).ok_or(Reject::Overflow)?,
        session: doc.session,
        op: doc.op.checked_add(1).ok_or(Reject::Overflow)?,
        tree: doc.tree,
        stand: Stand::Depleted,
        unit: 1,
        charge: Charge::None,
    };
    domain_ok(&next).then_some(next).ok_or(Reject::Preview)
}
/// Refine leg: Depleted + unit 1 → Depleted + unit 0, same charge. Not a harvest path.
fn consume(doc: &Doc) -> Result<Doc, Reject> {
    if doc.schema != SCHEMA || doc.tree != TREE_ID || doc.stand != Stand::Depleted || doc.unit != 1
    {
        return Err(Reject::Identity);
    }
    if doc.charge == Charge::Ready {
        return Err(Reject::Ready);
    }
    let next = Doc {
        revision: doc.revision.checked_add(1).ok_or(Reject::Overflow)?,
        generation: doc.generation.checked_add(1).ok_or(Reject::Overflow)?,
        op: doc.op.checked_add(1).ok_or(Reject::Overflow)?,
        unit: 0,
        ..doc.clone()
    };
    domain_ok(&next).then_some(next).ok_or(Reject::Preview)
}
fn enclosing_radius() -> f32 {
    (CANOPY_R * (1.0 + 0.04 + 0.06)).max(TRUNK_R * (1.0 + 0.08))
}
fn tree_sphere(site: Vec2) -> (Vec3, f32) {
    let bottom = crate::vegetation::seated_ground(site.x, site.y);
    let top = bottom + TRUNK_H * CANOPY_SIT + CANOPY_H * (1.0 + 0.04);
    let half = (top - bottom) * 0.5;
    (
        Vec3::new(site.x, bottom + half, site.y),
        half.hypot(enclosing_radius()),
    )
}
fn neighbor_feet() -> Vec<(Vec2, f32)> {
    let mut feet = Vec::new();
    let yard = (1.05_f32 * 1.04).max(crate::living_ecology::YARD_TRUNK_R);
    for spot in crate::living_ecology::YARD_TREE_SPOTS {
        feet.push((spot.xz(), yard));
    }
    let rock = 0.42 * 1.4 * 1.06;
    for (x, z) in [(4.2_f32, -5.5), (-4.8, -5.0), (7.0, 0.4)] {
        feet.push((Vec2::new(x, z), rock));
    }
    feet.push((Vec2::new(8.5, 6.2), 0.16));
    for (x, z) in [(5.0_f32, -6.2), (-5.4, -6.8), (7.4, 1.6), (-7.2, 2.2)] {
        feet.push((Vec2::new(x, z), 0.16));
    }
    let node = 0.48 * 0.92 * 1.06 * 1.38;
    let wells: Vec<Vec2> = shared::skirmish_well::WELL_ANCHORS
        .into_iter()
        .map(|(x, _, z)| Vec2::new(x, z))
        .collect();
    for xz in &wells {
        feet.push((*xz, node));
    }
    for (a, b) in [(0usize, 1), (1, 2), (2, 0)] {
        feet.push((
            (wells[a] + wells[b]) * 0.5,
            wells[a].distance(wells[b]) * 0.5 + 0.035,
        ));
    }
    feet
}
fn boot_camera() -> (Transform, bevy::camera::PerspectiveProjection) {
    (
        Transform::from_xyz(0.0, 8.0, 14.0).looking_at(Vec3::ZERO, Vec3::Y),
        bevy::camera::PerspectiveProjection::default(),
    )
}
fn camera_margin_ok(site: Vec2) -> bool {
    site.distance(crate::vegetation::CAMERA_XZ) - enclosing_radius()
        >= crate::vegetation::CAMERA_CLEAR + CAM_MARGIN
}
fn frustum_ok(site: Vec2) -> bool {
    let (cam, projection) = boot_camera();
    let (center, radius) = tree_sphere(site);
    let rel = center - cam.translation;
    let depth = rel.dot(cam.forward().as_vec3());
    let (vx, vy) = (rel.dot(cam.right().as_vec3()), rel.dot(cam.up().as_vec3()));
    let tan_y = (projection.fov * 0.5).tan();
    let tan_x =
        crate::title_screen::DECK_TITLE_WIDTH / crate::title_screen::DECK_TITLE_HEIGHT * tan_y;
    let pad = radius + CAM_MARGIN;
    depth - radius >= projection.near + CAM_MARGIN
        && depth + radius <= projection.far - CAM_MARGIN
        && (vx.abs() > depth * tan_x + pad || vy.abs() > depth * tan_y + pad)
}
fn ground_ok(site: Vec2) -> bool {
    crate::terrain::play_footprint(site.x, site.y)
        && crate::terrain::shelf_offset(site.x, site.y) == 0.0
}
fn neighbors_ok(site: Vec2) -> bool {
    let radius = enclosing_radius();
    neighbor_feet()
        .into_iter()
        .all(|(center, foot)| site.distance(center) >= radius + foot)
}
fn contact_inside(distance: f32) -> bool {
    distance <= TRUNK_R + BODY_R + IN_TOL
}
fn stance_offset() -> f32 {
    TRUNK_R + BODY_R
}
fn foot_clear(at: Vec2, feet: &[(Vec2, f32)]) -> bool {
    crate::terrain::play_footprint(at.x, at.y)
        && crate::terrain::shelf_offset(at.x, at.y) == 0.0
        && feet
            .iter()
            .all(|(center, foot)| at.distance(*center) >= *foot)
}
fn support_ok(site: Vec2) -> bool {
    let feet = neighbor_feet();
    let stand = stance_offset();
    [Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y]
        .into_iter()
        .all(|dir| {
            let center = site + dir * stand;
            contact_inside(center.distance(site))
                && [Vec2::ZERO, Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y]
                    .into_iter()
                    .all(|rim| foot_clear(center + rim * BODY_R, &feet))
        })
}
fn use_headroom(at: Vec2) -> f32 {
    let mut best = f32::INFINITY;
    let mut note = |p: Vec2, radius: f32| {
        best = best.min(at.distance(p) - radius);
    };
    for (x, _, z) in shared::skirmish_well::WELL_ANCHORS {
        let p = Vec2::new(x, z);
        note(p, shared::skirmish_well::CONTEST_REACH);
        note(p, crate::mercy_harvest_nodes::HARVEST_REACH);
    }
    for post in WARD_POST_CENTERS {
        note(Vec2::new(post[0], post[2]), WARD_USE_RADIUS);
    }
    for spot in [
        crate::wellsworn::PLAQUE_SPOT,
        crate::wellsworn::BOOK_SPOT,
        crate::wellsworn::STAVEHOLD_GATE_SPOT,
    ] {
        note(Vec2::new(spot[0], spot[1]), crate::wellsworn::SPOT_REACH);
    }
    let shelf = shared::threshold_shelf::THRESHOLD_SHELF_CENTER;
    note(
        Vec2::new(shelf[0], shelf[2]),
        shared::threshold_shelf::THRESHOLD_SHELF_USE_RADIUS,
    );
    note(
        Vec2::from(shared::heartwood_lamp::LAMP_DISK_CENTER),
        shared::heartwood_lamp::LAMP_DISK_RADIUS,
    );
    note(
        Vec2::from(shared::heartwood_lamp::WATER_POND_CENTER),
        shared::heartwood_lamp::WATER_POND_RADIUS,
    );
    best
}
fn reach_ok(site: Vec2) -> bool {
    let stand = stance_offset();
    [Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y]
        .into_iter()
        .all(|dir| {
            let at = site + dir * stand;
            contact_inside(at.distance(site))
                && at.distance(site) <= crate::mercy_harvest_nodes::HARVEST_REACH
                && use_headroom(at) > 0.0
        })
}
pub(crate) fn site_flags(site: Vec2) -> [(&'static str, bool); 6] {
    [
        ("camera-margin", camera_margin_ok(site)),
        ("frustum", frustum_ok(site)),
        ("ground", ground_ok(site)),
        ("support", support_ok(site)),
        ("neighbors", neighbors_ok(site)),
        ("reach", reach_ok(site)),
    ]
}
pub(crate) fn first_passing_site() -> Option<Vec2> {
    SITES
        .into_iter()
        .find(|site| site_flags(*site).iter().all(|(_, ok)| *ok))
}
pub(crate) fn wood_live(place: Option<PlaceId>, realm: Option<u8>) -> bool {
    place == Some(PlaceId::Sanctuary) && crate::vegetation::vegetation_shown(realm)
}
pub(crate) fn owned_blocker(
    hub: Option<&WoodHub>,
    place: Option<PlaceId>,
    realm: Option<u8>,
) -> Option<(Vec2, f32)> {
    if !wood_live(place, realm) {
        return None;
    }
    let doc = hub?.view_doc()?;
    (doc.stand == Stand::Standing && doc.unit == 0).then_some((first_passing_site()?, TRUNK_R))
}
pub(crate) fn use_in_reach(
    hub: Option<&WoodHub>,
    place: Option<PlaceId>,
    realm: Option<u8>,
    position: Option<Vec3>,
) -> bool {
    if !wood_live(place, realm) {
        return false;
    }
    let Some(hub) = hub.filter(|hub| hub.can_write()) else {
        return false;
    };
    if hub
        .view_doc()
        .filter(|doc| doc.stand == Stand::Standing && doc.unit == 0)
        .is_none()
    {
        return false;
    }
    let (Some(site), Some(position)) = (first_passing_site(), position) else {
        return false;
    };
    contact_inside(Vec2::new(position.x, position.z).distance(site))
}
fn yard_open(
    door: Option<&crate::title_screen::LaunchDoor>,
    house: Option<&crate::title_screen::HouseLabel>,
) -> bool {
    match (door, house) {
        (Some(door), Some(label)) => {
            *door == crate::title_screen::LaunchDoor::InYard && !label.settings_open
        }
        _ => true,
    }
}
pub(crate) fn mercy_blocks(origin: Option<Vec3>, nodes: impl IntoIterator<Item = Vec3>) -> bool {
    let Some(origin) = origin else { return false };
    let mut best = f32::INFINITY;
    let mut any = false;
    for node in nodes {
        any = true;
        best = best.min(origin.distance(node));
    }
    any && best <= crate::mercy_harvest_nodes::HARVEST_REACH
}
struct DoorSrc<'a> {
    hour: Option<&'a crate::hour_sacred::HourSacred>,
    voice: Option<&'a crate::coop_voice::VoiceYard>,
    ledger: Option<&'a crate::ledger_bind::LedgerYard>,
    embassy: Option<&'a crate::embassy::EmbassyYard>,
    crown: Option<&'a crate::crownstone::CrownstoneYard>,
    redemption: Option<&'a crate::species_redemption::RedemptionYard>,
    hybrid: Option<&'a crate::hybrid_matrix::HybridYard>,
    presence: Option<&'a crate::human_presence::SoftPresence>,
    place: Option<PlaceId>,
}
impl<'a> DoorSrc<'a> {
    fn bare() -> Self {
        Self {
            hour: None,
            voice: None,
            ledger: None,
            embassy: None,
            crown: None,
            redemption: None,
            hybrid: None,
            presence: None,
            place: None,
        }
    }
}
fn session_door(src: &DoorSrc<'_>) -> bool {
    let Some(hour) = src.hour else {
        return false;
    };
    let skin = hour.charter_skin_live();
    let voice_sash = src.voice.is_some_and(|yard| yard.sash_open);
    let ledger_sash = src.ledger.is_some_and(|yard| yard.sash_open);
    let beacon = skin
        && voice_sash
        && src
            .voice
            .is_some_and(|yard| yard.voice.open_card().is_some());
    let ledger = skin && ledger_sash;
    let embassy = src.place != Some(PlaceId::Heartwood)
        && hour.complete
        && skin
        && src
            .embassy
            .is_some_and(|yard| yard.embassy.lamp_live && !yard.embassy.seated)
        && !voice_sash
        && !ledger_sash;
    let crown = hour.hour_three_complete
        && skin
        && src.embassy.is_some_and(|yard| yard.embassy.seated)
        && src.crown.is_some_and(|yard| !yard.stone.witnessed)
        && !voice_sash
        && !ledger_sash;
    let redemption = skin
        && src.crown.is_some_and(|yard| yard.stone.witnessed)
        && src.redemption.is_some_and(|yard| yard.state.events == 0)
        && !voice_sash
        && !ledger_sash;
    let hybrid = skin
        && src.redemption.is_some_and(|yard| yard.state.events > 0)
        && src.hybrid.is_some_and(|yard| !yard.matrix.attuned)
        && !voice_sash
        && !ledger_sash;
    beacon || ledger || embassy || crown || redemption || hybrid
}
fn door_blocks(src: DoorSrc<'_>) -> bool {
    if src
        .hour
        .is_some_and(|hour| hour.session.peace_visitor_on_frontier())
        || session_door(&src)
        || src.place == Some(PlaceId::Depths)
    {
        return true;
    }
    let Some(presence) = src.presence else {
        return false;
    };
    let (x, z, here) = (
        presence.position.x,
        presence.position.z,
        src.place.unwrap_or(PlaceId::Sanctuary),
    );
    if crate::skirmish_well::near_first_well(presence)
        || crate::wellsworn::spot_at(here, x, z).is_some()
        || shared::threshold_shelf::threshold_use_in_reach(here, x, z)
        || crate::heartwood_plaque::plaque_in_reach(here, x, z)
        || shared::heartwood_lamp::in_lamp_disk(here, x, z)
        || shared::heartwood_lamp::in_water(here, x, z)
    {
        return true;
    }
    here == PlaceId::Heartwood
        && WARD_POST_CENTERS
            .iter()
            .any(|c| Vec2::new(x, z).distance(Vec2::new(c[0], c[2])) <= WARD_USE_RADIUS)
}
fn trunk_mesh() -> Mesh {
    Mesh::from(CylinderMeshBuilder::new(TRUNK_R, TRUNK_H, 8).anchor(CylinderAnchor::Bottom))
}
fn canopy_mesh() -> Mesh {
    Mesh::from(ConeMeshBuilder::new(CANOPY_R, CANOPY_H, 8).anchor(ConeAnchor::Base))
}
fn bark_mesh() -> Mesh {
    Mesh::from(CylinderMeshBuilder::new(0.04, 0.18, 8).anchor(CylinderAnchor::Bottom))
}
pub(crate) fn add_wood_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            poll_wood,
            refresh_wood_facts
                .after(poll_wood)
                .after(crate::feel_move::raise_buffered_use)
                .after(crate::feel_move::arm_use_buffer_from_edge)
                .after(crate::first_harvest_epiphany::mark_peace_visitor)
                .after(crate::first_harvest_epiphany::mark_epiphany_place),
            claim_wood_press
                .after(refresh_wood_facts)
                .before(crate::first_harvest_epiphany::handle_interact_harvest)
                .before(crate::heartwood_wards::use_heartwood_wards)
                .before(crate::living_practice_loop::soft_interact_harvest_credit),
            admit_wood_press
                .after(claim_wood_press)
                .after(crate::first_harvest_epiphany::handle_interact_harvest)
                .after(crate::heartwood_wards::use_heartwood_wards)
                .after(crate::living_practice_loop::soft_interact_harvest_credit)
                .before(project_wood),
            project_wood,
        ),
    );
}
pub(crate) fn refresh_wood_facts(
    mut facts: ResMut<WoodFacts>,
    hub: Option<Res<WoodHub>>,
    presence: Option<Res<crate::human_presence::SoftPresence>>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    realm: Option<Res<crate::living_practice_loop::SoftPlayerRealm>>,
    hour: Option<Res<crate::hour_sacred::HourSacred>>,
    voice: Option<Res<crate::coop_voice::VoiceYard>>,
    ledger: Option<Res<crate::ledger_bind::LedgerYard>>,
    embassy: Option<Res<crate::embassy::EmbassyYard>>,
    crown: Option<Res<crate::crownstone::CrownstoneYard>>,
    redemption: Option<Res<crate::species_redemption::RedemptionYard>>,
    hybrid: Option<Res<crate::hybrid_matrix::HybridYard>>,
    nodes: Query<&Transform, With<crate::mercy_harvest_nodes::MercyHarvestNode>>,
) {
    let place = travel.as_ref().map(|state| state.current);
    let realm = realm.as_ref().map(|state| state.current).unwrap_or(None);
    let position = presence.as_ref().map(|body| body.position);
    facts.live = wood_live(place, realm);
    facts.in_reach = use_in_reach(hub.as_deref(), place, realm, position);
    facts.mercy_blocks = mercy_blocks(position, nodes.iter().map(|tf| tf.translation));
    facts.door_blocks = door_blocks(DoorSrc {
        hour: hour.as_deref(),
        voice: voice.as_deref(),
        ledger: ledger.as_deref(),
        embassy: embassy.as_deref(),
        crown: crown.as_deref(),
        redemption: redemption.as_deref(),
        hybrid: hybrid.as_deref(),
        presence: presence.as_deref(),
        place,
    });
}
pub(crate) fn claim_wood_press(
    input: Option<Res<crate::input::PlayerInput>>,
    facts: Res<WoodFacts>,
    mut claim: ResMut<WoodClaim>,
    hub: Option<Res<WoodHub>>,
    door: Option<Res<crate::title_screen::LaunchDoor>>,
    house: Option<Res<crate::title_screen::HouseLabel>>,
) {
    claim.held = false;
    claim.prior = None;
    let (Some(input), Some(hub)) = (input.as_deref(), hub) else {
        return;
    };
    if !input.interact
        || facts.mercy_blocks
        || facts.door_blocks
        || !facts.in_reach
        || !facts.live
        || !yard_open(door.as_deref(), house.as_deref())
    {
        return;
    }
    let Ok(prior) = hub.claim_snap() else { return };
    claim.held = true;
    claim.prior = Some(prior);
}
pub(crate) fn admit_wood_press(
    claim: Res<WoodClaim>,
    mut hub: ResMut<WoodHub>,
    presence: Option<Res<crate::human_presence::SoftPresence>>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    realm: Option<Res<crate::living_practice_loop::SoftPlayerRealm>>,
    door: Option<Res<crate::title_screen::LaunchDoor>>,
    house: Option<Res<crate::title_screen::HouseLabel>>,
) {
    let Some(prior) = claim.prior.clone() else {
        return;
    };
    let place = travel.as_ref().map(|state| state.current);
    let shown = realm.as_ref().map(|state| state.current).unwrap_or(None);
    let position = presence.as_ref().map(|body| body.position);
    if !yard_open(door.as_deref(), house.as_deref())
        || !use_in_reach(Some(&hub), place, shown, position)
    {
        return;
    }
    let _ = hub.commit_at(&prior);
}
pub(crate) fn project_wood(
    mut commands: Commands,
    hub: Res<WoodHub>,
    mut view: ResMut<WoodView>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    realm: Option<Res<crate::living_practice_loop::SoftPlayerRealm>>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
    people: Query<Entity, With<crate::human_presence::HumanPresence>>,
    trees: Query<Entity, With<OwnedTree>>,
    bark: Query<Entity, With<BarkBit>>,
) {
    let doc = hub.view_doc();
    let unit = doc.as_ref().map(|doc| doc.unit).unwrap_or(0);
    let standing = doc
        .as_ref()
        .is_some_and(|doc| doc.stand == Stand::Standing && doc.unit == 0);
    view.unit = unit;
    view.standing = standing;
    let place = travel.as_ref().map(|state| state.current);
    let realm = realm.as_ref().map(|state| state.current).unwrap_or(None);
    let show_tree = standing && wood_live(place, realm);
    if show_tree {
        if trees.is_empty() {
            if let (Some(meshes), Some(materials), Some(site)) =
                (meshes.as_mut(), materials.as_mut(), first_passing_site())
            {
                let y = crate::vegetation::seated_ground(site.x, site.y);
                let bark_mat = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.29, 0.20, 0.13),
                    ..default()
                });
                let leaf = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.12, 0.34, 0.15),
                    ..default()
                });
                commands
                    .spawn((
                        Mesh3d(meshes.add(trunk_mesh())),
                        MeshMaterial3d(bark_mat),
                        Transform::from_xyz(site.x, y, site.y),
                        OwnedTree,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Mesh3d(meshes.add(canopy_mesh())),
                            MeshMaterial3d(leaf),
                            Transform::from_xyz(0.0, TRUNK_H * CANOPY_SIT, 0.0),
                        ));
                    });
            }
        }
    } else {
        for entity in trees.iter() {
            commands.entity(entity).despawn();
        }
    }
    if unit == 1 {
        if bark.is_empty() {
            if let (Some(meshes), Some(materials), Some(parent)) =
                (meshes.as_mut(), materials.as_mut(), people.iter().next())
            {
                let mat = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.29, 0.20, 0.13),
                    ..default()
                });
                commands.entity(parent).with_children(|child| {
                    child.spawn((
                        Mesh3d(meshes.add(bark_mesh())),
                        MeshMaterial3d(mat),
                        Transform::from_xyz(0.22, 0.55, 0.12),
                        BarkBit,
                    ));
                });
            }
        }
    } else {
        for entity in bark.iter() {
            commands.entity(entity).despawn();
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::sync::{Condvar, Mutex};
    use std::thread::{self, ThreadId};
    use std::time::{Duration, Instant};

    const SLOW: Duration = Duration::from_secs(2);
    #[derive(Clone)]
    enum Mode {
        Write,
        PreRename,
        Ambiguous(Vec<u8>),
    }
    struct Inner {
        primary: Mutex<Raw>,
        backup: Mutex<Raw>,
        mode: Mutex<Mode>,
        hold: Mutex<bool>,
        hold_cv: Condvar,
        after: Mutex<bool>,
        after_cv: Condvar,
        entered: AtomicUsize,
        publishes: Mutex<Vec<Vec<u8>>>,
        worker: Mutex<Option<ThreadId>>,
        off_worker: AtomicUsize,
    }
    #[derive(Clone)]
    struct Fake(Arc<Inner>);
    impl Fake {
        fn pair(primary: Raw, backup: Raw) -> Self {
            Self(Arc::new(Inner {
                primary: Mutex::new(primary),
                backup: Mutex::new(backup),
                mode: Mutex::new(Mode::Write),
                hold: Mutex::new(false),
                hold_cv: Condvar::new(),
                after: Mutex::new(false),
                after_cv: Condvar::new(),
                entered: AtomicUsize::new(0),
                publishes: Mutex::new(Vec::new()),
                worker: Mutex::new(None),
                off_worker: AtomicUsize::new(0),
            }))
        }
        fn touch(&self) {
            if *lock(&self.0.worker) != Some(thread::current().id()) {
                self.0.off_worker.fetch_add(1, Ordering::SeqCst);
            }
        }
        fn gate(flag: &Mutex<bool>, cv: &Condvar, block: bool) {
            *lock(flag) = block;
            if !block {
                cv.notify_all();
            }
        }
        fn wait(flag: &Mutex<bool>, cv: &Condvar) {
            let mut guard = lock(flag);
            while *guard {
                guard = cv.wait(guard).unwrap_or_else(|err| err.into_inner());
            }
        }
        fn block_before(&self) {
            Self::gate(&self.0.hold, &self.0.hold_cv, true);
        }
        fn unblock_before(&self) {
            Self::gate(&self.0.hold, &self.0.hold_cv, false);
        }
        fn block_after(&self) {
            Self::gate(&self.0.after, &self.0.after_cv, true);
        }
        fn unblock_after(&self) {
            Self::gate(&self.0.after, &self.0.after_cv, false);
        }
        fn set_mode(&self, mode: Mode) {
            *lock(&self.0.mode) = mode;
        }
        fn entered(&self) -> usize {
            self.0.entered.load(Ordering::SeqCst)
        }
        fn publishes(&self) -> Vec<Vec<u8>> {
            lock(&self.0.publishes).clone()
        }
        fn primary(&self) -> Raw {
            lock(&self.0.primary).clone()
        }
        fn backup(&self) -> Raw {
            lock(&self.0.backup).clone()
        }
        fn off_worker(&self) -> usize {
            self.0.off_worker.load(Ordering::SeqCst)
        }
    }
    impl Io for Fake {
        fn note_worker(&self) {
            *lock(&self.0.worker) = Some(thread::current().id());
        }
        fn read_live(&self) -> Raw {
            self.touch();
            self.primary()
        }
        fn read_backup(&self) -> Raw {
            self.touch();
            self.backup()
        }
        fn publish(&self, bytes: &[u8]) -> Result<(), Fail> {
            self.touch();
            self.0.entered.fetch_add(1, Ordering::SeqCst);
            Self::wait(&self.0.hold, &self.0.hold_cv);
            let mode = lock(&self.0.mode).clone();
            lock(&self.0.publishes).push(bytes.to_vec());
            let result = match mode {
                Mode::Write => {
                    *lock(&self.0.primary) = Raw::Bytes(bytes.to_vec());
                    Ok(())
                }
                Mode::PreRename => Err(Fail::PreRename),
                Mode::Ambiguous(found) => {
                    *lock(&self.0.primary) = Raw::Bytes(found);
                    Err(Fail::Ambiguous)
                }
            };
            Self::wait(&self.0.after, &self.0.after_cv);
            result
        }
    }
    fn sample(schema: u32, revision: u64, tree: u64, unit: u8) -> Doc {
        Doc {
            schema,
            revision,
            generation: 3,
            session: 8,
            op: 2,
            tree,
            stand: Stand::Depleted,
            unit,
            charge: Charge::Spent,
        }
    }
    fn edited(revision: u64, generation: u64) -> Doc {
        Doc {
            schema: SCHEMA,
            revision,
            generation,
            session: 11,
            op: revision,
            tree: TREE_ID,
            stand: Stand::Depleted,
            unit: 1,
            charge: Charge::Spent,
        }
    }
    fn until(mut ready: impl FnMut() -> bool) {
        let deadline = Instant::now() + SLOW;
        while !ready() {
            assert!(Instant::now() < deadline, "wait");
            thread::sleep(Duration::from_millis(1));
        }
    }
    fn started(fake: Fake, budget: Duration) -> WoodHub {
        let mut hub = WoodHub::spawn(fake, budget);
        until(|| {
            hub.poll();
            hub.body.booted
        });
        hub
    }
    pub(crate) fn open_hub() -> WoodHub {
        started(Fake::pair(Raw::NotFound, Raw::NotFound), SLOW)
    }
    pub(crate) fn tune_hub(
        hub: &mut WoodHub,
        booted: bool,
        inert: bool,
        conflict: bool,
        depleted: bool,
    ) {
        hub.body.booted = booted;
        hub.body.inert = inert;
        hub.body.blocked = inert || conflict;
        hub.body.conflict = conflict;
        if !booted {
            hub.body.recovery = None;
        } else if inert && !conflict {
            hub.body.recovery = Some(Recovery::Inert);
        } else {
            hub.body.recovery = Some(Recovery::Fresh);
        }
        if let Some(snap) = hub.body.memory.as_mut() {
            snap.doc.stand = if depleted {
                Stand::Depleted
            } else {
                Stand::Standing
            };
            snap.doc.unit = u8::from(depleted);
            snap.doc.charge = Charge::None;
        }
    }
    fn admit_now(hub: &mut WoodHub) -> Result<(), Reject> {
        let claimed = hub.body.memory.clone().ok_or(Reject::Writable)?;
        hub.commit_at(&claimed)
    }
    fn standing(session: u64, charge: Charge) -> Doc {
        Doc {
            schema: SCHEMA,
            revision: 2,
            generation: 5,
            session,
            op: 4,
            tree: TREE_ID,
            stand: Stand::Standing,
            unit: 0,
            charge,
        }
    }
    fn drive(hub: &mut WoodHub, mut ready: impl FnMut(&WoodHub) -> bool) {
        until(|| {
            hub.poll();
            ready(hub)
        });
    }

    #[test]
    fn resource_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<WoodHub>();
    }
    #[test]
    fn exit_wait_is_a_design_ceiling() {
        assert_eq!(EXIT_BUDGET, Duration::from_millis(250));
    }

    #[test]
    fn recover_classes() {
        let good_doc = sample(SCHEMA, 4, TREE_ID, 1);
        let good = canon(&good_doc).unwrap();
        let other = canon(&sample(SCHEMA, 4, TREE_ID, 0)).unwrap();
        let bad = b"{".to_vec();
        let foreign = canon(&sample(9, 4, 9, 1)).unwrap();
        let unit = canon(&sample(SCHEMA, 4, 9, 2)).unwrap();
        let mut extra: serde_json::Value = serde_json::from_slice(&good).unwrap();
        extra
            .as_object_mut()
            .unwrap()
            .insert("extra".into(), 1.into());
        let extra = serde_json::to_vec(&extra).unwrap();
        let session = 7u64;
        for (primary, backup) in [
            (Raw::Err, Raw::Bytes(good.clone())),
            (Raw::Bytes(bad.clone()), Raw::Bytes(good.clone())),
            (Raw::Bytes(foreign.clone()), Raw::Bytes(good.clone())),
            (Raw::Bytes(unit), Raw::Bytes(good.clone())),
            (Raw::Bytes(extra), Raw::Bytes(good.clone())),
            (Raw::NotFound, Raw::Err),
            (Raw::NotFound, Raw::Bytes(bad.clone())),
            (Raw::NotFound, Raw::Bytes(foreign)),
            (Raw::Bytes(bad.clone()), Raw::Bytes(bad)),
        ] {
            assert!(matches!(
                recover(primary, backup, session),
                Recovered::Inert
            ));
        }
        assert!(
            matches!(recover(Raw::NotFound, Raw::NotFound, session), Recovered::Fresh(doc) if doc == Doc::fresh(session))
        );
        assert!(
            matches!(recover(Raw::NotFound, Raw::Bytes(good.clone()), session), Recovered::Backup(doc) if doc == good_doc)
        );
        assert!(
            matches!(recover(Raw::Bytes(good.clone()), Raw::Bytes(other), session), Recovered::Primary(doc) if doc == good_doc)
        );
        assert!(
            matches!(recover(Raw::Bytes(good), Raw::Err, session), Recovered::Primary(doc) if doc == good_doc)
        );
        let mut ready = good_doc;
        ready.unit = 0;
        ready.charge = Charge::Ready;
        assert_eq!(parse_doc(&canon(&ready).unwrap()).unwrap(), ready);
        ready.charge = Charge::None;
        assert_eq!(parse_doc(&canon(&ready).unwrap()).unwrap(), ready);
    }

    #[test]
    fn invalid_evidence_bytes_stay() {
        let good = canon(&sample(SCHEMA, 4, 9, 1)).unwrap();
        let cases = [
            (Raw::Bytes(b"{}".to_vec()), Raw::Bytes(good.clone())),
            (Raw::Err, Raw::Bytes(good.clone())),
            (
                Raw::Bytes(canon(&sample(9, 1, 1, 1)).unwrap()),
                Raw::Bytes(good),
            ),
            (Raw::NotFound, Raw::Bytes(b"bak-bad".to_vec())),
            (Raw::Bytes(b"aa".to_vec()), Raw::Bytes(b"bb".to_vec())),
        ];
        for (primary, backup) in cases {
            let fake = Fake::pair(primary.clone(), backup.clone());
            let mut hub = started(fake.clone(), SLOW);
            assert!(hub.body.inert && hub.body.recovery == Some(Recovery::Inert));
            hub.submit(edited(4, 4));
            hub.shutdown();
            assert!(fake.publishes().is_empty());
            assert_eq!(
                (fake.primary(), fake.backup(), fake.off_worker()),
                (primary, backup, 0)
            );
        }
    }

    #[test]
    fn backup_restore_keeps_the_whole_document() {
        let doc = sample(SCHEMA, 6, TREE_ID, 1);
        let bytes = canon(&doc).unwrap();
        let fake = Fake::pair(Raw::NotFound, Raw::Bytes(bytes.clone()));
        let mut hub = started(fake.clone(), SLOW);
        assert_eq!(hub.body.memory.as_ref().unwrap().doc, doc);
        assert_eq!(
            (hub.body.recovery, hub.body.dirty),
            (Some(Recovery::Backup), false)
        );
        assert_eq!(
            (fake.primary(), fake.backup(), fake.publishes().len()),
            (Raw::NotFound, Raw::Bytes(bytes), 0)
        );
        hub.shutdown();
    }

    #[test]
    fn update_schedule_does_not_touch_disk() {
        let fake = Fake::pair(Raw::NotFound, Raw::NotFound);
        let mut app = App::new();
        app.insert_resource(WoodHub::spawn(fake.clone(), SLOW))
            .add_systems(Update, poll_wood);
        let end = Instant::now() + SLOW;
        loop {
            app.update();
            if app.world().resource::<WoodHub>().body.booted {
                break;
            }
            assert!(Instant::now() < end, "boot");
        }
        let mut doc = app
            .world()
            .resource::<WoodHub>()
            .body
            .memory
            .as_ref()
            .unwrap()
            .doc
            .clone();
        assert_eq!(doc, Doc::fresh(doc.session));
        doc.revision = 1;
        doc.op = 1;
        doc.generation = 3;
        app.world_mut()
            .resource_mut::<WoodHub>()
            .submit(doc.clone());
        while app.world().resource::<WoodHub>().body.confirmed.is_none() {
            app.update();
            assert!(Instant::now() < end, "pub");
        }
        assert_eq!(fake.off_worker(), 0);
        assert!(!app.world().resource::<WoodHub>().released);
        assert!(!app
            .world()
            .resource::<WoodHub>()
            .acked
            .load(Ordering::SeqCst));
        app.world_mut().resource_mut::<WoodHub>().shutdown();
        assert!(app.world().resource::<WoodHub>().released);
        assert!(app
            .world()
            .resource::<WoodHub>()
            .acked
            .load(Ordering::SeqCst));
        assert_eq!(fake.off_worker(), 0);
        assert_eq!(fake.primary(), Raw::Bytes(canon(&doc).unwrap()));
    }

    #[test]
    fn dropped_handle_is_not_stop() {
        let hub = started(Fake::pair(Raw::NotFound, Raw::NotFound), SLOW);
        let acked = Arc::clone(&hub.acked);
        drop(hub);
        let end = Instant::now() + Duration::from_millis(150);
        while Instant::now() < end {
            assert!(!acked.load(Ordering::SeqCst));
            thread::sleep(Duration::from_millis(10));
        }
    }

    #[test]
    fn stale_completion_keeps_newer_dirty() {
        let mut hub = started(Fake::pair(Raw::NotFound, Raw::NotFound), SLOW);
        let base = hub.body.memory.clone().unwrap();
        let older = snap_from(edited(1, 1)).unwrap();
        hub.body.inflight = Some(older.clone());
        hub.body.memory = snap_from(edited(2, 8));
        hub.body.confirmed = Some(base);
        hub.body.dirty = true;
        let mut gen = older.id;
        gen.generation = 99;
        let mut session = older.id;
        session.session = 99;
        let mut content = older.id;
        content.content ^= 1;
        for id in [gen, session, content] {
            hub.apply(Note::Done { id, result: Ok(()) });
            assert!(hub.body.dirty && hub.body.inflight.is_some());
            assert_eq!(hub.body.confirmed.as_ref().unwrap().doc.revision, 0);
        }
        hub.apply(Note::Done {
            id: older.id,
            result: Ok(()),
        });
        assert!(hub.body.dirty);
        assert_eq!(hub.body.memory.as_ref().unwrap().doc.revision, 2);
        assert_eq!(hub.body.confirmed.as_ref().unwrap().doc.revision, 1);
        hub.shutdown();
    }

    #[test]
    fn same_revision_conflict_and_stale_submit() {
        let fake = Fake::pair(Raw::NotFound, Raw::NotFound);
        let mut hub = started(fake.clone(), SLOW);
        let first = edited(2, 5);
        hub.submit(first.clone());
        drive(&mut hub, |hub| hub.body.confirmed.is_some());
        let mut older = first.clone();
        older.revision = 1;
        older.op = 1;
        older.generation = 9;
        hub.submit(older);
        assert!(!hub.body.conflict && hub.body.memory.as_ref().unwrap().doc.generation == 5);
        let mut clash = first.clone();
        clash.generation = 6;
        hub.submit(clash);
        assert!(hub.body.conflict && hub.body.memory.as_ref().unwrap().doc.generation == 5);
        assert_eq!(fake.publishes(), vec![canon(&first).unwrap()]);
        assert_eq!(fake.primary(), Raw::Bytes(canon(&first).unwrap()));
        hub.shutdown();
    }

    #[test]
    fn ambiguous_match_confirms_and_conflict_blocks() {
        let match_doc = edited(3, 3);
        let match_bytes = canon(&match_doc).unwrap();
        let fake = Fake::pair(Raw::NotFound, Raw::NotFound);
        fake.set_mode(Mode::Ambiguous(match_bytes.clone()));
        let mut hub = started(fake.clone(), SLOW);
        hub.submit(match_doc);
        drive(&mut hub, |hub| hub.body.confirmed.is_some());
        assert!(
            !hub.body.inert
                && fake.primary() == Raw::Bytes(match_bytes)
                && fake.publishes().len() == 1
        );
        hub.shutdown();

        let doc = edited(3, 3);
        let mut other = doc.clone();
        other.generation = 99;
        let other_bytes = canon(&other).unwrap();
        let fake = Fake::pair(Raw::NotFound, Raw::NotFound);
        fake.set_mode(Mode::Ambiguous(other_bytes.clone()));
        let mut hub = started(fake.clone(), SLOW);
        hub.submit(doc);
        drive(&mut hub, |hub| hub.body.inert);
        assert!(hub.body.conflict && hub.body.blocked && hub.body.confirmed.is_none());
        assert_eq!(fake.primary(), Raw::Bytes(other_bytes));
        hub.submit(edited(4, 4));
        hub.shutdown();
        assert_eq!(fake.publishes().len(), 1);
    }

    #[test]
    fn latest_unwritten_and_failed_inflight_order() {
        let fake = Fake::pair(Raw::NotFound, Raw::NotFound);
        let mut hub = started(fake.clone(), SLOW);
        let saved = edited(1, 1);
        hub.submit(saved.clone());
        drive(&mut hub, |hub| hub.body.confirmed.is_some());
        let failed = edited(2, 2);
        let skipped = edited(3, 3);
        let newest = edited(4, 4);
        fake.block_before();
        hub.submit(failed.clone());
        until(|| fake.entered() >= 2);
        hub.submit(skipped.clone());
        hub.submit(newest.clone());
        assert_eq!(
            (
                hub.body.inflight.as_ref().unwrap().doc.revision,
                hub.body.queued.as_ref().unwrap().doc.revision
            ),
            (2, 4)
        );
        fake.set_mode(Mode::PreRename);
        fake.unblock_before();
        until(|| fake.publishes().len() == 2);
        assert_eq!(fake.primary(), Raw::Bytes(canon(&saved).unwrap()));
        assert_eq!(hub.body.confirmed.as_ref().unwrap().doc.revision, 1);
        fake.set_mode(Mode::Write);
        drive(&mut hub, |hub| {
            hub.body.confirmed.as_ref().map(|s| s.doc.revision) == Some(4)
        });
        let pubs = fake.publishes();
        assert_eq!(
            pubs,
            vec![
                canon(&saved).unwrap(),
                canon(&failed).unwrap(),
                canon(&newest).unwrap()
            ]
        );
        assert!(!pubs.contains(&canon(&skipped).unwrap()));
        assert_eq!(fake.primary(), Raw::Bytes(canon(&newest).unwrap()));
        assert_eq!(hub.body.memory.as_ref().unwrap().doc, newest);
        assert!(!hub.body.dirty);
        hub.shutdown();
    }

    #[test]
    fn stop_budget_leaves_old_or_new_without_a_result() {
        let old = edited(1, 1);
        let old_bytes = canon(&old).unwrap();
        let fake = Fake::pair(Raw::Bytes(old_bytes.clone()), Raw::NotFound);
        let mut hub = started(fake.clone(), Duration::from_millis(40));
        fake.block_before();
        let attempted = edited(2, 2);
        hub.submit(attempted.clone());
        until(|| fake.entered() >= 1);
        hub.shutdown();
        assert!(!hub.released && !hub.acked.load(Ordering::SeqCst) && lock(&hub.join).is_some());
        let live = fake.primary();
        assert!(
            live == Raw::Bytes(old_bytes.clone()) || live == Raw::Bytes(canon(&attempted).unwrap())
        );
        assert_eq!(fake.primary(), Raw::Bytes(old_bytes));
        assert!(hub.body.confirmed.is_none() && fake.publishes().is_empty());
        fake.unblock_before();
        hub.join_after_stop();

        let fake = Fake::pair(Raw::NotFound, Raw::NotFound);
        let mut hub = started(fake.clone(), Duration::from_millis(40));
        let doc = edited(2, 2);
        let new_bytes = canon(&doc).unwrap();
        fake.block_after();
        hub.submit(doc.clone());
        until(|| fake.publishes().len() == 1);
        hub.shutdown();
        assert!(!hub.released && !hub.acked.load(Ordering::SeqCst) && lock(&hub.join).is_some());
        let live = fake.primary();
        assert!(live == Raw::NotFound || live == Raw::Bytes(new_bytes.clone()));
        assert_eq!(fake.primary(), Raw::Bytes(new_bytes));
        assert!(hub.body.confirmed.is_none() && hub.body.memory.as_ref().unwrap().doc == doc);
        assert_eq!(fake.publishes().len(), 1);
        fake.unblock_after();
        hub.join_after_stop();
        assert_eq!(fake.publishes().len(), 1);
    }

    fn witness(stand: Stand, unit: u8, charge: Charge, tree: u64) -> Doc {
        Doc {
            schema: SCHEMA,
            revision: 1,
            generation: 1,
            session: 4,
            op: 1,
            tree,
            stand,
            unit,
            charge,
        }
    }

    #[test]
    fn invalid_domain_witnesses_are_inert_and_unpublished() {
        let cases = [
            witness(Stand::Standing, 1, Charge::None, TREE_ID),
            witness(Stand::Standing, 0, Charge::Ready, TREE_ID),
            witness(Stand::Depleted, 1, Charge::Ready, TREE_ID),
            witness(Stand::Depleted, 1, Charge::Spent, TREE_ID + 1),
        ];
        for doc in cases {
            let bytes = canon(&doc).unwrap();
            assert!(parse_doc(&bytes).is_none());
            let fake = Fake::pair(Raw::Bytes(bytes.clone()), Raw::NotFound);
            let mut hub = started(fake.clone(), SLOW);
            assert!(hub.body.inert && hub.body.recovery == Some(Recovery::Inert));
            hub.shutdown();
            assert!(fake.publishes().is_empty() && fake.primary() == Raw::Bytes(bytes));
            let fake = Fake::pair(Raw::NotFound, Raw::NotFound);
            let mut hub = started(fake.clone(), SLOW);
            let before = hub.body.memory.clone();
            hub.submit(doc);
            assert_eq!(hub.body.memory, before);
            hub.shutdown();
            assert!(fake.publishes().is_empty());
        }
    }

    #[test]
    fn boot_then_exit_publishes_nothing_without_a_submission() {
        let primary = canon(&edited(4, 1)).unwrap();
        let backup = canon(&edited(6, 1)).unwrap();
        let cases = [
            (Raw::NotFound, Raw::NotFound, Recovery::Fresh),
            (
                Raw::Bytes(primary.clone()),
                Raw::NotFound,
                Recovery::Primary,
            ),
            (Raw::NotFound, Raw::Bytes(backup.clone()), Recovery::Backup),
        ];
        for (live, bak, recovery) in cases {
            let fake = Fake::pair(live.clone(), bak.clone());
            let mut hub = started(fake.clone(), EXIT_BUDGET);
            assert_eq!(hub.body.recovery, Some(recovery));
            assert!(!hub.body.dirty && hub.body.inflight.is_none());
            hub.shutdown();
            assert!(hub.released && fake.publishes().is_empty());
            assert_eq!((fake.primary(), fake.backup()), (live, bak));
        }
    }

    #[test]
    fn exit_publishes_inflight_then_queued_then_stops() {
        let fake = Fake::pair(Raw::NotFound, Raw::NotFound);
        let mut hub = started(fake.clone(), EXIT_BUDGET);
        let first = edited(1, 1);
        let second = edited(2, 2);
        fake.block_before();
        hub.submit(first.clone());
        until(|| fake.entered() >= 1);
        hub.submit(second.clone());
        assert_eq!(hub.body.queued.as_ref().unwrap().doc.revision, 2);
        fake.unblock_before();
        hub.shutdown();
        assert!(hub.released);
        assert_eq!(
            fake.publishes(),
            vec![canon(&first).unwrap(), canon(&second).unwrap()]
        );
    }

    #[test]
    fn exit_publishes_dirty_memory_left_by_a_prerename_failure() {
        let fake = Fake::pair(Raw::NotFound, Raw::NotFound);
        let mut hub = started(fake.clone(), EXIT_BUDGET);
        let doc = edited(1, 1);
        fake.set_mode(Mode::PreRename);
        hub.submit(doc.clone());
        drive(&mut hub, |hub| {
            hub.body.inflight.is_none() && hub.body.dirty
        });
        assert_eq!(fake.publishes().len(), 1);
        fake.set_mode(Mode::Write);
        hub.shutdown();
        assert!(hub.released && !hub.body.dirty);
        assert_eq!(
            fake.publishes(),
            vec![canon(&doc).unwrap(), canon(&doc).unwrap()]
        );
    }

    #[test]
    fn exit_deadline_leaves_a_queued_latest_unpublished() {
        let old = edited(1, 1);
        let old_bytes = canon(&old).unwrap();
        let fake = Fake::pair(Raw::Bytes(old_bytes.clone()), Raw::NotFound);
        let mut hub = started(fake.clone(), Duration::from_millis(40));
        fake.block_before();
        hub.submit(edited(2, 2));
        until(|| fake.entered() >= 1);
        hub.submit(edited(3, 3));
        hub.shutdown();
        assert!(!hub.released && lock(&hub.join).is_some());
        assert!(fake.publishes().is_empty());
        assert_eq!(fake.primary(), Raw::Bytes(old_bytes));
        fake.unblock_before();
        hub.join_after_stop();
        assert!(hub.released && fake.publishes().len() == 1);
    }

    fn failures(site: Vec2) -> Vec<&'static str> {
        site_flags(site)
            .into_iter()
            .filter(|(_, ok)| !ok)
            .map(|(name, _)| name)
            .collect()
    }

    #[test]
    fn first_ordered_site_passes_and_later_rows_do_not() {
        let (cam, projection) = boot_camera();
        assert_eq!(cam.translation.xz(), crate::vegetation::CAMERA_XZ);
        assert!(cam.forward().as_vec3().dot(-cam.translation.normalize()) > 0.999);
        assert_eq!(projection.fov, std::f32::consts::FRAC_PI_4);
        assert!((TRUNK_R + BODY_R - 0.41).abs() < 1.0e-5 && IN_TOL <= 1.0e-4 && OUT_TOL >= 1.0e-3);
        let ecology = include_str!("living_ecology.rs");
        let mercy = include_str!("mercy_harvest_nodes.rs");
        assert!(
            ecology.contains("const DEER_FAR: Vec3 = Vec3::new(8.5, 0.55, 6.2)")
                && ecology.contains("Vec3::new(4.2, 0.22, -5.5)")
        );
        assert!(
            ecology.contains("Sphere::new(0.42)")
                && ecology.contains("radius: 1.05")
                && ecology.contains("Cylinder::new(0.035, 1.0)")
        );
        assert!(
            ecology.contains("Capsule3d::new(0.16, 0.55)")
                && ecology.contains("Cylinder::new(0.16, 3.6)")
                && mercy.contains("Sphere::new(0.48)")
        );
        assert!(
            mercy.contains("Vec3::new(3.6, 0.55, 0.0)")
                && include_str!("main.rs").contains(
                    "Transform::from_xyz(0.0, 8.0, 14.0).looking_at(Vec3::ZERO, Vec3::Y)"
                )
        );
        let chosen = first_passing_site()
            .unwrap_or_else(|| panic!("HOLD {:?}", SITES.map(|site| (site, failures(site)))));
        assert_eq!(
            chosen,
            SITES[0],
            "{:?}",
            SITES.map(|site| (site, failures(site)))
        );
        assert!(
            failures(SITES[0]).is_empty() && failures(SITES[1]).is_empty(),
            "{:?}",
            failures(SITES[1])
        );
        assert_eq!(
            failures(SITES[2]),
            vec!["frustum"],
            "{:?}",
            failures(SITES[2])
        );
        assert_eq!(
            failures(SITES[3]),
            vec!["frustum"],
            "{:?}",
            failures(SITES[3])
        );
        assert!(failures(Vec2::ZERO).contains(&"frustum"));
        let mut hub = open_hub();
        let place = Some(PlaceId::Sanctuary);
        for dir in [Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y] {
            let inn = chosen + dir * stance_offset();
            let out = chosen + dir * (TRUNK_R + BODY_R + OUT_TOL);
            assert!(
                contact_inside(inn.distance(chosen)) && !contact_inside(out.distance(chosen)),
                "{dir:?}"
            );
            assert!(use_headroom(inn) > 0.0, "{}", use_headroom(inn));
            assert!(use_in_reach(
                Some(&hub),
                place,
                Some(0),
                Some(Vec3::new(inn.x, 0.9, inn.y))
            ));
            for rim in [Vec2::ZERO, Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y] {
                let at = inn + rim * BODY_R;
                assert!(
                    crate::terrain::play_footprint(at.x, at.y)
                        && crate::terrain::shelf_offset(at.x, at.y) == 0.0
                );
            }
        }
        let edge = chosen + Vec2::X * (TRUNK_R + BODY_R + IN_TOL);
        assert!(use_in_reach(
            Some(&hub),
            place,
            Some(0),
            Some(Vec3::new(edge.x, 0.9, edge.y))
        ));
        let outside = chosen + Vec2::X * (TRUNK_R + BODY_R + OUT_TOL);
        assert!(!use_in_reach(
            Some(&hub),
            place,
            Some(0),
            Some(Vec3::new(outside.x, 0.9, outside.y))
        ));
        let meter = chosen + Vec2::X;
        assert!(!use_in_reach(
            Some(&hub),
            place,
            Some(0),
            Some(Vec3::new(meter.x, 0.9, meter.y))
        ));
        assert!(!use_in_reach(
            Some(&hub),
            Some(PlaceId::Heartwood),
            Some(0),
            Some(Vec3::new(chosen.x, 0.9, chosen.y))
        ));
        assert!(owned_blocker(Some(&hub), place, Some(4)).is_none());
        tune_hub(&mut hub, true, false, false, true);
        assert!(!use_in_reach(
            Some(&hub),
            place,
            Some(0),
            Some(Vec3::new(chosen.x, 0.9, chosen.y))
        ));
        assert!(owned_blocker(Some(&hub), place, Some(0)).is_none());
        hub.shutdown();
    }

    fn ring(mesh: Mesh) -> (f32, f32) {
        let Some(bevy::mesh::VertexAttributeValues::Float32x3(pos)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION).cloned()
        else {
            panic!("positions")
        };
        let (mut top, mut base) = (0.0_f32, 0.0_f32);
        for p in pos {
            assert!(p[1] >= -1e-5);
            top = top.max(p[1]);
            if p[1].abs() <= 1e-5 {
                base = base.max((p[0] * p[0] + p[2] * p[2]).sqrt());
            }
        }
        (base, top)
    }
    #[test]
    fn primitive_meshes_keep_the_authored_extents() {
        let (base, top) = ring(trunk_mesh());
        assert!(
            (base - TRUNK_R).abs() < 1e-4 && (top - TRUNK_H).abs() < 1e-4,
            "{base} {top}"
        );
        let (base, top) = ring(canopy_mesh());
        assert!((base - CANOPY_R).abs() < 1e-4 && (top - CANOPY_H).abs() < 1e-4);
        let (base, top) = ring(bark_mesh());
        assert!((base - 0.04).abs() < 1e-4 && (top - 0.18).abs() < 1e-4);
        let site = first_passing_site().unwrap();
        let y = crate::vegetation::seated_ground(site.x, site.y);
        assert!((y + TRUNK_H * CANOPY_SIT - (y + 1.35 * 0.78)).abs() < 1e-5);
    }
    fn boot_pair(primary: Raw, backup: Raw) -> (WoodHub, Fake) {
        let fake = Fake::pair(primary, backup);
        (started(fake.clone(), SLOW), fake)
    }
    #[test]
    fn harvest_keeps_recovered_session_for_each_class() {
        let session = 42u64;
        let bytes = canon(&standing(session, Charge::None)).unwrap();
        let spent = canon(&standing(session, Charge::Spent)).unwrap();
        let (mut fresh, _) = boot_pair(Raw::NotFound, Raw::NotFound);
        let kept = fresh.body.memory.as_ref().unwrap().doc.session;
        let prior = fresh.body.memory.as_ref().unwrap().doc.clone();
        assert_eq!(fresh.body.recovery, Some(Recovery::Fresh));
        admit_now(&mut fresh).unwrap();
        let doc = &fresh.body.memory.as_ref().unwrap().doc;
        assert_eq!(
            (
                doc.session,
                doc.schema,
                doc.tree,
                doc.stand,
                doc.unit,
                doc.charge
            ),
            (kept, SCHEMA, TREE_ID, Stand::Depleted, 1, Charge::None)
        );
        assert_eq!(
            (doc.revision, doc.generation, doc.op, admit_now(&mut fresh)),
            (
                prior.revision + 1,
                prior.generation + 1,
                prior.op + 1,
                Err(Reject::Identity)
            )
        );
        assert_eq!(fresh.body.memory.as_ref().unwrap().doc.unit, 1);
        fresh.shutdown();
        for (primary, backup, class) in [
            (Raw::Bytes(bytes.clone()), Raw::NotFound, Recovery::Primary),
            (Raw::NotFound, Raw::Bytes(bytes), Recovery::Backup),
            (Raw::Bytes(spent), Raw::NotFound, Recovery::Primary),
        ] {
            let (mut hub, fake) = boot_pair(primary, backup);
            assert_eq!(hub.body.recovery, Some(class));
            let before = hub.body.memory.as_ref().unwrap().doc.clone();
            admit_now(&mut hub).unwrap();
            drive(&mut hub, |hub| {
                hub.body.confirmed.as_ref().map(|s| s.doc.unit) == Some(1)
            });
            let doc = &hub.body.memory.as_ref().unwrap().doc;
            assert_eq!(
                (doc.session, doc.charge, doc.revision),
                (session, Charge::None, before.revision + 1)
            );
            let Raw::Bytes(saved) = fake.primary() else {
                panic!("{:?}", fake.primary())
            };
            hub.shutdown();
            let (mut again, _) = boot_pair(Raw::Bytes(saved), Raw::NotFound);
            assert_eq!(
                (
                    again.body.recovery,
                    again
                        .body
                        .memory
                        .as_ref()
                        .map(|s| (s.doc.session, s.doc.unit))
                ),
                (Some(Recovery::Primary), Some((session, 1)))
            );
            again.shutdown();
        }
    }
    #[test]
    fn overflow_preview_and_publish_failure_leave_memory() {
        let mut cold = WoodHub::spawn(Fake::pair(Raw::NotFound, Raw::NotFound), SLOW);
        assert_eq!(admit_now(&mut cold), Err(Reject::Writable));
        assert!(cold.body.memory.is_none() && cold.body.inflight.is_none());
        cold.shutdown();
        let (mut hub, fake) = boot_pair(Raw::NotFound, Raw::NotFound);
        let before = hub.body.memory.clone();
        hub.body.inert = true;
        assert_eq!(admit_now(&mut hub), Err(Reject::Writable));
        hub.body.inert = false;
        hub.body.memory.as_mut().unwrap().doc.charge = Charge::Ready;
        assert_eq!(admit_now(&mut hub), Err(Reject::Ready));
        hub.body.memory.as_mut().unwrap().doc.charge = Charge::None;
        hub.body.memory.as_mut().unwrap().doc.tree = 9;
        assert_eq!(admit_now(&mut hub), Err(Reject::Identity));
        hub.body.memory.as_mut().unwrap().doc.tree = TREE_ID;
        for set in [
            |d: &mut Doc| d.revision = u64::MAX,
            |d: &mut Doc| d.generation = u64::MAX,
            |d: &mut Doc| d.op = u64::MAX,
        ] {
            let saved = hub.body.memory.as_ref().unwrap().doc.clone();
            set(&mut hub.body.memory.as_mut().unwrap().doc);
            assert_eq!(admit_now(&mut hub), Err(Reject::Overflow));
            hub.body.memory.as_mut().unwrap().doc = saved;
        }
        assert_eq!(hub.body.memory.as_ref().map(|s| s.doc.unit), Some(0));
        assert!(fake.publishes().is_empty());
        let prior = hub.body.memory.clone().unwrap();
        hub.body.inflight = Some(prior.clone());
        admit_now(&mut hub).unwrap();
        assert_eq!(
            (
                hub.body.memory.as_ref().unwrap().doc.unit,
                hub.body.queued.as_ref().unwrap().doc.unit,
                hub.body.inflight.as_ref().unwrap().doc.unit
            ),
            (1, 1, 0)
        );
        hub.body.memory = Some(prior);
        hub.body.queued = None;
        hub.body.inflight = None;
        hub.shutdown();
        assert_eq!(admit_now(&mut hub), Err(Reject::Publish));
        assert_eq!(hub.body.memory.as_ref().unwrap().doc, before.unwrap().doc);
        assert!(hub.body.inflight.is_none() && hub.body.queued.is_none());
    }
    #[test]
    fn ambiguous_match_confirms_and_a_stale_done_does_not() {
        let (mut hub, fake) = boot_pair(Raw::NotFound, Raw::NotFound);
        let next = successor(&hub.body.memory.as_ref().unwrap().doc).unwrap();
        let bytes = canon(&next).unwrap();
        fake.set_mode(Mode::Ambiguous(bytes.clone()));
        admit_now(&mut hub).unwrap();
        let id = hub.body.inflight.as_ref().unwrap().id;
        let mut stale = id;
        stale.op = stale.op.wrapping_add(3);
        hub.apply(Note::Done {
            id: stale,
            result: Ok(()),
        });
        assert!(hub.body.inflight.is_some() && hub.body.confirmed.is_none());
        drive(&mut hub, |hub| hub.body.confirmed.is_some());
        assert_eq!(hub.body.confirmed.as_ref().unwrap().bytes, bytes);
        assert_eq!(hub.body.memory.as_ref().unwrap().doc.unit, 1);
        hub.shutdown();
        let (mut hub, fake) = boot_pair(Raw::NotFound, Raw::NotFound);
        let mut other = successor(&hub.body.memory.as_ref().unwrap().doc).unwrap();
        other.session = other.session.wrapping_add(1);
        fake.set_mode(Mode::Ambiguous(canon(&other).unwrap()));
        admit_now(&mut hub).unwrap();
        drive(&mut hub, |hub| hub.body.inert);
        assert!(hub.body.conflict && hub.body.blocked && !hub.can_write());
        let seen = hub.view_doc().expect("accepted memory stays readable");
        assert_eq!(
            (seen.unit, seen.stand, seen.charge),
            (1, Stand::Depleted, Charge::None)
        );
        assert_eq!(admit_now(&mut hub), Err(Reject::Writable));
        let site = first_passing_site().unwrap();
        assert!(owned_blocker(Some(&hub), Some(PlaceId::Sanctuary), Some(0)).is_none());
        assert!(!use_in_reach(
            Some(&hub),
            Some(PlaceId::Sanctuary),
            Some(0),
            Some(Vec3::new(site.x, 0.9, site.y))
        ));
        hub.shutdown();
    }
    #[test]
    fn mercy_facts_follow_node_distance() {
        let reach = crate::mercy_harvest_nodes::HARVEST_REACH;
        assert!(!mercy_blocks(Some(Vec3::ZERO), std::iter::empty()));
        assert!(mercy_blocks(Some(Vec3::ZERO), [Vec3::new(reach, 0.0, 0.0)]));
        assert!(!mercy_blocks(
            Some(Vec3::ZERO),
            [Vec3::new(reach + 1.0e-3, 0.0, 0.0)]
        ));
        assert!(mercy_blocks(
            Some(Vec3::ZERO),
            [Vec3::new(reach + 4.0, 0.0, 0.0), Vec3::new(0.2, 0.0, 0.0)]
        ));
        let mut epi = crate::first_harvest_epiphany::FirstHarvestEpiphany::default();
        let mut hour = crate::hour_sacred::HourSacred::default();
        epi.peace_visitor = true;
        assert!(epi.peace_visitor);
        assert!(!door_blocks(DoorSrc {
            hour: Some(&hour),
            place: Some(PlaceId::Sanctuary),
            ..DoorSrc::bare()
        }));
        hour.session.hex = shared::space_law::HexFlag::Frontier;
        assert!(hour.session.peace_visitor_on_frontier());
        assert!(door_blocks(DoorSrc {
            hour: Some(&hour),
            place: Some(PlaceId::Sanctuary),
            ..DoorSrc::bare()
        }));
        let post = WARD_POST_CENTERS[0];
        let presence = crate::human_presence::SoftPresence {
            position: Vec3::new(post[0], post[1], post[2]),
            ..Default::default()
        };
        assert!(door_blocks(DoorSrc {
            presence: Some(&presence),
            place: Some(PlaceId::Heartwood),
            ..DoorSrc::bare()
        }));
        assert!(!door_blocks(DoorSrc {
            presence: Some(&presence),
            place: Some(PlaceId::Sanctuary),
            ..DoorSrc::bare()
        }));
        let (x, _, z) = shared::skirmish_well::WELL_ANCHORS[0];
        let at_well = crate::human_presence::SoftPresence {
            position: Vec3::new(x, 0.9, z),
            ..Default::default()
        };
        assert!(door_blocks(DoorSrc {
            presence: Some(&at_well),
            place: Some(PlaceId::Sanctuary),
            ..DoorSrc::bare()
        }));
    }
    #[derive(Resource, Default, Clone, Copy)]
    struct YieldSeen {
        memory: Option<u8>,
        view: u8,
        held: bool,
    }
    fn spy_yield(
        hub: Res<WoodHub>,
        claim: Res<WoodClaim>,
        view: Res<WoodView>,
        mut seen: ResMut<YieldSeen>,
    ) {
        seen.memory = hub.memory_unit();
        seen.view = view.unit;
        seen.held = claim.blocks();
    }
    fn schedule_app(hub: WoodHub) -> App {
        use crate::input::PlayerInput;
        let site = first_passing_site().unwrap();
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .insert_resource(hub)
            .init_resource::<WoodClaim>()
            .init_resource::<WoodFacts>()
            .init_resource::<WoodView>()
            .init_resource::<YieldSeen>()
            .init_resource::<PlayerInput>()
            .init_resource::<crate::feel_move::UseBuffer>()
            .init_resource::<crate::feel_move::SimUsePulse>()
            .init_resource::<crate::first_harvest_epiphany::FirstHarvestEpiphany>()
            .init_resource::<crate::first_session_guidance::FirstSessionGuidance>()
            .init_resource::<crate::thriving_moments::ThrivingMoments>()
            .init_resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>()
            .init_resource::<crate::mercy_harvest_nodes::NearbyMercyNode>()
            .init_resource::<crate::harvest_feel::SoftRbePool>()
            .init_resource::<crate::world_answer::WorldAnswer>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<crate::hour_sacred::HourSacred>()
            .init_resource::<crate::heartwood_wards::WardSession>()
            .init_resource::<crate::living_practice_loop::LivingPracticeLoop>()
            .insert_resource(crate::hex_travel::HexTravelState {
                current: PlaceId::Sanctuary,
            })
            .insert_resource(crate::living_practice_loop::SoftPlayerRealm { current: Some(0) })
            .insert_resource(crate::human_presence::SoftPresence {
                position: Vec3::new(site.x, 0.9, site.y),
                ..Default::default()
            })
            .add_message::<bevy::input::gamepad::GamepadRumbleRequest>();
        app.world_mut()
            .resource_mut::<crate::living_practice_loop::LivingPracticeLoop>()
            .active = true;
        add_wood_systems(&mut app);
        app.add_systems(
            Update,
            (
                crate::feel_move::raise_buffered_use,
                crate::feel_move::arm_use_buffer_from_edge,
            )
                .chain(),
        );
        app.add_systems(
            Update,
            (
                crate::first_harvest_epiphany::mark_peace_visitor,
                crate::first_harvest_epiphany::mark_epiphany_place,
                crate::first_harvest_epiphany::handle_interact_harvest,
                crate::heartwood_wards::use_heartwood_wards,
                crate::living_practice_loop::soft_interact_harvest_credit,
                spy_yield
                    .after(crate::first_harvest_epiphany::handle_interact_harvest)
                    .after(crate::heartwood_wards::use_heartwood_wards)
                    .after(crate::living_practice_loop::soft_interact_harvest_credit)
                    .before(admit_wood_press),
            ),
        );
        app
    }
    fn press(app: &mut App) {
        app.world_mut()
            .resource_mut::<crate::feel_move::SimUsePulse>()
            .fire = true;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(crate::soft_play_bindings::INTERACT);
        app.world_mut()
            .resource_mut::<crate::input::PlayerInput>()
            .interact = true;
    }
    fn ord(app: &mut App, id: std::any::TypeId) -> usize {
        use bevy::ecs::system::System;
        app.world_mut()
            .resource_mut::<bevy::ecs::schedule::Schedules>()
            .get_mut(Update)
            .unwrap()
            .systems()
            .unwrap()
            .position(|(_, system)| System::system_type(&**system) == id)
            .expect("missing")
    }
    #[test]
    fn buffered_press_admits_only_after_both_consumers_yield() {
        let mut app = schedule_app(open_hub());
        app.update();
        assert!(
            app.world().resource::<WoodView>().standing
                && app.world().resource::<WoodView>().unit == 0
        );
        press(&mut app);
        app.update();
        let seen = app.world().resource::<YieldSeen>().clone();
        let epi = app
            .world()
            .resource::<crate::first_harvest_epiphany::FirstHarvestEpiphany>();
        assert_eq!(
            (
                seen.memory,
                seen.view,
                seen.held,
                app.world().resource::<WoodHub>().memory_unit()
            ),
            (Some(0), 0, true, Some(1))
        );
        assert_eq!(
            (
                app.world().resource::<WoodView>().unit,
                app.world().resource::<WoodView>().standing,
                epi.harvests_this_session,
                epi.tends_this_session
            ),
            (1, false, 0, 0)
        );
        assert_eq!(
            app.world()
                .resource::<crate::living_practice_loop::LivingPracticeLoop>()
                .mercy_harvests_on_surface,
            0
        );
        assert_eq!(
            app.world()
                .resource::<crate::heartwood_wards::WardSession>()
                .dress
                .tends,
            0
        );
        app.world_mut()
            .resource_mut::<crate::input::PlayerInput>()
            .interact = false;
        app.update();
        assert!(
            !app.world().resource::<WoodClaim>().blocks()
                && app.world().resource::<WoodHub>().memory_unit() == Some(1)
        );
        app.world_mut().resource_mut::<WoodHub>().shutdown();
    }
    #[test]
    fn admit_failure_consumes_the_frame_and_resets() {
        let mut app = schedule_app(open_hub());
        app.update();
        app.world_mut().resource_mut::<WoodHub>().shutdown();
        app.world_mut()
            .resource_mut::<crate::feel_move::SimUsePulse>()
            .fire = true;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(crate::soft_play_bindings::INTERACT);
        app.update();
        let seen = app.world().resource::<YieldSeen>().clone();
        assert!(seen.held && seen.memory == Some(0) && seen.view == 0);
        assert_eq!(app.world().resource::<WoodHub>().memory_unit(), Some(0));
        assert!(
            app.world().resource::<WoodView>().standing
                && app.world().resource::<WoodView>().unit == 0
        );
        assert_eq!(
            app.world()
                .resource::<crate::first_harvest_epiphany::FirstHarvestEpiphany>()
                .harvests_this_session,
            0
        );
        assert_eq!(
            app.world()
                .resource::<crate::living_practice_loop::LivingPracticeLoop>()
                .mercy_harvests_on_surface,
            0
        );
        app.world_mut()
            .resource_mut::<crate::input::PlayerInput>()
            .interact = false;
        app.update();
        assert!(!app.world().resource::<WoodClaim>().blocks());
        assert_eq!(app.world().resource::<WoodHub>().memory_unit(), Some(0));
        assert_eq!(
            app.world()
                .resource::<crate::first_harvest_epiphany::FirstHarvestEpiphany>()
                .harvests_this_session,
            0
        );
    }
    #[test]
    fn plugin_reserves_the_passing_clearance_before_update() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_plugins(WoodExpeditionPlugin);
        let site = first_passing_site().unwrap();
        let disk = app.world().resource::<WoodClearance>();
        assert_eq!(disk.center, site);
        assert!((disk.radius - enclosing_radius()).abs() < 1e-6);
        app.world_mut().resource_mut::<WoodHub>().shutdown();
    }
    #[test]
    fn inert_boot_draws_nothing() {
        let (mut hub, _) = boot_pair(Raw::Err, Raw::NotFound);
        assert_eq!(hub.body.recovery, Some(Recovery::Inert));
        assert!(hub.body.memory.is_none() && hub.view_doc().is_none());
        assert!(owned_blocker(Some(&hub), Some(PlaceId::Sanctuary), Some(0)).is_none());
        hub.shutdown();
    }
    fn tamper_after_claim(claim: Res<WoodClaim>, mut hub: ResMut<WoodHub>) {
        let Some(prior) = claim.prior.as_ref() else {
            return;
        };
        let Some(snap) = hub.body.memory.as_mut() else {
            return;
        };
        if snap.id == prior.id {
            snap.doc.session = snap.doc.session.wrapping_add(11);
            snap.id.session = snap.id.session.wrapping_add(11);
        }
    }
    #[test]
    fn claim_rejects_an_intervening_memory_change() {
        let mut app = schedule_app(open_hub());
        app.add_systems(
            Update,
            tamper_after_claim
                .after(claim_wood_press)
                .before(admit_wood_press),
        );
        app.update();
        press(&mut app);
        app.update();
        let hub = app.world().resource::<WoodHub>();
        assert_eq!(hub.memory_unit(), Some(0));
        assert!(
            hub.body.inflight.is_none()
                && hub.body.queued.is_none()
                && hub.body.memory.as_ref().unwrap().doc.stand == Stand::Standing
        );
        assert!(
            app.world().resource::<WoodView>().standing
                && app.world().resource::<WoodView>().unit == 0
        );
        assert_eq!(
            app.world()
                .resource::<crate::first_harvest_epiphany::FirstHarvestEpiphany>()
                .harvests_this_session,
            0
        );
        assert!(app.world().resource::<WoodClaim>().blocks());
        app.world_mut()
            .resource_mut::<crate::input::PlayerInput>()
            .interact = false;
        app.update();
        assert!(
            !app.world().resource::<WoodClaim>().blocks()
                && app.world().resource::<WoodHub>().memory_unit() == Some(0)
        );
        app.world_mut().resource_mut::<WoodHub>().shutdown();
    }
    #[derive(Resource, Clone, Copy)]
    struct RealmWrite(Option<u8>);
    fn apply_realm(
        write: Res<RealmWrite>,
        mut realm: ResMut<crate::living_practice_loop::SoftPlayerRealm>,
    ) {
        realm.current = write.0;
    }
    #[test]
    fn accepted_unit_stays_visible_when_the_writer_blocks() {
        let mut app = schedule_app(open_hub());
        app.insert_resource(RealmWrite(Some(0)))
            .add_systems(PreUpdate, apply_realm);
        app.update();
        press(&mut app);
        app.update();
        assert_eq!(app.world().resource::<WoodView>().unit, 1);
        assert!(owned_blocker(
            Some(app.world().resource::<WoodHub>()),
            Some(PlaceId::Sanctuary),
            Some(0)
        )
        .is_none());
        {
            let mut hub = app.world_mut().resource_mut::<WoodHub>();
            hub.body.inert = true;
            hub.body.blocked = true;
            hub.body.conflict = true;
            let snap = hub.body.memory.clone().unwrap();
            assert!(!hub.can_write() && hub.view_doc().is_some_and(|doc| doc.unit == 1));
            assert_eq!(hub.commit_at(&snap), Err(Reject::Writable));
        }
        app.insert_resource(RealmWrite(Some(4)));
        app.world_mut()
            .resource_mut::<crate::input::PlayerInput>()
            .interact = false;
        app.update();
        assert_eq!(
            (
                app.world().resource::<WoodView>().unit,
                app.world().resource::<WoodView>().standing
            ),
            (1, false)
        );
        assert!(owned_blocker(
            Some(app.world().resource::<WoodHub>()),
            Some(PlaceId::Sanctuary),
            Some(4)
        )
        .is_none());
        app.insert_resource(RealmWrite(Some(0)));
        app.update();
        assert_eq!(app.world().resource::<WoodView>().unit, 1);
        assert!(owned_blocker(
            Some(app.world().resource::<WoodHub>()),
            Some(PlaceId::Sanctuary),
            Some(0)
        )
        .is_none());
        app.world_mut().resource_mut::<WoodHub>().shutdown();
    }
    fn pull_mercy(
        mut nodes: Query<&mut Transform, With<crate::mercy_harvest_nodes::MercyHarvestNode>>,
        presence: Res<crate::human_presence::SoftPresence>,
        mut nearby: ResMut<crate::mercy_harvest_nodes::NearbyMercyNode>,
        hour_write: Res<HourWrite>,
        mut hour: ResMut<crate::hour_sacred::HourSacred>,
    ) {
        hour.session.hex = hour_write.0;
        let at_body = hour_write.0 == shared::space_law::HexFlag::Peace;
        for mut tf in &mut nodes {
            tf.translation = if at_body {
                presence.position
            } else {
                Vec3::ZERO
            };
            break;
        }
        nearby.nodes_exist = true;
        nearby.in_range = at_body;
        nearby.distance = if at_body { 0.0 } else { 100.0 };
    }
    #[derive(Resource, Clone, Copy)]
    struct HourWrite(shared::space_law::HexFlag);
    #[test]
    fn plugins_stay_cycle_free_and_mercy_or_peace_yields_same_frame() {
        use crate::mercy_harvest_nodes::MercyHarvestNodesPlugin;
        let site = first_passing_site().unwrap();
        let mut app = App::new();
        app.add_plugins((
            bevy::MinimalPlugins,
            bevy::asset::AssetPlugin::default(),
            bevy::input::InputPlugin,
            crate::input::InputPlugin,
        ));
        app.init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            .init_resource::<crate::first_session_guidance::FirstSessionGuidance>()
            .init_resource::<crate::thriving_moments::ThrivingMoments>()
            .init_resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>()
            .init_resource::<crate::harvest_feel::SoftRbePool>()
            .init_resource::<crate::world_answer::WorldAnswer>()
            .init_resource::<crate::hour_sacred::HourSacred>()
            .init_resource::<crate::local_settings::LocalSettingsState>()
            .init_resource::<crate::fabricator::FabricatorYard>()
            .init_resource::<crate::lived_hour_support::RbeUiSync>()
            .init_resource::<ButtonInput<KeyCode>>()
            .insert_resource(crate::hex_travel::HexTravelState {
                current: PlaceId::Sanctuary,
            })
            .insert_resource(crate::human_presence::SoftPresence {
                position: Vec3::new(site.x, 0.9, site.y),
                ..Default::default()
            })
            .insert_resource(HourWrite(shared::space_law::HexFlag::Peace))
            .add_message::<bevy::input::gamepad::GamepadRumbleRequest>();
        app.add_plugins((
            crate::feel_move::FeelMovePlugin,
            crate::first_harvest_epiphany::FirstHarvestEpiphanyPlugin,
            crate::heartwood_wards::HeartwoodWardsPlugin,
            crate::living_practice_loop::LivingPracticeLoopPlugin,
            MercyHarvestNodesPlugin,
            crate::wellsworn::WellswornPlugin,
            crate::heartwood_plaque::HeartwoodPlaquePlugin,
            WoodExpeditionPlugin,
        ));
        app.add_systems(PreUpdate, pull_mercy);
        app.world_mut()
            .resource_mut::<crate::living_practice_loop::SoftPlayerRealm>()
            .current = Some(0);
        app.update();
        use bevy::ecs::system::IntoSystem as Sys;
        let ids = [
            Sys::system_type_id(&crate::feel_move::raise_buffered_use),
            Sys::system_type_id(&crate::feel_move::arm_use_buffer_from_edge),
            Sys::system_type_id(&crate::first_harvest_epiphany::mark_peace_visitor),
            Sys::system_type_id(&crate::first_harvest_epiphany::mark_epiphany_place),
            Sys::system_type_id(&refresh_wood_facts),
            Sys::system_type_id(&claim_wood_press),
            Sys::system_type_id(&crate::first_harvest_epiphany::handle_interact_harvest),
            Sys::system_type_id(&crate::heartwood_wards::use_heartwood_wards),
            Sys::system_type_id(&crate::living_practice_loop::soft_interact_harvest_credit),
            Sys::system_type_id(&admit_wood_press),
            Sys::system_type_id(&project_wood),
        ];
        let mut at = [0usize; 11];
        for (i, id) in ids.into_iter().enumerate() {
            at[i] = ord(&mut app, id);
        }
        assert!(at[0] < at[4] && at[1] < at[4] && at[2] < at[4] && at[3] < at[4] && at[4] < at[5]);
        assert!(at[5] < at[6] && at[5] < at[7] && at[5] < at[8] && at[6] < at[9] && at[9] < at[10]);
        press(&mut app);
        app.update();
        assert_eq!(app.world().resource::<WoodHub>().memory_unit(), Some(0));
        assert!(!app.world().resource::<WoodClaim>().blocks());
        assert_eq!(
            app.world()
                .resource::<crate::first_harvest_epiphany::FirstHarvestEpiphany>()
                .harvests_this_session,
            1
        );
        app.insert_resource(HourWrite(shared::space_law::HexFlag::Frontier));
        press(&mut app);
        app.update();
        assert!(
            !app.world().resource::<WoodFacts>().mercy_blocks
                && app.world().resource::<WoodFacts>().door_blocks
        );
        assert_eq!(app.world().resource::<WoodHub>().memory_unit(), Some(0));
        assert!(!app.world().resource::<WoodClaim>().blocks());
        assert_eq!(
            app.world()
                .resource::<crate::first_harvest_epiphany::FirstHarvestEpiphany>()
                .harvests_this_session,
            1
        );
        app.world_mut().resource_mut::<WoodHub>().shutdown();
    }

    #[derive(Resource, Clone, Copy)]
    struct SessionArm(u8);
    fn arm_session(
        arm: Res<SessionArm>,
        mut hour: ResMut<crate::hour_sacred::HourSacred>,
        mut voice: ResMut<crate::coop_voice::VoiceYard>,
        mut ledger: ResMut<crate::ledger_bind::LedgerYard>,
        mut embassy: ResMut<crate::embassy::EmbassyYard>,
        mut crown: ResMut<crate::crownstone::CrownstoneYard>,
        mut redemption: ResMut<crate::species_redemption::RedemptionYard>,
        mut hybrid: ResMut<crate::hybrid_matrix::HybridYard>,
        mut epi: ResMut<crate::first_harvest_epiphany::FirstHarvestEpiphany>,
        mut travel: ResMut<crate::hex_travel::HexTravelState>,
    ) {
        *hour = crate::hour_sacred::HourSacred::default();
        *voice = crate::coop_voice::VoiceYard::default();
        *ledger = crate::ledger_bind::LedgerYard::default();
        *embassy = crate::embassy::EmbassyYard::default();
        *crown = crate::crownstone::CrownstoneYard::default();
        *redemption = crate::species_redemption::RedemptionYard::default();
        *hybrid = crate::hybrid_matrix::HybridYard::default();
        travel.current = PlaceId::Sanctuary;
        epi.beacon_voice = true;
        epi.ledger_bind = true;
        epi.embassy_lamp = true;
        epi.crownstone_near = true;
        epi.redemption_near = true;
        epi.hybrid_near = true;
        let skin = |hour: &mut crate::hour_sacred::HourSacred| {
            hour.session.charter_id = Some("wood".into());
            hour.session.hex = shared::space_law::HexFlag::Frontier;
        };
        match arm.0 {
            1 => {
                skin(&mut hour);
                voice.sash_open = true;
                voice.voice.ensure_tutorial();
            }
            2 => {
                skin(&mut hour);
                ledger.sash_open = true;
            }
            3 => {
                skin(&mut hour);
                hour.complete = true;
                embassy.embassy.lamp_live = true;
            }
            4 => {
                skin(&mut hour);
                hour.hour_three_complete = true;
                embassy.embassy.seated = true;
            }
            5 => {
                skin(&mut hour);
                crown.stone.witnessed = true;
            }
            6 => {
                skin(&mut hour);
                redemption.state.events = 1;
            }
            7 => {
                skin(&mut hour);
                hour.complete = true;
                embassy.embassy.lamp_live = true;
                travel.current = PlaceId::Heartwood;
            }
            _ => {}
        }
    }

    #[test]
    fn session_sources_change_the_same_frame() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .init_resource::<crate::hour_sacred::HourSacred>()
            .init_resource::<crate::coop_voice::VoiceYard>()
            .init_resource::<crate::ledger_bind::LedgerYard>()
            .init_resource::<crate::embassy::EmbassyYard>()
            .init_resource::<crate::crownstone::CrownstoneYard>()
            .init_resource::<crate::species_redemption::RedemptionYard>()
            .init_resource::<crate::hybrid_matrix::HybridYard>()
            .init_resource::<crate::first_harvest_epiphany::FirstHarvestEpiphany>()
            .init_resource::<WoodFacts>()
            .insert_resource(crate::hex_travel::HexTravelState {
                current: PlaceId::Sanctuary,
            })
            .insert_resource(SessionArm(0))
            .add_systems(PreUpdate, arm_session)
            .add_systems(Update, refresh_wood_facts);
        let expect = [
            (0, false),
            (1, true),
            (2, true),
            (3, true),
            (4, true),
            (5, true),
            (6, true),
            (7, false),
        ];
        for (arm, door) in expect {
            app.insert_resource(SessionArm(arm));
            app.update();
            let epi = app
                .world()
                .resource::<crate::first_harvest_epiphany::FirstHarvestEpiphany>();
            assert!(
                epi.beacon_voice
                    && epi.ledger_bind
                    && epi.embassy_lamp
                    && epi.crownstone_near
                    && epi.redemption_near
                    && epi.hybrid_near
            );
            assert_eq!(
                app.world().resource::<WoodFacts>().door_blocks,
                door,
                "arm {arm}"
            );
        }
    }

    fn projected(app: &mut App) -> (usize, usize, usize) {
        let trees = {
            let mut query = app.world_mut().query::<&OwnedTree>();
            query.iter(app.world()).count()
        };
        let canopy = {
            let mut query = app.world_mut().query::<(&OwnedTree, &Children)>();
            query.iter(app.world()).map(|(_, kids)| kids.len()).sum()
        };
        let bark = {
            let mut query = app.world_mut().query::<&BarkBit>();
            query.iter(app.world()).count()
        };
        (trees, canopy, bark)
    }

    #[test]
    fn projected_entities_follow_harvest_and_realm() {
        let site = first_passing_site().unwrap();
        let mut app = App::new();
        app.add_plugins((bevy::MinimalPlugins, bevy::asset::AssetPlugin::default()))
            .init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            .insert_resource(open_hub())
            .init_resource::<WoodClaim>()
            .init_resource::<WoodFacts>()
            .init_resource::<WoodView>()
            .init_resource::<crate::input::PlayerInput>()
            .init_resource::<crate::feel_move::UseBuffer>()
            .init_resource::<crate::feel_move::SimUsePulse>()
            .init_resource::<ButtonInput<KeyCode>>()
            .insert_resource(crate::hex_travel::HexTravelState {
                current: PlaceId::Sanctuary,
            })
            .insert_resource(crate::living_practice_loop::SoftPlayerRealm { current: Some(0) })
            .insert_resource(crate::human_presence::SoftPresence {
                position: Vec3::new(site.x, 0.9, site.y),
                ..Default::default()
            })
            .insert_resource(RealmWrite(Some(0)))
            .add_systems(PreUpdate, apply_realm)
            .add_systems(Update, crate::feel_move::raise_buffered_use);
        app.world_mut().spawn(crate::human_presence::HumanPresence);
        add_wood_systems(&mut app);
        app.update();
        assert_eq!(projected(&mut app), (1, 1, 0));
        app.insert_resource(RealmWrite(Some(4)));
        app.update();
        assert_eq!(projected(&mut app), (0, 0, 0));
        app.insert_resource(RealmWrite(Some(0)));
        app.update();
        assert_eq!(projected(&mut app), (1, 1, 0));
        press(&mut app);
        app.update();
        assert_eq!(projected(&mut app), (0, 0, 1));
        assert_eq!(app.world().resource::<WoodView>().unit, 1);
        {
            let mut hub = app.world_mut().resource_mut::<WoodHub>();
            hub.body.inert = true;
            hub.body.blocked = true;
            hub.body.conflict = true;
            let snap = hub.body.memory.clone().unwrap();
            assert!(!hub.can_write());
            assert_eq!(hub.commit_at(&snap), Err(Reject::Writable));
            assert_eq!(hub.view_doc().map(|doc| doc.unit), Some(1));
        }
        app.world_mut()
            .resource_mut::<crate::input::PlayerInput>()
            .interact = false;
        app.update();
        assert_eq!(projected(&mut app), (0, 0, 1));
        app.world_mut().resource_mut::<WoodHub>().shutdown();
    }

    type Door = crate::title_screen::LaunchDoor;

    fn house_label(settings_open: bool) -> crate::title_screen::HouseLabel {
        crate::title_screen::HouseLabel {
            house: shared::house_name::HouseName::default(),
            persist_present: false,
            hour_two_held: false,
            book_held: false,
            settings_open,
            draft: String::new(),
            naming_offered: false,
            seals_offered: false,
        }
    }
    fn memory_bytes(app: &App) -> Vec<u8> {
        app.world()
            .resource::<WoodHub>()
            .body
            .memory
            .as_ref()
            .unwrap()
            .bytes
            .clone()
    }
    fn place_body(app: &mut App, offset: f32) {
        let site = first_passing_site().unwrap();
        let at = site + Vec2::X * offset;
        app.world_mut()
            .resource_mut::<crate::human_presence::SoftPresence>()
            .position = Vec3::new(at.x, 0.9, at.y);
    }
    fn boot_contact(
        offset: f32,
        door: Option<Door>,
        settings: Option<bool>,
    ) -> (App, Fake, Vec<u8>) {
        let fake = Fake::pair(Raw::NotFound, Raw::NotFound);
        let mut app = schedule_app(started(fake.clone(), SLOW));
        place_body(&mut app, offset);
        if let Some(door) = door {
            app.insert_resource(door);
        }
        if let Some(open) = settings {
            app.insert_resource(house_label(open));
        }
        let bytes = memory_bytes(&app);
        (app, fake, bytes)
    }
    fn pulse(app: &mut App) {
        app.world_mut()
            .resource_mut::<crate::feel_move::SimUsePulse>()
            .fire = true;
    }
    fn stop(app: &mut App) {
        app.world_mut().resource_mut::<WoodHub>().shutdown();
    }
    fn still(app: &App, fake: &Fake, bytes: &[u8]) {
        assert_eq!(app.world().resource::<WoodHub>().memory_unit(), Some(0));
        assert_eq!(memory_bytes(app), bytes);
        assert!(app.world().resource::<WoodHub>().body.inflight.is_none());
        assert!(app.world().resource::<WoodHub>().body.queued.is_none());
        assert!(fake.publishes().is_empty());
    }
    fn publish_once(offset: f32, door: Option<Door>, settings: Option<bool>) {
        let (mut app, fake, _) = boot_contact(offset, door, settings);
        pulse(&mut app);
        app.update();
        until(|| fake.publishes().len() == 1);
        assert_eq!(fake.publishes(), vec![memory_bytes(&app)]);
        assert_eq!(app.world().resource::<WoodHub>().memory_unit(), Some(1));
        assert!(app.world().resource::<WoodClaim>().blocks());
        app.world_mut()
            .resource_mut::<crate::input::PlayerInput>()
            .interact = false;
        app.update();
        pulse(&mut app);
        app.update();
        thread::sleep(Duration::from_millis(20));
        assert_eq!(fake.publishes().len(), 1);
        assert!(!app.world().resource::<WoodClaim>().blocks());
        stop(&mut app);
    }
    fn deny_wood(offset: f32, door: Option<Door>, settings: Option<bool>) {
        let (mut app, fake, bytes) = boot_contact(offset, door, settings);
        pulse(&mut app);
        app.update();
        still(&app, &fake, &bytes);
        assert!(!app.world().resource::<WoodClaim>().blocks());
        stop(&mut app);
    }
    #[derive(Resource, Clone, Copy)]
    enum MidFlip {
        Move,
        Settings,
    }
    fn flip_after_claim(
        flip: Res<MidFlip>,
        mut presence: ResMut<crate::human_presence::SoftPresence>,
        mut house: Option<ResMut<crate::title_screen::HouseLabel>>,
    ) {
        match *flip {
            MidFlip::Move => {
                let site = first_passing_site().unwrap();
                presence.position = Vec3::new(site.x + 1.0, 0.9, site.y);
            }
            MidFlip::Settings => house.as_mut().unwrap().settings_open = true,
        }
    }
    fn flipped_claim_is_kept(flip: MidFlip) {
        let (mut app, fake, bytes) = boot_contact(stance_offset(), Some(Door::InYard), Some(false));
        app.insert_resource(flip).add_systems(
            Update,
            flip_after_claim
                .after(claim_wood_press)
                .before(admit_wood_press),
        );
        pulse(&mut app);
        app.update();
        assert!(app.world().resource::<WoodClaim>().blocks());
        still(&app, &fake, &bytes);
        app.world_mut()
            .resource_mut::<crate::input::PlayerInput>()
            .interact = false;
        app.update();
        assert!(!app.world().resource::<WoodClaim>().blocks());
        still(&app, &fake, &bytes);
        stop(&mut app);
    }
    #[test]
    fn contact_and_yard_gate_the_real_press() {
        publish_once(stance_offset(), None, None);
        publish_once(TRUNK_R + BODY_R + IN_TOL, None, None);
        publish_once(stance_offset(), Some(Door::Title), None);
        publish_once(stance_offset(), Some(Door::InYard), None);
        publish_once(stance_offset(), None, Some(false));
        publish_once(stance_offset(), None, Some(true));
        publish_once(stance_offset(), Some(Door::InYard), Some(false));
        deny_wood(TRUNK_R + BODY_R + OUT_TOL, None, None);
        deny_wood(1.0, None, None);
        deny_wood(stance_offset(), Some(Door::Title), Some(false));
        deny_wood(stance_offset(), Some(Door::InYard), Some(true));
        deny_wood(stance_offset(), Some(Door::Title), Some(true));
        flipped_claim_is_kept(MidFlip::Move);
        flipped_claim_is_kept(MidFlip::Settings);
    }

    /// Exact harvested bytes at schema 1: `canon(&sample(SCHEMA, 4, TREE_ID, 1))`.
    pub(crate) const HARVESTED: &[u8] = br#"{"schema":1,"revision":4,"generation":3,"session":8,"op":2,"tree":0,"stand":"Depleted","unit":1,"charge":"Spent"}"#;
    fn live_bytes(fake: &Fake) -> Vec<u8> {
        let Raw::Bytes(bytes) = fake.primary() else { panic!("{:?}", fake.primary()) };
        bytes
    }
    fn both(b: &[u8]) -> [(Raw, Raw, Recovery); 2] {
        [(Raw::Bytes(b.to_vec()), Raw::NotFound, Recovery::Primary), (Raw::NotFound, Raw::Bytes(b.to_vec()), Recovery::Backup)]
    }
    /// A booted hub carrying the pinned harvested unit, and a read of its live bytes.
    pub(crate) fn carried_hub() -> (WoodHub, impl Fn() -> Vec<u8>) {
        let (hub, fake) = boot_pair(Raw::Bytes(HARVESTED.to_vec()), Raw::NotFound);
        (hub, move || live_bytes(&fake))
    }

    /// WU1 (client) — only a carried unit is taken; standing, empty or inert docs refuse and publish nothing.
    #[test]
    fn wu1_take_unit_consumes_only_a_carried_unit() {
        let (mut hub, fake) = boot_pair(Raw::Bytes(canon(&Doc::fresh(7)).unwrap()), Raw::NotFound);
        assert_eq!((hub.take_unit(), hub.view_doc(), fake.publishes().len()), (0, Some(Doc::fresh(7)), 0));
        admit_now(&mut hub).unwrap();
        assert_eq!((hub.memory_unit(), hub.take_unit()), (Some(1), 1));
        let doc = hub.view_doc().unwrap();
        assert_eq!((doc.schema, doc.stand, doc.unit, doc.charge), (SCHEMA, Stand::Depleted, 0, Charge::None));
        assert_eq!((doc.revision, doc.generation, doc.op, hub.take_unit()), (2, 2, 2, 0));
        assert_eq!(admit_now(&mut hub), Err(Reject::Identity), "Depleted + unit 0 does not re-harvest");
        drive(&mut hub, |hub| hub.body.confirmed.as_ref().map(|s| s.doc.unit) == Some(0));
        assert_eq!(fake.publishes().len(), 2);
        hub.shutdown();
        let (mut inert, fake) = boot_pair(Raw::Bytes(b"{".to_vec()), Raw::Bytes(HARVESTED.to_vec()));
        assert_eq!(inert.take_unit(), 0);
        inert.shutdown();
        assert!(fake.publishes().is_empty());
    }

    /// WU2 (H3) — the pinned harvested bytes still load from live and from bak.
    #[test]
    fn wu2_old_harvested_bytes_still_load_live_and_bak() {
        assert_eq!((canon(&sample(SCHEMA, 4, TREE_ID, 1)).unwrap(), SCHEMA), (HARVESTED.to_vec(), 1));
        for (primary, backup, class) in both(HARVESTED) {
            let (mut hub, _) = boot_pair(primary, backup);
            assert_eq!((hub.body.recovery, hub.view_doc()), (Some(class), Some(sample(SCHEMA, 4, TREE_ID, 1))));
            hub.shutdown();
        }
    }

    /// WU2 (H3) — one take turns those bytes into schema 1, Depleted, unit 0, Spent; that persists and reloads live or bak.
    #[test]
    fn wu2_harvested_bytes_consume_to_unit_zero_and_persist() {
        let want = Doc { revision: 5, generation: 4, op: 3, unit: 0, ..sample(SCHEMA, 4, TREE_ID, 1) };
        for (primary, backup, _) in both(HARVESTED) {
            let (mut hub, fake) = boot_pair(primary, backup);
            assert_eq!((hub.take_unit(), hub.view_doc()), (1, Some(want.clone())));
            drive(&mut hub, |hub| hub.body.confirmed.as_ref().map(|s| s.doc.unit) == Some(0));
            let saved = live_bytes(&fake);
            assert_eq!((saved.clone(), parse_doc(&saved)), (canon(&want).unwrap(), Some(want.clone())));
            hub.shutdown();
            for (primary, backup, class) in both(&saved) {
                let (mut again, fake) = boot_pair(primary, backup);
                assert_eq!((again.body.recovery, again.view_doc()), (Some(class), Some(want.clone())));
                assert_eq!(again.take_unit(), 0, "one unit, one take");
                again.shutdown();
                assert!(fake.publishes().is_empty());
            }
        }
    }

    /// WU7 — harvest still turns Standing + unit 0 into unit 1; Depleted never re-harvests; `domain_ok` holds its bounds.
    #[test]
    fn wu7_harvest_successor_and_domain_unchanged() {
        for charge in [Charge::None, Charge::Spent] {
            let next = successor(&standing(9, charge)).map(|n| (n.schema, n.stand, n.unit, n.charge));
            assert_eq!(next, Ok((SCHEMA, Stand::Depleted, 1, Charge::None)));
        }
        assert_eq!(successor(&standing(9, Charge::Ready)), Err(Reject::Ready));
        for unit in [0, 1] {
            assert_eq!(successor(&sample(SCHEMA, 4, TREE_ID, unit)), Err(Reject::Identity));
            assert!(domain_ok(&sample(SCHEMA, 4, TREE_ID, unit)));
        }
        assert!(!domain_ok(&sample(SCHEMA, 4, TREE_ID, 2)) && !domain_ok(&sample(2, 4, TREE_ID, 0)));
        assert!(!domain_ok(&witness(Stand::Depleted, 1, Charge::Ready, TREE_ID)));
    }

    /// WU10 (H3) — the harvested bytes plus an unknown key stay inert from live or bak, and nothing publishes.
    #[test]
    fn wu10_extra_field_wood_json_stays_inert() {
        let mut extra: serde_json::Value = serde_json::from_slice(HARVESTED).unwrap();
        extra["extra"] = 1.into();
        let extra = serde_json::to_vec(&extra).unwrap();
        assert!(parse_doc(&extra).is_none());
        for (primary, backup, _) in both(&extra) {
            let (mut hub, fake) = boot_pair(primary, backup);
            assert!(hub.body.recovery == Some(Recovery::Inert) && !hub.can_write() && hub.view_doc().is_none());
            hub.shutdown();
            assert!(fake.publishes().is_empty());
        }
    }
}
