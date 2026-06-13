/*!
 * Powrush-MMO Ship Definitions & Foundational Stubs
 *
 * This module provides the core enums, components, and data structures
 * for all ship classes across the five playable races.
 *
 * Fully aligned with:
 * - DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md
 * - All FLEET_CLASSES.md, SHIP_VISUAL_GUIDELINES.md, and per-race visual bibles
 * - Human Hybrid Protocol, Redemption Mechanics, Mirror Reckoning, etc.
 *
 * PATSAGi Council + Ra-Thor Quantum Swarm approved • AG-SML v1.0
 * Mercy-gated • Zero hallucination • Maximum coherence
 */

use bevy::prelude::*;
use crate::simulation_integration::{
    CrownstoneState, 
    // Add other shared simulation resources as needed
};

/// The five core playable races of Powrush-MMO.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum PlayableRace {
    Quellorian,   // Aetherion Luminari - Resonance & Unity
    Draek,        // Hivemind Dominion - Consumption & Domination
    Human,        // Adaptable Phoenix - Moral Complexity & Innovation
    Cydruid,      // Cyber-Organic Druids - Ecological Balance & Growth
    Ambrosian,    // Crystalline Lattice - Harmonic Attunement & Observation
}

/// Comprehensive ship class identifier.
/// Each variant maps directly to the designs in the visual bibles and fleet docs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum ShipClass {
    // Quellorian (Radial elegance, auroral effects)
    QuellorianAetherInterceptor,
    QuellorianLuminarHeavyCruiser,
    QuellorianHarmonySupportCarrier,
    QuellorianSeraphimCapitalEscort,

    // Draek (Biomechanical, oppressive, hivemind-coordinated)
    DraekSwarmDrone,
    DraekRavagerBioCorvette,
    DraekTyrantHeavyCruiser,
    DraekAbominationCapitalDevourer,
    DraekBroodSpire, // Mothership

    // Human (Patchwork, moral-reactive, hybrid-capable)
    HumanScavengerFrigate,
    HumanHybridCruiser,      // Signature vessel - uses Hybrid Protocol
    HumanMirrorCorvette,
    HumanForgeCarrier,
    HumanLastStandCapital,

    // Cydruid (Living organic-tech, grove/root network)
    CydruidGroveWardenFrigate,
    CydruidRootNetworkArchitectCruiser,
    CydruidSymbiontSwarmCoordinator,
    CydruidRestorationWeaver,
    CydruidPlanetaryBalanceKeeper,

    // Ambrosian (Crystalline lattice, iridescent, attunement-driven)
    AmbrosianLatticeWeaverFrigate,
    AmbrosianHarmonicBladeCruiser,
    AmbrosianChoirShardCarrier,
    AmbrosianAttunementNexus,
    AmbrosianLatticeSovereignCapital,
}

/// Tracks the visual and mechanical state of a ship.
/// This component drives VFX, shaders, voice processing, and redemption/Hybrid logic.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct ShipVisualState {
    pub race: PlayableRace,
    pub class: ShipClass,

    /// 0.0 = fully enslaved/corrupted, 1.0 = fully redeemed/attuned
    pub redemption_progress: f32,

    /// For Humans: current hybrid stability (0.0 = catastrophic instability)
    pub hybrid_stability: f32,

    /// Moral alignment influencing visual glow, particle color, and Mirror Reckoning feedback
    /// -100.0 (pure consumption/greed) to +100.0 (pure mercy/harmony)
    pub moral_alignment: f32,

    /// Crownstone corruption influence (especially on Luminari Exiles and corrupted Ambrosians)
    pub crownstone_corruption: f32,

    /// Whether this ship is currently in a hybrid configuration (Human-specific)
    pub is_hybrid_active: bool,

    /// For enslaved minion species ships (Veythari, Korrath, etc.)
    pub is_enslaved_minion: bool,
}

impl Default for ShipVisualState {
    fn default() -> Self {
        Self {
            race: PlayableRace::Human,
            class: ShipClass::HumanScavengerFrigate,
            redemption_progress: 0.0,
            hybrid_stability: 1.0,
            moral_alignment: 0.0,
            crownstone_corruption: 0.0,
            is_hybrid_active: false,
            is_enslaved_minion: false,
        }
    }
}

/// Marker component for ships that can participate in boarding actions.
#[derive(Component, Default)]
pub struct Boardable;

/// Marker for ships that project resonance fields (Quellorian + Ambrosian synergy).
#[derive(Component, Default)]
pub struct ResonanceProjector;

/// Marker for ships under active Hivelord command influence.
#[derive(Component, Default)]
pub struct HivemindDominated;

/// Basic ship stats stub (expand with full RPG stats later).
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct ShipStats {
    pub hull_integrity: f32,
    pub max_hull: f32,
    pub shield_strength: f32,
    pub max_shield: f32,
    pub speed: f32,
    pub maneuverability: f32,
}

/// System to initialize default ShipVisualState when a ship entity is spawned.
pub fn ship_visual_state_initialization_system(
    mut commands: Commands,
    query: Query<(Entity, &ShipClass, Option<&ShipVisualState>), Added<ShipClass>>,
) {
    for (entity, ship_class, existing_state) in query.iter() {
        if existing_state.is_none() {
            let race = match ship_class {
                ShipClass::QuellorianAetherInterceptor | 
                ShipClass::QuellorianLuminarHeavyCruiser | ... => PlayableRace::Quellorian,
                // ... fill mappings for all classes
                _ => PlayableRace::Human,
            };

            commands.entity(entity).insert(ShipVisualState {
                race,
                class: *ship_class,
                ..default()
            });
        }
    }
}

// TODO: Expand with:
// - Hybrid module attachment system (Human Hybrid Protocol)
// - Redemption state machine integration (Sylvaris, Luminari, etc.)
// - Visual parameter mapping to Bevy materials / shaders (velocity_prepass + custom WGSL)
// - Mirror Reckoning shadow personality influence from fleet composition
// - Full spawning helpers that pull from FLEET_CLASSES.md data

/*
 * PATSAGi Council Note:
 * These stubs are the foundation. Next iteration will wire
 * ShipVisualState directly into the render graph, VoiceDirector,
 * and WorldSimulationState for live moral/redeption/Hybrid reactivity.
 */
