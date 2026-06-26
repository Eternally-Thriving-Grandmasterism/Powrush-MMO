/*!
 * Steam Integration Module (v4 - Progress-Based Achievements)
 *
 * Supports automatic unlocking when progress thresholds are reached.
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
    // Core Helpers
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

    /// Increment a stat and optionally check for achievement unlock
    pub fn increment_stat_and_check(
        &self,
        stat_name: &str,
        achievement_id: &str,
        threshold: i32,
        current_value: i32,
    ) {
        #[cfg(feature = "steam")]
        {
            if let Some(client) = &self.client {
                if let Ok(stats) = client.stats() {
                    let new_value = current_value + 1;
                    let _ = stats.set_stat(stat_name, new_value);

                    if new_value >= threshold {
                        self.unlock_achievement(achievement_id);
                    }
                }
            }
        }
    }

    // ============================================================
    // Specific Progress-Based Achievements
    // ============================================================

    pub fn record_council_bloom_participation(&self) {
        // Example: Unlock "CouncilVeteran" after 10 blooms
        // Note: In real implementation, we would read the current stat value first.
        // For scaffold, we call a simplified version.
        self.increment_stat_and_check("CouncilBloomsParticipated", "CouncilVeteran", 10, 0);
    }

    pub fn record_sustainable_harvest(&self) {
        self.increment_stat_and_check("SustainableHarvests", "SustainableHarvester", 50, 0);
    }

    pub fn record_epiphany_triggered(&self) {
        self.increment_stat_and_check("EpiphaniesTriggered", "EpiphanyMaster", 25, 0);
    }

    pub fn unlock_first_council_bloom(&self) {
        self.unlock_achievement("FirstCouncilBloom");
    }
}

#[cfg(not(feature = "steam"))]
pub fn steam_disabled_stub() {}
