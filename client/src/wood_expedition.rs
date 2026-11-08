//! Inert wood document and off-thread publication.
//! Both siblings confirmed absent: offline local deletion resets. No tamper claim.
//! A crash may replay a checkpoint. No durable exactly-once. No one-flush backup bound.
//! One process. No cross-process exclusion.
//! Final exit wait is a 250 ms design ceiling, not a measured guarantee.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
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

#[derive(Resource)]
struct WoodHub {
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
    fn shutdown(&mut self) {
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
            .add_systems(Update, poll_wood)
            .add_systems(Last, stop_on_exit);
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

#[cfg(test)]
mod tests {
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
}
