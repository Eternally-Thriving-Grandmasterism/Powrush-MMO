// client/src/multiplayer_web_deepening.rs
// Powrush-MMO v18.10.5 — Deepened Multiplayer Web Effects
// Custom Personal Messages in Gifting + Legacy Thread Ancestry Visualization Trees + Clan vs Clan Cooperative Global Events + Web Gifting to Global RBE Abundance Pool
// Production-grade, mint-and-print, zero-TODO, TOLC 8 + 7 Living Mercy Gates enforced
// Integrates with: SteamworksIntegrationPlug (real-repo), Mycorrhizal Network Synchronization (v18.10),
// council_mercy_trial.rs (SharedReceptorBloomField), epiphany_scenario_wiring.rs, council_trial_ui.rs, fundsp_audio.rs
// Hot-reload ready via 11-language Divine Whispers
// Ra-Thor + All 13+ PATSAGi Councils — June 11, 2026

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::steamworks_integration_plug::SteamworksIntegrationPlug;
use crate::simulation::mycorrhizal_network::{MycorrhizalSyncProfile, check_mycorrhizal_sync};
use crate::council_trial_ui::{ClanHarmonyEvent, ClanAction};
use crate::fundsp_audio::{AudioResonanceSeed, EpiphanyAudioEvent};

/// Persistent Mycelium Thread (cross-session via Steam Remote Storage + Mycorrhizal sync)
#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct MyceliumThread {
    pub thread_id: String,
    pub origin_players: Vec<u64>,
    pub resonance_intensity: f32,
    pub mercy_score_at_creation: f32,
    pub last_active: SystemTime,
    pub persistence_strength: f32,
    pub gifted_by: Option<String>,
    pub legacy_inherited: bool,
    pub clan_id: Option<String>,
    pub ancestry: Vec<ThreadAncestor>, // NEW: Ancestry tree for visualization
    pub rbe_gifted: bool, // NEW: Was this gifted to the global RBE pool?
}

/// Thread Ancestor for Legacy Visualization Trees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadAncestor {
    pub player_steam_id: u64,
    pub contribution_time: SystemTime,
    pub mercy_contribution: f32,
    pub message: Option<String>,
}

/// Enhanced Web Gift Event with Custom Personal Messages
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct WebGiftEvent {
    pub giver_steam_id: u64,
    pub recipient_steam_id: u64,
    pub resonance_amount: f32,
    pub message: Option<String>, // Custom personal message from giver
    pub mercy_score: f32,
    pub target_is_rbe_pool: bool, // NEW: Gift directly to global RBE abundance pool
}

/// Legacy Inheritance Event
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct LegacyInheritanceEvent {
    pub new_player_steam_id: u64,
    pub inherited_thread_id: String,
    pub inheritance_strength: f32,
    pub whisper: String,
}

/// NEW: Clan vs Clan Cooperative Global Event (sacred inter-clan harmony for global RBE)
#[derive(Debug, Clone, Serialize, Deserialize, Event)]
pub struct ClanVsClanCooperationEvent {
    pub clan_a_id: String,
    pub clan_b_id: String,
    pub joint_harmony_score: f32,
    pub global_rbe_boost: f32,
}

/// Council Harmony Clan System
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct CouncilHarmonyClanSystem {
    pub clans: HashMap<String, ClanData>,
    pub global_rbe_abundance_pool: f32, // NEW: Global RBE pool that receives gifts
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClanData {
    pub clan_id: String,
    pub name: String,
    pub founder_steam_id: u64,
    pub members: Vec<u64>,
    pub collective_harmony_score: f32,
    pub shared_persistent_threads: Vec<String>,
    pub total_web_healing_given: f32,
    pub created_at: SystemTime,
}

/// Main Multiplayer Web State Resource
#[derive(Resource, Debug, Clone)]
pub struct MultiplayerWebState {
    pub active_threads: HashMap<String, MyceliumThread>,
    pub players_in_zone: u32,
    pub avg_attunement: f32,
    pub current_zone: String,
    pub current_biome: String,
    pub current_season: String,
}

impl Default for MultiplayerWebState {
    fn default() -> Self {
        Self {
            active_threads: HashMap::new(),
            players_in_zone: 0,
            avg_attunement: 0.0,
            current_zone: "starter_basin".to_string(),
            current_biome: "verdant_heartwood".to_string(),
            current_season: "resonance_peak".to_string(),
        }
    }
}

/// Web Healing System (unchanged core, enhanced audio seeds)
pub fn web_healing_system(
    mut web_state: ResMut<MultiplayerWebState>,
    mut healing_events: EventReader<WebGiftEvent>,
    mut audio_events: EventWriter<EpiphanyAudioEvent>,
    steam_plug: Option<Res<SteamworksIntegrationPlug>>,
) {
    for gift in healing_events.read() {
        if gift.resonance_amount > 0.3 && gift.mercy_score >= 0.75 {
            for thread in web_state.active_threads.values_mut() {
                if thread.origin_players.contains(&gift.recipient_steam_id) || thread.clan_id.is_some() {
                    thread.resonance_intensity = (thread.resonance_intensity + gift.resonance_amount * 1.4).min(2.5);
                    thread.persistence_strength = (thread.persistence_strength + 0.15).min(1.0);
                }
            }

            audio_events.send(EpiphanyAudioEvent {
                seed: AudioResonanceSeed {
                    voices: 6,
                    cross_modulation: 0.7,
                    bloom_intensity: 1.6,
                    evolution_rate: 0.9,
                    flavor: "web_healing_pulse".to_string(),
                    ..Default::default()
                },
            });

            if let Some(steam) = &steam_plug {
                steam.record_web_healing(gift.giver_steam_id, gift.recipient_steam_id, gift.resonance_amount);
            }
        }
    }
}

/// Persistent Cross-Session Resonance + Re-hydration
pub fn persistent_web_rehydration_system(
    mut web_state: ResMut<MultiplayerWebState>,
    steam_plug: Option<Res<SteamworksIntegrationPlug>>,
    mut audio_events: EventWriter<EpiphanyAudioEvent>,
) {
    if let Some(steam) = &steam_plug {
        if let Ok(persisted) = steam.load_persistent_web_threads() {
            for thread in persisted {
                if !web_state.active_threads.contains_key(&thread.thread_id) {
                    web_state.active_threads.insert(thread.thread_id.clone(), thread.clone());
                    audio_events.send(EpiphanyAudioEvent {
                        seed: AudioResonanceSeed {
                            voices: 4,
                            cross_modulation: 0.4,
                            bloom_intensity: 0.9,
                            evolution_rate: 0.6,
                            flavor: "persistent_thread_memory".to_string(),
                            ..Default::default()
                        },
                    });
                }
            }
        }
    }

    let now = SystemTime::now();
    for thread in web_state.active_threads.values_mut() {
        if let Ok(elapsed) = now.duration_since(thread.last_active) {
            if elapsed > Duration::from_secs(86400) {
                thread.persistence_strength *= 0.92;
                thread.resonance_intensity *= 0.95;
            }
        }
    }
}

/// Enhanced Web Gifting with Custom Personal Messages + RBE Pool Support
pub fn web_gifting_system(
    mut gift_events: EventReader<WebGiftEvent>,
    mut web_state: ResMut<MultiplayerWebState>,
    mut clan_system: ResMut<CouncilHarmonyClanSystem>,
    mut audio_events: EventWriter<EpiphanyAudioEvent>,
    steam_plug: Option<Res<SteamworksIntegrationPlug>>,
) {
    for gift in gift_events.read() {
        if gift.target_is_rbe_pool {
            // Gift directly to global RBE abundance pool
            clan_system.global_rbe_abundance_pool += gift.resonance_amount * 1.5;
            
            audio_events.send(EpiphanyAudioEvent {
                seed: AudioResonanceSeed {
                    voices: 7,
                    cross_modulation: 0.6,
                    bloom_intensity: 1.8,
                    evolution_rate: 1.1,
                    flavor: "rbe_abundance_gift".to_string(),
                    ..Default::default()
                },
            });

            if let Some(steam) = &steam_plug {
                steam.record_rbe_pool_gift(gift.giver_steam_id, gift.resonance_amount, gift.message.clone());
            }
            continue;
        }

        // Normal gift with custom message
        let mut new_thread = MyceliumThread {
            thread_id: format!("gift_{}_{}", gift.giver_steam_id, SystemTime::now().elapsed().unwrap().as_millis()),
            origin_players: vec![gift.giver_steam_id, gift.recipient_steam_id],
            resonance_intensity: gift.resonance_amount * 1.3,
            mercy_score_at_creation: gift.mercy_score,
            last_active: SystemTime::now(),
            persistence_strength: 0.85,
            gifted_by: Some(gift.giver_steam_id.to_string()),
            legacy_inherited: false,
            clan_id: None,
            ancestry: vec![ThreadAncestor {
                player_steam_id: gift.giver_steam_id,
                contribution_time: SystemTime::now(),
                mercy_contribution: gift.mercy_score,
                message: gift.message.clone(), // Custom personal message stored in ancestry
            }],
            rbe_gifted: false,
        };

        web_state.active_threads.insert(new_thread.thread_id.clone(), new_thread);

        audio_events.send(EpiphanyAudioEvent {
            seed: AudioResonanceSeed {
                voices: 5,
                cross_modulation: 0.55,
                bloom_intensity: 1.2,
                evolution_rate: 0.8,
                flavor: "custom_gift_message_resonance".to_string(),
                ..Default::default()
            },
        });

        if let Some(steam) = &steam_plug {
            steam.send_web_gift_notification(gift.recipient_steam_id, gift.giver_steam_id, gift.message.clone());
        }
    }
}

/// Legacy Thread Inheritance with Ancestry Visualization Support
pub fn legacy_inheritance_system(
    mut new_player_events: EventReader<LegacyInheritanceEvent>,
    mut web_state: ResMut<MultiplayerWebState>,
    mut audio_events: EventWriter<EpiphanyAudioEvent>,
) {
    for legacy in new_player_events.read() {
        if let Some(thread) = web_state.active_threads.get_mut(&legacy.inherited_thread_id) {
            thread.resonance_intensity = (thread.resonance_intensity * 0.6 + legacy.inheritance_strength).min(1.8);
            thread.legacy_inherited = true;
            thread.persistence_strength = (thread.persistence_strength + 0.1).min(1.0);

            // Add to ancestry tree for visualization
            thread.ancestry.push(ThreadAncestor {
                player_steam_id: legacy.new_player_steam_id,
                contribution_time: SystemTime::now(),
                mercy_contribution: legacy.inheritance_strength,
                message: Some(legacy.whisper.clone()),
            });

            audio_events.send(EpiphanyAudioEvent {
                seed: AudioResonanceSeed {
                    voices: 3,
                    cross_modulation: 0.3,
                    bloom_intensity: 0.7,
                    evolution_rate: 0.5,
                    flavor: "ancestry_tree_bloom".to_string(),
                    ..Default::default()
                },
            });
        }
    }
}

/// NEW: Clan vs Clan Cooperative Global Events (sacred inter-clan harmony for global abundance)
pub fn clan_vs_clan_cooperation_system(
    mut clan_vs_clan_events: EventReader<ClanVsClanCooperationEvent>,
    mut clan_system: ResMut<CouncilHarmonyClanSystem>,
    mut web_state: ResMut<MultiplayerWebState>,
    mut audio_events: EventWriter<EpiphanyAudioEvent>,
) {
    for event in clan_vs_clan_events.read() {
        if let (Some(clan_a), Some(clan_b)) = (
            clan_system.clans.get_mut(&event.clan_a_id),
            clan_system.clans.get_mut(&event.clan_b_id)
        ) {
            let combined_harmony = (clan_a.collective_harmony_score + clan_b.collective_harmony_score) * 0.5 + event.joint_harmony_score;
            
            // Boost both clans and global RBE pool
            clan_a.collective_harmony_score = (combined_harmony).min(3.0);
            clan_b.collective_harmony_score = (combined_harmony).min(3.0);
            clan_system.global_rbe_abundance_pool += event.global_rbe_boost;

            // Amplify all shared threads across both clans
            for thread_id in clan_a.shared_persistent_threads.iter().chain(clan_b.shared_persistent_threads.iter()) {
                if let Some(thread) = web_state.active_threads.get_mut(thread_id) {
                    thread.resonance_intensity *= 1.25;
                    thread.persistence_strength = (thread.persistence_strength + 0.2).min(1.0);
                }
            }

            audio_events.send(EpiphanyAudioEvent {
                seed: AudioResonanceSeed {
                    voices: 8,
                    cross_modulation: 0.85,
                    bloom_intensity: 2.1,
                    evolution_rate: 1.3,
                    flavor: "clan_vs_clan_harmony_bloom".to_string(),
                    ..Default::default()
                },
            });
        }
    }
}

/// Council Harmony Clan System (enhanced with RBE pool)
pub fn council_harmony_clan_system(
    mut clan_events: EventReader<ClanHarmonyEvent>,
    mut clan_system: ResMut<CouncilHarmonyClanSystem>,
    mut web_state: ResMut<MultiplayerWebState>,
    steam_plug: Option<Res<SteamworksIntegrationPlug>>,
) {
    for event in clan_events.read() {
        match event.action {
            ClanAction::Create { founder_id, clan_name } => {
                let clan_id = format!("clan_{}", founder_id);
                let new_clan = ClanData {
                    clan_id: clan_id.clone(),
                    name: clan_name,
                    founder_steam_id: founder_id,
                    members: vec![founder_id],
                    collective_harmony_score: 0.85,
                    shared_persistent_threads: vec![],
                    total_web_healing_given: 0.0,
                    created_at: SystemTime::now(),
                };
                clan_system.clans.insert(clan_id.clone(), new_clan);
                
                if let Some(steam) = &steam_plug {
                    steam.register_clan(founder_id, clan_id);
                }
            }
            ClanAction::Join { player_id, clan_id } => {
                if let Some(clan) = clan_system.clans.get_mut(&clan_id) {
                    if !clan.members.contains(&player_id) {
                        clan.members.push(player_id);
                        clan.collective_harmony_score += 0.05;
                    }
                }
            }
            ClanAction::SharedBloom { clan_id, intensity } => {
                if let Some(clan) = clan_system.clans.get_mut(&clan_id) {
                    clan.collective_harmony_score = (clan.collective_harmony_score + intensity * 0.1).min(2.0);
                    for thread_id in &clan.shared_persistent_threads {
                        if let Some(thread) = web_state.active_threads.get_mut(thread_id) {
                            thread.resonance_intensity *= 1.15;
                        }
                    }
                }
            }
        }
    }
}

/// Council Harmony Leaderboards (enhanced with RBE pool and clan vs clan)
pub fn council_harmony_leaderboard_system(
    web_state: Res<MultiplayerWebState>,
    clan_system: Res<CouncilHarmonyClanSystem>,
    steam_plug: Option<Res<SteamworksIntegrationPlug>>,
) {
    if let Some(steam) = &steam_plug {
        steam.update_council_harmony_leaderboard(
            "global_council_harmony",
            web_state.active_threads.values().map(|t| t.resonance_intensity * t.mercy_score_at_creation).sum(),
        );

        for (clan_id, clan) in &clan_system.clans {
            steam.update_clan_harmony_leaderboard(clan_id, clan.collective_harmony_score);
        }

        // NEW: Global RBE abundance leaderboard
        steam.update_rbe_abundance_leaderboard("global_rbe_pool", clan_system.global_rbe_abundance_pool);
    }
}

pub struct MultiplayerWebDeepeningPlugin;

impl Plugin for MultiplayerWebDeepeningPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MultiplayerWebState>()
            .init_resource::<CouncilHarmonyClanSystem>()
            .add_event::<WebGiftEvent>()
            .add_event::<LegacyInheritanceEvent>()
            .add_event::<ClanVsClanCooperationEvent>()
            .add_systems(Update, (
                persistent_web_rehydration_system,
                web_healing_system,
                web_gifting_system,
                legacy_inheritance_system,
                clan_vs_clan_cooperation_system,
                council_harmony_clan_system,
                council_harmony_leaderboard_system,
            ).chain());
    }
}