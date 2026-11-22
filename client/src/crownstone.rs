//! Lived-hour Crownstone set-piece — Slice 11 (v23.2.15)
//!
//! After Hour three held (book) + embassy seat, E Witness — read-first, not damage.
//! Path stays Unset. Dies in Peace. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::crownstone::CrownstoneState;

use crate::coop_voice::VoiceYard;
use crate::embassy::EmbassyYard;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hour_sacred::HourSacred;
use crate::ledger_bind::LedgerYard;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Resource, Debug, Clone, Default)]
pub struct CrownstoneYard {
    pub stone: CrownstoneState,
}

#[derive(Component)]
struct CrownstoneSlabRoot;
#[derive(Component)]
struct CrownstoneSlabText;

pub struct CrownstonePlugin;

impl Plugin for CrownstonePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CrownstoneYard>()
            .add_systems(Startup, spawn_crownstone_slab)
            .add_systems(PreUpdate, mark_crownstone_near)
            .add_systems(Update, (handle_crownstone, update_crownstone_slab));
    }
}

fn spawn_crownstone_slab(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(164.0),
                    right: Val::Px(16.0),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.10, 0.06, 0.14, 0.92)),
                BorderColor::all(Color::srgba(0.78, 0.52, 0.95, 0.55)),
                Visibility::Hidden,
            ),
            CrownstoneSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
TextColor(Color::srgb(0.94, 0.84, 1.0)),
),
                CrownstoneSlabText,
            ));
        });
}

fn stone_live(hour: &HourSacred, embassy: &EmbassyYard) -> bool {
    hour.hour_three_complete && hour.charter_skin_live() && embassy.embassy.seated
}

fn mark_crownstone_near(
    hour: Res<HourSacred>,
    yard: Res<CrownstoneYard>,
    embassy: Res<EmbassyYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    epi.crownstone_near = stone_live(&hour, &embassy)
        && !yard.stone.witnessed
        && !voice.sash_open
        && !ledger.sash_open;
}

fn handle_crownstone(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    embassy: Res<EmbassyYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut yard: ResMut<CrownstoneYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if !stone_live(&hour, &embassy) {
        return;
    }
    yard.stone.reveal();
    if voice.sash_open || ledger.sash_open {
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::INTERACT) {
        return;
    }
    let step = yard.stone.witness();
    if step == "witnessed" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstCrownstone,
            time.elapsed_secs_f64(),
        );
    }
}

fn update_crownstone_slab(
    hour: Res<HourSacred>,
    embassy: Res<EmbassyYard>,
    yard: Res<CrownstoneYard>,
    mut root: Query<&mut Visibility, With<CrownstoneSlabRoot>>,
    mut text_q: Query<&mut Text, With<CrownstoneSlabText>>,
) {
    let show = stone_live(&hour, &embassy);
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if !show {
        return;
    }
    let line = yard.stone.slab_line();
    for mut text in &mut text_q {
        if text.as_str() != line {
            **text = line.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embassy::EmbassyYard;
    use shared::space_law::HexFlag;

    #[test]
    fn hour_three_gates_witness() {
        let hour = HourSacred {
            session: shared::space_law::SpaceSession::default(),
            complete: true,
            hour_three_complete: false,
        };
        let embassy = EmbassyYard::default();
        assert!(!stone_live(&hour, &embassy));
    }

    #[test]
    fn peace_hides_stone() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let embassy = EmbassyYard::default();
        assert!(!stone_live(&hour, &embassy));
    }

    fn crown_app(inner: shared::local_settings::LocalSettings) -> App {
        use shared::embassy::Embassy;
        use shared::ledger_bind::LedgerBoard;
        use shared::space_law::{CharterKind, SpaceSession};
        let mut app = crate::heartwood_plaque::kh_sync::app(inner);
        app.insert_resource(HourSacred { session: SpaceSession { charter_id: Some("house-local".into()), hex: HexFlag::Frontier, kind: CharterKind::House, ..Default::default() }, complete: true, hour_three_complete: true }).insert_resource(EmbassyYard { embassy: Embassy { seated: true, ..Default::default() } }).insert_resource(LedgerYard { board: LedgerBoard::default(), sash_open: false }).insert_resource(VoiceYard::default()).insert_resource(ThrivingMoments::default()).insert_resource(FirstHarvestEpiphany::default()).add_plugins(CrownstonePlugin);
        app
    }
    fn witnessed(app: &App) -> bool { app.world().resource::<CrownstoneYard>().stone.witnessed }
    /// KEY-HINTS-SYNC-1 KH-C. Pad South witnesses; sash, rebound F, and default E are guards.
    #[test]
    fn kh_c_pad_south_sets_witnessed() {
        use shared::local_settings::{LocalSettings, PeaceKey};
        let mut sash = crown_app(LocalSettings::peace_defaults());
        sash.world_mut().resource_mut::<VoiceYard>().sash_open = true; crate::heartwood_plaque::kh_sync::south(&mut sash); sash.update(); assert!(!witnessed(&sash), "C2 sash");
        let mut rebound = LocalSettings::peace_defaults(); rebound.key_use = PeaceKey::F;
        let mut app = crown_app(rebound); crate::heartwood_plaque::kh_sync::key_f(&mut app); app.update(); assert!(witnessed(&app), "C3 rebound F");
        let mut app = crown_app(LocalSettings::peace_defaults());
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(soft_play_bindings::INTERACT); app.update(); assert!(witnessed(&app), "C4 default E");
        let mut app = crown_app(LocalSettings::peace_defaults()); app.update();
        let stone = &app.world().resource::<CrownstoneYard>().stone; assert!(!stone.witnessed && !stone.last_line.is_empty(), "C0");
        crate::heartwood_plaque::kh_sync::south(&mut app); app.update(); assert!(witnessed(&app), "KH-C");
    }
}
