/*!
 * Soft-play + core locomotion bindings — v21.97.0
 *
 * Mainstream muscle memory (PC Gamer, ARK, Starfield, CoD, Warframe):
 *   Space = jump · E = interact · Shift = sprint · WASD = move
 *
 * Soft-play educational layers stay semantic left-hand letters.
 *
 * Contact: info@Rathor.ai · PATSAGi · Yoi ⚡
 */

use bevy::prelude::KeyCode;

// ── Core locomotion (never remap casually) ──────────────────────────

/// Jump — universal PC muscle memory (thumb on Space).
pub const JUMP: KeyCode = KeyCode::Space;

/// Interact / use / soft harvest — mainstream open-world (E).
pub const INTERACT: KeyCode = KeyCode::KeyE;

/// Sprint (held) — Shift beside WASD.
pub const SPRINT_LEFT: KeyCode = KeyCode::ShiftLeft;
pub const SPRINT_RIGHT: KeyCode = KeyCode::ShiftRight;

// ── Soft-play educational surfaces ──────────────────────────────────

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
    "WASD move · Space jump · E interact · Shift sprint · P practice · R allocate · J journey · L lattice · G resonance · T transporters · U peer · M mercy · Z realm · Shift+T cloud"
}

/// Short first-session locomotion line.
pub fn locomotion_legend() -> &'static str {
    "WASD move · Space jump · E interact · Shift sprint"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_keys_are_mainstream() {
        assert_eq!(JUMP, KeyCode::Space);
        assert_eq!(INTERACT, KeyCode::KeyE);
    }

    #[test]
    fn legend_mentions_jump_and_interact() {
        let l = soft_play_legend();
        assert!(l.contains("Space jump"));
        assert!(l.contains("E interact"));
        assert!(l.contains("Shift sprint"));
    }
}
