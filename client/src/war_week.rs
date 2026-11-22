//! Lived-hour War week — Slice 9 (v23.2.13)
//!
//! Tab Chart declares the week. Score is tons + restored. Dies in Peace.
//! No combat key. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::space_law::HexFlag;
use shared::war_week::WarWeek;

use crate::fabricator::FabricatorYard;
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};
use crate::hour_sacred::HourSacred;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Resource, Debug, Clone, Default)]
pub struct WarYard {
    pub week: WarWeek,
    pub sash_open: bool,
}

#[derive(Component)]
struct WarSlabRoot;
#[derive(Component)]
struct WarSlabText;

pub struct WarWeekPlugin;

impl Plugin for WarWeekPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WarYard>()
            .add_systems(Startup, spawn_war_slab)
            .add_systems(Update, (handle_war_chart, ingest_graph, update_war_slab));
    }
}

fn spawn_war_slab(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(52.0),
                    right: Val::Px(16.0),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG),
                BorderColor::all(TITLE_BORDER),
                Visibility::Hidden,
            ),
            WarSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                WarSlabText,
            ));
        });
}

fn handle_war_chart(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    mut yard: ResMut<WarYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if hour.hex() == HexFlag::Peace || !hour.charter_skin_live() {
        yard.sash_open = false;
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::CHART) {
        return;
    }
    yard.sash_open = !yard.sash_open;
    if yard.sash_open && !yard.week.declared {
        let step = yard.week.declare();
        if step == "declared" {
            fire_thriving(
                &mut moments,
                ThrivingKind::FirstWarWeek,
                time.elapsed_secs_f64(),
            );
        }
    }
}

fn ingest_graph(fab: Res<FabricatorYard>, mut yard: ResMut<WarYard>) {
    let tons = if fab.fab.pack.logi { 1.0 } else { 0.0 };
    let restored = if fab.fab.pack.repair { 1 } else { 0 };
    yard.week.ingest(tons, restored);
}

fn update_war_slab(
    hour: Res<HourSacred>,
    yard: Res<WarYard>,
    mut root: Query<&mut Visibility, With<WarSlabRoot>>,
    mut text_q: Query<&mut Text, With<WarSlabText>>,
) {
    let show = hour.charter_skin_live() && yard.sash_open;
    for mut vis in &mut root {
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
    }
    if !show {
        return;
    }
    let line = yard.week.line();
    for mut text in &mut text_q {
        if text.as_str() != line {
            **text = line.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peace_does_not_declare() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = WarYard::default();
        assert!(!yard.week.declared);
        assert!(!yard.sash_open);
    }

    /// TOAST-VIS-NEQ-1 TV9–TV11. Last tape of (Visibility, changed) on WarSlabRoot; settle 2 then clear.
    #[derive(Resource, Default)] struct TvTape(Vec<(Visibility, bool)>);
    fn tv_tape(q: Query<Ref<Visibility>, With<WarSlabRoot>>, mut t: ResMut<TvTape>) { t.0.extend(q.iter().map(|v| (*v, v.is_changed()))); }
    fn tv_run(app: &mut App, n: usize) -> Vec<(Visibility, bool)> { for _ in 0..n { app.update(); } std::mem::take(&mut app.world_mut().resource_mut::<TvTape>().0) }
    fn tv_settle(mut app: App) -> App { app.update(); app.update(); app.world_mut().resource_mut::<TvTape>().0.clear(); app }
    fn tv_hide(app: &mut App) { let e = app.world_mut().query_filtered::<Entity, With<WarSlabRoot>>().single(app.world()).unwrap(); *app.world_mut().get_mut::<Visibility>(e).unwrap() = Visibility::Hidden; }
    const TV: Visibility = Visibility::Visible;
    const TH: Visibility = Visibility::Hidden;
    /// Peace by default; `charter` walks the embassy bench chain (ridge tab, plant house, hour two held) to a Charter-live hour.
    fn tv_app(charter: bool) -> App {
        let mut hour = HourSacred::default();
        if charter { assert!(crate::hour_sacred::try_ridge_tab(&mut hour, true)); let mut vf = shared::vertical_factory::VerticalFactory::default(); assert!(crate::hour_sacred::try_plant_house(&mut hour, &mut vf) && crate::hour_sacred::try_mark_hour_two_held(&mut hour, true, true) && hour.charter_skin_live()); }
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).insert_resource(hour).init_resource::<WarYard>().init_resource::<TvTape>().add_systems(Startup, spawn_war_slab).add_systems(Update, update_war_slab).add_systems(Last, tv_tape);
        tv_settle(app)
    }
    fn tv_sash(app: &mut App, open: bool) { app.world_mut().resource_mut::<WarYard>().sash_open = open; }
    #[test] fn toast_vis_neq_1_tv9_war_peace_steady_hidden_unmarked() { let mut app = tv_app(false); assert_eq!(tv_run(&mut app, 3), vec![(TH, false); 3], "TV9 war Peace steady hidden"); }
    #[test] fn toast_vis_neq_1_tv10_war_sash_open_close_once_each() {
        let mut app = tv_app(true); tv_sash(&mut app, true);
        assert_eq!(tv_run(&mut app, 2), vec![(TV, true), (TV, false)], "TV10 war sash open: Visible once, then quiet");
        tv_sash(&mut app, false);
        assert_eq!(tv_run(&mut app, 2), vec![(TH, true), (TH, false)], "TV10 war sash closed: Hidden once, then quiet");
    }
    #[test] fn toast_vis_neq_1_tv11_war_direct_hidden_restored_once() { let mut app = tv_app(true); tv_sash(&mut app, true); tv_run(&mut app, 2); tv_hide(&mut app); assert_eq!(tv_run(&mut app, 2), vec![(TV, true), (TV, false)], "TV11 war direct Hidden write: restored once, then quiet"); }
}
