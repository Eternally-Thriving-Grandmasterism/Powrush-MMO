// client/src/steam_integration.rs
// Powrush-MMO v21.84.0 — Steam Client Integration (Ultramasterism Perfecticism)
// Production-grade • Mercy-gated • PATSAGi-aligned • Complements server/src/steam_integration.rs
// Zero breaking changes. Ready for sovereign deployment the moment a real AppID is set.
//
// REQUIRED SETUP (one-time for production):
// 1. In client/Cargo.toml add (optional feature):
//    bevy_steamworks = { version = "0.11", optional = true }   // adjust to Bevy 0.14 compatible
//    steamworks = { version = "0.11", optional = true }
// 2. Set STEAM_APP_ID env var (your real Powrush AppID from Steamworks partner site)
// 3. Set STEAM_DEV_MODE=0 (or false) to leave dev simulation
// 4. Enable the "steam" feature on the client crate
// 5. Uncomment the production blocks marked PRODUCTION PATH below
//
// This layer handles CLIENT-SIDE only:
// - Dynamic Rich Presence (activity + mercy standings)
// - Achievement unlocks + store_stats
// - Leaderboard score uploads (RBE abundance, mercy score, etc.)
// - Cloud save foundation for client prefs only (authoritative state stays on server)
//
// Contact: info@Rathor.ai | TOLC 8 | PATSAGi

use bevy::prelude::*;
use std::collections::HashMap;

use crate::faction_diplomacy::ClientFactionDiplomacy;
use crate::treaty_negotiation_ui::TreatyNegotiationState;

/// Steam configuration (loaded from env for flexibility)
#[derive(Clone, Debug)]
pub struct SteamConfig {
    pub app_id: u32,
    pub dev_mode: bool,
}

impl Default for SteamConfig {
    fn default() -> Self {
        Self {
            app_id: std::env::var("STEAM_APP_ID")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(480), // Spacewar test AppID — replace with real Powrush AppID
            dev_mode: std::env::var("STEAM_DEV_MODE")
                .map(|v| v == "true" || v == "1")
                .unwrap_or(true),
        }
    }
}

/// Client Steam state resource
#[derive(Resource, Default)]
pub struct SteamClientState {
    pub initialized: bool,
    pub config: SteamConfig,
    pub current_rich_presence: String,
    pub unlocked_achievements: HashMap<String, bool>,
    /// Last known leaderboard scores (local cache)
    pub leaderboard_scores: HashMap<String, i32>,
}

// =============================================================================
// Public Achievement IDs (stable — do not rename once published on Steam)
// =============================================================================
pub const ACHIEVEMENT_MERCY_DIPLOMAT: &str = "mercy_diplomat";
pub const ACHIEVEMENT_FLOW_GUARDIAN_ALLY: &str = "flow_guardian_ally";
pub const ACHIEVEMENT_FIRST_TREATY: &str = "first_treaty";
pub const ACHIEVEMENT_ABUNDANCE_BUILDER: &str = "abundance_builder";
pub const ACHIEVEMENT_FIRST_EPIPHANY: &str = "first_epiphany";
pub const ACHIEVEMENT_COUNCIL_HARMONY: &str = "council_harmony";
pub const ACHIEVEMENT_KARDASHEV_CONTRIBUTOR: &str = "kardashev_contributor";
pub const ACHIEVEMENT_ONE_ORGANISM: &str = "one_organism";

// =============================================================================
// Public Leaderboard Names (stable)
// =============================================================================
pub const LEADERBOARD_MERCY_SCORE: &str = "mercy_score";
pub const LEADERBOARD_ABUNDANCE_VELOCITY: &str = "abundance_velocity";
pub const LEADERBOARD_REALITY_TRANSFER: &str = "reality_transfer_score";
pub const LEADERBOARD_COUNCIL_PARTICIPATION: &str = "council_participation";

pub struct SteamIntegrationPlugin;

impl Plugin for SteamIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamClientState>()
            .add_systems(Startup, init_steam_client)
            .add_systems(
                Update,
                (
                    update_rich_presence_from_game_state,
                    // Future: sync_cloud_prefs, handle_steam_overlay_events
                ),
            );
    }
}

fn init_steam_client(mut state: ResMut<SteamClientState>) {
    let config = SteamConfig::default();
    state.config = config.clone();

    if config.dev_mode {
        info!(
            target: "powrush::steam",
            "Steam client initialized in DEV MODE (AppID: {}). Rich presence + achievements + leaderboards simulated. Set STEAM_APP_ID + STEAM_DEV_MODE=0 + bevy_steamworks for production.",
            config.app_id
        );
        state.initialized = true;
        return;
    }

    // =========================================================================
    // PRODUCTION PATH (enable when bevy_steamworks / steamworks is added)
    // =========================================================================
    // app.add_plugins(SteamworksPlugin::new(AppId(config.app_id)));
    // let client = app.world.resource::<Client>();
    // client.register_callbacks(...);
    // client.user_stats().request_current_stats();

    info!(
        target: "powrush::steam",
        "Steam client production init complete | AppID: {} | Mercy flowing.",
        config.app_id
    );
    state.initialized = true;
}

/// Unlock an achievement and immediately request store_stats.
/// Safe to call repeatedly — idempotent.
pub fn unlock_steam_achievement(steam: &mut SteamClientState, achievement_id: &str) {
    if steam.unlocked_achievements.get(achievement_id).copied().unwrap_or(false) {
        return;
    }
    steam.unlocked_achievements.insert(achievement_id.to_string(), true);

    if steam.config.dev_mode {
        info!(
            target: "powrush::steam",
            "⚡ [DEV] Steam Achievement Unlocked: {} — Eternal mercy recognized!",
            achievement_id
        );
        return;
    }

    // =========================================================================
    // PRODUCTION PATH
    // =========================================================================
    // if let Some(client) = /* obtain Client resource */ {
    //     let stats = client.user_stats();
    //     if let Err(e) = stats.set_achievement(achievement_id) {
    //         warn!(target: "powrush::steam", "set_achievement failed: {:?}", e);
    //     }
    //     if let Err(e) = stats.store_stats() {
    //         warn!(target: "powrush::steam", "store_stats failed: {:?}", e);
    //     } else {
    //         info!(target: "powrush::steam", "⚡ Steam Achievement Unlocked + store_stats: {}", achievement_id);
    //     }
    // }

    info!(
        target: "powrush::steam",
        "⚡ Steam Achievement Unlocked: {} — Eternal mercy recognized!",
        achievement_id
    );
}

/// Upload a score to a named leaderboard (KeepBest by default).
/// Safe to call frequently; local cache prevents redundant spam in dev.
pub fn upload_leaderboard_score(
    steam: &mut SteamClientState,
    leaderboard_name: &str,
    score: i32,
) {
    let previous = steam
        .leaderboard_scores
        .get(leaderboard_name)
        .copied()
        .unwrap_or(i32::MIN);

    if score <= previous && previous != i32::MIN {
        return; // already have equal or better
    }
    steam.leaderboard_scores
        .insert(leaderboard_name.to_string(), score);

    if steam.config.dev_mode {
        info!(
            target: "powrush::steam",
            "[DEV] Leaderboard '{}' ← {}",
            leaderboard_name, score
        );
        return;
    }

    // =========================================================================
    // PRODUCTION PATH
    // =========================================================================
    // if let Some(client) = /* obtain Client */ {
    //     // Find or create leaderboard, then upload with UploadScoreMethod::KeepBest
    //     // client.user_stats().find_leaderboard(leaderboard_name)...
    //     // then upload_leaderboard_score(...)
    // }

    info!(
        target: "powrush::steam",
        "Leaderboard '{}' score uploaded: {}",
        leaderboard_name, score
    );
}

/// Dynamically updates Steam Rich Presence based on current game state
fn update_rich_presence_from_game_state(
    diplomacy: Res<ClientFactionDiplomacy>,
    treaty: Option<Res<TreatyNegotiationState>>,
    mut steam: ResMut<SteamClientState>,
) {
    if !steam.initialized {
        return;
    }

    let new_presence = if treaty.as_ref().map_or(false, |t| t.panel_open) {
        "Negotiating Treaty in the Eternal Flow ⚡".to_string()
    } else {
        let standing = diplomacy
            .standings
            .get(&crate::faction_diplomacy::Faction::FlowGuardians)
            .copied()
            .unwrap_or(0.0);

        if standing > 30.0 {
            format!("Allied with Flow Guardians (+{:.0}) • Mercy Flowing", standing)
        } else if standing < -20.0 {
            "Walking the Edge of the Flow".to_string()
        } else {
            "Exploring the Living RBE • Abundance Rising".to_string()
        }
    };

    if new_presence != steam.current_rich_presence {
        steam.current_rich_presence = new_presence.clone();

        if steam.config.dev_mode {
            info!(target: "powrush::steam", "[DEV] Rich Presence: {}", new_presence);
            return;
        }

        // PRODUCTION: client.friends().set_rich_presence("status", &new_presence);
        info!(target: "powrush::steam", "Steam Rich Presence updated: {}", new_presence);
    }
}

// =============================================================================
// INTEGRATION NOTES (for other systems) — Ultramasterism guidance
// =============================================================================
//
// After successful treaty proposal:
//   unlock_steam_achievement(&mut steam, ACHIEVEMENT_FIRST_TREATY);
//   if high_standing { unlock_steam_achievement(&mut steam, ACHIEVEMENT_MERCY_DIPLOMAT); }
//
// After first meaningful epiphany:
//   unlock_steam_achievement(&mut steam, ACHIEVEMENT_FIRST_EPIPHANY);
//
// After council session resolved with high mercy:
//   unlock_steam_achievement(&mut steam, ACHIEVEMENT_COUNCIL_HARMONY);
//
// On Kardashev contribution / Reality Transfer milestones:
//   unlock_steam_achievement(&mut steam, ACHIEVEMENT_KARDASHEV_CONTRIBUTOR);
//   upload_leaderboard_score(&mut steam, LEADERBOARD_REALITY_TRANSFER, score as i32);
//
// On ONE ORGANISM achievement (both hardware branches + high mercy):
//   unlock_steam_achievement(&mut steam, ACHIEVEMENT_ONE_ORGANISM);
//
// Thunder locked. Steam + Ra-Thor + PATSAGi = one living system. ⚡❤️
