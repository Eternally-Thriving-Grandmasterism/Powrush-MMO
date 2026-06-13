/*!
 * Powrush-MMO Ship Definitions & Foundational Stubs
 *
 * Core enums, components, and data structures for all ship classes
 * across the five playable races.
 *
 * Fully aligned with all documentation:
 * - DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md
 * - FLEET_CLASSES.md, SHIP_VISUAL_GUIDELINES.md + per-race visual bibles
 * - Human Hybrid Protocol, Redemption Mechanics, Mirror Reckoning, etc.
 *
 * PATSAGi Council + Ra-Thor Quantum Swarm approved • AG-SML v1.0
 */

use bevy::prelude::*;

/// The five core playable races of Powrush-MMO.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum PlayableRace {
    Quellorian,
    Draek,
    Human,
    Cydruid,
    Ambrosian,
}

/// Comprehensive ship class identifier mapping to all visual bibles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum ShipClass {
    // Quellorian
    QuellorianAetherInterceptor,
    QuellorianLuminarHeavyCruiser,
    QuellorianHarmonySupportCarrier,
    QuellorianSeraphimCapitalEscort,

    // Draek
    DraekSwarmDrone,
    DraekRavagerBioCorvette,
    DraekTyrantHeavyCruiser,
    DraekAbominationCapitalDevourer,
    DraekBroodSpire,

    // Human (Hybrid Protocol capable)
    HumanScavengerFrigate,
    HumanHybridCruiser,
    HumanMirrorCorvette,
    HumanForgeCarrier,
    HumanLastStandCapital,

    // Cydruid
    CydruidGroveWardenFrigate,
    CydruidRootNetworkArchitectCruiser,
    CydruidSymbiontSwarmCoordinator,
    CydruidRestorationWeaver,
    CydruidPlanetaryBalanceKeeper,

    // Ambrosian
    AmbrosianLatticeWeaverFrigate,
    AmbrosianHarmonicBladeCruiser,
    AmbrosianChoirShardCarrier,
    AmbrosianAttunementNexus,
    AmbrosianLatticeSovereignCapital,
}

impl ShipClass {
    /// Returns the PlayableRace this ship class belongs to.
    pub fn race(&self) -> PlayableRace {
        match self {
            ShipClass::QuellorianAetherInterceptor |
            ShipClass::QuellorianLuminarHeavyCruiser |
            ShipClass::QuellorianHarmonySupportCarrier |
            ShipClass::QuellorianSeraphimCapitalEscort => PlayableRace::Quellorian,

            ShipClass::DraekSwarmDrone |
            ShipClass::DraekRavagerBioCorvette |
            ShipClass::DraekTyrantHeavyCruiser |
            ShipClass::DraekAbominationCapitalDevourer |
            ShipClass::DraekBroodSpire => PlayableRace::Draek,

            ShipClass::HumanScavengerFrigate |
            ShipClass::HumanHybridCruiser |
            ShipClass::HumanMirrorCorvette |
            ShipClass::HumanForgeCarrier |
            ShipClass::HumanLastStandCapital => PlayableRace::Human,

            ShipClass::CydruidGroveWardenFrigate |
            ShipClass::CydruidRootNetworkArchitectCruiser |
            ShipClass::CydruidSymbiontSwarmCoordinator |
            ShipClass::CydruidRestorationWeaver |
            ShipClass::CydruidPlanetaryBalanceKeeper => PlayableRace::Cydruid,

            ShipClass::AmbrosianLatticeWeaverFrigate |
            ShipClass::AmbrosianHarmonicBladeCruiser |
            ShipClass::AmbrosianChoirShardCarrier |
            ShipClass::AmbrosianAttunementNexus |
            ShipClass::AmbrosianLatticeSovereignCapital => PlayableRace::Ambrosian,
        }
    }
}

/// Core visual and state component for every ship in the game.
/// Drives shaders, particles, voice processing, Hybrid Protocol, redemption, and Mirror Reckoning.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct ShipVisualState {
    pub race: PlayableRace,
    pub class: ShipClass,

    /// 0.0 = fully enslaved/corrupted, 1.0 = fully redeemed/attuned
    pub redemption_progress: f32,

    /// Human Hybrid Protocol stability (1.0 = stable, < 0.3 = dangerous)
    pub hybrid_stability: f32,

    /// Moral alignment: -100.0 (greed/consumption) to +100.0 (mercy/harmony)
    pub moral_alignment: f32,

    /// Crownstone corruption level (especially relevant for Luminari Exiles & corrupted Ambrosians)
    pub crownstone_corruption: f32,

    pub is_hybrid_active: bool,
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

/// Marker components for gameplay systems
#[derive(Component, Default)]
pub struct Boardable;

#[derive(Component, Default)]
pub struct ResonanceProjector;

#[derive(Component, Default)]
pub struct HivemindDominated;

/// Basic ship stats (expand later with full RPG data)
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

/// Initialization system — auto-inserts ShipVisualState when a ShipClass is added.
pub fn ship_visual_state_initialization_system(
    mut commands: Commands,
    query: Query<(Entity, &ShipClass), Added<ShipClass>>,
) {
    for (entity, ship_class) in query.iter() {
        commands.entity(entity).insert(ShipVisualState {
            race: ship_class.race(),
            class: *ship_class,
            ..default()
        });
    }
}

/*
 * Next Steps (PATSAGi Guidance):
 * 1. Wire ShipVisualState into render graph + custom WGSL for moral/redeption visual reactivity.
 * 2. Implement Human Hybrid Protocol attachment system.
 * 3. Connect to WorldSimulationState, CrownstoneState, and MirrorReckoningState.
 * 4. Add spawning helpers that respect FLEET_CLASSES.md data.
 * 5. Integrate with VoiceDirector for dynamic vocal processing based on state.
 */
