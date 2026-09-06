//! client/src/lived_hour_bind.rs
//! Bind first-hour hands to shared::climate_node::LivedHour.
//! E tend · I satchel · R 1 flow · R 2 reserve.
//! Persist: data/powrush_lived_tick.json + data/powrush_shard_climate.json (Phase Q)
//! Does not replace harvest_feel or rbe_allocate_choice.

use std::fs;
use std::path::Path;

use bevy::prelude::*;
use shared::climate_node::{AllocKind, LivedHour, NodeState, TendResult};
use shared::shard_climate::ShardClimate;
use shared::shard_standing::ShardStanding;

pub const LIVED_TICK_PATH: &str = "data/powrush_lived_tick.json";
pub const SHARD_CLIMATE_PATH: &str = "data/powrush_shard_climate.json";
pub const SHARD_STANDING_PATH: &str = "data/powrush_shard_standing.json";

#[derive(Resource, Debug, Clone)]
pub struct LivedHourBind {
    pub hour: LivedHour,
    pub climate: ShardClimate,
    pub standing: ShardStanding,
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

impl LivedHourBind {
    fn load_climate() -> ShardClimate {
        if let Ok(raw) = fs::read_to_string(SHARD_CLIMATE_PATH) {
            if let Ok(c) = ShardClimate::from_json(&raw) {
                return c;
            }
        }
        ShardClimate::default()
    }

    fn load_standing() -> ShardStanding {
        if let Ok(raw) = fs::read_to_string(SHARD_STANDING_PATH) {
            if let Ok(s) = ShardStanding::from_json(&raw) {
                return s;
            }
        }
        ShardStanding::default()
    }

    pub fn load_or_demo() -> Self {
        let climate = Self::load_climate();
        let standing = Self::load_standing();
        let climate_slab = standing
            .slab_line()
            .or_else(|| climate.slab_line())
            .map(|s| s.to_string());
        if let Ok(raw) = fs::read_to_string(LIVED_TICK_PATH) {
            if let Ok(hour) = LivedHour::from_json(&raw) {
                return Self {
                    hour,
                    climate,
                    standing,
                    last_line: "resumed".to_string(),
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
            last_line: "walk to a glow".to_string(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab,
        }
    }

    pub fn persist(&self) {
        if let Some(parent) = Path::new(LIVED_TICK_PATH).parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = self.hour.to_json() {
            let _ = fs::write(LIVED_TICK_PATH, json);
        }
        // Soft-fail climate / standing I/O — never block the hour.
        if let Ok(json) = self.climate.to_json() {
            let _ = fs::write(SHARD_CLIMATE_PATH, json);
        }
        if let Ok(json) = self.standing.to_json() {
            let _ = fs::write(SHARD_STANDING_PATH, json);
        }
    }

    fn refresh_climate_slab(&mut self) {
        self.climate_slab = self
            .standing
            .slab_line()
            .or_else(|| self.climate.slab_line())
            .map(|s| s.to_string());
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
            None => TendResult::NoTake {
                reason: "no glow",
            },
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
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: Some(2),
            climate_slab: None,
        };
        assert!(matches!(bind.tend_nearest(), TendResult::Taken { .. }));
        assert_eq!(bind.hour.nodes[1].state, NodeState::Tended);
        assert_eq!(bind.hour.nodes[0].state, NodeState::Glowing);
    }

    #[test]
    fn json_roundtrip_path_constant() {
        assert_eq!(LIVED_TICK_PATH, "data/powrush_lived_tick.json");
        assert_eq!(SHARD_CLIMATE_PATH, "data/powrush_shard_climate.json");
        assert_eq!(SHARD_STANDING_PATH, "data/powrush_shard_standing.json");
    }

    #[test]
    fn extract_on_tired_stays_no_take() {
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
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
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        };
        let _ = bind.tend(1);
        assert!(bind.allocate(AllocKind::Reserve));
        assert_eq!(bind.climate.reserve_pool, 1);
        assert!(bind.standing.steward > 0.40);
    }

    #[test]
    fn standing_lethal_stays_parked() {
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        };
        bind.care_tend();
        let _ = bind.tend(1);
        assert!(bind.allocate(AllocKind::Flow));
        assert!(!bind.standing.declared_lethal);
        assert_eq!(bind.standing.human_hybrid_heat, 0.0);
    }
}
