/*!
 * Soft-play ergonomic bindings — v21.96.0
 *
 * Derived from multi-persona human feedback under TOLC:
 * F-row is unreachable on laptops (Fn), far from WASD, and hostile to muscle memory.
 *
 * Principles:
 *   • Left-hand reach while right hand stays on mouse
 *   • Semantic letters (J=Journey, L=Lattice, T=Transport…)
 *   • Rare power actions use Shift chords
 *   • Core loop keys (P, R, Space) unchanged — already excellent
 *
 * Contact: info@Rathor.ai · PATSAGi · Yoi ⚡
 */

use bevy::prelude::KeyCode;

/// Abundance Journey Echo panel toggle.
pub const JOURNEY_ECHO: KeyCode = KeyCode::KeyJ;

/// Foundation Lattice educational overlay.
pub const FOUNDATION_LATTICE: KeyCode = KeyCode::KeyL;

/// Cycle Resonance Flavor.
pub const RESONANCE_CYCLE: KeyCode = KeyCode::KeyG;

/// Toggle Mercy Transporters.
pub const MERCY_TRANSPORTERS: KeyCode = KeyCode::KeyT;

/// Soft peer lattice ingest.
pub const PEER_INGEST: KeyCode = KeyCode::KeyU;

/// My Mercy Journey panel.
pub const MY_MERCY_JOURNEY: KeyCode = KeyCode::KeyM;

/// Realm / zone travel panel.
pub const REALM_TRAVEL: KeyCode = KeyCode::KeyZ;

/// Force Steam Auto-Cloud flush (rare) — use with Shift held.
pub const FORCE_CLOUD_FLUSH: KeyCode = KeyCode::KeyT;

/// Human-readable legend for UI footers and onboarding.
pub fn soft_play_legend() -> &'static str {
    "P practice · R allocate · J journey · L lattice · G resonance · T transporters · U peer · M mercy · Z realm · Shift+T cloud"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legend_mentions_core() {
        let l = soft_play_legend();
        assert!(l.contains('P'));
        assert!(l.contains('J'));
        assert!(l.contains('L'));
        assert!(l.contains("Shift+T"));
    }
}
