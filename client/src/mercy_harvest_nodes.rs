/*!
 * Mercy Harvest Nodes — embodied first-hour world (v22.6.0)
 *
 * Reach uses the human body. Recovery rate follows BiomeFeel.
 * climate_id maps 1:1 onto shared::climate_node LivedHour ids.
 *
 * T4 (H-2026-09-11-T4): one care-cycle offer after Idle-after-tend.
 * Zero persons OK. Dismiss always valid. Never blocks WASD / E.
 * No second HUD — transient one-card strip only while the offer is live.
 *
 * T5 (H-2026-09-11-T5): Distill Shell Ward on Stressed→Idle (Digit2).
 * Seats WardKind::Shell in an empty Lumen; harvest stress *= 0.7.
 * Climate Pick / Mend Spindle / Harmony Loom stay later. Online grey.
 *
 * PATSAGi + TOLC 8 | Contact: info@Rathor.ai | Yoi ⚡
 */

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

use bevy::prelude::*;

use shared::temper::{seat_ward, TemperError, TemperedItem, WardKind};

use crate::fabricator::FabricatorYard;
use crate::human_presence::SoftPresence;
use crate::living_ecology::BiomeFeel;
use crate::living_practice_loop::SoftPlayerRealm;

pub const HARVEST_REACH: f32 = 2.85;
const RECOVER_PER_SEC: f32 = 0.038;
const STING_SHARED: &str = "audio/mercy_harvest_sting.ogg";
const STING_SANCTUARY: &str = "audio/mercy_harvest_sting_sanctuary.ogg";
const STING_VERDANT: &str = "audio/mercy_harvest_sting_verdant.ogg";
const STING_HORIZON: &str = "audio/mercy_harvest_sting_horizon.ogg";

/// Tend writes pulse ≈ 0.55; harvest writes 1.0. Used to spot Idle-after-tend.
const TEND_PULSE: f32 = 0.55;
const TEND_PULSE_LO: f32 = 0.50;
const TEND_PULSE_HI: f32 = 0.60;
/// Vitality at or above this with pulse settled ⇒ Idle face for care-cycle.
pub const IDLE_VITALITY: f32 = 1.0;
/// Pre-tend vitality below this ⇒ Shell Ward path on the offer.
pub const STRESSED_VITALITY: f32 = 0.55;
/// T5 Shell Ward: turtle −30% well-stress (not PvP). `stress_inflicted *= 0.7`.
pub const SHELL_STRESS_FACTOR: f32 = 0.7;
/// Baseline vitality keep-fraction on harvest (stress fraction = 1 − keep).
const HARVEST_VITALITY_KEEP: f32 = 0.92;
/// Floor after harvest — Hook + Shell still cannot force below this alone.
const HARVEST_VITALITY_FLOOR: f32 = 0.45;

/// Live Shell Ward seat flag so `apply_node_harvest` can apply 0.7 without a second HUD file.
static SHELL_WARD_SEATED: AtomicBool = AtomicBool::new(false);

pub fn shell_ward_is_seated() -> bool {
    SHELL_WARD_SEATED.load(Ordering::Relaxed)
}

fn mark_shell_ward_seated(seated: bool) {
    SHELL_WARD_SEATED.store(seated, Ordering::Relaxed);
}

#[derive(Component, Debug)]
pub struct MercyHarvestNode {
    pub name: &'static str,
    pub climate_id: u32,
    pub vitality: f32,
    pub harvests: u32,
    pub pulse: f32,
}

#[derive(Resource, Debug)]
pub struct NearbyMercyNode {
    pub entity: Option<Entity>,
    pub name: Option<&'static str>,
    pub distance: f32,
    pub in_range: bool,
    pub nodes_exist: bool,
    pub last_harvested: Option<Entity>,
}

impl Default for NearbyMercyNode {
    fn default() -> Self {
        Self {
            entity: None,
            name: None,
            distance: f32::MAX,
            in_range: false,
            nodes_exist: false,
            last_harvested: None,
        }
    }
}

/// One choice on the Idle-after-tend card. Dismiss is always valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CareCycleChoice {
    TemperTool,
    DistillWard,
    Dismiss,
}

#[derive(Debug, Clone, Copy)]
struct NodeSnapshot {
    vitality: f32,
    pulse: f32,
}

#[derive(Debug, Clone)]
struct PendingCare {
    name: &'static str,
    from_stressed: bool,
    /// Offer already raised for this tend→idle; blocks spam while still Idle.
    emitted: bool,
}

/// MERCY_TEMPER T4 — one care-cycle offer after Idle-after-tend.
/// Works with zero persons. Never traps walk / E.
#[derive(Resource, Debug)]
pub struct CareCycleOffer {
    pub active: bool,
    pub node_entity: Option<Entity>,
    pub node_name: Option<&'static str>,
    /// True when the tend that led here was on a stressed (low-vitality) well.
    pub offer_shell: bool,
    pub choice: Option<CareCycleChoice>,
    snapshots: HashMap<Entity, NodeSnapshot>,
    pending: HashMap<Entity, PendingCare>,
}

impl Default for CareCycleOffer {
    fn default() -> Self {
        Self {
            active: false,
            node_entity: None,
            node_name: None,
            offer_shell: false,
            choice: None,
            snapshots: HashMap::new(),
            pending: HashMap::new(),
        }
    }
}

impl CareCycleOffer {
    pub fn dismiss(&mut self) {
        self.choice = Some(CareCycleChoice::Dismiss);
        self.active = false;
    }

    pub fn choose(&mut self, choice: CareCycleChoice) {
        self.choice = Some(choice);
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

/// Honest one-card line. No mall / P2W / gold.
pub fn care_cycle_card_line(offer_shell: bool) -> &'static str {
    if offer_shell {
        "Care cycle · 1 Temper · 2 Shell Ward · Esc dismiss"
    } else {
        "Care cycle · 1 Temper · 2 Distill Ward · Esc dismiss"
    }
}

/// Pure Idle-after-tend gate (pulse settled + full vitality + pending tend).
pub fn is_idle_after_tend(pulse: f32, vitality: f32, pending: bool, already_emitted: bool) -> bool {
    pending && !already_emitted && pulse <= 0.0 && vitality >= IDLE_VITALITY
}

/// Tend signature written by `apply_node_tend` (not harvest's pulse=1.0).
pub fn looks_like_tend_pulse(prev_pulse: f32, pulse: f32, prev_vitality: f32, vitality: f32) -> bool {
    prev_pulse < TEND_PULSE_LO
        && pulse >= TEND_PULSE_LO
        && pulse <= TEND_PULSE_HI
        && vitality + f32::EPSILON >= prev_vitality
}

/// Approximate pre-tend vitality after a tend write (vitality += 0.14, cap 1.0).
pub fn vitality_before_tend(vitality_after: f32) -> f32 {
    if vitality_after >= IDLE_VITALITY - f32::EPSILON {
        // Could have been anything in (0.86, 1.0]; treat high post as unknown — not stressed.
        (vitality_after - 0.14).clamp(0.0, IDLE_VITALITY)
    } else {
        (vitality_after - 0.14).max(0.0)
    }
}

/// Harvest stress multiplier: Shell Ward seats turtle mitigation at 0.7.
pub fn harvest_stress_factor(shell_ward_seated: bool) -> f32 {
    if shell_ward_seated {
        SHELL_STRESS_FACTOR
    } else {
        1.0
    }
}

/// Vitality after one harvest take. Shell multiplies inflicted stress by 0.7.
pub fn vitality_after_harvest(vitality: f32, shell_ward_seated: bool) -> f32 {
    let stress_frac = (1.0 - HARVEST_VITALITY_KEEP) * harvest_stress_factor(shell_ward_seated);
    (vitality * (1.0 - stress_frac)).max(HARVEST_VITALITY_FLOOR)
}

/// T5 Distill Shell Ward — seat `WardKind::Shell` in the first empty Lumen.
pub fn distill_shell_ward(item: &mut TemperedItem) -> Result<(), TemperError> {
    seat_ward(item, WardKind::Shell)
}

pub fn item_has_empty_lumen(item: &TemperedItem) -> bool {
    item.lumens.iter().any(|l| l.ward.is_none())
}

pub fn item_has_shell_ward(item: &TemperedItem) -> bool {
    item.lumens.iter().any(|l| l.ward == Some(WardKind::Shell))
}

pub fn sting_path_for_realm(realm: Option<u8>) -> &'static str {
    match realm {
        Some(0) | Some(3) => STING_SANCTUARY,
        Some(2) => STING_VERDANT,
        Some(4) | Some(1) => STING_HORIZON,
        _ => STING_SHARED,
    }
}

#[derive(Component)]
struct CareCycleStrip;

#[derive(Component)]
struct CareCycleStripText;

pub struct MercyHarvestNodesPlugin;

impl Plugin for MercyHarvestNodesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NearbyMercyNode>()
            .init_resource::<CareCycleOffer>()
            .add_systems(Startup, (spawn_mercy_nodes, spawn_care_cycle_strip))
            .add_systems(
                Update,
                (
                    track_nearby_node,
                    pulse_harvested_nodes,
                    watch_idle_after_tend,
                    handle_care_cycle_input,
                    update_care_cycle_strip,
                    try_soft_harvest_sting,
                ),
            );
    }
}

fn spawn_mercy_nodes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut nearby: ResMut<NearbyMercyNode>,
) {
    let mesh = meshes.add(Sphere::new(0.48));
    let placements: [(&'static str, u32, Vec3, Color); 3] = [
        (
            "Sanctuary ember",
            1,
            Vec3::new(3.6, 0.55, 0.0),
            Color::srgb(0.35, 0.95, 0.62),
        ),
        (
            "Verdant well",
            2,
            Vec3::new(-2.4, 0.55, 3.1),
            Color::srgb(0.45, 0.88, 0.95),
        ),
        (
            "Horizon seed",
            3,
            Vec3::new(1.2, 0.55, -3.4),
            Color::srgb(0.95, 0.86, 0.42),
        ),
    ];

    for (name, climate_id, pos, color) in placements {
        let emissive = LinearRgba::from(color).with_alpha(1.0) * 2.4;
        commands
            .spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: color,
                        emissive,
                        perceptual_roughness: 0.35,
                        metallic: 0.05,
                        ..default()
                    }),
                    transform: Transform::from_translation(pos),
                    ..default()
                },
                MercyHarvestNode {
                    name,
                    climate_id,
                    vitality: 1.0,
                    harvests: 0,
                    pulse: 0.0,
                },
                Name::new(format!("MercyNode:{name}")),
            ))
            .with_children(|c| {
                c.spawn(PointLightBundle {
                    point_light: PointLight {
                        color,
                        intensity: 420.0,
                        range: 6.5,
                        shadows_enabled: false,
                        ..default()
                    },
                    ..default()
                });
            });
    }
    nearby.nodes_exist = true;
    info!(target: "powrush::nodes", "three mercy harvest nodes seeded in the walk plane");
}

fn spawn_care_cycle_strip(mut commands: Commands) {
    // Transient one-card strip (not a second permanent HUD). Hidden until Idle-after-tend.
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(128.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(560.0),
                    margin: UiRect::left(Val::Px(-280.0)),
                    padding: UiRect::axes(Val::Px(16.0), Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: Color::srgba(0.03, 0.05, 0.04, 0.92).into(),
                border_color: Color::srgba(0.72, 0.92, 0.78, 0.75).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            CareCycleStrip,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    care_cycle_card_line(false),
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.94, 0.98, 0.90),
                        ..default()
                    },
                ),
                CareCycleStripText,
            ));
        });
}

fn player_xy(presence: Option<&SoftPresence>) -> Vec3 {
    presence.map(|p| p.position).unwrap_or(Vec3::ZERO)
}

fn track_nearby_node(
    presence: Option<Res<SoftPresence>>,
    nodes: Query<(Entity, &Transform, &MercyHarvestNode)>,
    mut nearby: ResMut<NearbyMercyNode>,
) {
    let pos = player_xy(presence.as_deref());
    nearby.nodes_exist = !nodes.is_empty();
    let mut best: Option<(Entity, &'static str, f32)> = None;
    for (entity, tf, node) in &nodes {
        let d = tf.translation.distance(pos);
        match best {
            None => best = Some((entity, node.name, d)),
            Some((_, _, bd)) if d < bd => best = Some((entity, node.name, d)),
            _ => {}
        }
    }
    if let Some((entity, name, d)) = best {
        nearby.entity = Some(entity);
        nearby.name = Some(name);
        nearby.distance = d;
        nearby.in_range = d <= HARVEST_REACH;
    } else {
        nearby.entity = None;
        nearby.name = None;
        nearby.distance = f32::MAX;
        nearby.in_range = false;
    }
}

fn pulse_harvested_nodes(
    time: Res<Time>,
    feel: Option<Res<BiomeFeel>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut nodes: Query<(
        &mut MercyHarvestNode,
        &mut Transform,
        &Handle<StandardMaterial>,
        Option<&Children>,
    )>,
    mut lights: Query<&mut PointLight>,
) {
    let dt = time.delta_seconds();
    let t = time.elapsed_seconds();
    let mul = feel.map(|f| f.regen_mul).unwrap_or(1.0);
    for (mut node, mut tf, handle, children) in &mut nodes {
        if node.pulse > 0.0 {
            node.pulse = (node.pulse - dt * 1.35).max(0.0);
        } else if node.vitality < 1.0 {
            node.vitality = (node.vitality + dt * RECOVER_PER_SEC * mul).min(1.0);
        }
        let breathe = 1.0 + (t * 1.7).sin() * 0.06 * node.vitality;
        let burst = 1.0 + node.pulse * 0.38;
        tf.scale = Vec3::splat(0.92 * breathe * burst);
        if let Some(mat) = materials.get_mut(handle) {
            let glow = 2.4 + node.pulse * 6.2;
            mat.emissive = LinearRgba::from(mat.base_color) * glow;
        }
        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(mut light) = lights.get_mut(*child) {
                    light.intensity = 420.0 + node.pulse * 2800.0;
                    light.range = 6.5 + node.pulse * 3.5;
                }
            }
        }
    }
}

/// Observe tend → Idle. Person count is never consulted (solo / empty Place OK).
fn watch_idle_after_tend(
    nodes: Query<(Entity, &MercyHarvestNode)>,
    mut offer: ResMut<CareCycleOffer>,
) {
    // One live offer at a time — no loop spam while the card is up.
    let mut busy = offer.active;

    for (entity, node) in &nodes {
        let prev = offer.snapshots.get(&entity).copied();
        if let Some(prev) = prev {
            if looks_like_tend_pulse(prev.pulse, node.pulse, prev.vitality, node.vitality) {
                let before = vitality_before_tend(node.vitality);
                // Prefer the snapshot vitality when it still looks pre-tend.
                let stressed_probe = if prev.vitality <= node.vitality {
                    prev.vitality
                } else {
                    before
                };
                offer.pending.insert(
                    entity,
                    PendingCare {
                        name: node.name,
                        from_stressed: stressed_probe < STRESSED_VITALITY,
                        emitted: false,
                    },
                );
            }
        }

        if !busy {
            let raise = offer.pending.get(&entity).map(|p| {
                (
                    is_idle_after_tend(node.pulse, node.vitality, true, p.emitted),
                    p.name,
                    p.from_stressed,
                )
            });
            if let Some((true, name, from_stressed)) = raise {
                if let Some(pending) = offer.pending.get_mut(&entity) {
                    pending.emitted = true;
                }
                offer.active = true;
                offer.node_entity = Some(entity);
                offer.node_name = Some(name);
                offer.offer_shell = from_stressed;
                offer.choice = None;
                busy = true;
                info!(
                    target: "powrush::temper",
                    "care-cycle offer after Idle-after-tend on {name} (shell={from_stressed})"
                );
            }
        }

        offer.snapshots.insert(
            entity,
            NodeSnapshot {
                vitality: node.vitality,
                pulse: node.pulse,
            },
        );
    }
}

/// Esc / 3 dismiss always. 1 Temper · 2 Ward. Never reads WASD or E.
fn handle_care_cycle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut offer: ResMut<CareCycleOffer>,
    mut yard: ResMut<FabricatorYard>,
) {
    if !offer.active {
        return;
    }
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Digit3) {
        offer.dismiss();
        return;
    }
    if keyboard.just_pressed(KeyCode::Digit1) {
        offer.choose(CareCycleChoice::TemperTool);
        return;
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        // T5: Stressed→Idle Digit2 is real Distill Shell Ward (seat empty Lumen).
        if offer.offer_shell {
            match yard.fab.last_tempered.as_mut() {
                Some(item) => match distill_shell_ward(item) {
                    Ok(()) => {
                        mark_shell_ward_seated(true);
                        info!(
                            target: "powrush::temper",
                            "Distill Shell Ward seated on empty Lumen (stress×{SHELL_STRESS_FACTOR})"
                        );
                    }
                    Err(TemperError::NoEmptyLumen) => {
                        info!(
                            target: "powrush::temper",
                            "Distill Shell Ward refused — no empty Lumen"
                        );
                    }
                    Err(other) => {
                        info!(
                            target: "powrush::temper",
                            "Distill Shell Ward refused — {other:?}"
                        );
                    }
                },
                None => {
                    info!(
                        target: "powrush::temper",
                        "Distill Shell Ward waiting — no tempered tool in satchel"
                    );
                }
            }
        }
        offer.choose(CareCycleChoice::DistillWard);
    }
}

fn update_care_cycle_strip(
    offer: Res<CareCycleOffer>,
    mut strips: Query<&mut Visibility, With<CareCycleStrip>>,
    mut texts: Query<&mut Text, With<CareCycleStripText>>,
) {
    let show = offer.active;
    let line = if show {
        care_cycle_card_line(offer.offer_shell).to_string()
    } else {
        String::new()
    };
    for mut vis in &mut strips {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if !offer.is_changed() && !show {
        return;
    }
    for mut text in &mut texts {
        if let Some(section) = text.sections.get_mut(0) {
            if section.value != line {
                section.value = line.clone();
            }
        }
    }
}

pub fn apply_node_harvest(node: &mut MercyHarvestNode) {
    node.harvests = node.harvests.saturating_add(1);
    // T5: when Shell Ward is seated, inflicted stress *= 0.7.
    node.vitality = vitality_after_harvest(node.vitality, shell_ward_is_seated());
    node.pulse = 1.0;
}

pub fn apply_node_tend(node: &mut MercyHarvestNode) {
    node.vitality = (node.vitality + 0.14).min(1.0);
    node.pulse = TEND_PULSE;
}

fn try_soft_harvest_sting(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    nearby: Res<NearbyMercyNode>,
    realm: Option<Res<SoftPlayerRealm>>,
    mut last: Local<Option<u32>>,
    nodes: Query<&MercyHarvestNode>,
    voice: Option<Res<crate::peace_audio::PeaceAudioState>>,
    mute: Option<Res<crate::local_settings::MasterMuteGain>>,
) {
    // U4 owns the well sting on the existing Use. Skip this path when the
    // Peace mixer is live so take does not double-fire. Still refuse to
    // blast when mute is on or the box has no output.
    if voice.is_some() {
        return;
    }
    if mute.map(|m| m.muted || m.gain <= 0.0).unwrap_or(false) {
        return;
    }
    if !shared::peace_audio::audio_output_safe() {
        return;
    }
    let Some(entity) = nearby.last_harvested else {
        return;
    };
    let Ok(node) = nodes.get(entity) else {
        return;
    };
    if *last == Some(node.harvests) {
        return;
    }
    *last = Some(node.harvests);
    let path = sting_path_for_realm(realm.and_then(|r| r.current));
    commands.spawn(AudioBundle {
        source: asset_server.load(path),
        settings: PlaybackSettings::DESPAWN,
        ..default()
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::temper::temper_copy_is_honest;

    #[test]
    fn harvest_leaves_node_alive() {
        mark_shell_ward_seated(false);
        let mut n = MercyHarvestNode {
            name: "test",
            climate_id: 1,
            vitality: 1.0,
            harvests: 0,
            pulse: 0.0,
        };
        apply_node_harvest(&mut n);
        assert!(n.vitality < 1.0 && n.vitality >= 0.45);
        assert!((n.vitality - 0.92).abs() < 1e-5);
        assert_eq!(n.harvests, 1);
    }

    #[test]
    fn tend_restores_vitality() {
        let mut n = MercyHarvestNode {
            name: "test",
            climate_id: 1,
            vitality: 0.50,
            harvests: 1,
            pulse: 0.0,
        };
        apply_node_tend(&mut n);
        assert!(n.vitality > 0.50);
        assert!((n.pulse - TEND_PULSE).abs() < f32::EPSILON);
    }

    #[test]
    fn wells_map_onto_lived_hour_ids() {
        assert_eq!(("Sanctuary ember", 1).1, 1);
        assert_eq!(("Verdant well", 2).1, 2);
        assert_eq!(("Horizon seed", 3).1, 3);
    }

    #[test]
    fn idle_after_tend_gate_requires_settled_pulse_and_full_vitality() {
        assert!(!is_idle_after_tend(0.55, 1.0, true, false));
        assert!(!is_idle_after_tend(0.0, 0.9, true, false));
        assert!(!is_idle_after_tend(0.0, 1.0, false, false));
        assert!(!is_idle_after_tend(0.0, 1.0, true, true));
        assert!(is_idle_after_tend(0.0, 1.0, true, false));
    }

    #[test]
    fn tend_pulse_signature_distinct_from_harvest() {
        assert!(looks_like_tend_pulse(0.0, TEND_PULSE, 0.50, 0.64));
        assert!(!looks_like_tend_pulse(0.0, 1.0, 1.0, 0.92)); // harvest
        assert!(!looks_like_tend_pulse(TEND_PULSE, TEND_PULSE, 0.64, 0.64));
    }

    #[test]
    fn care_cycle_dismiss_always_valid_and_clears_active() {
        let mut offer = CareCycleOffer {
            active: true,
            node_entity: None,
            node_name: Some("Verdant well"),
            offer_shell: false,
            choice: None,
            snapshots: HashMap::new(),
            pending: HashMap::new(),
        };
        offer.dismiss();
        assert!(!offer.active);
        assert_eq!(offer.choice, Some(CareCycleChoice::Dismiss));
        // Dismiss again stays safe (never traps).
        offer.dismiss();
        assert!(!offer.active);
        assert_eq!(offer.choice, Some(CareCycleChoice::Dismiss));
    }

    #[test]
    fn care_cycle_choices_temper_and_ward() {
        let mut offer = CareCycleOffer::default();
        offer.active = true;
        offer.choose(CareCycleChoice::TemperTool);
        assert!(!offer.active);
        assert_eq!(offer.choice, Some(CareCycleChoice::TemperTool));

        offer.active = true;
        offer.choose(CareCycleChoice::DistillWard);
        assert_eq!(offer.choice, Some(CareCycleChoice::DistillWard));
        assert!(!offer.active);
    }

    #[test]
    fn care_cycle_offer_works_with_zero_persons() {
        // No SoftPresence / person count consulted — empty Place is enough.
        let mut offer = CareCycleOffer::default();
        let pending = true;
        let emitted = false;
        assert!(is_idle_after_tend(0.0, 1.0, pending, emitted));
        offer.active = true;
        offer.node_name = Some("Sanctuary ember");
        offer.offer_shell = false;
        assert!(offer.is_active());
        offer.dismiss();
        assert!(!offer.is_active());
    }

    #[test]
    fn one_offer_not_spam_while_emitted() {
        assert!(is_idle_after_tend(0.0, 1.0, true, false));
        assert!(!is_idle_after_tend(0.0, 1.0, true, true));
    }

    #[test]
    fn care_cycle_copy_refuses_mall_p2w_gold() {
        let a = care_cycle_card_line(false);
        let b = care_cycle_card_line(true);
        assert!(temper_copy_is_honest(a), "{a}");
        assert!(temper_copy_is_honest(b), "{b}");
        assert!(!a.to_lowercase().contains("gold"));
        assert!(!b.to_lowercase().contains("mall"));
    }

    #[test]
    fn stressed_tend_marks_shell_path() {
        let before = 0.40_f32;
        assert!(before < STRESSED_VITALITY);
        let mut n = MercyHarvestNode {
            name: "test",
            climate_id: 1,
            vitality: before,
            harvests: 0,
            pulse: 0.0,
        };
        apply_node_tend(&mut n);
        let approx = vitality_before_tend(n.vitality);
        assert!(approx < STRESSED_VITALITY || before < STRESSED_VITALITY);
    }

    fn hook_with_empty_lumen(temper: u8) -> TemperedItem {
        let mut item = TemperedItem::hands(11, "stranger");
        item.tier = shared::temper::ToolTier::TendHook;
        item.temper = temper;
        let want = shared::temper::lumen_slots(temper) as usize;
        item.lumens = (0..want)
            .map(|i| shared::temper::Lumen {
                index: i as u8,
                ward: None,
            })
            .collect();
        item
    }

    #[test]
    fn distill_shell_ward_seats_empty_lumen() {
        let mut item = hook_with_empty_lumen(3);
        assert!(item_has_empty_lumen(&item));
        distill_shell_ward(&mut item).expect("empty lumen");
        assert!(item_has_shell_ward(&item));
        assert!(!item_has_empty_lumen(&item));
        assert_eq!(
            distill_shell_ward(&mut item),
            Err(TemperError::NoEmptyLumen)
        );
    }

    #[test]
    fn distill_shell_ward_needs_lumen_slot() {
        let mut bare = hook_with_empty_lumen(0);
        assert!(bare.lumens.is_empty());
        assert_eq!(
            distill_shell_ward(&mut bare),
            Err(TemperError::NoEmptyLumen)
        );
    }

    #[test]
    fn shell_stress_factor_is_point_seven() {
        assert!((harvest_stress_factor(false) - 1.0).abs() < f32::EPSILON);
        assert!((harvest_stress_factor(true) - SHELL_STRESS_FACTOR).abs() < f32::EPSILON);
        assert!((SHELL_STRESS_FACTOR - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn harvest_with_shell_inflicts_less_stress() {
        mark_shell_ward_seated(false);
        let v = 1.0_f32;
        let plain = vitality_after_harvest(v, false);
        let shelled = vitality_after_harvest(v, true);
        // Shell keeps more vitality (less stress inflicted).
        assert!(shelled > plain);
        assert!((plain - 0.92).abs() < 1e-5);
        let expected_shelled = 1.0 - (1.0 - 0.92) * 0.7;
        assert!((shelled - expected_shelled).abs() < 1e-5);
        assert!(shelled >= HARVEST_VITALITY_FLOOR);

        let mut n = MercyHarvestNode {
            name: "test",
            climate_id: 1,
            vitality: 1.0,
            harvests: 0,
            pulse: 0.0,
        };
        mark_shell_ward_seated(true);
        apply_node_harvest(&mut n);
        assert!((n.vitality - expected_shelled).abs() < 1e-5);
        mark_shell_ward_seated(false);
    }

    #[test]
    fn care_cycle_shell_copy_stays_honest() {
        let line = care_cycle_card_line(true);
        assert!(line.contains("Shell Ward"));
        assert!(temper_copy_is_honest(line), "{line}");
    }
}
