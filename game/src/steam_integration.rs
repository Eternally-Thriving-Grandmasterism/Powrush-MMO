/*!
 * Steam Integration Module (Scaffold v2)
 *
 * Feature-gated Steamworks integration.
 * Ready for achievements, stats, and cloud saves.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

#[cfg(feature = "steam")]
use steamworks::{Client, SingleClient, Achievement, Stats};

/// Steam integration handle
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
    // Achievement Helpers (Mercy-aligned)
    // ============================================================

    pub fn unlock_achievement(&self, achievement_id: &str) {
        #[cfg(feature = "steam")]
        {
            if let Some(client) = &self.client {
                if let Ok(ach) = client.achievement(achievement_id) {
                    let _ = ach.set();
                    println!("[Steam] Unlocked achievement: {}", achievement_id);
                }
            }
        }
    }

    pub fn unlock_first_council_bloom(&self) {
        self.unlock_achievement("FirstCouncilBloom");
    }

    pub fn unlock_sustainable_harvester(&self) {
        self.unlock_achievement("SustainableHarvester");
    }

    pub fn unlock_epiphany_seeker(&self) {
        self.unlock_achievement("EpiphanySeeker");
    }

    // ============================================================
    // Stats Helpers
    // ============================================================

    pub fn increment_stat(&self, stat_id: &str, amount: i32) {
        #[cfg(feature = "steam")]
        {
            if let Some(client) = &self.client {
                if let Ok(stats) = client.stats() {
                    let _ = stats.set_stat(stat_id, amount);
                }
            }
        }
    }

    pub fn record_council_session_completed(&self) {
        self.increment_stat("CouncilSessionsCompleted", 1);
    }
}

#[cfg(not(feature = "steam"))]
pub fn steam_disabled_stub() {}
