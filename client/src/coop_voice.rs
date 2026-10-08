//! Lived-hour Co-op Voice — Slice 4 (v23.2.8)
//!
//! G opens Voice (quorum cards). E on the beacon votes aye; Digit2 nay.
//! Dies in Peace. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::coop_voice::CoopVoice;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hud_anchor_registry::VOICE;
use crate::hour_sacred::HourSacred;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};

/// Client wrap. Shared `CoopVoice` stays Bevy-free (HourSacred / FactoryYard pattern).
#[derive(Resource, Debug, Clone, Default)]
pub struct VoiceYard {
    pub voice: CoopVoice,
    pub sash_open: bool,
}

#[derive(Component)]
struct VoiceSlabRoot;
#[derive(Component)]
struct VoiceSlabText;

pub struct CoopVoicePlugin;

impl Plugin for CoopVoicePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoiceYard>()
            .add_systems(Startup, spawn_voice_slab)
            .add_systems(PreUpdate, mark_beacon_voice)
            .add_systems(Update, (handle_voice, update_voice_slab));
    }
}

fn spawn_voice_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: VOICE.bottom(),
                    right: VOICE.right(),
                    width: Val::Px(560.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: TITLE_PLATE_BG.with_alpha(1.0).into(),
                border_color: TITLE_BORDER.with_alpha(1.0).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            VoiceSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: TITLE_TEXT_PRIMARY,
                        ..default()
                    },
                ),
                VoiceSlabText,
            ));
        });
}

fn mark_beacon_voice(
    hour: Res<HourSacred>,
    yard: Res<VoiceYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    epi.beacon_voice = hour.charter_skin_live()
        && yard.sash_open
        && yard.voice.open_card().is_some();
}

fn handle_voice(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    mut yard: ResMut<VoiceYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if hour.hex() == shared::space_law::HexFlag::Peace {
        yard.sash_open = false;
        return;
    }
    if !hour.charter_skin_live() {
        yard.sash_open = false;
        return;
    }
    yard.voice.ensure_tutorial();
    if keyboard.just_pressed(soft_play_bindings::SASH) {
        yard.sash_open = !yard.sash_open;
        return;
    }
    if !yard.sash_open {
        return;
    }
    let aye = keyboard.just_pressed(soft_play_bindings::INTERACT)
        || keyboard.just_pressed(KeyCode::Digit1);
    let nay = keyboard.just_pressed(KeyCode::Digit2);
    if !aye && !nay {
        return;
    }
    let step = yard.voice.vote_local(aye);
    if step == "carried" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstVoice,
            time.elapsed_seconds_f64(),
        );
    }
}

fn update_voice_slab(
    hour: Res<HourSacred>,
    yard: Res<VoiceYard>,
    mut root: Query<&mut Visibility, With<VoiceSlabRoot>>,
    mut text_q: Query<&mut Text, With<VoiceSlabText>>,
) {
    let show = hour.charter_skin_live() && yard.sash_open;
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
    let line = yard.voice.beacon_line();
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
    use shared::space_law::HexFlag;

    /// CARD VP-SLABS-REGAL-3 — the co-op voice slab rests on the title palette:
    /// opaque TITLE_PLATE_BG plate, TITLE_BORDER rim at alpha 1, TITLE_TEXT_PRIMARY text.
    #[test]
    fn slabs_regal3_voice_slab_rests_on_title_palette() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_voice_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor), With<VoiceSlabRoot>>();
        let (border, bg) = q.single(app.world());
        let (border, bg) = (border.0.to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&Text, With<VoiceSlabText>>();
        let txt = t.single(app.world()).sections[0].style.color.to_srgba();
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

    /// CARD VOICE-ABOVE-LEDGER-1 — voice slab bottom clears the ledger
    /// (ledger bottom 16 + Model B height 119 = 135, plus a 9 px gap).
    #[test]
    fn voice_above_ledger1_slab_bottom_clears_ledger() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_voice_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Style, With<VoiceSlabRoot>>();
        let style = q.single(app.world());
        let bottom = style.bottom;
        assert_eq!(bottom, crate::hud_anchor_registry::VOICE.bottom());
        assert_eq!(style.right, crate::hud_anchor_registry::VOICE.right());
        assert_eq!(style.left, Val::Auto);
        assert_eq!(style.width, Val::Px(560.0));
        assert_eq!(style.margin, UiRect::default());
        let Val::Px(px) = bottom else {
            panic!("voice slab bottom is not Val::Px");
        };
        assert!(
            px > 135.0,
            "voice slab bottom {px} must clear ledger top 135"
        );
    }

    #[test]
    fn peace_keeps_sash_closed() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = VoiceYard::default();
        assert!(!yard.sash_open);
        assert!(yard.voice.cards.is_empty());
    }
}
