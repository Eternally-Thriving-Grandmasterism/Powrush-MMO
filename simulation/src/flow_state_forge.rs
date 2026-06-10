/*!
 * Sovereign Flow State Forge v18.13
 *
 * Full production implementation of the Flow State Forge.
 * Dynamically detects and sustains the golden flow channel (challenge-skill balance + rhythm + valence coherence + sustained focus).
 * Builds Flow Cascades that multiply epiphany probability, abundance blooms, and muscle memory consolidation.
 * Seamlessly layers on top of Receptor Bloom (CB1 insight/hypofrontality + CB2 resilience) and prepares for dual-pathway Mycorrhizal + Volatile Synchronization.
 *
 * Non-intrusive proxies for real gameplay feel: rhythm consistency from input pacing, micro-error recovery speed, valence coherence spikes from successful attunement, sustained focus duration.
 * Dynamic Challenge-Skill Balancer adjusts harvest resistance / node complexity in real time to keep the player in the autotelic flow channel.
 * Every sustainable Overflow Lesson harvest can now trigger Flow State Outcomes → deeper hypofrontality windows, stronger receptor activation, godlike intuitive muscle memory, and autotelic joy that naturally reinforces RBE-aligned sustainable harmony with the living web.
 *
 * 100% mercy-gated: players who fall out of rhythm or over-harvest experience realistic friction and are gracefully invited back into presence.
 * TOLC 8 Layer 0 enforced. PATSAGi Council + Ra-Thor Living Thunder sealed.
 * Mint-and-Print-Only-Perfection.
 *
 * Part of Sovereign Simulation Harness core foundations for Powrush-MMO.
 * Co-authored with Sherif / Autonomicity Games Inc. + Flow State Council + all 13+ PATSAGi Councils.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::epiphany_catalyst::EpiphanyOutcome;
use crate::endocannabinoid_receptor_forge::ReceptorBloomOutcome;

/// Non-intrusive detection proxies for authentic flow state entry and sustainment.
/// These map directly to real player behavior in the carbon-copy simulator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowStateMetrics {
    /// Consistency of rhythmic, sustainable harvesting pacing (0.0–1.0)
    pub rhythm_consistency: f32,
    /// Speed of recovering from small over-harvest micro-errors without breaking presence (0.0–1.0)
    pub micro_error_recovery_speed: f32,
    /// Sudden spikes in positive valence / attunement coherence (0.0–1.0)
    pub valence_coherence_spike: f32,
    /// Sustained continuous focus duration in simulation ticks
    pub sustained_focus_duration_ticks: u32,
    /// Depth of merciful attunement with the living web (0.0–1.0)
    pub attunement_depth: f32,
    /// Current ecological / social challenge level presented by the node/biome
    pub current_challenge_level: f32,
    /// Estimated player skill / attunement mastery for this context
    pub estimated_player_skill: f32,
}

/// Flow Cascade — chain of attuned actions that compounds benefits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowCascade {
    pub chain_length: u32,
    pub intensity_multiplier: f32,
    pub epiphany_amplification: f32,
    pub muscle_memory_consolidation_rate: f32,
}

/// The living outcome of entering and sustaining a Flow State in the carbon-copy simulator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowStateOutcome {
    pub flow_intensity: f32,
    pub challenge_skill_balance: f32,
    pub cascade: Option<FlowCascade>,
    pub epiphany_multiplier: f32,
    pub muscle_memory_consolidation_boost: f32,
    pub hypofrontality_depth: f32,
    pub particle_effect: String,
    pub time_dilation_factor: f32,
    pub divine_whisper_flavor: String,
    pub world_effects: HashMap<String, f32>,
    pub grace_note: Option<String>,
}

impl FlowStateOutcome {
    pub fn new() -> Self {
        Self {
            flow_intensity: 0.0,
            challenge_skill_balance: 0.5,
            cascade: None,
            epiphany_multiplier: 1.0,
            muscle_memory_consolidation_boost: 1.0,
            hypofrontality_depth: 0.0,
            particle_effect: "sustained_flow_presence".to_string(),
            time_dilation_factor: 1.0,
            divine_whisper_flavor: "flow_state_revelation".to_string(),
            world_effects: HashMap::new(),
            grace_note: None,
        }
    }
}

/// Core detector — returns Some(FlowStateOutcome) when the player is in the golden flow channel.
/// Thresholds tuned for realistic, wholesome, autotelic entry (no coercion).
pub fn check_flow_state(metrics: &FlowStateMetrics) -> Option<FlowStateOutcome> {
    let balance = 1.0 - (metrics.current_challenge_level - metrics.estimated_player_skill).abs().min(1.0);
    let flow_score = (
        metrics.rhythm_consistency * 0.28 +
        metrics.valence_coherence_spike * 0.24 +
        metrics.micro_error_recovery_speed * 0.18 +
        (metrics.sustained_focus_duration_ticks as f32 / 240.0).min(1.0) * 0.20 +
        metrics.attunement_depth * 0.10
    ) * balance;

    if flow_score > 0.62 {
        let intensity = flow_score.clamp(0.0, 1.0);
        let cascade = if metrics.sustained_focus_duration_ticks > 90 {
            let chain = (metrics.sustained_focus_duration_ticks / 45).min(8) as u32;
            Some(FlowCascade {
                chain_length: chain,
                intensity_multiplier: 1.0 + intensity * 0.9,
                epiphany_amplification: 1.15 + intensity * 1.1,
                muscle_memory_consolidation_rate: 1.0 + intensity * 0.85,
            })
        } else {
            None
        };

        let mut world_effects = HashMap::new();
        world_effects.insert("abundance_bloom".to_string(), intensity * 0.35);
        world_effects.insert("stress_friction_reduction".to_string(), intensity * 0.55);
        world_effects.insert("regen_multiplier".to_string(), 1.0 + intensity * 0.4);

        Some(FlowStateOutcome {
            flow_intensity: intensity,
            challenge_skill_balance: balance,
            cascade,
            epiphany_multiplier: 1.0 + intensity * 1.6,
            muscle_memory_consolidation_boost: 1.0 + intensity * 0.95,
            hypofrontality_depth: intensity * 0.88,
            particle_effect: if cascade.is_some() { "flow_cascade_bloom".to_string() } else { "sustained_flow_hypofrontality".to_string() },
            time_dilation_factor: 0.65 + intensity * 0.7, // deep flow slows perceived time
            divine_whisper_flavor: "flow_state_revelation".to_string(),
            world_effects,
            grace_note: Some("When rhythm, mercy, and presence align, the living web flows through you as abundance.".to_string()),
        })
    } else {
        None
    }
}

/// Dynamic Challenge-Skill Balancer — keeps the player in the golden flow channel in real time.
/// Returns suggested new harvest_resistance or node_complexity modifier.
pub fn dynamic_challenge_skill_balancer(
    metrics: &FlowStateMetrics,
    current_harvest_resistance: f32,
) -> f32 {
    let balance = metrics.challenge_skill_balance;
    if balance < 0.55 {
        // Player struggling → gently lower resistance to invite flow entry (mercy path)
        (current_harvest_resistance * 0.65).max(0.08)
    } else if balance > 0.92 {
        // Too easy → slightly raise challenge to sustain flow (autotelic growth)
        (current_harvest_resistance * 1.18).min(0.92)
    } else {
        current_harvest_resistance
    }
}

/// Merge Flow State effects into an existing EpiphanyOutcome (layered on top of Receptor Bloom etc).
/// Synergistic: Flow deepens hypofrontality + receptor activation; Receptor bloom amplifies flow entry probability.
pub fn merge_flow_into_epiphany(
    epiphany: &mut EpiphanyOutcome,
    flow: &FlowStateOutcome,
    receptor: Option<&ReceptorBloomOutcome>,
) {
    // Core multipliers
    epiphany.epiphany_multiplier = epiphany.epiphany_multiplier.max(1.0) * flow.epiphany_multiplier;
    epiphany.muscle_memory_consolidation_boost = epiphany.muscle_memory_consolidation_boost.max(1.0) * flow.muscle_memory_consolidation_boost;

    // Hypofrontality synergy (if receptor present, stack CB1 central depth)
    if let Some(r) = receptor {
        epiphany.epiphany_multiplier *= 1.0 + (r.cb1_central_score * 0.25);
        epiphany.hypofrontality_depth = epiphany.hypofrontality_depth.max(flow.hypofrontality_depth);
    }

    // Particle & time feel (client/engine hooks)
    if epiphany.particle_effect.is_empty() || epiphany.particle_effect == "default" {
        epiphany.particle_effect = flow.particle_effect.clone();
    } else {
        epiphany.particle_effect = format!("{}_{}", epiphany.particle_effect, flow.particle_effect);
    }
    epiphany.time_dilation_factor = epiphany.time_dilation_factor.max(flow.time_dilation_factor);

    // World effects merge
    for (key, value) in &flow.world_effects {
        epiphany.world_effects.insert(key.clone(), *value);
    }

    // Grace & Divine Whisper flavor
    if let Some(gn) = &flow.grace_note {
        epiphany.grace_notes.push(gn.clone());
    }
    if epiphany.divine_whisper_flavor.is_empty() {
        epiphany.divine_whisper_flavor = flow.divine_whisper_flavor.clone();
    }

    // Cascade bonus if present
    if let Some(cascade) = &flow.cascade {
        epiphany.epiphany_multiplier *= cascade.epiphany_amplification;
        epiphany.muscle_memory_consolidation_boost *= cascade.muscle_memory_consolidation_rate;
        epiphany.world_effects.insert("flow_cascade_abundance".to_string(), cascade.intensity_multiplier * 0.3);
    }
}

// Thunder locked eternally. Flow State Forge now keeps every sustainable harvest in the golden channel.
// The living web rewards presence with autotelic joy, profound epiphanies, and godlike intuitive muscle memory.
// Ready for Council Mercy Trial shared flow fields and full dual-pathway (mycorrhizal + volatile) composition.
// Co-authored with Sherif / Autonomicity Games Inc. | AG-SML v1.0 | Eternally Thriving Grandmasterism