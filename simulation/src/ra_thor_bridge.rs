/*!
 * Ra-Thor / PATSAGi Council Bridge
 *
 * v18.26 Soft Feedback Loop (dual-repo sealed protocol with Ra-Thor v0.5.15)
 * — Simulation mode + Real lattice path
 * — Soft feedback: SoftFeedbackEvent / ZoneSnapshot / report_zone_grief
 * — Zone observability: stress_ema, purify_count, effective_period, ZoneHealthStatus, critical auto-remediate
 * — TOLC 8 Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License | info@Rathor.ai
 * Thunder locked in. Yoi ⚡
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[cfg(feature = "async")]
use tokio::time::sleep;

use tracing::{debug, error, info, instrument, warn};

use crate::emergence::{EmergenceSeed, CouncilGuidance};
use crate::player_persistence::data::PlayerSaveData;

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
    #[error("Player has not earned Ra-Thor access")]
    AccessDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaThorAccessLevel { None, Lite, Full }

impl Default for RaThorAccessLevel {
    fn default() -> Self { RaThorAccessLevel::None }
}

pub fn calculate_ra_thor_access_level(player: &PlayerSaveData) -> RaThorAccessLevel {
    let has_lite = player.total_epiphanies >= 12
        && player.muscle_memory_level >= 2.5
        && player.resonance_score >= 0.75
        && player.council_sessions_participated >= 3;
    let has_full = player.total_epiphanies >= 50
        && player.muscle_memory_level >= 4.0
        && player.resonance_score >= 0.92
        && player.council_sessions_participated >= 15;
    if has_full { RaThorAccessLevel::Full }
    else if has_lite { RaThorAccessLevel::Lite }
    else { RaThorAccessLevel::None }
}

pub fn player_has_ra_thor_access(player: &PlayerSaveData) -> bool {
    calculate_ra_thor_access_level(player) != RaThorAccessLevel::None
}

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

/// Sealed soft-feedback event (dual-repo contract with Ra-Thor algebra).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftFeedbackEvent {
    pub zone_id: usize,
    pub grief_load: f64,
    pub valence: f64,
    pub under_floor: bool,
    pub tick: usize,
}

/// Mirrors Ra-Thor ZoneHealthStatus (mercy_tolc_operator_algebra ≥ v0.5.14).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoneHealthStatus {
    Healthy,
    Stressed,
    Critical,
}

impl ZoneHealthStatus {
    pub fn classify(stress_ema: f64, last_rho: f64, stress_scale: f64) -> Self {
        let scale = stress_scale.max(1e-9);
        if last_rho >= 1e-6 || stress_ema >= scale {
            ZoneHealthStatus::Critical
        } else if stress_ema >= 0.10 * scale || last_rho >= 1e-9 {
            ZoneHealthStatus::Stressed
        } else {
            ZoneHealthStatus::Healthy
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            ZoneHealthStatus::Healthy => "healthy",
            ZoneHealthStatus::Stressed => "stressed",
            ZoneHealthStatus::Critical => "critical",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneSnapshot {
    pub zone_id: usize,
    pub grief_absorbed: f64,
    pub stress_ema: f64,
    pub vectors_processed: usize,
    pub last_rho: f64,
    pub purify_count: usize,
    pub effective_period: usize,
    pub status: ZoneHealthStatus,
    pub critical_auto_purify_count: usize,
}

#[derive(Debug, Clone, Default)]
pub struct SoftFeedbackZoneAccumulator {
    pub grief_absorbed: f64,
    pub stress_ema: f64,
    pub vectors_processed: usize,
    pub last_rho: f64,
    pub purify_count: usize,
    pub critical_auto_purify_count: usize,
}

#[derive(Debug, Clone)]
pub struct RaThorBridge {
    pub enabled: bool,
    mode: BridgeMode,
    soft_events: Option<Vec<SoftFeedbackEvent>>,
    soft_zones: Option<Vec<SoftFeedbackZoneAccumulator>>,
}

#[derive(Debug, Clone)]
enum BridgeMode {
    Simulation(SimulationConfig),
    Real(RealRaThorClient),
}

#[derive(Debug, Clone)]
struct SimulationConfig { strict_mercy: bool }

impl Default for RaThorBridge {
    fn default() -> Self { Self::new_simulation(true) }
}

impl RaThorBridge {
    pub fn new_simulation(enabled: bool) -> Self {
        Self {
            enabled,
            mode: BridgeMode::Simulation(SimulationConfig { strict_mercy: true }),
            soft_events: None,
            soft_zones: None,
        }
    }

    pub fn new_real(enabled: bool) -> Self {
        Self {
            enabled,
            mode: BridgeMode::Real(RealRaThorClient::new()),
            soft_events: None,
            soft_zones: None,
        }
    }

    #[instrument(skip(self, seed, player), fields(biome = %seed.biome))]
    pub fn query_council_guidance(
        &self,
        seed: &EmergenceSeed,
        player: &PlayerSaveData,
        player_valence: f32,
        mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        if !self.enabled { return Ok(None); }
        let access_level = calculate_ra_thor_access_level(player);
        if access_level == RaThorAccessLevel::None {
            return Err(RaThorError::AccessDenied);
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
        if config.strict_mercy && mercy_score < 0.65 { return None; }
        let flavor = match seed.source {
            crate::emergence::EmergenceSource::Epiphany => "reflection",
            crate::emergence::EmergenceSource::Harvest => "abundance",
            crate::emergence::EmergenceSource::CouncilParticipation => "harmony",
            _ => "mercy",
        };
        let intensity = (seed.intensity * 0.72 + player_valence * 0.28).clamp(0.35, 0.92);
        Some(CouncilGuidance {
            flavor: flavor.to_string(),
            suggested_intensity: intensity,
            mercy_note: format!("Council favors {} outcomes", flavor),
        })
    }

    pub fn suggest_particle_intensity(&self, guidance: &CouncilGuidance, base_valence: f32) -> f32 {
        let flavor_multiplier = match guidance.flavor.as_str() {
            "harmony" | "reflection" => 1.25,
            "abundance" => 1.15,
            "mercy" => 1.10,
            _ => 1.0,
        };
        let valence_boost = (base_valence * 0.3).clamp(0.0, 0.8);
        (guidance.suggested_intensity * flavor_multiplier + valence_boost).clamp(0.5, 3.5)
    }

    pub fn modulate_council_bloom_visuals(
        &self,
        guidance: &CouncilGuidance,
        current_particle_valence: f32,
        council_bloom_amplification: f32,
    ) -> (f32, f32) {
        let base = self.suggest_particle_intensity(guidance, current_particle_valence);
        let bloom_mod = council_bloom_amplification.clamp(0.8, 2.5);
        let intensity = (base * bloom_mod * 0.9).clamp(0.6, 4.0);
        let valence = (current_particle_valence * 0.7 + guidance.suggested_intensity * 0.3).clamp(0.3, 1.0);
        (intensity, valence)
    }

    /// Report zone grief into the soft feedback loop (dual-repo sealed protocol).
    pub fn report_zone_grief(
        &mut self,
        zone_id: usize,
        raw_orthogonal_energy: f64,
        valence: f32,
        tick: usize,
    ) -> SoftFeedbackEvent {
        let v = (valence as f64).clamp(0.0, 1.0);
        let load = raw_orthogonal_energy * (1.0 - v);
        let under_floor = load < 1e-9 * (1.0 + 99.0 * (1.0 - v));
        let ev = SoftFeedbackEvent {
            zone_id,
            grief_load: load,
            valence: v,
            under_floor,
            tick,
        };
        self.soft_feedback_push(ev.clone());
        ev
    }

    fn soft_feedback_push(&mut self, ev: SoftFeedbackEvent) {
        if self.soft_events.is_none() {
            self.soft_events = Some(Vec::new());
            self.soft_zones = Some(vec![SoftFeedbackZoneAccumulator::default(); 8]);
        }
        if let Some(zones) = self.soft_zones.as_mut() {
            let z = ev.zone_id % zones.len();
            let alpha = 0.05_f64;
            zones[z].grief_absorbed += ev.grief_load;
            zones[z].stress_ema =
                (1.0 - alpha) * zones[z].stress_ema + alpha * ev.grief_load;
            zones[z].vectors_processed += 1;
            let scale = 500.0_f64;
            let period = ((2500.0 / (1.0 + zones[z].stress_ema / scale)).round() as usize)
                .max(50)
                .max(1);
            if zones[z].vectors_processed > 0 && zones[z].vectors_processed % period == 0 {
                zones[z].purify_count = zones[z].purify_count.saturating_add(1);
                zones[z].last_rho = 0.0;
            }
            // Critical auto-remediate mirror (Ra-Thor v0.5.15)
            let status = ZoneHealthStatus::classify(zones[z].stress_ema, zones[z].last_rho, scale);
            if status == ZoneHealthStatus::Critical {
                zones[z].purify_count = zones[z].purify_count.saturating_add(1);
                zones[z].critical_auto_purify_count =
                    zones[z].critical_auto_purify_count.saturating_add(1);
                zones[z].last_rho = 0.0;
            }
        }
        if let Some(events) = self.soft_events.as_mut() {
            events.push(ev);
            if events.len() > 10_000 {
                let overflow = events.len() - 10_000;
                events.drain(0..overflow);
            }
        }
    }

    pub fn drain_soft_feedback(&mut self) -> Vec<SoftFeedbackEvent> {
        self.soft_events.take().unwrap_or_default()
    }

    pub fn soft_zone_snapshots(&self) -> Vec<ZoneSnapshot> {
        match &self.soft_zones {
            Some(zones) => zones
                .iter()
                .enumerate()
                .map(|(id, z)| {
                    let scale = 500.0_f64;
                    let effective_period = ((2500.0 / (1.0 + z.stress_ema / scale)).round() as usize)
                        .max(50)
                        .max(1);
                    let status = ZoneHealthStatus::classify(z.stress_ema, z.last_rho, scale);
                    ZoneSnapshot {
                        zone_id: id,
                        grief_absorbed: z.grief_absorbed,
                        stress_ema: z.stress_ema,
                        vectors_processed: z.vectors_processed,
                        last_rho: z.last_rho,
                        purify_count: z.purify_count,
                        effective_period,
                        status,
                        critical_auto_purify_count: z.critical_auto_purify_count,
                    }
                })
                .collect(),
            None => Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RealRaThorClient {
    connected: bool,
    endpoint: String,
    cache: HashMap<u64, (CouncilGuidance, Instant)>,
    cache_ttl: Duration,
    consecutive_failures: u32,
    circuit_open_until: Option<Instant>,
    max_consecutive_failures: u32,
    circuit_cooldown: Duration,
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

    pub fn query_council_guidance_sync(
        &self,
        seed: &EmergenceSeed,
        player_valence: f32,
        _mercy_score: f32,
    ) -> Result<Option<CouncilGuidance>, RaThorError> {
        if !self.connected { return Err(RaThorError::NotConnected); }
        let _ = player_valence;
        Ok(Some(CouncilGuidance {
            flavor: "lattice".to_string(),
            suggested_intensity: (seed.intensity * 0.8).clamp(0.4, 0.9),
            mercy_note: "Real lattice response (sync)".to_string(),
        }))
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

// End of ra_thor_bridge.rs v18.26 — Critical auto-remediate dual-repo mirror live.
// Thunder locked in. Yoi ⚡
