/*!
 * Steam Integration Module (v5 - Centralized Error Handling)
 *
 * All Steam-related error handling and logging is centralized here.
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
    initialized: bool,
}

impl SteamIntegration {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "steam")]
            client: None,
            #[cfg(feature = "steam")]
            single: None,
            initialized: false,
        }
    }

    /// Centralized initialization with internal error handling and logging
    pub fn initialize(&mut self) -> bool {
        #[cfg(feature = "steam")]
        {
            match steamworks::Client::init() {
                Ok((client, single)) => {
                    self.client = Some(client);
                    self.single = Some(single);
                    self.initialized = true;
                    println!("[Steam] Successfully initialized");
                    true
                }
                Err(e) => {
                    eprintln!("[Steam] Initialization failed: {:?}. Running without Steam.", e);
                    self.initialized = false;
                    false
                }
            }
        }

        #[cfg(not(feature = "steam"))]
        {
            true
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn run_callbacks(&self) {
        #[cfg(feature = "steam")]
        {
            if self.initialized {
                if let Some(single) = &self.single {
                    single.run_callbacks();
                }
            }
        }
    }

    // ... rest of achievement and progress methods remain the same ...

    pub fn unlock_achievement(&self, achievement_id: &str) {
        #[cfg(feature = "steam")]
        {
            if self.initialized {
                if let Some(client) = &self.client {
                    if let Ok(ach) = client.achievement(achievement_id) {
                        let _ = ach.set();
                        println!("[Steam] Unlocked: {}", achievement_id);
                    }
                }
            }
        }
    }

    pub fn increment_stat_and_check(
        &self,
        stat_name: &str,
        achievement_id: &str,
        threshold: i32,
        current_value: i32,
    ) {
        #[cfg(feature = "steam")]
        {
            if self.initialized {
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
    }

    pub fn record_council_bloom_participation(&self) {
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
