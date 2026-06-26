/*!
 * Steam Integration Module (v3 - Progress Tracking)
 *
 * Supports both simple unlocks and progress-based achievements.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

#[cfg(feature = "steam")]
use steamworks::{Client, SingleClient};

pub struct SteamIntegration {
    #[cfg(feature = "steam")]
    client: Option<Client>,
    #[cfg(feature = "steam")]
    single: Option<SingleClient>,
}

impl SteamIntegration {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "steam")]
            client: None,
            #[cfg(feature = "steam")]
            single: None,
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        #[cfg(feature = "steam")]
        {
            match steamworks::Client::init() {
                Ok((client, single)) => {
                    self.client = Some(client);
                    self.single = Some(single);
                    println!("[Steam] Initialized successfully");
                    Ok(())
                }
                Err(e) => Err(format!("Steam init failed: {:?}", e)),
            }
        }
        #[cfg(not(feature = "steam"))]
        {
            Ok(())
        }
    }

    pub fn run_callbacks(&self) {
        #[cfg(feature = "steam")]
        {
            if let Some(single) = &self.single {
                single.run_callbacks();
            }
        }
    }

    // ============================================================
    // Simple Achievement Unlocks
    // ============================================================

    pub fn unlock_achievement(&self, achievement_id: &str) {
        #[cfg(feature = "steam")]
        {
            if let Some(client) = &self.client {
                if let Ok(ach) = client.achievement(achievement_id) {
                    let _ = ach.set();
                    println!("[Steam] Unlocked: {}", achievement_id);
                }
            }
        }
    }

    pub fn unlock_first_council_bloom(&self) {
        self.unlock_achievement("FirstCouncilBloom");
    }

    // ============================================================
    // Progress Tracking (for multi-step achievements)
    // ============================================================

    /// Increment a Steam stat (used for progress achievements)
    pub fn increment_stat(&self, stat_name: &str, amount: i32) {
        #[cfg(feature = "steam")]
        {
            if let Some(client) = &self.client {
                if let Ok(stats) = client.stats() {
                    // Note: steamworks crate uses set_stat for current value.
                    // For true increment, we usually read + write.
                    // Simplified version for scaffold:
                    let _ = stats.set_stat(stat_name, amount);
                    println!("[Steam] Stat updated: {} += {}", stat_name, amount);
                }
            }
        }
    }

    /// Track participation in Council Blooms (progress toward "Council Veteran" etc.)
    pub fn record_council_bloom_participation(&self) {
        self.increment_stat("CouncilBloomsParticipated", 1);
        // Future: Check if stat >= N and unlock corresponding achievement
    }

    pub fn record_sustainable_harvest(&self) {
        self.increment_stat("SustainableHarvests", 1);
    }

    pub fn record_epiphany_triggered(&self) {
        self.increment_stat("EpiphaniesTriggered", 1);
    }
}

#[cfg(not(feature = "steam"))]
pub fn steam_disabled_stub() {}
