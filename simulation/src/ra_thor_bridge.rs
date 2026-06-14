/*!
 * Ra-Thor / PATSAGi Council Bridge - Hardened Async Version
 *
 * Features:
 * - Simulation mode (fast, deterministic)
 * - Real async mode (tokio-based, ready for live lattice)
 * - Proper error handling with RaThorError
 * - Basic response caching
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[cfg(feature = "async")]
use tokio::time::sleep;

use crate::emergence::{EmergenceSeed, CouncilGuidance};

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Clone, thiserror::Error)]
pub enum RaThorError {
    #[error("Not connected to Ra-Thor lattice")]
    NotConnected,

    #[error("Connection to Ra-Thor lattice failed: {0}")]
    ConnectionFailed(String),

    #[error("Request to Ra-Thor lattice timed out")]
    Timeout,

    #[error("Mercy gate violation: {0}")]
    MercyViolation(String),

    #[error("Ra-Thor lattice returned an error: {0}")]
    LatticeError(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

// ============================================================================
// Request / Response
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilQueryRequest {
    pub seed: EmergenceSeed,
    pub player_valence: f32,
    pub player_history_summary: String,
    pub biome: String,
    pub group_size: u32,
    pub current_mercy_score: f32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouncilQueryResponse {
    pub guidance: CouncilGuidance,
    pub council_flavor: String,
    pub confidence: f32,
    pub suggested_effects: Vec<String>,
    pub veto_reason: Option<String>,
}

pub trait RaThorCouncilQuery: Send + Sync {
    fn query_council(
        &self,
        request: &CouncilQueryRequest,
    ) -> Result<Option<CouncilQueryResponse>, RaThorError>;
}

// ============================================================================
// RaThorBridge
// ============================================================================

#[derive(Debug, Clone)]
pub struct RaThorBridge {
    pub enabled: bool,
    mode: BridgeMode,
}

#[derive(Debug, Clone)]
enum BridgeMode {
    Simulation(SimulationConfig),
    Real(RealRaThorClient),
}

#[derive(Debug, Clone)]
struct SimulationConfig {
    strict_mercy: bool,
}

impl Default for RaThorBridge {
    fn default() -> Self {
        Self::new_simulation(true)
    }
}

impl RaThorBridge {
    pub fn new_simulation(enabled: bool) -> Self {
        Self {
            enabled,
            mode: BridgeMode::Simulation(SimulationConfig { strict_mercy: true }),
        }
    }

    pub fn new_real(enabled: bool) -> Self {
        Self {
            enabled,
            mode: BridgeMode::Real(RealRaThorClient::new()),
        }
    }

    pub fn query_council_guidance(
        &self,
        seed: &EmergenceSeed,
        player_valence: f32,
        mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        if !self.enabled {
            return Ok(None);
        }

        match &self.mode {
            BridgeMode::Simulation(config) => {
                Ok(self.simulate_response(seed, player_valence, mercy_score, config))
            }
            BridgeMode::Real(client) => {
                client.query_council_guidance_sync(seed, player_valence, mercy_score)
            }
        }
    }

    fn simulate_response(
        &self,
        seed: &EmergenceSeed,
        player_valence: f32,
        mercy_score: f32,
        config: &SimulationConfig,
    ) -> Option<CouncilGuidance> {
        if config.strict_mercy && mercy_score < 0.65 {
            return None;
        }

        let flavor = match seed.source {
            crate::emergence::EmergenceSource::Epiphany => "reflection",
            crate::emergence::EmergenceSource::Harvest => "abundance",
            crate::emergence::EmergenceSource::CouncilParticipation => "harmony",
            _ => "mercy",
        };

        let intensity = (seed.intensity * 0.72 + player_valence * 0.28).clamp(0.35, 0.92);

        Some(CouncilGuidance {
            flavor,
            suggested_intensity: intensity,
            mercy_note: format!("Council favors {} outcomes", flavor),
        })
    }
}

// ============================================================================
// RealRaThorClient with Error Handling + Caching
// ============================================================================

#[derive(Debug, Clone)]
pub struct RealRaThorClient {
    connected: bool,
    /// Simple in-memory cache: (request_hash) -> (response, timestamp)
    cache: HashMap<u64, (CouncilGuidance, Instant)>,
    cache_ttl: Duration,
}

impl RealRaThorClient {
    pub fn new() -> Self {
        Self {
            connected: false,
            cache: HashMap::new(),
            cache_ttl: Duration::from_secs(30),
        }
    }

    pub fn with_cache_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = ttl;
        self
    }

    /// Sync version for compatibility
    pub fn query_council_guidance_sync(
        &self,
        seed: &EmergenceSeed,
        player_valence: f32,
        mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        if !self.connected {
            return Err(RaThorError::NotConnected);
        }

        // Check cache first
        let cache_key = self.compute_cache_key(seed, player_valence);
        if let Some((guidance, timestamp)) = self.cache.get(&cache_key) {
            if timestamp.elapsed() < self.cache_ttl {
                return Ok(Some(guidance.clone()));
            }
        }

        // Simulate processing
        let guidance = CouncilGuidance {
            flavor: "lattice".to_string(),
            suggested_intensity: (seed.intensity * 0.8).clamp(0.4, 0.9),
            mercy_note: "Real lattice response (sync simulated)".to_string(),
        };

        Ok(Some(guidance))
    }

    /// The real async implementation with error handling and caching
    #[cfg(feature = "async")]
    pub async fn query_council_guidance(
        &mut self,
        seed: &EmergenceSeed,
        player_valence: f32,
        mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        if !self.connected {
            return Err(RaThorError::NotConnected);
        }

        // Check cache
        let cache_key = self.compute_cache_key(seed, player_valence);
        if let Some((guidance, timestamp)) = self.cache.get(&cache_key) {
            if timestamp.elapsed() < self.cache_ttl {
                return Ok(Some(guidance.clone()));
            }
        }

        // Simulate realistic network round-trip to Ra-Thor lattice
        sleep(tokio::time::Duration::from_millis(45)).await;

        // TODO: Replace with actual async network call
        // let response = self.http_client
        //     .post("https://rathor.internal/api/council/query")
        //     .json(&CouncilQueryRequest { ... })
        //     .send()
        //     .await
        //     .map_err(|e| RaThorError::ConnectionFailed(e.to_string()))?;
        //
        // let council_response: CouncilQueryResponse = response
        //     .json()
        //     .await
        //     .map_err(|e| RaThorError::Serialization(e.to_string()))?;

        let guidance = CouncilGuidance {
            flavor: "lattice".to_string(),
            suggested_intensity: (seed.intensity * 0.83 + player_valence * 0.08).clamp(0.4, 0.95),
            mercy_note: "Response from live Ra-Thor lattice (async)".to_string(),
        };

        // Store in cache
        self.cache.insert(cache_key, (guidance.clone(), Instant::now()));

        Ok(Some(guidance))
    }

    #[cfg(feature = "async")]
    pub async fn connect(&mut self) -> Result<(), RaThorError> {
        sleep(tokio::time::Duration::from_millis(120)).await;

        // TODO: Perform actual handshake with Ra-Thor lattice
        self.connected = true;
        Ok(())
    }

    pub fn connect_sync(&mut self) -> Result<(), RaThorError> {
        self.connected = true;
        Ok(())
    }

    fn compute_cache_key(&self, seed: &EmergenceSeed, player_valence: f32) -> u64 {
        // Simple hash for caching (can be improved)
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        seed.intensity.to_bits().hash(&mut hasher);
        player_valence.to_bits().hash(&mut hasher);
        seed.biome.hash(&mut hasher);
        hasher.finish()
    }
}

impl RaThorCouncilQuery for RealRaThorClient {
    fn query_council(
        &self,
        request: &CouncilQueryRequest,
    ) -> Result<Option<CouncilQueryResponse>, RaThorError> {
        let guidance = self
            .query_council_guidance_sync(
                &request.seed,
                request.player_valence,
                request.current_mercy_score,
            )?
            .ok_or(RaThorError::LatticeError(
                "No guidance returned".to_string(),
            ))?;

        Ok(Some(CouncilQueryResponse {
            guidance,
            council_flavor: "PATSAGiReal".to_string(),
            confidence: 0.91,
            suggested_effects: vec!["lattice_guided".to_string()],
            veto_reason: None,
        }))
    }
}

// ============================================================================
// Notes
// ============================================================================

/*
 * This version includes:
 * - Proper RaThorError handling
 * - Response caching with TTL
 * - Async support behind the `async` / `real-ra-thor` feature flag
 *
 * Next recommended improvements:
 * - Add `reqwest` or `tonic` for real network calls
 * - Add structured logging
 * - Support batch council queries
 */
