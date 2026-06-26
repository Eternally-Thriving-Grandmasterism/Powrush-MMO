/*!
 * Steam Integration Module (Scaffold)
 *
 * This module provides Steamworks integration for Powrush-MMO.
 * It is feature-gated behind `steam` so the game can run without Steam during development.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

#[cfg(feature = "steam")]
use steamworks::{Client, SingleClient, Achievement, Stats, Cloud};

/// Steam integration handle
#[derive(Debug)]
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

    /// Initialize Steam API (call early in main)
    pub fn initialize(&mut self) -> Result<(), String> {
        #[cfg(feature = "steam")]
        {
            match steamworks::Client::init() {
                Ok((client, single)) => {
                    self.client = Some(client);
                    self.single = Some(single);
                    println!("Steamworks initialized successfully");
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Failed to initialize Steamworks: {:?}", e);
                    Err(format!("Steam init failed: {:?}", e))
                }
            }
        }

        #[cfg(not(feature = "steam"))]
        {
            println!("Steam feature not enabled — running in standalone mode");
            Ok(())
        }
    }

    /// Run Steam callbacks (call every frame/tick)
    pub fn run_callbacks(&self) {
        #[cfg(feature = "steam")]
        {
            if let Some(single) = &self.single {
                single.run_callbacks();
            }
        }
    }

    // TODO (future): Add achievement unlock helpers, stat tracking, cloud save integration
}

// Placeholder for when Steam feature is disabled
#[cfg(not(feature = "steam"))]
pub fn steam_disabled_stub() {
    // No-op when Steam is not compiled in
}
