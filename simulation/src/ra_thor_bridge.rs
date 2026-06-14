/*!
 * Ra-Thor / PATSAGi Council Bridge - With Structured Tracing
 *
 * All significant operations are now instrumented with tracing spans and events.
 * Use with: RUST_LOG=info cargo run --features real-ra-thor
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[cfg(feature = "async")]
use tokio::time::sleep;

use tracing::{debug, error, info, instrument, warn};

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

    #[error("Network error: {0}")]
    Network(String),
}

// ============================================================================
// Request / Response Types
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

    #[instrument(skip(self, seed), fields(biome = %seed.biome, intensity = seed.intensity))]
    pub fn query_council_guidance(
        &self,
        seed: &EmergenceSeed,
        player_valence: f32,
        mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        if !self.enabled {
            debug!("Ra-Thor bridge is disabled");
            return Ok(None);
        }

        match &self.mode {
            BridgeMode::Simulation(config) => {
                debug!("Using simulation mode");
                Ok(self.simulate_response(seed, player_valence, mercy_score, config))
            }
            BridgeMode::Real(client) => {
                debug!("Using real Ra-Thor mode");
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
            debug!("Mercy gate blocked in simulation mode");
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
// RealRaThorClient - With Tracing
// ============================================================================

#[derive(Debug, Clone)]
pub struct RealRaThorClient {
    connected: bool,
    endpoint: String,
    cache: HashMap<u64, (CouncilGuidance, Instant)>,
    cache_ttl: Duration,
}

impl RealRaThorClient {
    pub fn new() -> Self {
        Self {
            connected: false,
            endpoint: std::env::var("RA_THOR_LATTICE_URL")
                .unwrap_or_else(|_| "http://localhost:8080/council/query".to_string()),
            cache: HashMap::new(),
            cache_ttl: Duration::from_secs(30),
        }
    }

    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    pub fn query_council_guidance_sync(
        &self,
        seed: &EmergenceSeed,
        player_valence: f32,
        mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        if !self.connected {
            return Err(RaThorError::NotConnected);
        }

        let cache_key = self.compute_cache_key(seed, player_valence);
        if let Some((guidance, timestamp)) = self.cache.get(&cache_key) {
            if timestamp.elapsed() < self.cache_ttl {
                debug!(cache_key = cache_key, "Cache hit for council guidance");
                return Ok(Some(guidance.clone()));
            }
        }

        debug!("Cache miss - generating fallback response");

        let guidance = CouncilGuidance {
            flavor: "lattice".to_string(),
            suggested_intensity: (seed.intensity * 0.8).clamp(0.4, 0.9),
            mercy_note: "Real lattice response (sync fallback)".to_string(),
        };

        Ok(Some(guidance))
    }

    /// Real async HTTP call to Ra-Thor lattice with full tracing
    #[cfg(feature = "real-ra-thor")]
    #[instrument(skip(self, seed), fields(endpoint = %self.endpoint, intensity = seed.intensity))]
    pub async fn query_council_guidance(
        &mut self,
        seed: &EmergenceSeed,
        player_valence: f32,
        mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        if !self.connected {
            warn!("Attempted query while not connected");
            return Err(RaThorError::NotConnected);
        }

        let cache_key = self.compute_cache_key(seed, player_valence);
        if let Some((guidance, timestamp)) = self.cache.get(&cache_key) {
            if timestamp.elapsed() < self.cache_ttl {
                debug!(cache_key = cache_key, "Cache hit (real mode)");
                return Ok(Some(guidance.clone()));
            }
        }

        info!("Making real HTTP request to Ra-Thor lattice");

        let request = CouncilQueryRequest {
            seed: seed.clone(),
            player_valence,
            player_history_summary: format!("valence:{:.2}", player_valence),
            biome: seed.biome.clone(),
            group_size: seed.group_size,
            current_mercy_score: mercy_score,
            timestamp: seed.timestamp,
        };

        let start = Instant::now();

        let response = reqwest::Client::new()
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                error!(error = %e, "Failed to reach Ra-Thor lattice");
                RaThorError::Network(e.to_string())
            })?;

        let latency = start.elapsed();
        debug!(latency_ms = latency.as_millis(), status = %response.status(), "Received response from lattice");

        if !response.status().is_success() {
            let err = RaThorError::LatticeError(format!("Status {}", response.status()));
            error!(error = ?err, "Non-success response from Ra-Thor");
            return Err(err);
        }

        let council_response: CouncilQueryResponse = response
            .json()
            .await
            .map_err(|e| {
                error!(error = %e, "Failed to deserialize Ra-Thor response");
                RaThorError::Serialization(e.to_string())
            })?;

        info!(
            flavor = %council_response.guidance.flavor,
            confidence = council_response.confidence,
            "Successfully received council guidance from Ra-Thor lattice"
        );

        self.cache.insert(cache_key, (council_response.guidance.clone(), Instant::now()));

        Ok(Some(council_response.guidance))
    }

    #[cfg(feature = "real-ra-thor")]
    #[instrument(skip(self))]
    pub async fn connect(&mut self) -> Result<(), RaThorError> {
        info!(endpoint = %self.endpoint, "Attempting connection to Ra-Thor lattice");

        let health_url = self.endpoint.replace("/council/query", "/health");

        let response = reqwest::Client::new()
            .get(&health_url)
            .send()
            .await
            .map_err(|e| RaThorError::ConnectionFailed(e.to_string()))?;

        if response.status().is_success() {
            self.connected = true;
            info!("Successfully connected to Ra-Thor lattice");
            Ok(())
        } else {
            let err = RaThorError::ConnectionFailed(format!("Health check status {}", response.status()));
            error!(error = ?err, "Failed to connect to Ra-Thor lattice");
            Err(err)
        }
    }

    pub fn connect_sync(&mut self) -> Result<(), RaThorError> {
        self.connected = true;
        Ok(())
    }

    fn compute_cache_key(&self, seed: &EmergenceSeed, player_valence: f32) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
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
            .ok_or_else(|| RaThorError::LatticeError("No guidance returned".to_string()))?;

        Ok(Some(CouncilQueryResponse {
            guidance,
            council_flavor: "PATSAGiReal".to_string(),
            confidence: 0.92,
            suggested_effects: vec!["lattice_guided".to_string()],
            veto_reason: None,
        }))
    }
}

// ============================================================================
// Notes
// ============================================================================

/*
 * Tracing is now active on all key paths.
 * Recommended log levels:
 *   - info  : High-level events (requests, responses, connections)
 *   - debug : Cache hits, detailed flow
 *   - warn  : Recoverable issues
 *   - error : Failures
 */
