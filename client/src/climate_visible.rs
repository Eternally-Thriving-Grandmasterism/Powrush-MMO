//! Lived-hour climate visible — Slice 16 (v23.2.23)
//!
//! Node states paint the three wells. Tick restores a tired field.
//! Does not replace harvest_feel. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::climate_node::NodeState;

use crate::lived_hour_bind::LivedHourBind;
use crate::mercy_harvest_nodes::{MercyHarvestNode, NearbyMercyNode};

#[derive(Component)]
struct ClimateStateRoot;
#[derive(Component)]
struct ClimateStateText;

pub struct ClimateVisiblePlugin;

impl Plugin for ClimateVisiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_climate_state_slab)
            .add_systems(
                PreUpdate,
                focus_lived_hour_on_nearby,
            )
            .add_systems(
                Update,
                (
                    paint_nodes_from_hour,
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
    nodes: Query<(&MercyHarvestNode, &Handle<StandardMaterial>, Option<&Children>)>,
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
    nodes: Query<&MercyHarvestNode>,
    mut root: Query<&mut Visibility, With<ClimateStateRoot>>,
    mut text_q: Query<&mut Text, With<ClimateStateText>>,
) {
    let show = nearby.in_range;
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
    let line = nearby
        .entity
        .and_then(|e| nodes.get(e).ok())
        .map(|n| {
            let state = bind
                .hour
                .nodes
                .iter()
                .find(|c| c.id == n.climate_id)
                .map(|c| c.state)
                .unwrap_or(NodeState::Idle);
            format!("{} · {} · {}", n.name, state.label(), state.hand_hint())
        })
        .unwrap_or_else(|| bind.last_line.clone());
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
    use shared::climate_node::LivedHour;

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
}
