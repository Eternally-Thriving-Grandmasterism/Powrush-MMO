//! Client net mode resource — default Offline (v23.2.60 / F8)
//!
//! shared::NetMode: Offline (default) | LoopbackDev | Online (grey, never lit).
//! Outbound WS only when POWRUSH_NET=localhost **or** Settings LAN · loopback.
//! Title Online stays grey and does not bind. Never POWRUSH_NET=on from Title.
//! LAN off (default): no listen, no outbound — boot exactly as today.
//! Loopback: existing F8 door, 127.0.0.1 only; may carry L0 GenShare + climate.
//! Never invent peer counts. No second net stack. No listen server. Contact: info@Rathor.ai

use std::net::SocketAddr;

use bevy::prelude::*;
use shared::genshare::{load_genshare, GenShare};
use shared::hex_listen::{
    default_localhost_shard_url, loopback_share_payload, parse_powrush_net, resolve_powrush_net,
    settings_lan_listen_bind, PowrushNet,
};
use shared::net_mode::NetMode;
use shared::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

use crate::lived_hour_bind::LivedHourBind;
use crate::local_settings::LocalSettingsState;

#[derive(Resource, Debug, Clone)]
pub struct SessionNetMode {
    pub mode: NetMode,
    /// Parsed POWRUSH_NET (off | localhost).
    pub powrush_net: PowrushNet,
    /// True only when POWRUSH_NET=localhost — intent to dial loopback shard.
    pub outbound_ws: bool,
    /// Hint URL; unused unless outbound_ws.
    pub shard_url: &'static str,
    /// Settings LAN permitted listen. None when LAN off. Never 0.0.0.0.
    /// Client does not open a second stack — F8 shard is the listen implementation.
    pub lan_listen: Option<SocketAddr>,
}

/// Prepared loopback lab payload (L0 GenShare line + climate). None when LAN off.
#[derive(Resource, Debug, Default, Clone)]
pub struct LanLoopbackShare {
    pub payload: Option<String>,
}

impl Default for SessionNetMode {
    fn default() -> Self {
        Self::from_powrush_net(parse_powrush_net())
    }
}

impl SessionNetMode {
    /// Build from an explicit net flag — tests must not race on process env.
    pub fn from_powrush_net(powrush_net: PowrushNet) -> Self {
        let outbound_ws = powrush_net.may_outbound_ws();
        // LoopbackDev when localhost door is open; still no peer count / no Online light.
        // Online variant exists in shared for Title grey label — never selected by default boot.
        let mode = if outbound_ws {
            NetMode::LoopbackDev
        } else {
            NetMode::Offline
        };
        Self {
            mode,
            powrush_net,
            outbound_ws,
            shard_url: default_localhost_shard_url(),
            lan_listen: None,
        }
    }

    /// Settings LAN + existing env flag. LAN off + env unset = today's Offline boot.
    /// LAN loopback → existing localhost door. Never sets POWRUSH_NET=on.
    /// Title Online stays disabled. Listen addr is 127.0.0.1 only when LAN loopback.
    pub fn from_lan_and_env(lan: &str, env: Option<&str>) -> Self {
        let powrush_net = resolve_powrush_net(lan, env);
        let mut s = Self::from_powrush_net(powrush_net);
        s.lan_listen = settings_lan_listen_bind(lan).ok();
        debug_assert!(!s.title_online_enabled());
        debug_assert!(s.lan_listen.map(|a| a.ip().is_loopback()).unwrap_or(true));
        s
    }

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
        app.init_resource::<SessionNetMode>()
            .init_resource::<LanLoopbackShare>()
            .add_systems(Update, sync_lan_door_from_settings);
    }
}

/// LAN off: session net follows env only (default no listen, no outbound).
/// LAN loopback: existing F8 localhost flag + 127.0.0.1 listen permit.
/// Prepares L0 GenShare line + climate only. No fake peers. Title Online untouched.
fn sync_lan_door_from_settings(
    settings: Res<LocalSettingsState>,
    bind: Option<Res<LivedHourBind>>,
    mut net: ResMut<SessionNetMode>,
    mut share: ResMut<LanLoopbackShare>,
) {
    let want = SessionNetMode::from_lan_and_env(
        settings.inner.lan.as_str(),
        std::env::var("POWRUSH_NET").ok().as_deref(),
    );
    if net.mode != want.mode
        || net.powrush_net != want.powrush_net
        || net.outbound_ws != want.outbound_ws
        || net.lan_listen != want.lan_listen
    {
        *net = want;
    }
    // Hard law: Title Online never binds from this door.
    debug_assert!(!net.title_online_enabled());

    if !settings.inner.lan_is_loopback() {
        if share.payload.is_some() {
            share.payload = None;
        }
        return;
    }

    let (genshare_line, climate_json) = match bind.as_ref() {
        Some(b) => {
            let hex = if b.climate.hex_id.trim().is_empty() {
                "local-hex"
            } else {
                b.climate.hex_id.as_str()
            };
            let line = load_genshare(hex, None)
                .and_then(|row| GenShare::to_json_line(&row).ok())
                .unwrap_or_default();
            let climate = b.climate.to_json().unwrap_or_default();
            (line, climate)
        }
        None => (String::new(), String::new()),
    };
    let payload = loopback_share_payload(&genshare_line, &climate_json);
    if share.payload != payload {
        share.payload = payload;
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
        assert!(online_row_is_honest_disabled(
            ONLINE_STUB_LABEL,
            s.title_online_enabled()
        ));
    }

    #[test]
    fn powrush_net_localhost_gates_connect_title_stays_grey() {
        assert_eq!(
            parse_powrush_net_from(Some("localhost")),
            PowrushNet::Localhost
        );
        let s = SessionNetMode::from_powrush_net(PowrushNet::Localhost);
        assert!(s.outbound_ws);
        assert!(s.powrush_net.may_outbound_ws());
        assert!(!s.opens_listen_socket());
        assert!(!s.title_online_enabled());
        assert_eq!(s.mode, NetMode::LoopbackDev);
        assert!(s.mode.peer_count_for_peace_boot().is_none());
        assert!(!s.mode.title_online_enabled());
        assert!(!NetMode::Online.title_online_enabled());
        // Title Online row remains honest grey — Settings/env is the door.
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!online_row_is_honest_disabled(ONLINE_STUB_LABEL, true));
    }

    #[test]
    fn lan_off_boots_like_today_no_listen_no_outbound() {
        let s = SessionNetMode::from_lan_and_env("off", None);
        assert_eq!(s.mode, NetMode::Offline);
        assert!(!s.outbound_ws);
        assert!(s.lan_listen.is_none());
        assert!(!s.opens_listen_socket());
        assert!(!s.title_online_enabled());
        assert!(s.mode.peer_count_for_peace_boot().is_none());
        assert!(online_row_is_honest_disabled(
            ONLINE_STUB_LABEL,
            s.title_online_enabled()
        ));
    }

    #[test]
    fn lan_loopback_reuses_f8_localhost_title_online_does_not_bind() {
        let s = SessionNetMode::from_lan_and_env("loopback", None);
        assert_eq!(s.powrush_net, PowrushNet::Localhost);
        assert!(s.outbound_ws);
        assert!(!s.opens_listen_socket());
        assert!(!s.title_online_enabled());
        let addr = s.lan_listen.expect("127.0.0.1 listen permit");
        assert_eq!(addr.ip().to_string(), "127.0.0.1");
        assert_eq!(addr.port(), 7788);
        assert_eq!(s.shard_url, "ws://127.0.0.1:7788");
        // Unknown / on does not open the door.
        let off = SessionNetMode::from_lan_and_env("on", Some("on"));
        assert!(!off.outbound_ws);
        assert!(off.lan_listen.is_none());
        assert!(!off.title_online_enabled());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
    }
}
