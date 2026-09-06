//! Client net mode resource — default Offline (v23.2.42)
//! Not a second HUD. Label only. Contact: info@Rathor.ai

use bevy::prelude::*;
use shared::net_mode::NetMode;

#[derive(Resource, Debug, Clone, Copy)]
pub struct SessionNetMode {
    pub mode: NetMode,
}

impl Default for SessionNetMode {
    fn default() -> Self {
        Self {
            mode: NetMode::Offline,
        }
    }
}

pub struct NetModePlugin;

impl Plugin for NetModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SessionNetMode>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_is_offline_without_peer_count() {
        let s = SessionNetMode::default();
        assert_eq!(s.mode, NetMode::Offline);
        assert!(s.mode.peer_count_for_peace_boot().is_none());
    }
}
