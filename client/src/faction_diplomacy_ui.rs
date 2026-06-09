// client/src/faction_diplomacy_ui.rs
// Powrush-MMO v17.32 — Faction Diplomacy UI
// Production quality • Mercy-gated • Beautiful abundance-themed panel
// Exact visual language match with onboarding_ui.rs, settings_menu.rs, quest_log.rs
// Toggle via Pause Menu button or hotkey 'D' (World Council)
// Live-updating standings, color-coded mercy/abundance bars, action buttons with feedback
// Foundation ready for full networking replication + server event integration

use bevy::prelude::*;
use std::collections::HashMap;

use crate::faction_diplomacy::{Faction, DiplomacyStatus};

#[derive(Resource, Clone, Debug, Default)]
pub struct ClientFactionDiplomacy {
    pub standings: HashMap<Faction, f32>,      // -100.0 to +100.0, replicated from server
    pub relations: HashMap<(Faction, Faction), DiplomacyStatus>,
    pub last_proposal_time: Option<u64>,
}

#[derive(Component)]
pub struct FactionDiplomacyPanel;

#[derive(Component)]
pub struct FactionCard {
    pub faction: Faction,
}

#[derive(Component)]
pub struct StandingBar;

#[derive(Component)]
pub struct DiplomacyActionButton {
    pub action: DiplomacyAction,
    pub target_faction: Faction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiplomacyAction {
    ProposeTreaty,
    DeclareRivalry,
    // Future: AcceptTreaty, BreakAlliance, RequestAid, etc.
}

pub struct FactionDiplomacyUIPlugin;

impl Plugin for FactionDiplomacyUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ClientFactionDiplomacy>()
            .add_systems(Startup, setup_faction_diplomacy_ui)
            .add_systems(Update, (
                toggle_diplomacy_panel,
                update_faction_standings,
                handle_diplomacy_buttons,
            ));
    }
}

fn setup_faction_diplomacy_ui(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(720.0),
                height: Val::Px(540.0),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            background_color: Color::srgba(0.04, 0.06, 0.10, 0.97).into(), // Deep cosmic with mercy veil
            border_color: Color::srgb(0.25, 0.65, 0.45).into(), // Abundance green
            border: UiRect::all(Val::Px(4.0)),
            box_shadow: vec![ShadowStyle { // Subtle divine glow
                color: Color::srgba(0.2, 0.7, 0.5, 0.3),
                offset: Vec2::new(0.0, 8.0),
                blur_radius: 24.0,
            }],
            visibility: Visibility::Hidden,
            ..default()
        },
        FactionDiplomacyPanel,
        Name::new("FactionDiplomacyPanel"),
    )).with_children(|parent| {
        // Sacred Header
        parent.spawn(TextBundle {
            text: Text::from_section(
                "✧ FACTION DIPLOMACY COUNCIL ✧",
                TextStyle {
                    font_size: 26.0,
                    color: Color::srgb(0.92, 0.96, 0.82), // Warm abundance gold
                    ..default()
                },
            ),
            style: Style { margin: UiRect::bottom(Val::Px(4.0)), ..default() },
            ..default()
        });

        parent.spawn(TextBundle {
            text: Text::from_section(
                "Aligned with the Eternal Flow • Mercy-Guided Relations • RBE Harmony",
                TextStyle {
                    font_size: 13.0,
                    color: Color::srgb(0.55, 0.78, 0.68),
                    ..default()
                },
            ),
            style: Style { margin: UiRect::bottom(Val::Px(18.0)), ..default() },
            ..default()
        });

        // Cards row
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(16.0),
                row_gap: Val::Px(12.0),
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        }).with_children(|cards| {
            for faction in [Faction::SeedOfAbundance, Faction::FlowGuardians, Faction::EternalWeavers] {
                spawn_faction_card(cards, faction);
            }
        });

        // Divine footer
        parent.spawn(TextBundle {
            text: Text::from_section(
                "⚡ Your sovereign choices shape the living world. Choose with radical mercy. ⚡",
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.65, 0.82, 0.72),
                    ..default()
                },
            ),
            style: Style { margin: UiRect::top(Val::Px(20.0)), ..default() },
            ..default()
        });
    });
}

fn spawn_faction_card(parent: &mut ChildBuilder, faction: Faction) {
    let (name, accent, lore_snippet) = match faction {
        Faction::SeedOfAbundance => ("Seed of Abundance", Color::srgb(0.28, 0.72, 0.42), "Nurturers of growth, prosperity & universal thriving for all sentience."),
        Faction::FlowGuardians => ("Flow Guardians", Color::srgb(0.28, 0.62, 0.88), "Protectors of balance, rivers of mercy, and harmonious natural cycles."),
        Faction::EternalWeavers => ("Eternal Weavers", Color::srgb(0.78, 0.48, 0.88), "Weavers of fate, stories, and the grand tapestry of peaceful coexistence."),
    };

    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(215.0),
                height: Val::Px(295.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(14.0)),
                margin: UiRect::all(Val::Px(6.0)),
                border: UiRect::all(Val::Px(2.5)),
                ..default()
            },
            background_color: Color::srgba(0.06, 0.09, 0.13, 0.92).into(),
            border_color: accent.into(),
            ..default()
        },
        FactionCard { faction },
    )).with_children(|card| {
        // Faction title
        card.spawn(TextBundle {
            text: Text::from_section(
                name,
                TextStyle { font_size: 15.0, color: Color::WHITE, ..default() },
            ),
            style: Style { margin: UiRect::bottom(Val::Px(6.0)), ..default() },
            ..default()
        });

        // Standing bar container
        card.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(92.0),
                height: Val::Px(16.0),
                border: UiRect::all(Val::Px(1.5)),
                margin: UiRect::bottom(Val::Px(4.0)),
                ..default()
            },
            background_color: Color::srgb(0.15, 0.17, 0.22).into(),
            border_color: Color::srgb(0.35, 0.38, 0.42).into(),
            ..default()
        }).with_children(|bar_container| {
            bar_container.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0), // Dynamically updated
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::srgb(0.35, 0.78, 0.48).into(),
                    ..default()
                },
                StandingBar,
            ));
        });

        // Live standing value
        card.spawn((
            TextBundle {
                text: Text::from_section("Standing: 0.0", TextStyle { font_size: 11.0, color: Color::WHITE, ..default() }),
                style: Style { margin: UiRect::bottom(Val::Px(6.0)), ..default() },
                ..default()
            },
        ));

        // Lore
        card.spawn(TextBundle {
            text: Text::from_section(
                lore_snippet,
                TextStyle { font_size: 9.5, color: Color::srgb(0.72, 0.78, 0.82), ..default() },
            ),
            style: Style { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
            ..default()
        });

        // Action buttons
        for &action in &[DiplomacyAction::ProposeTreaty, DiplomacyAction::DeclareRivalry] {
            let (label, btn_color) = match action {
                DiplomacyAction::ProposeTreaty => ("Propose Treaty", Color::srgb(0.18, 0.52, 0.38)),
                DiplomacyAction::DeclareRivalry => ("Declare Rivalry", Color::srgb(0.62, 0.22, 0.22)),
            };
            card.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Percent(88.0),
                        height: Val::Px(30.0),
                        margin: UiRect::top(Val::Px(3.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: btn_color.into(),
                    ..default()
                },
                DiplomacyActionButton { action, target_faction: faction },
            )).with_children(|btn| {
                btn.spawn(TextBundle {
                    text: Text::from_section(label, TextStyle { font_size: 12.0, color: Color::WHITE, ..default() }),
                    ..default()
                });
            });
        }
    });
}

fn toggle_diplomacy_panel(
    keys: Res<Input<KeyCode>>,
    mut panel_q: Query<(&mut Visibility, &mut Style), With<FactionDiplomacyPanel>>,
) {
    if keys.just_pressed(KeyCode::KeyD) {
        for (mut vis, mut style) in panel_q.iter_mut() {
            *vis = if *vis == Visibility::Hidden { 
                Visibility::Visible 
            } else { 
                Visibility::Hidden 
            };
            if *vis == Visibility::Visible {
                style.left = Val::Percent(50.0);
                style.top = Val::Percent(50.0);
            }
        }
    }
}

fn update_faction_standings(
    client: Res<ClientFactionDiplomacy>,
    card_q: Query<(&FactionCard, &Children), With<FactionCard>>,
    mut text_q: Query<&mut Text>,
    mut bar_q: Query<(&mut Style, &mut BackgroundColor), With<StandingBar>>,
) {
    for (card, children) in card_q.iter() {
        let standing = client.standings.get(&card.faction).copied().unwrap_or(0.0);

        for child in children.iter() {
            // Update standing text
            if let Ok(mut text) = text_q.get_mut(*child) {
                if text.sections[0].value.starts_with("Standing:") {
                    text.sections[0].value = format!("Standing: {:.1}", standing);
                    text.sections[0].style.color = if standing > 45.0 {
                        Color::srgb(0.35, 0.88, 0.52)
                    } else if standing < -25.0 {
                        Color::srgb(0.92, 0.38, 0.38)
                    } else {
                        Color::WHITE
                    };
                }
            }

            // Update bar width + color (mercy/abundance spectrum)
            if let Ok((mut style, mut bg)) = bar_q.get_mut(*child) {
                let norm = ((standing + 100.0) / 200.0).clamp(0.0, 1.0);
                style.width = Val::Percent(norm * 100.0);

                *bg = if standing < -25.0 {
                    Color::srgb(0.82, 0.28, 0.28).into()
                } else if standing < 25.0 {
                    Color::srgb(0.82, 0.72, 0.28).into()
                } else {
                    Color::srgb(0.28, 0.78, 0.48).into()
                };
            }
        }
    }
}

fn handle_diplomacy_buttons(
    mut interactions: Query<(&Interaction, &DiplomacyActionButton, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_q: Query<&mut Text>,
    mut client: ResMut<ClientFactionDiplomacy>,
) {
    for (interaction, btn, children) in interactions.iter() {
        if *interaction == Interaction::Pressed {
            match btn.action {
                DiplomacyAction::ProposeTreaty => {
                    // TODO: Send ProposeTreaty event to server (networking layer)
                    // Optimistic UI or await confirmation
                    if let Some(last) = client.last_proposal_time {
                        // cooldown visual hint
                    }
                    println!("⚡ Proposing Treaty with {:?} — Mercy & abundance ripple outward.", btn.target_faction);
                    // In full: client.last_proposal_time = Some(current_unix_time());
                }
                DiplomacyAction::DeclareRivalry => {
                    println!("🔥 Declaring Rivalry with {:?} — Strength in defense of the Flow.", btn.target_faction);
                    // TODO: Server validates via FactionDiplomacyManager + broadcasts via DynamicEvent
                }
            }

            // Press feedback
            for child in children.iter() {
                if let Ok(mut text) = text_q.get_mut(*child) {
                    text.sections[0].style.color = Color::srgb(1.0, 0.95, 0.6);
                }
            }
        } else if *interaction == Interaction::Hovered {
            for child in children.iter() {
                if let Ok(mut text) = text_q.get_mut(*child) {
                    text.sections[0].style.color = Color::srgb(1.0, 1.0, 0.85);
                }
            }
        } else {
            // Reset
            for child in children.iter() {
                if let Ok(mut text) = text_q.get_mut(*child) {
                    text.sections[0].style.color = Color::WHITE;
                }
            }
        }
    }
}