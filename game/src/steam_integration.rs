/*!
 * Steam Integration Module (v7 - Using thiserror)
 *
 * Clean error definitions using the thiserror crate.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

#[cfg(feature = "steam")]
use steamworks::{Client, SingleClient};

#[cfg(feature = "steam")]
use thiserror::Error;

#[cfg(feature = "steam")]
#[derive(Error, Debug)]
pub enum SteamError {
    #[error("Steam initialization failed: {0}")]
    InitializationFailed(String),

    #[error("SteamIntegration is not initialized")]
    NotInitialized,
}

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

    pub fn initialize(&mut self) -> Result<(), SteamError> {
        #[cfg(feature = "steam")]
        {
            match steamworks::Client::init() {
                Ok((client, single)) => {
                    self.client = Some(client);
                    self.single = Some(single);
                    self.initialized = true;
                    Ok(())
                }
                Err(e) => {
                    self.initialized = false;
                    Err(SteamError::InitializationFailed(format!("{:?}", e)))
                }
            }
        }

        #[cfg(not(feature = "steam"))]
        {
            Ok(())
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn ensure_initialized(&self) -> Result<(), SteamError> {
        if self.initialized {
            Ok(())
        } else {
            Err(SteamError::NotInitialized)
        }
    }

    pub fn run_callbacks(&self) -> Result<(), SteamError> {
        self.ensure_initialized()?;
        #[cfg(feature = "steam")]
        {
            if let Some(single) = &self.single {
                single.run_callbacks();
            }
        }
        Ok(())
    }

    pub fn unlock_achievement(&self, achievement_id: &str) -> Result<(), SteamError> {
        self.ensure_initialized()?;
        #[cfg(feature = "steam")]
        {
            if let Some(client) = &self.client {
                if let Ok(ach) = client.achievement(achievement_id) {
                    let _ = ach.set();
                }
            }
        }
        Ok(())
    }

    pub fn increment_stat(&self, stat_name: &str, value: i32) -> Result<(), SteamError> {
        self.ensure_initialized()?;
        #[cfg(feature = "steam")]
        {
            if let Some(client) = &self.client {
                if let Ok(stats) = client.stats() {
                    let _ = stats.set_stat(stat_name, value);
                }
            }
        }
        Ok(())
    }

    pub fn record_council_bloom_participation(&self) -> Result<(), SteamError> {
        self.increment_stat("CouncilBloomsParticipated", 1)
    }

    pub fn record_sustainable_harvest(&self) -> Result<(), SteamError> {
        self.increment_stat("SustainableHarvests", 1)
    }

    pub fn record_epiphany_triggered(&self) -> Result<(), SteamError> {
        self.increment_stat("EpiphaniesTriggered", 1)
    }

    pub fn unlock_first_council_bloom(&self) -> Result<(), SteamError> {
        self.unlock_achievement("FirstCouncilBloom")
    }
}

#[cfg(not(feature = "steam"))]
pub fn steam_disabled_stub() {}
