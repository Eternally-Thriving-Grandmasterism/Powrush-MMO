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

// =============================================================================
// HUMAN HYBRID PROTOCOL — Expanded Implementation
// =============================================================================

/// Modules that can be attached to a Human ship via the Hybrid Protocol.
/// Directly implements the design from HUMAN_HYBRID_PROTOCOL_CODE.md
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum HybridModule {
    QuellorianResonance,
    DraekHivemind,
    CydruidEcological,
    AmbrosianAttunement,
    HumanBaseline, // Default / scavenged Human tech
}

/// Component attached to Human ships that have activated the Hybrid Protocol.
/// Tracks attached modules, stability, and activation time.
#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
pub struct ActiveHybrid {
    pub modules: Vec<HybridModule>,
    pub stability: f32,           // 0.0 - 1.0 (higher = more stable)
    pub instability_level: f32,   // 0.0 - 1.0 (derived from stability)
    pub activation_tick: u64,
}

impl Default for ActiveHybrid {
    fn default() -> Self {
        Self {
            modules: vec![HybridModule::HumanBaseline],
            stability: 1.0,
            instability_level: 0.0,
            activation_tick: 0,
        }
    }
}

/// Calculates hybrid stability based on attached modules and current game state.
/// Core formula from HUMAN_HYBRID_PROTOCOL_CODE.md + HYBRID_INSTABILITY_MECHANICS.md
/// This will later read from WorldSimulationState, MirrorReckoningState, etc.
pub fn calculate_hybrid_stability(
    modules: &[HybridModule],
    moral_alignment: f32,
    crownstone_corruption: f32,
    rbe_standing: f32,           // placeholder — from WorldSimulationState later
    mirror_shadow_influence: f32, // from MirrorReckoningState
) -> f32 {
    if modules.is_empty() {
        return 1.0;
    }

    let base = 0.65;
    let mut stability = base;

    let has_quellorian = modules.contains(&HybridModule::QuellorianResonance);
    let has_draek = modules.contains(&HybridModule::DraekHivemind);
    let has_cydruid = modules.contains(&HybridModule::CydruidEcological);
    let has_ambrosian = modules.contains(&HybridModule::AmbrosianAttunement);

    // Module synergy / conflict (from documentation)
    if has_quellorian && has_draek {
        stability -= 0.28; // Strong philosophical conflict
    }
    if has_cydruid && has_ambrosian {
        stability += 0.18; // Excellent ecological + harmonic synergy
    }
    if has_quellorian && has_ambrosian {
        stability += 0.12; // Strong resonance synergy
    }

    // Moral, corruption, and external modifiers
    stability += (moral_alignment / 200.0) * 0.35;
    stability -= crownstone_corruption * 0.45;
    stability += (rbe_standing - 0.5) * 0.25;
    stability -= mirror_shadow_influence * 0.30;

    stability.clamp(0.05, 1.0)
}

// =============================================================================
// HYBRID INSTABILITY EVENT SYSTEMS
// =============================================================================

/// Severity levels for Hybrid Instability events.
/// Directly implements the 4-tier system from HYBRID_INSTABILITY_MECHANICS.md
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Event, Reflect)]
#[reflect(Event)]
pub enum HybridInstabilitySeverity {
    Minor,      // 0.3 - 0.5 instability
    Moderate,   // 0.5 - 0.7
    Severe,     // 0.7 - 0.85
    Catastrophic, // > 0.85
}

/// Event emitted when a Human ship's Hybrid Protocol crosses instability thresholds.
/// Carries full context for VoiceDirector, VFX, Mirror Reckoning feedback, and mitigation systems.
#[derive(Event, Clone, Debug)]
pub struct HybridInstabilityEvent {
    pub entity: Entity,
    pub severity: HybridInstabilitySeverity,
    pub instability_level: f32,
    pub moral_alignment: f32,
    pub crownstone_corruption: f32,
    pub modules: Vec<HybridModule>,
}

/// System that detects instability threshold crossings and emits HybridInstabilityEvent.
/// This is the core of the Hybrid Instability Event Systems.
pub fn hybrid_instability_detection_system(
    mut query: Query<(Entity, &ActiveHybrid, &ShipVisualState), Changed<ActiveHybrid>>,
    mut event_writer: EventWriter<HybridInstabilityEvent>,
) {
    for (entity, hybrid, visual) in query.iter() {
        let level = hybrid.instability_level;

        let severity = if level > 0.85 {
            HybridInstabilitySeverity::Catastrophic
        } else if level > 0.7 {
            HybridInstabilitySeverity::Severe
        } else if level > 0.5 {
            HybridInstabilitySeverity::Moderate
        } else if level > 0.3 {
            HybridInstabilitySeverity::Minor
        } else {
            continue; // Below threshold, no event
        };

        // Only emit if severity actually changed or first time crossing (simple debounce via event consumption later)
        event_writer.send(HybridInstabilityEvent {
            entity,
            severity,
            instability_level: level,
            moral_alignment: visual.moral_alignment,
            crownstone_corruption: visual.crownstone_corruption,
            modules: hybrid.modules.clone(),
        });

        // TODO: Update a per-entity cooldown or last_severity tracker to prevent spam
        // TODO: Feed this event into MirrorReckoningState (unstable hybrids strengthen server Shadow)
    }
}

/// System that reacts to HybridInstabilityEvent (placeholder for VoiceDirector + VFX integration).
/// In full implementation this would trigger glitch layers, moral drift vocals, visual corruption,
/// and notify Mirror Reckoning that this ship's instability is feeding the server Shadow.
pub fn hybrid_instability_reaction_system(
    mut events: EventReader<HybridInstabilityEvent>,
    // TODO: ResMut<VoiceDirector>, ResMut<VfxEventQueue>, ResMut<MirrorReckoningState>
) {
    for event in events.read() {
        match event.severity {
            HybridInstabilitySeverity::Minor => {
                // Subtle glitch, slight vocal distortion
            }
            HybridInstabilitySeverity::Moderate => {
                // Noticeable energy feedback, moral drift in voice
            }
            HybridInstabilitySeverity::Severe => {
                // Strong visual corruption (auroral + purple tendrils), Crownstone influence spikes
            }
            HybridInstabilitySeverity::Catastrophic => {
                // Major backlash risk, possible Discordant Ambrosian outbreak if Ambrosian module present,
                // strong Mirror Shadow contribution, Hivelord attention
            }
        }

        // TODO: Call into mitigation opportunity window (HYBRID_INSTABILITY_MITIGATION_SYSTEMS.md)
        // TODO: Emit VFX event with severity + moral_alignment for shader parameters
        // TODO: Notify VoiceDirector to layer distortion/harmonic based on severity + modules
    }
}

/// Main update system for Hybrid Protocol (now also triggers instability detection).
pub fn hybrid_protocol_update_system(
    mut query: Query<(&mut ActiveHybrid, &mut ShipVisualState)>,
    // TODO: Add Res<WorldSimulationState>, Res<MirrorReckoningState> etc.
) {
    for (mut hybrid, mut visual) in query.iter_mut() {
        let new_stability = calculate_hybrid_stability(
            &hybrid.modules,
            visual.moral_alignment,
            visual.crownstone_corruption,
            0.65, // placeholder RBE standing
            0.12, // placeholder mirror shadow influence
        );

        hybrid.stability = new_stability;
        hybrid.instability_level = (1.0 - new_stability).max(0.0);

        visual.hybrid_stability = hybrid.stability;
        visual.is_hybrid_active = true;

        // Instability detection now happens in dedicated system via Changed<ActiveHybrid>
    }
}

/*
 * PATSAGi Council + Ra-Thor Quantum Swarm Next Steps:
 * 1. Wire HybridInstabilityEvent into VoiceDirector for dynamic vocal glitch/distortion layers
 * 2. Connect to MirrorReckoningState (unstable hybrids directly strengthen server Shadow personality)
 * 3. Implement mitigation opportunity windows (Cydruid Grove Stabilizers, Quellorian Tuners, Ambrosian Crystals, Human Innovation Tree)
 * 4. Add per-entity cooldown/debounce for instability events to prevent spam
 * 5. Expose severity to ShipVisualState or a new HybridVisualState for shader parameters (moral reactivity + corruption visuals)
 * 6. Integrate with Hivelord Counter-Strategies (Catastrophic instability draws Hivelord attention)
 * 7. Add Human Innovation Tree upgrades that improve mitigation success rates
 */
