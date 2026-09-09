//! Net mode foundations — Offline | LoopbackDev | Online(grey) (v23.2.60)
//!
//! Default remains one human / one machine. Net is an explicit option.
//! Never invent peer counts on Peace-hour boot. server/ stays parked until wired.
//! Title Online stays grey — the Online variant is a label only, never a bind.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// How this session meets other humans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum NetMode {
    /// Lived hour. One human, one machine. Default binary.
    #[default]
    Offline,
    /// Loopback lab (127.0.0.1 / Settings LAN · loopback). Never a public bind.
    LoopbackDev,
    /// Title Online row — always grey. No listen. No public net.
    Online,
}

impl NetMode {
    pub fn label(self) -> &'static str {
        match self {
            NetMode::Offline => "offline · one human · one machine",
            NetMode::LoopbackDev => "loopback · 127.0.0.1 lab",
            NetMode::Online => "online · grey · no listen",
        }
    }

    /// Peace-hour boot must never show a fake population.
    pub fn peer_count_for_peace_boot(self) -> Option<u32> {
        match self {
            NetMode::Offline | NetMode::LoopbackDev | NetMode::Online => None,
        }
    }

    pub fn server_unparked(self) -> bool {
        // Constitution: do not unpark server/ in this slice.
        false
    }

    /// Title Online never lights from any foundation mode.
    pub fn title_online_enabled(self) -> bool {
        false
    }

    /// Default play path — no sockets.
    pub fn is_offline_default(self) -> bool {
        matches!(self, NetMode::Offline)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_offline() {
        assert_eq!(NetMode::default(), NetMode::Offline);
        assert!(NetMode::Offline.is_offline_default());
        assert!(NetMode::Offline.peer_count_for_peace_boot().is_none());
        assert!(!NetMode::Offline.server_unparked());
        assert!(!NetMode::Offline.title_online_enabled());
    }

    #[test]
    fn loopback_dev_never_fakes_peers_or_public_bind() {
        assert!(NetMode::LoopbackDev.peer_count_for_peace_boot().is_none());
        assert!(!NetMode::LoopbackDev.server_unparked());
        assert!(!NetMode::LoopbackDev.title_online_enabled());
        assert!(NetMode::LoopbackDev.label().contains("loopback"));
        assert!(!NetMode::LoopbackDev.is_offline_default());
    }

    #[test]
    fn online_stays_grey_no_listen() {
        assert_eq!(NetMode::Online.label(), "online · grey · no listen");
        assert!(NetMode::Online.peer_count_for_peace_boot().is_none());
        assert!(!NetMode::Online.server_unparked());
        assert!(!NetMode::Online.title_online_enabled());
        assert!(!NetMode::Online.is_offline_default());
    }
}
