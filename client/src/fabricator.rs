//! Lived-hour fabricator — Slice 7 (v23.2.11) + P2 week answer
//!
//! After Hour two held and the crate arrives, Q plants a fabricator then runs MendSpool and LaneCrate.
//! Mend/Lane refresh the week audit slab (tons + restored). Soft bench light (S+). Dies in Peace.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::fabricator::Fabricator;
use shared::hour_two::HourTwoPack;
use shared::space_law::HexFlag;

use crate::hour_sacred::{read_hour_two_json, HourSacred};
use crate::lived_hour_bind::LivedHourBind;
use crate::soft_play_bindings;
use crate::input::{InputMapSet, PlayerInput};
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::vertical_factory::FactoryYard;

#[derive(Resource, Debug, Clone)]
pub struct FabricatorYard {
    pub fab: Fabricator,
    /// Soft bench light after MendSpool / LaneCrate (S+). Not a HUD.
    pub bench_glow: f32,
}

impl Default for FabricatorYard {
    fn default() -> Self {
        if let Some(raw) = read_hour_two_json() {
            return Self {
                fab: HourTwoPack::from_json(&raw).fabricator,
                bench_glow: 0.0,
            };
        }
        Self {
            fab: Fabricator::default(),
            bench_glow: 0.0,
        }
    }
}

#[derive(Component)]
struct FabSlabRoot;
#[derive(Component)]
struct FabSlabText;

pub struct FabricatorPlugin;

impl Plugin for FabricatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FabricatorYard>()
            .add_systems(Startup, spawn_fab_slab)
            .add_systems(Update, (handle_fab_q, tick_bench_glow, update_fab_slab).after(InputMapSet));
    }
}

fn spawn_fab_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(88.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(520.0),
                    margin: UiRect::left(Val::Px(-260.0)),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.07, 0.06, 0.10, 0.90).into(),
                border_color: Color::srgba(0.78, 0.70, 0.92, 0.45).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            FabSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.92, 0.88, 1.0),
                        ..default()
                    },
                ),
                FabSlabText,
            ));
        });
}

fn handle_fab_q(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_input: Res<PlayerInput>,
    hour: Res<HourSacred>,
    factory: Res<FactoryYard>,
    mut yard: ResMut<FabricatorYard>,
    mut bind: ResMut<LivedHourBind>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    // Q / pad West (same sheet verb).
    if !(keyboard.just_pressed(soft_play_bindings::BUILD_WHEEL) || player_input.sheet_q) {
        return;
    }
    if hour.hex() == HexFlag::Peace || !hour.charter_skin_live() {
        return;
    }
    // Hour three civic door: fabricator after Hour two held.
    if !hour.complete {
        return;
    }
    if !factory.factory.tutorial_complete() {
        return;
    }
    let had_repair = yard.fab.pack.repair;
    let had_logi = yard.fab.pack.logi;
    let step = yard.fab.craft_next();
    if yard.fab.pack.repair && !had_repair {
        yard.bench_glow = 1.0;
        bind.climate.on_mend();
        bind.standing.on_mend();
        // P2: prefer week line after restored↑ — do not mute with standing-only clause.
        bind.refresh_climate_slab();
        bind.persist();
    }
    if yard.fab.pack.logi && !had_logi {
        yard.bench_glow = 1.0;
        bind.climate.on_lane();
        bind.standing.on_lane();
        // P2: prefer week line after tons↑ — stranger path hears the audit.
        bind.refresh_climate_slab();
        bind.persist();
    }
    if step == "unlocked" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstProofPack,
            time.elapsed_seconds_f64(),
        );
    }
}

fn tick_bench_glow(time: Res<Time>, mut yard: ResMut<FabricatorYard>) {
    if yard.bench_glow > 0.0 {
        yard.bench_glow = (yard.bench_glow - time.delta_seconds() * 0.55).max(0.0);
    }
}

fn update_fab_slab(
    hour: Res<HourSacred>,
    factory: Res<FactoryYard>,
    yard: Res<FabricatorYard>,
    mut root: Query<(&mut Visibility, &mut BorderColor, &mut BackgroundColor), With<FabSlabRoot>>,
    mut text_q: Query<&mut Text, With<FabSlabText>>,
) {
    let show = hour.complete && hour.charter_skin_live() && factory.factory.tutorial_complete();
    let glow = yard.bench_glow;
    for (mut vis, mut border, mut bg) in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        if show {
            let a = 0.45 + glow * 0.45;
            *border = Color::srgba(0.78 + glow * 0.18, 0.70 + glow * 0.22, 0.92, a).into();
            *bg = Color::srgba(0.07 + glow * 0.10, 0.06 + glow * 0.08, 0.10 + glow * 0.12, 0.90).into();
        }
    }
    if !show {
        return;
    }
    let line = yard.fab.slab_line();
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peace_does_not_plant() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = FabricatorYard::default();
        assert!(!yard.fab.planted);
    }
}
