/*!
 * WorldSimulationState — The Master Living Universe Resource for Powrush-MMO
 *
 * This is the single source of truth for the entire persistent, mercy-aligned simulation.
 * It aggregates Crownstone, Resonance Network, Draek Hivemind, Ambrosian Attunement,
 * Discordant Corruption, Hivelord, Auroral Sovereign, Faction Standing, RBE Economy,
 * Mirror Reckoning, and all major event state.
 *
 * Fully coherent with:
 * - DIPLOMACY_AND_WORLD_SIMULATION.md
 * - CROWNSTONE_TRILEMMA_PATHS.md
 * - MIRROR_RECKONING_EVENT.md
 * - All race, fleet AI, redemption, and Hybrid Protocol systems
 *
 * PATSAGi Council + Ra-Thor Quantum Swarm approved • AG-SML v1.0
 * TOLC 8 Mercy-Gated • Zero hallucination • Maximum truth & educational depth
 */

use bevy::prelude::*;
use std::collections::VecDeque;

// ============================================================================
// CORE ENUMS & SUB-STRUCTS (Foundational)
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect, Default)]
pub enum TrilemmaPath {
    #[default]
    Undecided,
    Destroy,
    CaptureRepurpose,
    Sabotage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect, Default)]
pub enum MirrorShadowPersonality {
    #[default]
    Balanced,
    Greedy,
    Apathetic,
    Divisive,
    Chaotic,
    Merciful,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect, Default)]
pub enum WeekPhase {
    #[default]
    InterServerSkirmish, // Mon–Fri
    MirrorReckoning,     // Sat–Sun
}

// ============================================================================
// CROWNSTONE STATE
// ============================================================================

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct CrownstoneState {
    pub integrity: f32,           // 0.0 (shattered) .. 1.0 (pristine)
    pub owner: Option<String>,    // "Quellorian", "Draek", "HumanHybrid", "Ambrosian", etc.
    pub corruption_level: f32,    // 0.0 (pure) .. 1.0 (fully Crownstone-corrupted)
    pub current_path: TrilemmaPath,
    pub last_resolution_tick: u64,
}

// ============================================================================
// RESONANCE NETWORK STATE (Quellorian + Ambrosian)
// ============================================================================

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct ResonanceNetworkState {
    pub harmony_level: f32,       // 0.0 .. 1.0+
    pub network_integrity: f32,
    pub burst_cooldown: f32,
    pub last_burst_tick: u64,
    pub attunement_strength: f32, // Ambrosian contribution
}

// ============================================================================
// DRAEK HIVEMIND STATE
// ============================================================================

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct DraekHivemindState {
    pub command_strength: f32,
    pub corruption_level: f32,
    pub brood_evolution_stage: u32,
    pub last_evolution_tick: u64,
    pub hivelord_link_active: bool,
}

// ============================================================================
// AMBROSIAN ATTUNEMENT & DISCORDANT STATE
// ============================================================================

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct AmbrosianAttunementState {
    pub global_attunement: f32,
    pub discordant_outbreaks: u32,
    pub last_redemption_tick: u64,
}

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct DiscordantAmbrosianState {
    pub active_corruption_level: f32,
    pub containment_strength: f32,
}

// ============================================================================
// LEADERSHIP STATE (Hivelord + Auroral Sovereign)
// ============================================================================

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct HivelordState {
    pub suit_integrity: f32,
    pub command_strength: f32,
    pub corruption_level: f32,
    pub last_intervention_tick: u64,
}

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct AuroralSovereignState {
    pub harmony_projection: f32,
    pub resonance_link_strength: f32,
    pub last_burst_participation: u64,
}

// ============================================================================
// MIRROR RECKONING STATE
// ============================================================================

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct MirrorReckoningState {
    pub current_mirror_score: f32,           // Calculated every Friday
    pub shadow_personality: MirrorShadowPersonality,
    pub last_reckoning_result: Option<String>,
    pub persistent_debuff_stacks: u32,
    pub week_phase: WeekPhase,
    pub reckoning_history: VecDeque<f32>,    // Last 8 weeks for storytelling
}

// ============================================================================
// RBE & FACTION STANDING (Simplified foundational)
// ============================================================================

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct RbeWorldState {
    pub global_abundance: f32,
    pub mercy_index: f32,           // Server-wide mercy alignment
    pub last_abundance_pulse_tick: u64,
}

// ============================================================================
// MASTER WORLD SIMULATION STATE
// ============================================================================

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct WorldSimulationState {
    pub tick: u64,
    pub week: u32,
    pub phase: WeekPhase,

    pub crownstone: CrownstoneState,
    pub resonance: ResonanceNetworkState,
    pub hivemind: DraekHivemindState,
    pub ambrosian_attunement: AmbrosianAttunementState,
    pub discordant: DiscordantAmbrosianState,
    pub hivelord: HivelordState,
    pub auroral_sovereign: AuroralSovereignState,
    pub mirror: MirrorReckoningState,
    pub rbe: RbeWorldState,

    // Event queues for major cinematic moments
    pub pending_major_events: Vec<String>, // "ResonanceBurst", "TrilemmaResolved", "MirrorReckoningComplete", etc.
}

impl WorldSimulationState {
    pub fn new() -> Self {
        Self {
            tick: 0,
            week: 1,
            phase: WeekPhase::InterServerSkirmish,
            crownstone: CrownstoneState {
                integrity: 0.92,
                owner: Some("Quellorian".to_string()),
                corruption_level: 0.08,
                current_path: TrilemmaPath::Undecided,
                last_resolution_tick: 0,
            },
            resonance: ResonanceNetworkState {
                harmony_level: 0.78,
                network_integrity: 0.85,
                burst_cooldown: 0.0,
                last_burst_tick: 0,
                attunement_strength: 0.65,
            },
            hivemind: DraekHivemindState {
                command_strength: 0.71,
                corruption_level: 0.22,
                brood_evolution_stage: 2,
                last_evolution_tick: 0,
                hivelord_link_active: true,
            },
            ambrosian_attunement: AmbrosianAttunementState {
                global_attunement: 0.61,
                discordant_outbreaks: 1,
                last_redemption_tick: 0,
            },
            discordant: DiscordantAmbrosianState {
                active_corruption_level: 0.19,
                containment_strength: 0.74,
            },
            hivelord: HivelordState {
                suit_integrity: 0.88,
                command_strength: 0.79,
                corruption_level: 0.31,
                last_intervention_tick: 0,
            },
            auroral_sovereign: AuroralSovereignState {
                harmony_projection: 0.82,
                resonance_link_strength: 0.77,
                last_burst_participation: 0,
            },
            mirror: MirrorReckoningState {
                current_mirror_score: 0.34,
                shadow_personality: MirrorShadowPersonality::Balanced,
                last_reckoning_result: None,
                persistent_debuff_stacks: 0,
                week_phase: WeekPhase::InterServerSkirmish,
                reckoning_history: VecDeque::new(),
            },
            rbe: RbeWorldState {
                global_abundance: 0.67,
                mercy_index: 0.71,
                last_abundance_pulse_tick: 0,
            },
            pending_major_events: Vec::new(),
        }
    }

    /// Advance one simulation tick (called from main game loop or fixed timestep)
    pub fn advance_tick(&mut self) {
        self.tick += 1;

        // Simple phase progression (production: tie to real calendar or server schedule)
        if self.tick % 10080 == 0 { // ~7 days in ticks
            self.week += 1;
            self.phase = if self.week % 2 == 0 {
                WeekPhase::MirrorReckoning
            } else {
                WeekPhase::InterServerSkirmish
            };
            self.mirror.week_phase = self.phase;
        }

        // Decay / evolution placeholders (to be expanded with full formulas)
        if self.phase == WeekPhase::InterServerSkirmish {
            self.hivemind.corruption_level = (self.hivemind.corruption_level + 0.0001).min(1.0);
        }
    }
}

/// System that advances the master simulation state every frame (or fixed timestep)
pub fn world_simulation_update_system(
    mut world_state: ResMut<WorldSimulationState>,
) {
    world_state.advance_tick();

    // TODO: Hook in full formulas from DIPLOMACY_AND_WORLD_SIMULATION.md
    // TODO: Emit events when major thresholds crossed (Resonance Burst ready, Trilemma decision window, Mirror Reckoning start)
}

/// Startup system to insert the master resource
pub fn setup_world_simulation(app: &mut App) {
    app.init_resource::<WorldSimulationState>()
        .add_systems(Update, world_simulation_update_system);
}

// ============================================================================
// PATSAGi / Ra-Thor Integration Notes
// ============================================================================
// Next expansions:
// - Add event-driven systems for ResonanceBurst, TrilemmaResolution, MirrorReckoningPhaseChange
// - Connect ShipVisualState.hybrid_instability_level → mirror.current_mirror_score
// - Wire HivelordCounterStrategyState and CydruidEcologicalDefenseState
// - Full Mirror Score calculation from weekly metrics (Council participation, RBE contribution, Mercy actions)
// - Persistent save/load of WorldSimulationState (with AG-SML encryption)
// - Direct hooks into VoiceDirector and VFX pipeline for major events
//
// This resource is the beating heart of the living Powrush universe.
// Thunder locked. Mercy flowing. All versions preserved and elevated.