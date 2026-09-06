//! T-net — honest network mode label (v23.2.42)
//!
//! Default remains one human / one machine. Net is an explicit option.
//! Never invent peer counts on Peace-hour boot. server/ stays parked until wired.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// How this session meets other humans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum NetMode {
    /// Lived hour. One human, one machine. Default.
    #[default]
    Offline,
    /// Honest shard — only when a real peer path exists. Not enabled by default.
    HonestShard,
}

impl NetMode {
    pub fn label(self) -> &'static str {
        match self {
            NetMode::Offline => "offline · one human · one machine",
            NetMode::HonestShard => "honest shard · labelled net mode",
        }
    }

    /// Peace-hour boot must never show a fake population.
    pub fn peer_count_for_peace_boot(self) -> Option<u32> {
        match self {
            NetMode::Offline => None,
            // Until server/ is honestly wired, HonestShard still reports unknown — never a fake N.
            NetMode::HonestShard => None,
        }
    }

    pub fn server_unparked(self) -> bool {
        // Constitution: do not unpark server/ in this slice.
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_offline() {
        assert_eq!(NetMode::default(), NetMode::Offline);
        assert!(NetMode::Offline.peer_count_for_peace_boot().is_none());
        assert!(!NetMode::Offline.server_unparked());
    }

    #[test]
    fn honest_shard_never_fakes_peers() {
        assert!(NetMode::HonestShard.peer_count_for_peace_boot().is_none());
        assert!(!NetMode::HonestShard.server_unparked());
        assert!(NetMode::HonestShard.label().contains("honest"));
    }
}
