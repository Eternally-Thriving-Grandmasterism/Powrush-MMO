//! Lived-hour climate visible — Slice 16 (v23.2.23) + P2 week feel
//!
//! Node states paint the three wells. Tick restores a tired field.
//! Teaching claim (23.2.25) may replace the hand hint with one sentence.
//! Soft border breath when the week audit line answers (not a second HUD).
//! Does not replace harvest_feel. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::climate_node::NodeState;
use shared::heartwood_wards::WARDS_NOTICE;

use crate::climate_script::TeachingClaim;
use crate::first_session_guidance::FirstSessionGuidance;
use crate::heartwood_lip::ThresholdShelfSession;
use crate::hex_travel::HexTravelState;
use shared::hex_travel::PlaceId;
use crate::heartwood_wards::WardSession;
use crate::lived_hour_bind::LivedHourBind;
use crate::mercy_harvest_nodes::{MercyHarvestNode, NearbyMercyNode};
use crate::depths_landing::DepthsPeaceTend;

#[derive(Component)]
struct ClimateStateRoot;
#[derive(Component)]
struct ClimateStateText;

/// Soft pulse when week tons/restored answers on the existing slab (P2).
#[derive(Resource, Default)]
struct WeekFeelGlow {
    glow: f32,
    last_updated: u64,
}

#[derive(Resource, Default)]
struct WardsNoticeGlow {
    glow: f32,
    last_near: bool,
    last_tends: u32,
}

pub struct ClimateVisiblePlugin;

impl Plugin for ClimateVisiblePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeekFeelGlow>()
            .init_resource::<WardsNoticeGlow>()
            .add_systems(Startup, spawn_climate_state_slab)
            .add_systems(PreUpdate, focus_lived_hour_on_nearby)
            .add_systems(
                Update,
                (
                    paint_nodes_from_hour,
                    tick_week_feel_glow,
                    tick_wards_notice_glow,
                    update_climate_state_slab,
                ),
            );
    }
}

fn focus_lived_hour_on_nearby(
    nearby: Res<NearbyMercyNode>,
    nodes: Query<&MercyHarvestNode>,
    mut bind: ResMut<LivedHourBind>,
) {
    let id = nearby
        .entity
        .and_then(|e| nodes.get(e).ok())
        .map(|n| n.climate_id);
    if bind.focus_id != id {
        bind.focus_id = id;
    }
}

fn paint_nodes_from_hour(
    bind: Res<LivedHourBind>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    nodes: Query<(
        &MercyHarvestNode,
        &Handle<StandardMaterial>,
        Option<&Children>,
    )>,
    mut lights: Query<&mut PointLight>,
) {
    for (node, handle, children) in &nodes {
        let state = bind
            .hour
            .nodes
            .iter()
            .find(|n| n.id == node.climate_id)
            .map(|n| n.state)
            .unwrap_or(NodeState::Idle);
        let mul = state.glow_mul();
        if let Some(mat) = materials.get_mut(handle) {
            mat.emissive = LinearRgba::from(mat.base_color) * (2.4 * mul);
        }
        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(mut light) = lights.get_mut(*child) {
                    light.intensity = 80.0 + 420.0 * mul;
                    light.range = 3.0 + 4.0 * mul;
                }
            }
        }
    }
}

fn tick_week_feel_glow(bind: Res<LivedHourBind>, time: Res<Time>, mut glow: ResMut<WeekFeelGlow>) {
    if bind.week.updated_at != glow.last_updated {
        glow.last_updated = bind.week.updated_at;
        if bind.week.tons_moved > 0 || bind.week.restored_count > 0 {
            glow.glow = 1.0;
        }
    }
    if glow.glow > 0.0 {
        glow.glow = (glow.glow - time.delta_seconds() * 0.55).max(0.0);
    }
}

fn tick_wards_notice_glow(
    wards: Option<Res<WardSession>>,
    time: Res<Time>,
    mut glow: ResMut<WardsNoticeGlow>,
) {
    let (near, tends) = wards
        .as_deref()
        .map(|session| (session.near, session.dress.tends))
        .unwrap_or((false, 0));
    if (near && !glow.last_near) || tends != glow.last_tends {
        glow.glow = 1.0;
    }
    glow.last_near = near;
    glow.last_tends = tends;
    if glow.glow > 0.0 {
        glow.glow = (glow.glow - time.delta_seconds() * 0.55).max(0.0);
    }
}

fn spawn_climate_state_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(176.0),
                    left: Val::Px(16.0),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.06, 0.08, 0.07, 0.88).into(),
                border_color: Color::srgba(0.48, 0.78, 0.58, 0.42).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            ClimateStateRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.84, 0.96, 0.86),
                        ..default()
                    },
                ),
                ClimateStateText,
            ));
        });
}

fn update_climate_state_slab(
    nearby: Res<NearbyMercyNode>,
    bind: Res<LivedHourBind>,
    travel: Res<HexTravelState>,
    threshold: Option<Res<ThresholdShelfSession>>,
    wards: Option<Res<WardSession>>,
    depths: Option<Res<DepthsPeaceTend>>,
    claim: Option<Res<TeachingClaim>>,
    guidance: Option<Res<FirstSessionGuidance>>,
    week_glow: Res<WeekFeelGlow>,
    wards_glow: Res<WardsNoticeGlow>,
    nodes: Query<&MercyHarvestNode>,
    mut root: Query<
        (&mut Visibility, &mut BorderColor, &mut BackgroundColor),
        With<ClimateStateRoot>,
    >,
    mut text_q: Query<&mut Text, With<ClimateStateText>>,
) {
    let threshold_line = threshold_speech_if_near(threshold.as_deref());
    let wards_line = wards_notice_if_near(wards.as_deref());
    let depths_line = depths_restore_line(depths.as_deref());
    let guidance_hidden = climate_caption_guidance_hidden(
        bind.guidance_hidden,
        guidance.as_ref().map(|g| g.dismissed).unwrap_or(false),
    );
    let place = place_clarity_label(travel.current, threshold_line.is_some());
    // H-2026-09-10-1: place + well mood stay on this one slab even when H hid the card.
    let show = climate_slab_should_show(
        nearby.in_range,
        threshold_line.is_some(),
        wards_line.is_some(),
        depths_line.is_some(),
        guidance_hidden,
    );
    let glow = week_glow.glow.max(if wards_line.is_some() {
        wards_glow.glow
    } else {
        0.0
    });
    let week_live = bind
        .climate_slab
        .as_deref()
        .map(|s| s.starts_with("this week"))
        .unwrap_or(false);
    for (mut vis, mut border, mut bg) in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        if show {
            let pulse = if week_live {
                0.12 + glow * 0.38
            } else {
                glow * 0.25
            };
            let a = 0.42 + pulse;
            *border = Color::srgba(
                0.48 + glow * 0.10,
                0.78 + glow * 0.14,
                0.58 + glow * 0.08,
                a,
            )
            .into();
            *bg = Color::srgba(
                0.06 + glow * 0.06,
                0.08 + glow * 0.08,
                0.07 + glow * 0.05,
                0.88,
            )
            .into();
        }
    }
    if !show {
        return;
    }
    let well_line = nearby
        .in_range
        .then(|| {
            nearby.entity.and_then(|e| nodes.get(e).ok()).map(|n| {
                let state = bind
                    .hour
                    .nodes
                    .iter()
                    .find(|c| c.id == n.climate_id)
                    .map(|c| c.state)
                    .unwrap_or(NodeState::Idle);
                if guidance_hidden {
                    return well_state_sentence(n.name, state);
                }
                let hint = claim
                    .as_ref()
                    .and_then(|c| c.sentence_for(n.climate_id))
                    .unwrap_or(state.hand_hint());
                let mut line = format!("{} · {} · {}", n.name, well_state_caption(state), hint);
                if let Some(slab) = bind.climate_slab.as_deref() {
                    line = format!("{line} · {slab}");
                }
                line
            })
        })
        .flatten();
    let fallback = match bind.climate_slab.as_deref() {
        Some(slab) if !bind.last_line.is_empty() => {
            format!("{} · {}", bind.last_line, slab)
        }
        Some(slab) => slab.to_string(),
        None => bind.last_line.clone(),
    };
    // P2: when a well and Wards posts overlap, keep both on one slab — well first.
    // P3: after a Depths Peace restore, show that line on this same slab.
    let body = compose_climate_slab_line(
        well_line,
        wards_line,
        depths_line,
        threshold_line,
        fallback,
    );
    let line = place_clarity_line(place, body);
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.clone();
            }
        }
    }
}

fn climate_caption_guidance_hidden(bind_hidden: bool, session_dismissed: bool) -> bool {
    bind_hidden || session_dismissed
}

fn climate_slab_should_show(
    well_in_range: bool,
    threshold_near: bool,
    wards_near: bool,
    depths_restore: bool,
    guidance_hidden: bool,
) -> bool {
    let _ = (well_in_range, threshold_near, wards_near, depths_restore, guidance_hidden);
    // H-2026-09-10-1: one climate slab always carries the place name; well mood
    // and near-speech still compose on top. H hides the guidance card, not this line.
    true
}

/// Hour place name on the one climate slab (four rooms, not a second HUD).
fn place_clarity_label(current: PlaceId, threshold_near: bool) -> &'static str {
    match current {
        PlaceId::Sanctuary => "Sanctuary",
        PlaceId::Depths => "Depths",
        PlaceId::Heartwood if threshold_near => "Threshold",
        PlaceId::Heartwood => "Heartwood",
    }
}

fn place_clarity_line(place: &str, body: String) -> String {
    let body = body.trim();
    if body.is_empty() {
        place.to_string()
    } else if body.starts_with(place) {
        body.to_string()
    } else {
        format!("{place} · {body}")
    }
}

fn well_state_caption(state: NodeState) -> String {
    format!("{} · {}", state.label(), well_state_token(state))
}

fn well_state_token(state: NodeState) -> &'static str {
    match state {
        NodeState::Idle => "•",
        NodeState::Glowing => "○",
        NodeState::Tended => "✓",
        NodeState::Resting => "—",
        NodeState::Stressed => "!",
    }
}

fn well_state_sentence(name: &str, state: NodeState) -> String {
    format!("{name} is {}", well_state_caption(state))
}


fn depths_restore_line(depths: Option<&DepthsPeaceTend>) -> Option<String> {
    depths
        .map(|session| session.last_line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
}

fn compose_climate_slab_line(
    well: Option<String>,
    wards: Option<String>,
    depths: Option<String>,
    threshold: Option<String>,
    fallback: String,
) -> String {
    match (well, wards) {
        (Some(well), Some(wards)) => format!("{well} · {wards}"),
        (Some(well), None) => well,
        (None, Some(wards)) => wards,
        (None, None) => depths.or(threshold).unwrap_or(fallback),
    }
}

fn threshold_speech_if_near(threshold: Option<&ThresholdShelfSession>) -> Option<String> {
    threshold
        .filter(|session| session.near)
        .map(|session| session.node.speech())
}

fn wards_notice_if_near(wards: Option<&WardSession>) -> Option<String> {
    wards
        .filter(|session| session.near)
        .map(|_| WARDS_NOTICE.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::climate_node::LivedHour;
    use shared::threshold_shelf::THRESHOLD_PEACE_VERBS;

    #[test]
    fn demo_nodes_map_to_three_wells() {
        let hour = LivedHour::new_demo();
        assert_eq!(hour.nodes.len(), 3);
        assert_eq!(hour.nodes[0].id, 1);
        assert_eq!(hour.nodes[1].id, 2);
        assert_eq!(hour.nodes[2].id, 3);
        assert_eq!(hour.nodes[2].state, NodeState::Idle);
    }

    #[test]
    fn stressed_is_dimmer_than_glow() {
        assert!(NodeState::Glowing.glow_mul() > NodeState::Tended.glow_mul());
        assert!(NodeState::Tended.glow_mul() > NodeState::Resting.glow_mul());
        assert!(NodeState::Resting.glow_mul() > NodeState::Stressed.glow_mul());
        assert!(NodeState::Stressed.glow_mul() > 0.0);
    }

    #[test]
    fn threshold_uses_the_existing_speech_slab() {
        let mut session = ThresholdShelfSession::default();
        assert_eq!(threshold_speech_if_near(Some(&session)), None);
        session.near = true;
        let line = threshold_speech_if_near(Some(&session)).expect("Threshold speech");
        assert!(THRESHOLD_PEACE_VERBS.iter().all(|verb| line.contains(verb)));
    }

    #[test]
    fn each_well_mood_keeps_its_word_and_distinct_shape() {
        let moods = [
            (NodeState::Idle, "Idle", "•"),
            (NodeState::Glowing, "Glowing", "○"),
            (NodeState::Tended, "Tended", "✓"),
            (NodeState::Resting, "Resting", "—"),
            (NodeState::Stressed, "Stressed", "!"),
        ];
        for (state, label, token) in moods {
            assert_eq!(well_state_caption(state), format!("{label} · {token}"));
        }

        let mut tokens: Vec<_> = moods
            .iter()
            .map(|(state, _, _)| well_state_token(*state))
            .collect();
        tokens.sort_unstable();
        tokens.dedup();
        assert_eq!(tokens.len(), moods.len());
    }

    #[test]
    fn hidden_guidance_keeps_each_well_state_sentence_visible() {
        for (state, label, token) in [
            (NodeState::Idle, "Idle", "•"),
            (NodeState::Glowing, "Glowing", "○"),
            (NodeState::Tended, "Tended", "✓"),
            (NodeState::Resting, "Resting", "—"),
            (NodeState::Stressed, "Stressed", "!"),
        ] {
            assert!(climate_slab_should_show(true, false, false, false, true));
            assert_eq!(
                well_state_sentence("North Well", state),
                format!("North Well is {label} · {token}")
            );
        }
        assert_eq!(
            well_state_sentence("Sanctuary ember", NodeState::Glowing),
            "Sanctuary ember is Glowing · ○"
        );
    }

    #[test]
    fn hidden_guidance_keeps_threshold_place_readable() {
        // H-2026-09-10-1: Threshold room name/speech survives H on this slab.
        assert!(climate_slab_should_show(false, true, false, false, true));
        assert!(climate_slab_should_show(false, true, false, false, false));
    }

    #[test]
    fn place_clarity_prefixes_well_mood_when_h_hid() {
        let line = place_clarity_line(
            "Sanctuary",
            well_state_sentence("North Well", NodeState::Idle),
        );
        assert_eq!(line, "Sanctuary · North Well is Idle · •");
        assert!(line.contains("Idle"));
        let heart = place_clarity_label(PlaceId::Heartwood, false);
        assert_eq!(heart, "Heartwood");
        assert_eq!(place_clarity_label(PlaceId::Heartwood, true), "Threshold");
        assert_eq!(place_clarity_label(PlaceId::Depths, false), "Depths");
    }

    #[test]
    fn dismissed_session_guidance_hides_climate_hints() {
        assert!(climate_caption_guidance_hidden(false, true));
        assert!(climate_caption_guidance_hidden(true, false));
        assert!(!climate_caption_guidance_hidden(false, false));
    }

    #[test]
    fn wards_use_the_existing_slab_even_when_guidance_is_hidden() {
        let mut wards = WardSession::default();
        wards.near = true;
        assert!(climate_slab_should_show(false, false, true, false, true));
        assert_eq!(
            wards_notice_if_near(Some(&wards)).as_deref(),
            Some(WARDS_NOTICE)
        );
    }
    #[test]
    fn well_stays_ahead_of_wards_when_both_are_near() {
        let well = well_state_sentence("Sanctuary ember", NodeState::Glowing);
        let line = compose_climate_slab_line(
            Some(well.clone()),
            Some(WARDS_NOTICE.to_string()),
            None,
            None,
            "fallback".into(),
        );
        assert!(line.starts_with(&well), "well sentence must remain first");
        assert!(line.contains(WARDS_NOTICE), "Wards notice stays on the same slab");
        assert_eq!(line, format!("{well} · {WARDS_NOTICE}"));
        assert_eq!(
            compose_climate_slab_line(Some(well.clone()), None, None, None, "fallback".into()),
            well
        );
        assert_eq!(
            compose_climate_slab_line(None, Some(WARDS_NOTICE.to_string()), None, None, "fallback".into()),
            WARDS_NOTICE
        );
    }

    #[test]
    fn depths_restore_line_shows_on_existing_slab() {
        let mut tend = DepthsPeaceTend::default();
        assert_eq!(depths_restore_line(Some(&tend)), None);
        // Place clarity: slab stays up with the place name even with no near speech.
        assert!(climate_slab_should_show(false, false, false, false, true));
        tend.last_line = "Depths Peace · restored".into();
        assert_eq!(
            depths_restore_line(Some(&tend)).as_deref(),
            Some("Depths Peace · restored")
        );
        assert!(climate_slab_should_show(false, false, false, true, true));
        assert_eq!(
            compose_climate_slab_line(
                None,
                None,
                Some("Depths Peace · restored".into()),
                None,
                "fallback".into(),
            ),
            "Depths Peace · restored"
        );
    }

}
