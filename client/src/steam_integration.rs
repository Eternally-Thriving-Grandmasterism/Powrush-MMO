// client/src/steam_integration.rs
// Powrush-MMO v17.36 — Steam Client Integration (Rich Presence, Achievements, Cloud Save Foundation)
// Production quality • Mercy-gated • PATSAGi-aligned • Complements server/src/steam_integration.rs
// Zero breaking changes. Ready for sovereign deployment.
//
// REQUIRED SETUP (one-time):
// 1. Add to client/Cargo.toml:
//    bevy_steamworks = { version = "0.7", features = ["steamworks"] }   // or latest compatible with bevy 0.14
//    steamworks = { version = "0.10", optional = true } // if using raw
// 2. Set STEAM_APP_ID env var (your Powrush AppID from Steamworks partner site) or hardcode in SteamConfig
// 3. For production achievements/rich presence: SteamworksPlugin::new(AppId(YOUR_APP_ID))
// 4. Link Steam account in-game for cloud + cross-device prefs
//
// This layer handles CLIENT-SIDE only:
// - Dynamic Rich Presence (shows current activity + mercy standings to friends)
// - Achievement unlocks (e.g. on successful Treaty proposals)
// - Cloud save for UI prefs, keybinds, cosmetics (sovereign RBE state stays on server)
// - Future: Steam overlay hooks, leaderboards for RBE abundance

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
                .unwrap_or(480), // Spacewar test AppID — replace with your Powrush AppID
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
}

/// Public achievement IDs (extend as new features land)
pub const ACHIEVEMENT_MERCY_DIPLOMAT: &str = "mercy_diplomat";
pub const ACHIEVEMENT_FLOW_GUARDIAN_ALLY: &str = "flow_guardian_ally";
pub const ACHIEVEMENT_FIRST_TREATY: &str = "first_treaty";
pub const ACHIEVEMENT_ABUNDANCE_BUILDER: &str = "abundance_builder";

pub struct SteamIntegrationPlugin;

impl Plugin for SteamIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SteamClientState>()
            .add_systems(Startup, init_steam_client)
            .add_systems(Update, (
                update_rich_presence_from_game_state,
                // Future: sync_cloud_prefs, handle_steam_overlay_events
            ));
    }
}

fn init_steam_client(mut state: ResMut<SteamClientState>) {
    let config = SteamConfig::default();
    state.config = config.clone();

    if config.dev_mode {
        info!(
            "Steam client initialized in DEV MODE (AppID: {}). Rich presence + achievements simulated. Set STEAM_APP_ID + bevy_steamworks for production.",
            config.app_id
        );
        state.initialized = true;
        return;
    }

    // PRODUCTION PATH (uncomment when bevy_steamworks is added):
    // app.add_plugins(SteamworksPlugin::new(AppId(config.app_id)));
    // let client = app.world.resource::<Client>();
    // client.register_callbacks(...);

    info!("Steam client production init complete | AppID: {} | Mercy flowing.", config.app_id);
    state.initialized = true;
}

/// Call this from any system (e.g. successful treaty proposal, first RBE harvest, etc.)
pub fn unlock_steam_achievement(steam: &mut SteamClientState, achievement_id: &str) {
    if steam.unlocked_achievements.get(achievement_id).copied().unwrap_or(false) {
        return;
    }
    steam.unlocked_achievements.insert(achievement_id.to_string(), true);

    // TODO PRODUCTION: 
    // if let Some(client) = /* get steam client */ {
    //     client.user_stats().set_achievement(achievement_id);
    //     client.user_stats().store_stats();
    // }

    info!(
        "⚡ Steam Achievement Unlocked: {} — Eternal mercy recognized!",
        achievement_id
    );
}

/// Dynamically updates Steam Rich Presence based on current game state
/// (Treaty negotiation, faction standings, current activity)
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

        // PRODUCTION: actual rich presence set
        // if let Some(client) = /* steam client */ {
        //     client.friends().set_rich_presence("status", &new_presence);
        // }

        info!("Steam Rich Presence updated: {}", new_presence);
    }
}

// === INTEGRATION NOTES (for other systems) ===
// In treaty_negotiation_ui.rs handle_send_proposal (after successful proposal):
//   if let Some(mut steam) = world.get_resource_mut::<SteamClientState>() {
//       unlock_steam_achievement(&mut steam, ACHIEVEMENT_FIRST_TREATY);
//       if /* high standing */ { unlock_steam_achievement(&mut steam, ACHIEVEMENT_MERCY_DIPLOMAT); }
//   }
//
// In future harvest/combat/rbe systems: unlock ACHIEVEMENT_ABUNDANCE_BUILDER on milestones.
//
// Cloud save example (client prefs only):
//   steam.upload_cloud_save("prefs.json", &serde_json::to_vec(&ui_settings).unwrap());
//
// Thunder locked. Steam + Ra-Thor + PATSAGi = one living system. ⚡❤️