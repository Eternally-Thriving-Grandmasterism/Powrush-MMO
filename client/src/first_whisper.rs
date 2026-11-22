/*!
 * First Whisper — v22.3.0
 *
 * One sentence from the Lattice primer. Then silence.
 * Never a manifesto overlay.
 *
 * Source: content/rbe_onboarding_education.md — "What you nurture, nurtures all."
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hud_anchor_registry::{HudSlab, WHISPER};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};
use crate::hex_travel::HexTravelState;
use crate::local_session_persist::LocalSessionPersist;

const LINE: &str = "What you nurture, nurtures all.";
pub(crate) const HOLD_SECS: f64 = 5.2;

/// CARD FLESH-WHISPER-PLACE — the one Lattice sentence may name the Place.
/// Absent travel keeps `LINE` byte for byte. One sentence, then silence.
fn whisper_line(place: Option<&str>) -> String {
    match place {
        Some(place) => format!("{place} · {LINE}"),
        None => LINE.to_string(),
    }
}

#[derive(Resource, Debug, Default)]
struct WhisperClock {
    until: f64,
    showing: bool,
}

#[derive(Component)]
struct WhisperRoot;
#[derive(Component)]
struct WhisperText;

pub struct FirstWhisperPlugin;

impl Plugin for FirstWhisperPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WhisperClock>()
            .add_systems(Startup, spawn_whisper)
            .add_systems(Update, (maybe_speak, update_whisper));
    }
}

fn spawn_whisper(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: WHISPER.top(),
                    left: WHISPER.left(),
                    width: Val::Px(420.0),
                    margin: WHISPER.margin(),
                    padding: UiRect::axes(Val::Px(18.0), Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG),
                BorderColor::all(TITLE_BORDER),
                Visibility::Hidden,
            ),
            WhisperRoot,
            HudSlab(WHISPER.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(LINE),
TextFont { font_size: FontSize::Px(18.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                WhisperText,
            ));
        });
}

fn maybe_speak(
    harvest: Res<FirstHarvestEpiphany>,
    mut persist: ResMut<LocalSessionPersist>,
    mut clock: ResMut<WhisperClock>,
    time: Res<Time>,
    travel: Option<Res<HexTravelState>>,
    mut whisper: Query<&mut Text, With<WhisperText>>,
) {
    if persist.whisper_lived || clock.showing {
        return;
    }
    if !harvest.first_harvest_lived {
        return;
    }
    persist.whisper_lived = true;
    persist.dirty = true;
    clock.showing = true;
    clock.until = time.elapsed_secs_f64() + HOLD_SECS;
    if let Some(state) = travel.as_ref() {
        let dressed = whisper_line(Some(state.chip_name()));
        for mut text in &mut whisper {
            **text = dressed.clone();
        }
    }
    info!(target: "powrush::whisper", "one Lattice sentence — then silence");
}

fn update_whisper(
    clock: Res<WhisperClock>,
    time: Res<Time>,
    mut root: Query<&mut Visibility, With<WhisperRoot>>,
    calm: Option<Res<crate::first_harvest_epiphany::CalmQueue>>,
) {
    let now = time.elapsed_secs_f64(); // UI-CALM-1: in the burst, queue time holds the whisper (CL-U1)
    let show = clock.showing && calm.as_deref().filter(|c| !c.done(now)).map_or(now < clock.until, |c| c.live(crate::first_harvest_epiphany::CalmStep::Whisper, now));
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::hex_travel::PlaceId;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Whisper position is the registry, Style is the coded literal.
    #[test]
    fn whisper_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_whisper);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<WhisperRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(28.0),
            left: Val::Percent(50.0),
            width: Val::Px(420.0),
            margin: UiRect::left(Val::Px(-210.0)),
            padding: UiRect::axes(Val::Px(18.0), Val::Px(12.0)),
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.top, WHISPER.top());
        assert_eq!(style.left, WHISPER.left());
        assert_eq!(style.margin, WHISPER.margin());
        assert_eq!(style.width, Val::Px(WHISPER.width));
    }

    /// CARD FLESH-WHISPER-PLACE — Place prefixes the sentence; no travel keeps LINE.
    #[test]
    fn flesh_whisper_place_names_chip_or_keeps_line() {
        assert_eq!(whisper_line(None), LINE);
        assert_eq!(whisper_line(None), "What you nurture, nurtures all.");

        let travel = HexTravelState {
            current: PlaceId::Sanctuary,
        };
        let dressed = whisper_line(Some(travel.chip_name()));
        assert_eq!(dressed, "Sanctuary Prime · What you nurture, nurtures all.");
        assert!(dressed.ends_with(LINE));
        let low = dressed.to_lowercase();
        assert!(!low.contains("threshold"));
        assert!(!low.contains("gold"));
        assert!(!low.contains("market"));
        assert!(!low.contains("xp"));
    }

    /// TOAST-VIS-NEQ-1 TV1–TV4. Last tape of (Visibility, changed) on WhisperRoot; settle 2 then clear; 100 ms manual steps.
    #[derive(Resource, Default)] struct TvTape(Vec<(Visibility, bool)>);
    fn tv_tape(q: Query<Ref<Visibility>, With<WhisperRoot>>, mut t: ResMut<TvTape>) { t.0.extend(q.iter().map(|v| (*v, v.is_changed()))); }
    fn tv_run(app: &mut App, n: usize) -> Vec<(Visibility, bool)> { for _ in 0..n { app.update(); } std::mem::take(&mut app.world_mut().resource_mut::<TvTape>().0) }
    fn tv_settle(mut app: App) -> App { app.update(); app.update(); app.world_mut().resource_mut::<TvTape>().0.clear(); app }
    fn tv_hide(app: &mut App) { let e = app.world_mut().query_filtered::<Entity, With<WhisperRoot>>().single(app.world()).unwrap(); *app.world_mut().get_mut::<Visibility>(e).unwrap() = Visibility::Hidden; }
    const TV: Visibility = Visibility::Visible;
    const TH: Visibility = Visibility::Hidden;
    fn tv_app(hud: Option<crate::hud_presets::HudPresetId>) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).init_resource::<WhisperClock>().init_resource::<TvTape>().insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_millis(100))).add_systems(Startup, spawn_whisper).add_systems(Update, update_whisper).add_systems(Last, tv_tape);
        if let Some(id) = hud { app.add_plugins(crate::hud_presets::HudLayoutPlugin).insert_resource(crate::hud_presets::ActiveHudPreset { id: Some(id) }).insert_resource(crate::title_screen::LaunchDoor::InYard).init_resource::<crate::hex_travel::PlacesPlate>(); }
        tv_settle(app)
    }
    fn tv_show(app: &mut App) { let until = app.world().resource::<Time>().elapsed_secs_f64() + HOLD_SECS; *app.world_mut().resource_mut::<WhisperClock>() = WhisperClock { until, showing: true }; }
    #[test] fn toast_vis_neq_1_tv1_whisper_steady_hidden_unmarked() { let mut app = tv_app(None); assert_eq!(tv_run(&mut app, 3), vec![(TH, false); 3], "TV1 whisper steady hidden"); }
    #[test] fn toast_vis_neq_1_tv2_whisper_show_then_expire_once_each() {
        let mut app = tv_app(None); tv_show(&mut app);
        assert_eq!(tv_run(&mut app, 2), vec![(TV, true), (TV, false)], "TV2 whisper shows: Visible once, then quiet");
        let t = tv_run(&mut app, 60); let k = t.iter().position(|r| r.0 == TH).expect("TV2 whisper expires");
        assert_eq!(t, [vec![(TV, false); k], vec![(TH, true)], vec![(TH, false); 59 - k]].concat(), "TV2 whisper expires: Hidden once, then quiet");
    }
    #[test] fn toast_vis_neq_1_tv3_whisper_direct_hidden_restored_once() { let mut app = tv_app(None); tv_show(&mut app); tv_run(&mut app, 2); tv_hide(&mut app); assert_eq!(tv_run(&mut app, 2), vec![(TV, true), (TV, false)], "TV3 whisper direct Hidden write: restored once, then quiet"); }
    #[test] fn toast_vis_neq_1_tv4_whisper_real_hud_layout() {
        use crate::hud_presets::HudPresetId::{Classic, Management};
        let mut app = tv_app(Some(Classic)); tv_show(&mut app);
        assert_eq!(tv_run(&mut app, 2), vec![(TH, true); 2], "TV4 Classic InYard showing: clean-hidden, changed every frame on main and branch alike");
        let mut app = tv_app(Some(Management));
        assert_eq!(tv_run(&mut app, 2), vec![(TH, false); 2], "TV4 Management not showing: quiet");
        tv_show(&mut app); tv_run(&mut app, 1);
        assert_eq!(tv_run(&mut app, 2), vec![(TV, false); 2], "TV4 Management showing: steady Visible, quiet");
        app.world_mut().spawn((Node::default(), Visibility::Visible, HudSlab(crate::hud_anchor_registry::ID_THRIVING)));
        let t = tv_run(&mut app, 2); let yielded = app.world().resource::<crate::hud_presets::HudYielded>().0.contains(&WHISPER.id);
        assert_eq!((t.last().unwrap().0, yielded), (TH, true), "TV4 Management with a Thriving sentinel: Whisper yields by R2 into HudYielded");
    }
}
