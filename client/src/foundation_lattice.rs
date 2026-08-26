/*!
 * Foundation Lattice — soft educational dependency surface (v21.96.0)
 *
 * Toggle: **L** (Lattice — ergonomic left-hand)
 * Shows climates, allocate, journey, resonance (G), transporters (T), realm.
 *
 * PATSAGi + TOLC 8 · Contact: info@Rathor.ai · Yoi ⚡
 */

use bevy::prelude::*;

use crate::abundance_journey_echo::AbundanceJourneyEcho;
use crate::living_practice_loop::{LivingPracticeLoop, PracticeSurface, SoftPlayerRealm};
use crate::mercy_transporters::MercyTransporters;
use crate::rbe_allocate_choice::{AllocatePath, RbeAllocateChoice};
use crate::resonance_flavors::ResonanceState;
use crate::soft_play_bindings;

#[derive(Resource, Debug)]
pub struct FoundationLattice {
    pub panel_open: bool,
}

impl Default for FoundationLattice {
    fn default() -> Self {
        Self { panel_open: false }
    }
}

#[derive(Component)]
pub struct FoundationLatticeRoot;

#[derive(Component)]
pub struct FoundationLatticeBody;

pub struct FoundationLatticePlugin;

impl Plugin for FoundationLatticePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FoundationLattice>()
            .add_systems(Startup, spawn_lattice_panel)
            .add_systems(
                Update,
                (
                    toggle_lattice_panel,
                    update_lattice_visibility,
                    update_lattice_body,
                ),
            );
    }
}

fn spawn_lattice_panel(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(14.0),
                    right: Val::Percent(2.0),
                    width: Val::Px(380.0),
                    max_height: Val::Px(500.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    border: UiRect::all(Val::Px(1.5)),
                    border_radius: BorderRadius::all(Val::Px(14.0)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                background_color: Color::srgba(0.04, 0.07, 0.09, 0.95).into(),
                border_color: Color::srgba(0.55, 0.82, 0.72, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            FoundationLatticeRoot,
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "FOUNDATION LATTICE",
                TextStyle {
                    font_size: 15.5,
                    color: Color::srgb(0.78, 0.96, 0.88),
                    ..default()
                },
            ));
            p.spawn((
                TextBundle::from_section(
                    "Loading soft foundations…",
                    TextStyle {
                        font_size: 12.5,
                        color: Color::srgb(0.86, 0.92, 0.96),
                        ..default()
                    },
                ),
                FoundationLatticeBody,
            ));
            p.spawn(TextBundle::from_section(
                soft_play_bindings::soft_play_legend(),
                TextStyle {
                    font_size: 10.5,
                    color: Color::srgb(0.52, 0.68, 0.72),
                    ..default()
                },
            ));
        });
}

fn toggle_lattice_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut lattice: ResMut<FoundationLattice>,
) {
    if keyboard.just_pressed(soft_play_bindings::FOUNDATION_LATTICE) {
        lattice.panel_open = !lattice.panel_open;
        info!(
            target: "powrush::foundation",
            open = lattice.panel_open,
            "Foundation Lattice toggled"
        );
    }
}

fn update_lattice_visibility(
    lattice: Res<FoundationLattice>,
    mut q: Query<&mut Visibility, With<FoundationLatticeRoot>>,
) {
    for mut vis in &mut q {
        *vis = if lattice.panel_open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn climate_order(s: PracticeSurface) -> u8 {
    match s {
        PracticeSurface::SanctuaryCap => 0,
        PracticeSurface::VerdantSurge => 1,
        PracticeSurface::HorizonScarcity => 2,
        PracticeSurface::PrincipleSealed => 3,
    }
}

fn climate_mark(surface: PracticeSurface, current: PracticeSurface, sealed: bool) -> &'static str {
    if sealed {
        return if matches!(surface, PracticeSurface::PrincipleSealed) {
            "◎"
        } else {
            "○"
        };
    }
    let cur = climate_order(current);
    let here = climate_order(surface);
    if here == cur {
        "●"
    } else if here < cur {
        "○"
    } else {
        "·"
    }
}

fn build_lattice_body(
    practice: &LivingPracticeLoop,
    allocate: &RbeAllocateChoice,
    echo: &AbundanceJourneyEcho,
    soft_realm: &SoftPlayerRealm,
    resonance: &ResonanceState,
    transporters: &MercyTransporters,
) -> String {
    let sealed = practice.principle_sealed;
    let current = practice.surface;

    let climates = [
        PracticeSurface::SanctuaryCap,
        PracticeSurface::VerdantSurge,
        PracticeSurface::HorizonScarcity,
        PracticeSurface::PrincipleSealed,
    ];

    let mut climate_lines = String::new();
    for s in climates {
        let mark = climate_mark(s, current, sealed);
        climate_lines.push_str(&format!("{}  {}\n", mark, s.title()));
    }

    let harvest_progress = if sealed {
        "Principle sealed across climates".to_string()
    } else {
        format!(
            "Current climate progress  {}/{}",
            practice.mercy_harvests_on_surface, practice.harvests_needed
        )
    };

    let last_path = allocate
        .last_choice
        .map(|p| p.title())
        .unwrap_or("—");

    let realm_note = match soft_realm.current {
        Some(r) => format!("Soft realm resonance  #{r}"),
        None => "Soft realm resonance  open".to_string(),
    };

    let journey_depth = if echo.lines.is_empty() {
        "Journey echo  waiting for first thriving acts".to_string()
    } else {
        format!(
            "Journey echo  {} lines · sealed {}",
            echo.lines.len(),
            if echo.last_practice_sealed {
                "yes"
            } else {
                "not yet"
            }
        )
    };

    let resonance_block = format!(
        "RESONANCE  (G cycle)\n\
●  {}\n\
{}\n\
{}",
        resonance.current.title(),
        resonance.current.line(),
        resonance.current.soft_allocate_hint()
    );

    let transport_block = format!(
        "CARE HELPERS  (T)\n\
{}",
        transporters.status_line()
    );

    format!(
        "CLIMATES  (Caps Across Climates)\n\
{climates}\
{harvest}\n\
\n\
ALLOCATE DIRECTION\n\
→  Flow outward     {flow:.1}\n\
◇  Steward reserve  {reserve:.1}\n\
Last path  {last}\n\
Choices    {choices}\n\
\n\
{resonance}\n\
\n\
{transport}\n\
\n\
{journey}\n\
{realm}\n\
\n\
Durable foundations · care before force\n\
Voluntary mastery · never scarcity",
        climates = climate_lines.trim_end(),
        harvest = harvest_progress,
        flow = allocate.flow_total,
        reserve = allocate.reserve_total,
        last = last_path,
        choices = allocate.choices_made,
        resonance = resonance_block,
        transport = transport_block,
        journey = journey_depth,
        realm = realm_note,
    )
}

fn update_lattice_body(
    lattice: Res<FoundationLattice>,
    practice: Res<LivingPracticeLoop>,
    allocate: Res<RbeAllocateChoice>,
    echo: Res<AbundanceJourneyEcho>,
    soft_realm: Res<SoftPlayerRealm>,
    resonance: Res<ResonanceState>,
    transporters: Res<MercyTransporters>,
    mut q: Query<&mut Text, With<FoundationLatticeBody>>,
) {
    if !lattice.panel_open {
        return;
    }
    if !(lattice.is_changed()
        || practice.is_changed()
        || allocate.is_changed()
        || echo.is_changed()
        || soft_realm.is_changed()
        || resonance.is_changed()
        || transporters.is_changed())
    {
        return;
    }

    let body = build_lattice_body(
        &practice,
        &allocate,
        &echo,
        &soft_realm,
        &resonance,
        &transporters,
    );
    for mut text in &mut q {
        if let Some(s) = text.sections.get_mut(0) {
            s.value = body.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resonance_flavors::ResonanceFlavor;

    #[test]
    fn body_contains_core_layers() {
        let practice = LivingPracticeLoop {
            active: true,
            surface: PracticeSurface::VerdantSurge,
            mercy_harvests_on_surface: 1,
            harvests_needed: 2,
            ..Default::default()
        };
        let allocate = RbeAllocateChoice {
            flow_total: 2.0,
            reserve_total: 1.0,
            choices_made: 3,
            last_choice: Some(AllocatePath::FlowOutward),
            ..Default::default()
        };
        let echo = AbundanceJourneyEcho::default();
        let realm = SoftPlayerRealm { current: Some(2) };
        let resonance = ResonanceState {
            current: ResonanceFlavor::BalancedFlow,
            shifts: 0,
        };
        let transporters = MercyTransporters::default();
        let body = build_lattice_body(
            &practice,
            &allocate,
            &echo,
            &realm,
            &resonance,
            &transporters,
        );
        assert!(body.contains("Verdant"));
        assert!(body.contains("RESONANCE"));
        assert!(body.contains("CARE HELPERS") || body.contains("Mercy"));
        assert!(body.contains("never scarcity"));
    }
}
