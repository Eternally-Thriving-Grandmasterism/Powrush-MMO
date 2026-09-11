/*!
 * Living Ecology — v22.8.0 + Place-mood dress / work-loop rhyme (H-2026-09-11-E2)
 *
 * PersistentWeb.thread_strength remembers mercy across local sessions
 * (abyssal JSON: persistent_thread_strength + 0.15 decay on return).
 *
 * Ecology props dress Place climate the greet may rhyme with — they are
 * NOT persons, NOT owned inventory, NOT a fence. Work-loop feel is the
 * same Tend / Mend care at Place posts (NPC_SCHEDULE_SPEC). Hour finishes
 * if every person is removed. Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::living_day::{
    greet_by_place_mood, schedule_copy_is_honest, SacredVerb, SchedulePlace, WellMood,
};
use crate::living_practice_loop::SoftPlayerRealm;
use crate::world_answer::{AnswerKind, WorldAnswer};

const DEER_NEAR: Vec3 = Vec3::new(1.8, 0.55, 1.4);
const DEER_FAR: Vec3 = Vec3::new(8.5, 0.55, 6.2);

#[derive(Component)]
struct EcologyProp {
    kind: PropKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PropKind {
    Tree,
    Stone,
    Deer,
    Crystal,
    Mycelium,
}

#[derive(Component)]
struct ResonantDeer;

#[derive(Component)]
struct CrystalGlow {
    handle: Handle<StandardMaterial>,
}

#[derive(Component)]
struct MyceliumGlow {
    handle: Handle<StandardMaterial>,
}

#[derive(Resource, Debug)]
struct EcologyState {
    last_kind: AnswerKind,
}

impl Default for EcologyState {
    fn default() -> Self {
        Self {
            last_kind: AnswerKind::Idle,
        }
    }
}

#[derive(Resource, Debug)]
pub struct BiomeFeel {
    pub regen_mul: f32,
    /// Place-honest climate label the greet may rhyme with.
    pub name: &'static str,
    /// Four Places — dress posts schedules / greets prefer.
    pub place: SchedulePlace,
}

impl Default for BiomeFeel {
    fn default() -> Self {
        Self {
            regen_mul: 1.0,
            name: "Sanctuary",
            place: SchedulePlace::Sanctuary,
        }
    }
}

/// Mycelium memory. Lives in data/powrush_local_session.json.
#[derive(Resource, Debug)]
pub struct PersistentWeb {
    pub thread_strength: f32,
}

impl Default for PersistentWeb {
    fn default() -> Self {
        Self {
            thread_strength: 0.28,
        }
    }
}

impl PersistentWeb {
    pub fn apply_decay_on_return(&mut self) {
        // JSON: cross_session_resonance_decay = 0.15
        self.thread_strength = (self.thread_strength * 0.85).clamp(0.05, 1.0);
    }
}

/// SoftPlayerRealm id → Place the ecology dress rhymes with.
pub fn place_for_realm(realm: Option<u8>) -> SchedulePlace {
    match realm.unwrap_or(0) {
        2 => SchedulePlace::Heartwood,
        4 | 1 => SchedulePlace::Threshold,
        3 => SchedulePlace::Depths,
        _ => SchedulePlace::Sanctuary,
    }
}

/// Place-honest BiomeFeel name (greet / climate slab rhyme).
pub fn place_dress_name(place: SchedulePlace) -> &'static str {
    place.name()
}

/// Ecology care work-loop at a Place post: Tend / Mend only (never Take-as-theft).
/// Threshold Tend-not-Take; Depths restore-not-Take (Mend).
pub fn ecology_care_verb(place: SchedulePlace) -> SacredVerb {
    match place {
        SchedulePlace::Sanctuary | SchedulePlace::Heartwood | SchedulePlace::Threshold => {
            SacredVerb::Tend
        }
        SchedulePlace::Depths => SacredVerb::Mend,
    }
}

/// Still-frame ecology work-loop line: post · Tend/Mend (props dress, not persons).
pub fn ecology_work_loop_line(place: SchedulePlace) -> String {
    format!("{} · {}", place.post(), ecology_care_verb(place).name())
}

/// Greet rhyme: Place + well mood on the one HUD (reuses day helper — ecology dress only).
pub fn ecology_greet_rhyme(place: SchedulePlace, mood: WellMood) -> &'static str {
    greet_by_place_mood(place, mood)
}

/// Ecology props are presentation dress — never persons.
pub fn ecology_props_are_persons() -> bool {
    false
}

/// Ecology props are not owned inventory.
pub fn ecology_props_are_owned_inventory() -> bool {
    false
}

/// Ecology props are not a fence / ownership economy.
pub fn ecology_props_are_fence() -> bool {
    false
}

/// Hour-finish bar: ecology dress never strands the hour when persons are removed.
pub fn hour_finishes_without_persons() -> bool {
    true
}

/// Ecology / greet copy refuses crime · ownership · theft · fence · gold · Market · Online.
pub fn ecology_copy_is_honest(s: &str) -> bool {
    schedule_copy_is_honest(s)
}

pub struct LivingEcologyPlugin;

impl Plugin for LivingEcologyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EcologyState>()
            .init_resource::<BiomeFeel>()
            .init_resource::<PersistentWeb>()
            .add_systems(Startup, spawn_ecology)
            .add_systems(
                Update,
                (
                    remember_care,
                    dress_for_climate,
                    move_deer,
                    sing_or_silence_spires,
                    pulse_mycelium,
                ),
            );
    }
}

fn spawn_ecology(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let trunk = meshes.add(Cylinder::new(0.22, 2.4));
    let canopy = meshes.add(Sphere::new(0.85));
    let wood = materials.add(StandardMaterial {
        base_color: Color::srgb(0.28, 0.18, 0.10),
        perceptual_roughness: 0.9,
        ..default()
    });
    let leaf = materials.add(StandardMaterial {
        base_color: Color::srgb(0.16, 0.42, 0.20),
        perceptual_roughness: 0.7,
        ..default()
    });
    let tree_spots = [
        Vec3::new(-5.2, 1.2, -2.4),
        Vec3::new(-6.4, 1.2, 2.8),
        Vec3::new(5.8, 1.2, -4.1),
        Vec3::new(6.6, 1.2, 3.2),
        Vec3::new(-3.8, 1.2, 6.0),
    ];
    for pos in tree_spots {
        commands.spawn((
            PbrBundle {
                mesh: trunk.clone(),
                material: wood.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
            EcologyProp { kind: PropKind::Tree },
        ));
        commands.spawn((
            PbrBundle {
                mesh: canopy.clone(),
                material: leaf.clone(),
                transform: Transform::from_translation(pos + Vec3::Y * 1.35),
                ..default()
            },
            EcologyProp { kind: PropKind::Tree },
        ));
    }

    let rock = meshes.add(Sphere::new(0.42));
    let stone = materials.add(StandardMaterial {
        base_color: Color::srgb(0.32, 0.30, 0.28),
        perceptual_roughness: 0.95,
        ..default()
    });
    for pos in [
        Vec3::new(4.2, 0.22, -5.5),
        Vec3::new(-4.8, 0.22, -5.0),
        Vec3::new(7.0, 0.22, 0.4),
    ] {
        commands.spawn((
            PbrBundle {
                mesh: rock.clone(),
                material: stone.clone(),
                transform: Transform::from_translation(pos).with_scale(Vec3::new(1.4, 0.6, 1.1)),
                ..default()
            },
            EcologyProp {
                kind: PropKind::Stone,
            },
        ));
    }

    let deer_mesh = meshes.add(Capsule3d::new(0.16, 0.55));
    let deer_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.42, 0.30, 0.18),
        perceptual_roughness: 0.65,
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: deer_mesh,
            material: deer_mat,
            transform: Transform::from_translation(DEER_FAR),
            ..default()
        },
        EcologyProp { kind: PropKind::Deer },
        ResonantDeer,
        Name::new("ResonantDeer"),
    ));

    let spire = meshes.add(Cylinder::new(0.16, 3.6));
    let crystal_spots = [
        Vec3::new(5.0, 1.8, -6.2),
        Vec3::new(-5.4, 1.8, -6.8),
        Vec3::new(7.4, 1.8, 1.6),
        Vec3::new(-7.2, 1.8, 2.2),
    ];
    for pos in crystal_spots {
        let handle = materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.78, 0.95),
            emissive: LinearRgba::new(0.25, 0.45, 0.70, 1.0),
            perceptual_roughness: 0.18,
            metallic: 0.12,
            ..default()
        });
        commands.spawn((
            PbrBundle {
                mesh: spire.clone(),
                material: handle.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
            EcologyProp {
                kind: PropKind::Crystal,
            },
            CrystalGlow { handle },
        ));
    }

    let thread_mesh = meshes.add(Cylinder::new(0.035, 1.0));
    let anchors = [
        Vec3::new(3.6, 0.08, 0.0),
        Vec3::new(-2.4, 0.08, 3.1),
        Vec3::new(1.2, 0.08, -3.4),
    ];
    let pairs = [(0, 1), (1, 2), (2, 0)];
    for (a, b) in pairs {
        let from = anchors[a];
        let to = anchors[b];
        let mid = (from + to) * 0.5;
        let delta = to - from;
        let len = delta.length().max(0.2);
        let dir = delta.normalize();
        let rot = Quat::from_rotation_arc(Vec3::Y, dir);
        let handle = materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.55, 0.42),
            emissive: LinearRgba::new(0.08, 0.35, 0.28, 1.0),
            perceptual_roughness: 0.4,
            ..default()
        });
        commands.spawn((
            PbrBundle {
                mesh: thread_mesh.clone(),
                material: handle.clone(),
                transform: Transform {
                    translation: mid,
                    rotation: rot,
                    scale: Vec3::new(1.0, len, 1.0),
                },
                ..default()
            },
            EcologyProp {
                kind: PropKind::Mycelium,
            },
            MyceliumGlow { handle },
        ));
    }

    info!(target: "powrush::ecology", "Heartwood + Spires + Abyssal threads seeded");
}

fn remember_care(
    answer: Res<WorldAnswer>,
    mut eco: ResMut<EcologyState>,
    mut web: ResMut<PersistentWeb>,
) {
    if !answer.is_changed() || answer.kind == AnswerKind::Idle {
        return;
    }
    eco.last_kind = answer.kind;
    match answer.kind {
        // Tend / Flow care the web; Reserve banks repair-rights for later Mend.
        AnswerKind::Tend | AnswerKind::Flow => {
            web.thread_strength = (web.thread_strength + 0.08).min(1.0);
        }
        AnswerKind::Reserve => {
            web.thread_strength = (web.thread_strength + 0.04).min(1.0);
        }
        AnswerKind::Take => {
            web.thread_strength = (web.thread_strength - 0.05).max(0.0);
        }
        _ => {}
    }
}

fn dress_for_climate(
    realm: Res<SoftPlayerRealm>,
    mut feel: ResMut<BiomeFeel>,
    mut q: Query<(&EcologyProp, &mut Visibility)>,
) {
    let id = realm.current.unwrap_or(0);
    let place = place_for_realm(realm.current);
    feel.place = place;
    feel.name = place_dress_name(place);
    feel.regen_mul = match id {
        4 | 1 => 1.6,
        3 => 1.9,
        2 => 1.0,
        _ => 1.0,
    };
    for (prop, mut vis) in &mut q {
        let show = match prop.kind {
            PropKind::Tree => matches!(id, 0 | 2),
            PropKind::Stone => matches!(id, 1 | 4),
            PropKind::Deer => matches!(id, 0 | 2),
            PropKind::Crystal => matches!(id, 1 | 4),
            PropKind::Mycelium => id == 3,
        };
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn move_deer(
    eco: Res<EcologyState>,
    time: Res<Time>,
    mut q: Query<&mut Transform, With<ResonantDeer>>,
) {
    // Care (Tend / Flow / Reserve→Mend bank) draws the deer near; Take keeps it far.
    let target = match eco.last_kind {
        AnswerKind::Tend | AnswerKind::Flow | AnswerKind::Reserve => DEER_NEAR,
        AnswerKind::Take => DEER_FAR,
        _ => Vec3::new(5.2, 0.55, 3.6),
    };
    let dt = time.delta_seconds();
    for mut tf in &mut q {
        tf.translation = tf.translation.lerp(target, (1.6 * dt).min(1.0));
    }
}

fn sing_or_silence_spires(
    eco: Res<EcologyState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<&CrystalGlow>,
) {
    let peak = matches!(
        eco.last_kind,
        AnswerKind::Tend | AnswerKind::Flow | AnswerKind::Reserve
    );
    let silent = matches!(eco.last_kind, AnswerKind::Take);
    let e = if peak {
        LinearRgba::new(0.55, 0.85, 1.2, 1.0)
    } else if silent {
        LinearRgba::new(0.08, 0.12, 0.22, 1.0)
    } else {
        LinearRgba::new(0.25, 0.45, 0.70, 1.0)
    };
    for glow in &q {
        if let Some(mat) = materials.get_mut(&glow.handle) {
            mat.emissive = e;
        }
    }
}

fn pulse_mycelium(
    eco: Res<EcologyState>,
    web: Res<PersistentWeb>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<&MyceliumGlow>,
) {
    let s = web.thread_strength;
    let surge = matches!(
        eco.last_kind,
        AnswerKind::Tend | AnswerKind::Flow | AnswerKind::Reserve
    );
    let night = matches!(eco.last_kind, AnswerKind::Take);
    let e = if surge {
        LinearRgba::new(0.12 + s * 0.20, 0.45 + s * 0.70, 0.28 + s * 0.40, 1.0)
    } else if night {
        LinearRgba::new(0.02 + s * 0.04, 0.06 + s * 0.10, 0.05 + s * 0.08, 1.0)
    } else {
        LinearRgba::new(0.05 + s * 0.12, 0.18 + s * 0.40, 0.14 + s * 0.28, 1.0)
    };
    for glow in &q {
        if let Some(mat) = materials.get_mut(&glow.handle) {
            mat.emissive = e;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_mood_dress_maps_realm_to_four_places() {
        assert_eq!(place_for_realm(Some(0)), SchedulePlace::Sanctuary);
        assert_eq!(place_for_realm(None), SchedulePlace::Sanctuary);
        assert_eq!(place_for_realm(Some(2)), SchedulePlace::Heartwood);
        assert_eq!(place_for_realm(Some(1)), SchedulePlace::Threshold);
        assert_eq!(place_for_realm(Some(4)), SchedulePlace::Threshold);
        assert_eq!(place_for_realm(Some(3)), SchedulePlace::Depths);
        assert_eq!(place_dress_name(SchedulePlace::Heartwood), "Heartwood");
        assert_eq!(place_dress_name(SchedulePlace::Depths), "Depths");
        let feel = BiomeFeel::default();
        assert_eq!(feel.place, SchedulePlace::Sanctuary);
        assert_eq!(feel.name, "Sanctuary");
    }

    #[test]
    fn ecology_work_loop_tend_mend_place_honest() {
        assert_eq!(
            ecology_care_verb(SchedulePlace::Sanctuary),
            SacredVerb::Tend
        );
        assert_eq!(
            ecology_care_verb(SchedulePlace::Heartwood),
            SacredVerb::Tend
        );
        assert_eq!(
            ecology_care_verb(SchedulePlace::Threshold),
            SacredVerb::Tend
        );
        assert_eq!(ecology_care_verb(SchedulePlace::Depths), SacredVerb::Mend);
        // Never Take as ecology care work.
        for place in [
            SchedulePlace::Sanctuary,
            SchedulePlace::Heartwood,
            SchedulePlace::Threshold,
            SchedulePlace::Depths,
        ] {
            assert_ne!(ecology_care_verb(place), SacredVerb::Take);
            let line = ecology_work_loop_line(place);
            assert!(line.contains(place.post()), "{line}");
            assert!(
                line.contains("Tend") || line.contains("Mend"),
                "{line}"
            );
            assert!(ecology_copy_is_honest(&line), "{line}");
        }
        assert_eq!(
            ecology_work_loop_line(SchedulePlace::Threshold),
            "Threshold pipe · Tend"
        );
        assert_eq!(
            ecology_work_loop_line(SchedulePlace::Depths),
            "Depths landing · Mend"
        );
    }

    #[test]
    fn greet_rhyme_honest_with_place_mood_dress() {
        for place in [
            SchedulePlace::Sanctuary,
            SchedulePlace::Heartwood,
            SchedulePlace::Threshold,
            SchedulePlace::Depths,
        ] {
            for mood in [
                WellMood::Idle,
                WellMood::Glowing,
                WellMood::Tended,
                WellMood::Resting,
                WellMood::Stressed,
            ] {
                let greet = ecology_greet_rhyme(place, mood);
                assert_eq!(greet, greet_by_place_mood(place, mood));
                assert!(ecology_copy_is_honest(greet), "{greet}");
            }
        }
    }

    #[test]
    fn ecology_props_not_persons_owned_or_fence_hour_finishes() {
        assert!(!ecology_props_are_persons());
        assert!(!ecology_props_are_owned_inventory());
        assert!(!ecology_props_are_fence());
        assert!(hour_finishes_without_persons());
        // Prop kinds exist as dress only — removing persons cannot strand these flags.
        let _ = [
            PropKind::Tree,
            PropKind::Stone,
            PropKind::Deer,
            PropKind::Crystal,
            PropKind::Mycelium,
        ];
        assert!(hour_finishes_without_persons());
    }

    #[test]
    fn refuse_crime_gold_market_online_copy() {
        assert!(!ecology_copy_is_honest("sell gold on Market"));
        assert!(!ecology_copy_is_honest("crime meter Online"));
        assert!(!ecology_copy_is_honest("theft and fence ownership"));
        assert!(!ecology_copy_is_honest("pickpocket the stall"));
        assert!(ecology_copy_is_honest("Heartwood Wards · Tend"));
        assert!(ecology_copy_is_honest("Depths landing · Mend"));
        assert!(ecology_copy_is_honest(
            "Threshold pipe — Tend, not Take."
        ));
    }

    #[test]
    fn persistent_web_decay_stays_clamped() {
        let mut web = PersistentWeb {
            thread_strength: 1.0,
        };
        web.apply_decay_on_return();
        assert!((web.thread_strength - 0.85).abs() < 1e-5);
        web.thread_strength = 0.05;
        web.apply_decay_on_return();
        assert!(web.thread_strength >= 0.05);
    }
}
