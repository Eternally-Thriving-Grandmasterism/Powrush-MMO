//! client/src/lived_hour_bind.rs
//! Bind first-hour hands to shared::climate_node::LivedHour.
//! E tend · I satchel · R 1 flow · R 2 reserve.
//! Persist: data/powrush_lived_tick.json
//! Does not replace harvest_feel or rbe_allocate_choice.

use std::fs;
use std::path::Path;

use bevy::prelude::*;
use shared::climate_node::{AllocKind, LivedHour, NodeState, TendResult};

pub const LIVED_TICK_PATH: &str = "data/powrush_lived_tick.json";

#[derive(Resource, Debug, Clone)]
pub struct LivedHourBind {
    pub hour: LivedHour,
    pub last_line: String,
    pub guidance_hidden: bool,
}

impl Default for LivedHourBind {
    fn default() -> Self {
        Self::load_or_demo()
    }
}

impl LivedHourBind {
    pub fn load_or_demo() -> Self {
        if let Ok(raw) = fs::read_to_string(LIVED_TICK_PATH) {
            if let Ok(hour) = LivedHour::from_json(&raw) {
                return Self {
                    hour,
                    last_line: "resumed".to_string(),
                    guidance_hidden: false,
                };
            }
        }
        Self {
            hour: LivedHour::new_demo(),
            last_line: "walk to a glow".to_string(),
            guidance_hidden: false,
        }
    }

    pub fn persist(&self) {
        if let Some(parent) = Path::new(LIVED_TICK_PATH).parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = self.hour.to_json() {
            let _ = fs::write(LIVED_TICK_PATH, json);
        }
    }

    /// E on a node id (nearest glow is the client's job).
    pub fn tend(&mut self, node_id: u32) -> TendResult {
        let result = self.hour.tend(node_id);
        self.last_line = match &result {
            TendResult::Taken { item } => format!("tended node {}", item.node_id),
            TendResult::NoTake { reason } => (*reason).to_string(),
        };
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
        match self.nearest_glow_id() {
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
                AllocKind::Flow => "flow restored a tired node".to_string(),
                AllocKind::Reserve => "reserve held as repair-rights".to_string(),
            }
        } else {
            "satchel empty".to_string()
        };
        self.persist();
        ok
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

pub struct LivedHourBindPlugin;

impl Plugin for LivedHourBindPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LivedHourBind>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tend_fills_satchel() {
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            last_line: String::new(),
            guidance_hidden: false,
        };
        assert!(matches!(bind.tend(1), TendResult::Taken { .. }));
        assert_eq!(bind.satchel_count(), 1);
    }

    #[test]
    fn flow_needs_a_take() {
        let mut bind = LivedHourBind {
            hour: LivedHour::new_demo(),
            last_line: String::new(),
            guidance_hidden: false,
        };
        assert!(!bind.allocate(AllocKind::Flow));
        let _ = bind.tend(1);
        assert!(bind.allocate(AllocKind::Flow));
        assert_eq!(bind.satchel_count(), 0);
        assert_eq!(bind.hour.allocation.flow, 1);
    }

    #[test]
    fn json_roundtrip_path_constant() {
        assert_eq!(LIVED_TICK_PATH, "data/powrush_lived_tick.json");
    }
}
