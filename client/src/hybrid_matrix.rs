//! Lived-hour Hybrid Matrix v0 — Slice 13 (v23.2.17)
//!
//! After Offer, E Attune — read-first stability tell, not a second body. Dies in Peace.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::hybrid_matrix::HybridMatrix;

use crate::coop_voice::VoiceYard;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hour_sacred::HourSacred;
use crate::hud_anchor_registry::{HudSlab, HYBRID};
use crate::ledger_bind::LedgerYard;
use crate::soft_play_bindings;
use crate::species_redemption::RedemptionYard;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};

#[derive(Resource, Debug, Clone, Default)]
pub struct HybridYard {
    pub matrix: HybridMatrix,
}

#[derive(Component)]
struct HybridSlabRoot;
#[derive(Component)]
struct HybridSlabText;

pub struct HybridMatrixPlugin;

impl Plugin for HybridMatrixPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HybridYard>()
            .add_systems(Startup, spawn_hybrid_slab)
            .add_systems(PreUpdate, mark_hybrid_near)
            .add_systems(Update, (handle_hybrid, update_hybrid_slab));
    }
}

fn spawn_hybrid_slab(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: HYBRID.top(),
                    right: HYBRID.right(),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG.with_alpha(1.0)),
                BorderColor::all(TITLE_BORDER.with_alpha(1.0)),
                Visibility::Hidden,
            ),
            HybridSlabRoot,
            HudSlab(HYBRID.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: 14.0 / 1.2, ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                HybridSlabText,
            ));
        });
}

fn hybrid_live(hour: &HourSacred, red: &RedemptionYard) -> bool {
    hour.charter_skin_live() && red.state.events > 0
}

fn mark_hybrid_near(
    hour: Res<HourSacred>,
    red: Res<RedemptionYard>,
    yard: Res<HybridYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    epi.hybrid_near = hybrid_live(&hour, &red)
        && !yard.matrix.attuned
        && !voice.sash_open
        && !ledger.sash_open;
}

fn handle_hybrid(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    red: Res<RedemptionYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut yard: ResMut<HybridYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if !hybrid_live(&hour, &red) {
        return;
    }
    yard.matrix.reveal();
    if voice.sash_open || ledger.sash_open {
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::INTERACT) {
        return;
    }
    let step = yard.matrix.attune();
    if step == "attuned" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstHybrid,
            time.elapsed_secs_f64(),
        );
    }
}

fn update_hybrid_slab(
    hour: Res<HourSacred>,
    red: Res<RedemptionYard>,
    yard: Res<HybridYard>,
    mut root: Query<&mut Visibility, With<HybridSlabRoot>>,
    mut text_q: Query<&mut Text, With<HybridSlabText>>,
) {
    let show = hybrid_live(&hour, &red);
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
    let line = yard.matrix.slab_line();
    for mut text in &mut text_q {
        if text.as_str() != line {
            **text = line.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::space_law::HexFlag;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Hybrid position is the registry. Style is the coded literal.
    #[test]
    fn hybrid_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_hybrid_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<HybridSlabRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Px(244.0),
            right: Val::Px(16.0),
            width: Val::Px(420.0),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
            justify_content: JustifyContent::FlexStart,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.top, HYBRID.top());
        assert_eq!(style.right, HYBRID.right());
        assert_eq!(style.margin, UiRect::default());
        assert_eq!(style.width, Val::Px(420.0));
    }

    /// CARD VP-SLABS-REGAL-2 — the hybrid matrix slab rests on the title palette: opaque
    /// TITLE_PLATE_BG plate, TITLE_BORDER rim at alpha 1, TITLE_TEXT_PRIMARY text.
    #[test]
    fn slabs_regal2_hybrid_slab_rests_on_title_palette() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_hybrid_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor), With<HybridSlabRoot>>();
        let (border, bg) = q.single(app.world()).unwrap();
        let (border, bg) = (({
            assert_eq!(border.top, border.right, "border edges");
            assert_eq!(border.top, border.bottom, "border edges");
            assert_eq!(border.top, border.left, "border edges");
            border.top
        }).to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&TextColor, With<HybridSlabText>>();
        let txt = t.single(app.world()).unwrap().0.to_srgba();
        for (got, want, what) in [
            (bg, TITLE_PLATE_BG.to_srgba(), "plate"),
            (border, TITLE_BORDER.to_srgba(), "rim"),
            (txt, TITLE_TEXT_PRIMARY.to_srgba(), "text"),
        ] {
            assert!((got.red - want.red).abs() < 1e-6, "{what} red");
            assert!((got.green - want.green).abs() < 1e-6, "{what} green");
            assert!((got.blue - want.blue).abs() < 1e-6, "{what} blue");
            assert_eq!(got.alpha, 1.0, "{what} alpha");
        }
    }

    #[test]
    fn peace_hides_hybrid() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let red = RedemptionYard::default();
        assert!(!hybrid_live(&hour, &red));
    }
}
