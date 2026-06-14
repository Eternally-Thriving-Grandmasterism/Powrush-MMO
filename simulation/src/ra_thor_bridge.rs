/*!
 * Ra-Thor / PATSAGi Council Bridge - With Retry + Circuit Breaker
 *
 * The real async path now includes:
 * - Retry with exponential backoff on transient errors
 * - Basic circuit breaker (opens after repeated failures)
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

    #[error("Circuit breaker is open")]
    CircuitOpen,
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

    #[instrument(skip(self, seed), fields(biome = %seed.biome))]
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
// RealRaThorClient - With Retry + Circuit Breaker
// ============================================================================

#[derive(Debug, Clone)]
pub struct RealRaThorClient {
    connected: bool,
    endpoint: String,
    cache: HashMap<u64, (CouncilGuidance, Instant)>,
    cache_ttl: Duration,

    // Circuit breaker state
    consecutive_failures: u32,
    circuit_open_until: Option<Instant>,
    max_consecutive_failures: u32,
    circuit_cooldown: Duration,

    // Retry config
    max_retries: u32,
    base_retry_delay: Duration,
}

impl RealRaThorClient {
    pub fn new() -> Self {
        Self {
            connected: false,
            endpoint: std::env::var("RA_THOR_LATTICE_URL")
                .unwrap_or_else(|_| "http://localhost:8080/council/query".to_string()),
            cache: HashMap::new(),
            cache_ttl: Duration::from_secs(30),
            consecutive_failures: 0,
            circuit_open_until: None,
            max_consecutive_failures: 5,
            circuit_cooldown: Duration::from_secs(30),
            max_retries: 3,
            base_retry_delay: Duration::from_millis(200),
        }
    }

    pub fn with_retry_config(mut self, max_retries: u32, base_delay: Duration) -> Self {
        self.max_retries = max_retries;
        self.base_retry_delay = base_delay;
        self
    }

    pub fn with_circuit_breaker(mut self, max_failures: u32, cooldown: Duration) -> Self {
        self.max_consecutive_failures = max_failures;
        self.circuit_cooldown = cooldown;
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
                debug!(cache_key = cache_key, "Cache hit (sync)");
                return Ok(Some(guidance.clone()));
            }
        }

        let guidance = CouncilGuidance {
            flavor: "lattice".to_string(),
            suggested_intensity: (seed.intensity * 0.8).clamp(0.4, 0.9),
            mercy_note: "Real lattice response (sync fallback)".to_string(),
        };

        Ok(Some(guidance))
    }

    /// Async query with retry + circuit breaker
    #[cfg(feature = "real-ra-thor")]
    #[instrument(skip(self, seed), fields(endpoint = %self.endpoint))]
    pub async fn query_council_guidance(
        &mut self,
        seed: &EmergenceSeed,
        player_valence: f32,
        mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        // Circuit breaker check
        if let Some(open_until) = self.circuit_open_until {
            if Instant::now() < open_until {
                warn!("Circuit breaker is open - rejecting request");
                return Err(RaThorError::CircuitOpen);
            } else {
                // Cooldown finished, try to close circuit
                self.circuit_open_until = None;
                self.consecutive_failures = 0;
                info!("Circuit breaker closed - attempting request");
            }
        }

        if !self.connected {
            return Err(RaThorError::NotConnected);
        }

        // Check cache
        let cache_key = self.compute_cache_key(seed, player_valence);
        if let Some((guidance, timestamp)) = self.cache.get(&cache_key) {
            if timestamp.elapsed() < self.cache_ttl {
                debug!(cache_key = cache_key, "Cache hit");
                return Ok(Some(guidance.clone()));
            }
        }

        let mut last_error: Option<RaThorError> = None;

        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                let delay = self.base_retry_delay * (1 << (attempt - 1));
                debug!(attempt = attempt, delay_ms = delay.as_millis(), "Retrying after backoff");
                sleep(delay).await;
            }

            match self.try_single_request(seed, player_valence, mercy_score).await {
                Ok(guidance) => {
                    self.consecutive_failures = 0;
                    if let Some(g) = &guidance {
                        self.cache.insert(cache_key, (g.clone(), Instant::now()));
                    }
                    return Ok(guidance);
                }
                Err(e) => {
                    last_error = Some(e.clone());
                    self.consecutive_failures += 1;

                    warn!(
                        attempt = attempt,
                        consecutive_failures = self.consecutive_failures,
                        error = ?e,
                        "Request to Ra-Thor failed"
                    );

                    // Only retry on transient errors
                    if !matches!(
                        e,
                        RaThorError::Network(_) | RaThorError::Timeout | RaThorError::ConnectionFailed(_)
                    ) {
                        break;
                    }
                }
            }
        }

        // All retries exhausted - check if we should open circuit
        if self.consecutive_failures >= self.max_consecutive_failures {
            self.circuit_open_until = Some(Instant::now() + self.circuit_cooldown);
            error!(
                consecutive_failures = self.consecutive_failures,
                cooldown_secs = self.circuit_cooldown.as_secs(),
                "Opening circuit breaker after repeated failures"
            );
        }

        Err(last_error.unwrap_or(RaThorError::LatticeError(
            "All retries exhausted".to_string(),
        )))
    }

    #[cfg(feature = "real-ra-thor")]
    async fn try_single_request(
        &self,
        seed: &EmergenceSeed,
        player_valence: f32,
        mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        let request = CouncilQueryRequest {
            seed: seed.clone(),
            player_valence,
            player_history_summary: format!("valence:{:.2}", player_valence),
            biome: seed.biome.clone(),
            group_size: seed.group_size,
            current_mercy_score: mercy_score,
            timestamp: seed.timestamp,
        };

        let response = reqwest::Client::new()
            .post(&self.endpoint)
            .json(&request)
            .timeout(Duration::from_secs(8))
            .send()
            .await
            .map_err(|e| RaThorError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(RaThorError::LatticeError(format!("Status {}", response.status())));
        }

        let council_response: CouncilQueryResponse = response
            .json()
            .await
            .map_err(|e| RaThorError::Serialization(e.to_string()))?;

        Ok(Some(council_response.guidance))
    }

    #[cfg(feature = "real-ra-thor")]
    #[instrument(skip(self))]
    pub async fn connect(&mut self) -> Result<(), RaThorError> {
        let health_url = self.endpoint.replace("/council/query", "/health");

        let response = reqwest::Client::new()
            .get(&health_url)
            .send()
            .await
            .map_err(|e| RaThorError::ConnectionFailed(e.to_string()))?;

        if response.status().is_success() {
            self.connected = true;
            self.consecutive_failures = 0;
            self.circuit_open_until = None;
            info!("Connected to Ra-Thor lattice");
            Ok(())
        } else {
            Err(RaThorError::ConnectionFailed(format!("Status {}", response.status())))
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
            .ok_or_else(|| RaThorError::LatticeError("No guidance".to_string()))?;

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
 * Retry + Circuit Breaker behavior:
 * - Retries transient errors (Network, Timeout, ConnectionFailed) up to max_retries
 * - Uses exponential backoff
 * - Opens circuit after max_consecutive_failures
 * - Circuit stays open for circuit_cooldown duration
 *
 * Configuration can be adjusted via with_retry_config() and with_circuit_breaker()
 */
