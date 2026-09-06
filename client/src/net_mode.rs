//! Client net mode resource — default Offline (v23.2.57 / F8)
//!
//! Outbound WS only when POWRUSH_NET=localhost. Title Online stays grey.
//! Never invent peer counts. No listen socket. Contact: info@Rathor.ai

use bevy::prelude::*;
use shared::hex_listen::{default_localhost_shard_url, parse_powrush_net, PowrushNet};
use shared::net_mode::NetMode;
use shared::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

#[derive(Resource, Debug, Clone)]
pub struct SessionNetMode {
    pub mode: NetMode,
    /// Parsed POWRUSH_NET (off | localhost).
    pub powrush_net: PowrushNet,
    /// True only when POWRUSH_NET=localhost — intent to dial loopback shard.
    pub outbound_ws: bool,
    /// Hint URL; unused unless outbound_ws.
    pub shard_url: &'static str,
}

impl Default for SessionNetMode {
    fn default() -> Self {
        let powrush_net = parse_powrush_net();
        let outbound_ws = powrush_net.may_outbound_ws();
        // HonestShard label only when localhost door is open; still no peer count / no Online light.
        let mode = if outbound_ws {
            NetMode::HonestShard
        } else {
            NetMode::Offline
        };
        Self {
            mode,
            powrush_net,
            outbound_ws,
            shard_url: default_localhost_shard_url(),
        }
    }

    /// Build from an explicit net flag — tests must not race on process env.
    pub fn from_powrush_net(powrush_net: PowrushNet) -> Self {
        let outbound_ws = powrush_net.may_outbound_ws();
        let mode = if outbound_ws {
            NetMode::HonestShard
        } else {
            NetMode::Offline
        };
        Self {
            mode,
            powrush_net,
            outbound_ws,
            shard_url: default_localhost_shard_url(),
        }
    }
}

impl SessionNetMode {
    /// Client never listens. Shard bin is the listen door.
    pub fn opens_listen_socket(&self) -> bool {
        self.powrush_net.opens_listen_socket()
    }

    /// Title Online stays disabled/grey even when localhost outbound is allowed.
    pub fn title_online_enabled(&self) -> bool {
        self.powrush_net.title_online_enabled()
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
    use shared::hex_listen::parse_powrush_net_from;

    #[test]
    fn boot_is_offline_without_peer_count() {
        let s = SessionNetMode::from_powrush_net(PowrushNet::Off);
        assert_eq!(s.mode, NetMode::Offline);
        assert_eq!(s.powrush_net, PowrushNet::Off);
        assert!(!s.outbound_ws);
        assert!(!s.opens_listen_socket());
        assert!(s.mode.peer_count_for_peace_boot().is_none());
        assert!(!PowrushNet::Off.may_outbound_ws());
        assert!(!PowrushNet::Off.opens_listen_socket());
    }

    #[test]
    fn powrush_net_off_asserts_no_listen_no_outbound() {
        assert_eq!(parse_powrush_net_from(Some("off")), PowrushNet::Off);
        let s = SessionNetMode::from_powrush_net(PowrushNet::Off);
        assert!(!s.outbound_ws);
        assert!(!s.opens_listen_socket());
        assert!(!s.title_online_enabled());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, s.title_online_enabled()));
    }

    #[test]
    fn powrush_net_localhost_gates_connect_title_stays_grey() {
        assert_eq!(parse_powrush_net_from(Some("localhost")), PowrushNet::Localhost);
        let s = SessionNetMode::from_powrush_net(PowrushNet::Localhost);
        assert!(s.outbound_ws);
        assert!(s.powrush_net.may_outbound_ws());
        assert!(!s.opens_listen_socket());
        assert!(!s.title_online_enabled());
        assert_eq!(s.mode, NetMode::HonestShard);
        assert!(s.mode.peer_count_for_peace_boot().is_none());
        // Title Online row remains honest grey — Settings/env is the door.
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!online_row_is_honest_disabled(ONLINE_STUB_LABEL, true));
    }
}
