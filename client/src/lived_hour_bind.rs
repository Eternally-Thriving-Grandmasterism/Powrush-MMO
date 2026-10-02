//! client/src/lived_hour_bind.rs
//! Bind first-hour hands to shared::climate_node::LivedHour.
//! E tend · I satchel · R 1 flow · R 2 reserve.
//! Persist: powrush_lived_tick.json + powrush_shard_climate.json in the
//! OS user-data dir (or `POWRUSH_USER_DIR`). Phase Q.
//!
//! Tick path honesty: default `powrush_lived_tick.json` is **session persist**
//! (Mode B resume / Continuity) — not Ra-Thor ingest. Keep writing the lived-hour
//! blob whenever the client needs it. `POWRUSH_INGEST=on` soft-writes a versioned
//! lattice overlay on the same path; checklist “no tick” means no ingest overlay.
//! Do not delete the blob. Does not replace harvest_feel or rbe_allocate_choice.
//!
//! CARD L3 PEOPLE-DOOR-LAND — apply_place climate swap (Places + People-door).
//! CARD L4 PLACE-DRESS-ON-LAND — People-door dress reuses this same swap
//! (no second dresser). Cite PLACE_DRESS_SPEC · ART_BIBLE accents.
//!
//! CARD FLESH-RESUME-PLACE — Mode B resume / Continuity names the Place
//! already on the climate hex when the lived-hour blob wakes
//! (Sanctuary / Heartwood / Depths). `local-hex` stays Sanctuary dirt.
//! Threshold-near is shelf reach, not a wake hex. L3/L4 land swap stays.
//! Do not delete the blob. One line. No second HUD.

use bevy::prelude::*;
use shared::climate_node::{AllocKind, LivedHour, NodeState, TendResult};
use shared::hex_travel::{
    read_hex_named, sanctuary_fresh_climate, stub_hex_file, PlaceId,
};
use shared::lived_tick_ingest::{self, LivedTickIngest};
use shared::shard_climate::ShardClimate;
use shared::shard_standing::ShardStanding;
use shared::week_audit::WeekAudit;

pub const LIVED_TICK_PATH: &str = "data/powrush_lived_tick.json";
pub const SHARD_CLIMATE_PATH: &str = "data/powrush_shard_climate.json";
pub const SHARD_STANDING_PATH: &str = "data/powrush_shard_standing.json";
pub const WEEK_AUDIT_PATH: &str = "data/powrush_week_audit.json";

#[derive(Resource, Debug, Clone)]
pub struct LivedHourBind {
    pub hour: LivedHour,
    pub climate: ShardClimate,
    pub standing: ShardStanding,
    pub week: WeekAudit,
    pub last_line: String,
    pub guidance_hidden: bool,
    /// Nearest well the body is looking at. tend_nearest uses this first.
    pub focus_id: Option<u32>,
    /// Optional climate/standing clause after allocate / resume (not a second HUD).
    pub climate_slab: Option<String>,
}

impl Default for LivedHourBind {
    fn default() -> Self {
        Self::load_or_demo()
    }
}

/// One file in an hour-set persist that did not land.
/// `detail` is absent on the ingest branch: `soft_write_if_enabled` returns a
/// bool and drops the io error, so the warn names the file only.
struct HourSetMiss {
    path: &'static str,
    detail: Option<String>,
}

/// Paths attempted, then paths that failed, both in attempt order.
struct HourSetWrite {
    /// Read by the hour-set tests. `persist` warns from `failed` only.
    #[allow(dead_code)]
    attempted: Vec<&'static str>,
    failed: Vec<HourSetMiss>,
}

impl LivedHourBind {
    fn load_climate() -> ShardClimate {
        if let Ok(raw) = shared::user_persist::read_named(SHARD_CLIMATE_PATH) {
            if let Ok(c) = ShardClimate::from_json(&raw) {
                return c;
            }
        }
        ShardClimate::default()
    }

    fn load_standing() -> ShardStanding {
        if let Ok(raw) = shared::user_persist::read_named(SHARD_STANDING_PATH) {
            if let Ok(s) = ShardStanding::from_json(&raw) {
                return s;
            }
        }
        ShardStanding::default()
    }

    fn load_week() -> WeekAudit {
        if let Ok(raw) = shared::user_persist::read_named(WEEK_AUDIT_PATH) {
            if let Ok(w) = WeekAudit::from_json(&raw) {
                return w;
            }
        }
        WeekAudit::default()
    }

    pub fn load_or_demo() -> Self {
        let climate = Self::load_climate();
        let standing = Self::load_standing();
        let mut week = Self::load_week();
        week.sync_from_climate(climate.tons_moved, climate.restored_count);
        let climate_slab = Self::compose_slab(&climate, &standing, &week);
        let resume_line = Self::resume_place_line(&climate.hex_id);
        if let Ok(raw) = shared::user_persist::read_named(LIVED_TICK_PATH) {
            // L3 composite (ingest on) nests hour — Mode B resume still works.
            if let Ok(tick) = LivedTickIngest::from_json(&raw) {
                if let Some(hour) = tick.hour {
                    return Self {
                        hour,
                        climate,
                        standing,
                        week,
                        last_line: resume_line.clone(),
                        guidance_hidden: false,
                        focus_id: None,
                        climate_slab,
                    };
                }
            }
            if let Ok(hour) = LivedHour::from_json(&raw) {
                return Self {
                    hour,
                    climate,
                    standing,
                    week,
                    last_line: resume_line.clone(),
                    guidance_hidden: false,
                    focus_id: None,
                    climate_slab,
                };
            }
        }
        Self {
            hour: LivedHour::new_demo(),
            climate,
            standing,
            week,
            last_line: "walk to a glow".to_string(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab,
        }
    }

    /// CARD FLESH-RESUME-PLACE — dress the existing Continuity verb with the
    /// wake Place. Unknown hex keeps `resumed`. Blob and land swap stay.
    fn resume_place_line(hex_id: &str) -> String {
        match PlaceId::parse(hex_id) {
            Some(place) => format!("{} · resumed", place.display_name()),
            None => "resumed".to_string(),
        }
    }

    fn compose_slab(
        climate: &ShardClimate,
        standing: &ShardStanding,
        week: &WeekAudit,
    ) -> Option<String> {
        // After any tons/restored, prefer the honest week line.
        if week.tons_moved > 0 || week.restored_count > 0 {
            return Some(week.slab_line());
        }
        standing
            .slab_line()
            .or_else(|| climate.slab_line())
            .map(|s| s.to_string())
    }

    pub fn persist(&self) {
        // Never delete the blob. Never block the hour or WASD.
        let set = self.write_hour_set();
        for miss in &set.failed {
            warn!("{}", hour_set_failure_line(miss));
        }
    }

    /// CARD LR-02 SAVE-HOUR-SET-1 — climate, standing, week, then the tick file.
    /// The tick path is the commit marker. A failed write is recorded and the
    /// rest still run.
    fn write_hour_set(&self) -> HourSetWrite {
        // Other --lib tests call persist without setting POWRUSH_USER_DIR.
        // They must wait out an override so they do not write into that scratch.
        #[cfg(test)]
        let _user_dir = crate::test_env::lock();
        let mut attempted = Vec::with_capacity(4);
        let mut failed = Vec::new();
        // Soft-fail climate / standing / week I/O — never block the hour.
        record_named_json(
            SHARD_CLIMATE_PATH,
            self.climate.to_json(),
            &mut attempted,
            &mut failed,
        );
        record_named_json(
            SHARD_STANDING_PATH,
            self.standing.to_json(),
            &mut attempted,
            &mut failed,
        );
        record_named_json(
            WEEK_AUDIT_PATH,
            self.week.to_json(),
            &mut attempted,
            &mut failed,
        );
        // Session persist always: bare LivedHour blob for Continuity (not Ra-Thor ingest).
        // L3: when POWRUSH_INGEST=on, soft-write versioned lattice overlay (nested hour).
        // Checklist "no tick" = no ingest overlay. Never delete the blob. Never block WASD.
        attempted.push(LIVED_TICK_PATH);
        if lived_tick_ingest::ingest_enabled() {
            if !lived_tick_ingest::soft_write_if_enabled(
                &self.climate,
                &self.standing,
                &self.week,
                &self.hour,
            ) {
                // soft_write_if_enabled returns a bool and drops the io error.
                failed.push(HourSetMiss {
                    path: LIVED_TICK_PATH,
                    detail: None,
                });
            }
        } else {
            match self.hour.to_json() {
                Ok(json) => {
                    if let Err(err) = shared::user_persist::write_named(LIVED_TICK_PATH, json) {
                        failed.push(HourSetMiss {
                            path: LIVED_TICK_PATH,
                            detail: Some(err.to_string()),
                        });
                    }
                }
                Err(err) => failed.push(HourSetMiss {
                    path: LIVED_TICK_PATH,
                    detail: Some(err.to_string()),
                }),
            }
        }
        HourSetWrite { attempted, failed }
    }

    pub fn refresh_climate_slab(&mut self) {
        self.week
            .sync_from_climate(self.climate.tons_moved, self.climate.restored_count);
        self.climate_slab = Self::compose_slab(&self.climate, &self.standing, &self.week);
    }

    /// Keep House week footer (may sum hexes). Do not copy current-hex tons into week.
    pub fn refresh_climate_slab_keep_week(&mut self) {
        self.climate_slab = Self::compose_slab(&self.climate, &self.standing, &self.week);
    }

    /// CARD L3 — Places / People-door climate swap. Same hex files as title boot.
    /// CARD L4 — this is the Esc→Places dress climate path; People-door calls it.
    /// Does not persist. Week footer stays House-summed by the travel caller.
    pub fn apply_place(&mut self, dest: PlaceId) {
        match dest {
            PlaceId::Sanctuary => self.load_sanctuary_climate(),
            PlaceId::Heartwood | PlaceId::Depths => {
                let file = read_hex_named(dest).unwrap_or_else(|| stub_hex_file(dest));
                self.climate = file.climate;
                self.standing = file.standing;
                self.refresh_climate_slab_keep_week();
            }
        }
    }

    fn load_sanctuary_climate(&mut self) {
        if let Some(file) = read_hex_named(PlaceId::Sanctuary) {
            self.climate = file.climate;
            self.standing = file.standing;
        } else if self.climate.hex_id == PlaceId::Heartwood.as_str()
            || self.climate.hex_id == PlaceId::Depths.as_str()
        {
            self.climate = sanctuary_fresh_climate();
            self.standing = shared::hex_travel::sanctuary_fresh_standing();
        } else if self.climate.hex_id.is_empty() || self.climate.hex_id == "local-hex" {
            self.climate.hex_id = PlaceId::Sanctuary.as_str().into();
            self.standing.hex_id = PlaceId::Sanctuary.as_str().into();
        }
        self.refresh_climate_slab_keep_week();
    }

    /// E on a node id (nearest glow is the client's job).
    pub fn tend(&mut self, node_id: u32) -> TendResult {
        let prior = self
            .hour
            .nodes
            .iter()
            .find(|n| n.id == node_id)
            .map(|n| n.state);
        let result = self.hour.tend(node_id);
        match &result {
            TendResult::Taken { item } => {
                self.climate.on_glowing_take();
                self.standing.on_glowing_take();
                self.last_line = format!("tended node {}", item.node_id);
            }
            TendResult::NoTake { reason } => {
                if matches!(prior, Some(NodeState::Resting | NodeState::Stressed)) {
                    self.climate.on_tired_refuse();
                    self.standing.on_tired_refuse();
                }
                self.last_line = (*reason).to_string();
            }
        }
        self.refresh_climate_slab();
        self.persist();
        result
    }

    /// Nearest glowing node, or first node if none glow.
    pub fn nearest_glow_id(&self) -> Option<u32> {
        self.hour
            .nodes
            .iter()
            .find(|n| n.state == NodeState::Glowing)
            .or_else(|| self.hour.nodes.first())
            .map(|n| n.id)
    }

    pub fn tend_nearest(&mut self) -> TendResult {
        match self.focus_id.or_else(|| self.nearest_glow_id()) {
            Some(id) => self.tend(id),
            None => TendResult::NoTake { reason: "no glow" },
        }
    }

    /// R then 1 / 2.
    pub fn allocate(&mut self, kind: AllocKind) -> bool {
        let ok = self.hour.allocate(kind);
        self.last_line = if ok {
            match kind {
                AllocKind::Flow => {
                    self.climate.on_flow();
                    self.standing.on_flow();
                    "flow restored the well".to_string()
                }
                AllocKind::Reserve => {
                    self.climate.on_reserve();
                    self.standing.on_reserve();
                    "reserve held as repair-rights".to_string()
                }
            }
        } else {
            "satchel empty".to_string()
        };
        if ok {
            self.refresh_climate_slab();
        }
        self.persist();
        ok
    }

    /// Tend at a room's own node (the Threshold pipe). Leaves restored ink on
    /// this room's climate so the room's hex file is not blank when the House
    /// bill adds it up. Not a take: satchel and pools are untouched.
    pub fn room_tend(&mut self) {
        self.climate.on_room_tend();
        self.standing.on_care_tend();
        self.refresh_climate_slab();
        self.persist();
    }

    /// Hold-E care tend (ledger only — does not rewrite harvest_feel take).
    pub fn care_tend(&mut self) {
        self.climate.on_care_tend();
        self.standing.on_care_tend();
        self.refresh_climate_slab();
        self.persist();
    }

    pub fn satchel_count(&self) -> usize {
        self.hour.satchel.count()
    }

    pub fn toggle_guidance(&mut self) {
        self.guidance_hidden = !self.guidance_hidden;
    }

    pub fn tick(&mut self) {
        self.hour.tick();
    }
}

fn record_named_json(
    path: &'static str,
    json: Result<String, impl std::fmt::Display>,
    attempted: &mut Vec<&'static str>,
    failed: &mut Vec<HourSetMiss>,
) {
    attempted.push(path);
    match json {
        Ok(body) => {
            if let Err(err) = shared::user_persist::write_named(path, body) {
                failed.push(HourSetMiss {
                    path,
                    detail: Some(err.to_string()),
                });
            }
        }
        Err(err) => failed.push(HourSetMiss {
            path,
            detail: Some(err.to_string()),
        }),
    }
}

fn hour_set_failure_line(miss: &HourSetMiss) -> String {
    match &miss.detail {
        Some(detail) => format!("lived hour persist failed for {}: {detail}", miss.path),
        None => format!("lived hour persist failed for {}", miss.path),
    }
}

fn tick_lived_hour(time: Res<Time>, mut bind: ResMut<LivedHourBind>, mut acc: Local<f32>) {
    *acc += time.delta_seconds();
    if *acc < 1.0 {
        return;
    }
    *acc = 0.0;
    let before: Vec<NodeState> = bind.hour.nodes.iter().map(|n| n.state).collect();
    bind.tick();
    let after: Vec<NodeState> = bind.hour.nodes.iter().map(|n| n.state).collect();
    if before != after {
        bind.persist();
    }
}

pub struct LivedHourBindPlugin;

impl Plugin for LivedHourBindPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LivedHourBind>()
            .add_systems(Update, tick_lived_hour);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tend_fills_satchel() {
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        };
        assert!(matches!(bind.tend(1), TendResult::Taken { .. }));
        assert_eq!(bind.satchel_count(), 1);
    }

    #[test]
    fn flow_needs_a_take() {
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        };
        assert!(!bind.allocate(AllocKind::Flow));
        let _ = bind.tend(1);
        assert!(bind.allocate(AllocKind::Flow));
        assert_eq!(bind.satchel_count(), 0);
        assert_eq!(bind.hour.allocation.flow, 1);
    }

    #[test]
    fn focus_beats_first_glow() {
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: Some(2),
            climate_slab: None,
        };
        assert!(matches!(bind.tend_nearest(), TendResult::Taken { .. }));
        assert_eq!(bind.hour.nodes[1].state, NodeState::Tended);
        assert_eq!(bind.hour.nodes[0].state, NodeState::Glowing);
    }

    /// CARD FLESH-RESUME-PLACE — Mode B resume names the climate hex.
    /// `local-hex` is Sanctuary dirt. Threshold-near is not a wake PlaceId.
    #[test]
    fn resume_names_the_wake_place() {
        assert_eq!(
            LivedHourBind::resume_place_line("sanctuary"),
            "Sanctuary · resumed"
        );
        assert_eq!(
            LivedHourBind::resume_place_line("local-hex"),
            "Sanctuary · resumed"
        );
        assert_eq!(
            LivedHourBind::resume_place_line("heartwood"),
            "Heartwood · resumed"
        );
        assert_eq!(
            LivedHourBind::resume_place_line("depths"),
            "Depths · resumed"
        );
        assert_eq!(LivedHourBind::resume_place_line("nowhere"), "resumed");
        let dirt = LivedHourBind::resume_place_line("local-hex");
        assert!(dirt.contains("resumed"));
        assert!(dirt.contains("Sanctuary"));
        assert!(!dirt.to_lowercase().contains("gold"));
        assert!(!dirt.contains("Market"));
        assert!(!dirt.contains("XP"));
        assert_ne!(
            LivedHourBind::resume_place_line("heartwood"),
            "Threshold-near · resumed"
        );
        assert_eq!(LIVED_TICK_PATH, "data/powrush_lived_tick.json");
    }

    #[test]
    fn json_roundtrip_path_constant() {
        assert_eq!(LIVED_TICK_PATH, "data/powrush_lived_tick.json");
        assert_eq!(lived_tick_ingest::LIVED_TICK_INGEST_PATH, LIVED_TICK_PATH);
        assert_eq!(SHARD_CLIMATE_PATH, "data/powrush_shard_climate.json");
        assert_eq!(SHARD_STANDING_PATH, "data/powrush_shard_standing.json");
        assert_eq!(WEEK_AUDIT_PATH, "data/powrush_week_audit.json");
        let resolved = shared::user_persist::persist_path(LIVED_TICK_PATH);
        assert_eq!(
            resolved.file_name().and_then(|s| s.to_str()),
            Some("powrush_lived_tick.json")
        );
        assert!(shared::user_persist::is_writable_user_dir_rule(&resolved));
        assert!(!shared::user_persist::is_program_files_path(&resolved));
        assert!(!resolved.to_string_lossy().contains("f-book"));
    }

    #[test]
    fn ingest_default_off_in_client_bind() {
        // Default boot path must not require POWRUSH_INGEST.
        // Flag false unless explicitly on/1/true.
        let raw = std::env::var("POWRUSH_INGEST").unwrap_or_default();
        let on = matches!(
            raw.trim().to_ascii_lowercase().as_str(),
            "on" | "1" | "true" | "yes"
        );
        assert_eq!(lived_tick_ingest::ingest_enabled(), on);
    }

    #[test]
    fn extract_on_tired_stays_no_take() {
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        };
        let _ = bind.tend(1);
        let second = bind.tend(1);
        assert!(matches!(second, TendResult::NoTake { .. }));
        assert!(bind.climate.stress > 0.15);
    }

    #[test]
    fn flow_lowers_climate_stress() {
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate {
                stress: 0.7,
                ..Default::default()
            },
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        };
        let _ = bind.tend(1);
        assert!(bind.allocate(AllocKind::Flow));
        assert!(bind.climate.stress < 0.7);
    }

    #[test]
    fn reserve_raises_reserve_pool() {
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        };
        let _ = bind.tend(1);
        assert!(bind.allocate(AllocKind::Reserve));
        assert_eq!(bind.hour.allocation.reserve, 1);
        assert_eq!(bind.climate.reserve_pool, 1);
        assert!(bind.standing.steward > 0.40);
        let line = bind.hour.allocation.reserve_bank_line().expect("banked");
        assert!(line.contains("Reserve 1"));
        assert!(!line.contains("0.0"));
        assert!(bind.last_line.contains("repair-rights"));
    }

    #[test]
    fn week_slab_tracks_tons_and_restored() {
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        };
        bind.climate.tons_moved = 2;
        bind.climate.restored_count = 3;
        bind.refresh_climate_slab();
        let slab = bind.climate_slab.unwrap();
        assert!(slab.contains("2 tons"));
        assert!(slab.contains("3 restored"));
    }

    #[test]
    fn standing_lethal_default_false_until_declare() {
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        };
        bind.care_tend();
        let _ = bind.tend(1);
        assert!(bind.allocate(AllocKind::Flow));
        assert!(!bind.standing.declared_lethal);
        assert!(!bind.standing.declare_lethal(false));
        assert!(!bind.standing.confirm_hex_sign(false, true));
        assert!(!bind.standing.confirm_hex_sign(true, false));
        assert!(bind.standing.declare_lethal(true));
        bind.climate.reserve_pool = 1;
        let paid = bind.climate.on_lethal_declare();
        assert_eq!(paid, 1);
        bind.refresh_climate_slab();
        // Week slab still tons + restored language when week has counts.
        bind.climate.tons_moved = 2;
        bind.climate.restored_count = 1;
        bind.refresh_climate_slab();
        let slab = bind.climate_slab.unwrap();
        assert!(slab.contains("tons"));
        assert!(slab.contains("restored"));
        assert!(!slab.to_lowercase().contains("kill"));
    }

    #[test]
    fn mend_then_refresh_prefers_week_line() {
        // P2 mute-hole: MendSpool / LaneCrate must not bury the week audit.
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        };
        bind.climate.on_mend();
        bind.standing.on_mend();
        bind.refresh_climate_slab();
        let slab = bind.climate_slab.as_deref().unwrap_or("");
        assert!(slab.starts_with("this week"), "got {slab}");
        assert!(slab.contains("restored"));
        bind.climate.on_lane();
        bind.standing.on_lane();
        bind.refresh_climate_slab();
        let slab = bind.climate_slab.as_deref().unwrap_or("");
        assert!(slab.contains("tons"));
        assert!(slab.contains("restored"));
    }

    struct ScratchDir(std::path::PathBuf);

    impl ScratchDir {
        fn new(tag: &str) -> Self {
            let dir = std::env::temp_dir().join(format!(
                "powrush-lr02-{}-{}-{tag}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or(0)
            ));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("scratch");
            Self(dir)
        }

        fn path(&self) -> &std::path::Path {
            &self.0
        }
    }

    impl Drop for ScratchDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn fresh_bind() -> LivedHourBind {
        LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        }
    }

    fn hour_set_order() -> [&'static str; 4] {
        [
            SHARD_CLIMATE_PATH,
            SHARD_STANDING_PATH,
            WEEK_AUDIT_PATH,
            LIVED_TICK_PATH,
        ]
    }

    fn named_file(dir: &std::path::Path, logical: &str) -> std::path::PathBuf {
        dir.join(shared::user_persist::persist_file_name(logical))
    }

    fn with_persist_env<R>(
        dir: &std::path::Path,
        ingest: Option<&str>,
        body: impl FnOnce() -> R,
    ) -> R {
        let _guard = crate::test_env::lock();
        let prev_dir = std::env::var(shared::user_persist::USER_DIR_OVERRIDE_ENV).ok();
        let prev_ingest = std::env::var("POWRUSH_INGEST").ok();
        std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, dir);
        match ingest {
            Some(value) => std::env::set_var("POWRUSH_INGEST", value),
            None => std::env::remove_var("POWRUSH_INGEST"),
        }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(body));
        match &prev_dir {
            Some(value) => std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, value),
            None => std::env::remove_var(shared::user_persist::USER_DIR_OVERRIDE_ENV),
        }
        match &prev_ingest {
            Some(value) => std::env::set_var("POWRUSH_INGEST", value),
            None => std::env::remove_var("POWRUSH_INGEST"),
        }
        match result {
            Ok(value) => value,
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }

    /// CARD LR-02 — write order is climate, standing, week, then tick last.
    #[test]
    fn hour_set_write_order_is_climate_standing_week_then_tick() {
        let scratch = ScratchDir::new("order");
        let bind = fresh_bind();
        let set = with_persist_env(scratch.path(), None, || {
            assert!(!lived_tick_ingest::ingest_enabled());
            bind.persist();
            let set = bind.write_hour_set();
            for logical in hour_set_order() {
                let path = named_file(scratch.path(), logical);
                assert!(path.is_file(), "{}", path.display());
                let raw = std::fs::read_to_string(&path).unwrap();
                serde_json::from_str::<serde_json::Value>(&raw).expect(logical);
            }
            set
        });
        assert!(
            set.failed.is_empty(),
            "{:?}",
            set.failed.iter().map(|m| m.path).collect::<Vec<_>>()
        );
        assert_eq!(set.attempted, hour_set_order());
    }

    /// CARD LR-02 — a directory at the standing path fails that write only.
    #[test]
    fn hour_set_reports_directory_failure_and_still_writes_the_rest() {
        let scratch = ScratchDir::new("standing-dir");
        let standing = named_file(scratch.path(), SHARD_STANDING_PATH);
        std::fs::create_dir_all(&standing).unwrap();
        let bind = fresh_bind();
        let set = with_persist_env(scratch.path(), Some("off"), || {
            bind.persist();
            let set = bind.write_hour_set();
            assert!(named_file(scratch.path(), SHARD_CLIMATE_PATH).is_file());
            assert!(standing.is_dir(), "failed target stays a directory");
            assert!(named_file(scratch.path(), WEEK_AUDIT_PATH).is_file());
            assert!(named_file(scratch.path(), LIVED_TICK_PATH).is_file());
            set
        });
        assert_eq!(set.attempted, hour_set_order());
        assert_eq!(
            set.failed.iter().map(|m| m.path).collect::<Vec<_>>(),
            vec![SHARD_STANDING_PATH]
        );
        let detail = set.failed[0].detail.as_deref().expect("io error text");
        assert!(!detail.is_empty());
        let line = hour_set_failure_line(&set.failed[0]);
        assert!(line.contains(SHARD_STANDING_PATH));
        assert!(line.contains(detail));
    }

    /// CARD LR-02 — ingest off still writes the bare hour blob last.
    #[test]
    fn hour_set_tick_is_last_when_ingest_is_off() {
        let scratch = ScratchDir::new("ingest-off");
        let bind = fresh_bind();
        let set = with_persist_env(scratch.path(), Some("off"), || {
            assert!(!lived_tick_ingest::ingest_enabled());
            let set = bind.write_hour_set();
            let tick = named_file(scratch.path(), LIVED_TICK_PATH);
            let raw = std::fs::read_to_string(&tick).unwrap();
            assert_eq!(raw, bind.hour.to_json().unwrap());
            assert!(LivedHour::from_json(&raw).is_ok());
            assert!(LivedTickIngest::from_json(&raw).is_err());
            assert!(!raw.contains("powrush_lived_tick_v1"));
            set
        });
        assert!(set.failed.is_empty());
        assert_eq!(set.attempted, hour_set_order());
        assert_eq!(set.attempted.last().copied(), Some(LIVED_TICK_PATH));
    }

    /// CARD LR-02 — ingest branch names the tick file and keeps no io error text.
    #[test]
    fn hour_set_ingest_tick_failure_names_file_without_io_text() {
        let scratch = ScratchDir::new("ingest-tick-dir");
        let tick = named_file(scratch.path(), LIVED_TICK_PATH);
        std::fs::create_dir_all(&tick).unwrap();
        let bind = fresh_bind();
        let set = with_persist_env(scratch.path(), Some("on"), || {
            assert!(lived_tick_ingest::ingest_enabled());
            let set = bind.write_hour_set();
            assert!(named_file(scratch.path(), SHARD_CLIMATE_PATH).is_file());
            assert!(named_file(scratch.path(), SHARD_STANDING_PATH).is_file());
            assert!(named_file(scratch.path(), WEEK_AUDIT_PATH).is_file());
            assert!(tick.is_dir(), "failed tick target stays a directory");
            set
        });
        assert_eq!(set.attempted, hour_set_order());
        assert_eq!(
            set.failed.iter().map(|m| m.path).collect::<Vec<_>>(),
            vec![LIVED_TICK_PATH]
        );
        assert!(set.failed[0].detail.is_none());
        let line = hour_set_failure_line(&set.failed[0]);
        assert_eq!(
            line,
            format!("lived hour persist failed for {}", LIVED_TICK_PATH)
        );
        assert!(!line.to_lowercase().contains("os error"));
    }

    /// CARD LR-02 — a failed to_json counts as a failed write and keeps the error text.
    #[test]
    fn hour_set_json_encode_failure_is_a_failed_write() {
        let mut attempted = Vec::new();
        let mut failed = Vec::new();
        record_named_json(
            SHARD_CLIMATE_PATH,
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "encode",
            )),
            &mut attempted,
            &mut failed,
        );
        assert_eq!(attempted, vec![SHARD_CLIMATE_PATH]);
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].path, SHARD_CLIMATE_PATH);
        assert_eq!(failed[0].detail.as_deref(), Some("encode"));
        let line = hour_set_failure_line(&failed[0]);
        assert!(line.contains(SHARD_CLIMATE_PATH));
        assert!(line.contains("encode"));
    }
}
