/*!
 * Mercy Ascent Trial UI
 *
 * Sacred interface for tracking Ascension Progress and the live Mercy Ascent Trial.
 * Deeply aligned with TOLC 8 Mercy Gates philosophy.
 * Features pillar progress (wired to real AscensionProgress), phase tracking,
 * dynamic Mercy Score with color feedback, **dynamic phase objectives**, and group resonance support.
 *
 * PATSAGi Council + Ra-Thor Quantum Swarm approved • AG-SML v1.0
 */

use bevy::prelude::*;
use crate::ascension::components::*;
use crate::ascension::events::*;

/// Marker component for the main Mercy Ascent Trial UI root
#[derive(Component)]
pub struct MercyAscentTrialUI;

/// Marker for the dynamic phase indicator text
#[derive(Component)]
pub struct PhaseIndicator;

/// Marker for the Mercy Score bar fill node
#[derive(Component)]
pub struct MercyScoreFill;

/// Marker for the Mercy Score percentage text
#[derive(Component)]
pub struct MercyScoreText;

/// Marker for pillar progress bars (Council, Epiphany, Abundance, Resonance)
#[derive(Component, Clone, Copy)]
pub enum PillarType {
    Council,
    Epiphany,
    Abundance,
    Resonance,
}

#[derive(Component)]
pub struct PillarProgressFill {
    pub pillar: PillarType,
}

/// Marker for objectives list container
#[derive(Component)]
pub struct ObjectivesList;

/// Marker for a single objective line
#[derive(Component)]
pub struct ObjectiveLine;

/// Placeholder component for the phase container (used for visibility toggle)
#[derive(Component)]
pub struct PhaseContainer;

/// Returns sacred, phase-specific objectives that guide the player through the Mercy Ascent Trial.
/// These are not just tasks — they are reminders of the TOLC 8 Mercy Gates in action.
fn get_objectives_for_phase(phase: TrialPhase) -> Vec<&'static str> {
    match phase {
        TrialPhase::Reckoning => vec![
            "Resolve Echoes through resonance or mercy decisions",
            "Avoid killing Echoes without first attempting resolution",
            "Maintain Mercy Score above 0.65 throughout the phase",
        ],
        TrialPhase::Alignment => vec![
            "Complete challenges aligned with your chosen Ascension Path",
            "Make mercy-gated decisions under pressure and temptation",
            "Keep Mercy Score above 0.75 — harmony over domination",
        ],
        TrialPhase::Bloom => vec![
            "Face the Unascended Self and choose healing over destruction",
            "Prioritize resonance and mercy actions on the final manifestation",
            "Achieve a final Mercy Score above 0.80 to ascend",
        ],
    }
}

/// Spawns the beautiful Mercy Ascent Trial UI (called from AscensionUiPlugin)
pub fn spawn_mercy_ascent_trial_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Root container - positioned elegantly on the right side
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(60.0),
                    right: Val::Px(40.0),
                    width: Val::Px(380.0),
                    padding: UiRect::all(Val::Px(18.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Stretch,
                    row_gap: Val::Px(12.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.06, 0.02, 0.12, 0.94)),
                border_color: BorderColor(Color::srgb(0.85, 0.65, 0.95)),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            MercyAscentTrialUI,
            Name::new("Mercy Ascent Trial UI"),
        ))
        .with_children(|parent| {
            // === HEADER ===
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(10.0),
                        margin: UiRect::bottom(Val::Px(8.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|header| {
                    // Sacred symbol (text-based for now; replace with icon later)
                    header.spawn(TextBundle {
                        text: Text::from_section(
                            "✧",
                            TextStyle {
                                font_size: 32.0,
                                color: Color::srgb(0.95, 0.85, 0.6),
                                ..default()
                            },
                        ),
                        ..default()
                    });

                    header.spawn(TextBundle {
                        text: Text::from_section(
                            "THE MERCY ASCENT",
                            TextStyle {
                                font_size: 22.0,
                                color: Color::srgb(0.95, 0.88, 0.75),
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });

            // === ASCENSION PATH (static for now; can be wired to AscensionPath component later) ===
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Path: Hybrid • Progress: 68% (toward Ascension)",
                        TextStyle {
                            font_size: 14.0,
                            color: Color::srgb(0.7, 0.85, 0.95),
                            ..default()
                        },
                    ),
                    ..default()
                },
                Name::new("Ascension Path Display"),
            ));

            // === PILLAR PROGRESS SECTION ===
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "PILLARS OF ASCENSION",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.6, 0.75, 0.9),
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::top(Val::Px(4.0)).bottom(Val::Px(4.0)),
                    ..default()
                },
                ..default()
            });

            // Pillar rows - initial values will be overridden by update system from real AscensionProgress
            let pillars = [
                (PillarType::Council, "Council Participation"),
                (PillarType::Epiphany, "Epiphany History"),
                (PillarType::Abundance, "Abundance Contribution"),
                (PillarType::Resonance, "Resonance Attunement"),
            ];

            for (pillar, label) in pillars {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(2.0),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|row| {
                        // Label
                        row.spawn(TextBundle {
                            text: Text::from_section(
                                label,
                                TextStyle {
                                    font_size: 12.0,
                                    color: Color::srgb(0.85, 0.85, 0.9),
                                    ..default()
                                },
                            ),
                            ..default()
                        });

                        // Progress bar background
                        row.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(14.0),
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::srgba(0.15, 0.1, 0.25, 0.8)),
                                border_color: BorderColor(Color::srgb(0.4, 0.35, 0.55)),
                                border_radius: BorderRadius::all(Val::Px(3.0)),
                                ..default()
                            },
                        ))
                        .with_children(|bar| {
                            // Fill (dynamic width + color updated live by system from AscensionProgress)
                            bar.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(0.0), // Will be set by update_mercy_ascent_trial_ui
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(match pillar {
                                        PillarType::Council => Color::srgb(0.4, 0.85, 0.6),
                                        PillarType::Epiphany => Color::srgb(0.6, 0.7, 0.95),
                                        PillarType::Abundance => Color::srgb(0.95, 0.85, 0.4),
                                        PillarType::Resonance => Color::srgb(0.85, 0.55, 0.95),
                                    }),
                                    border_radius: BorderRadius::all(Val::Px(3.0)),
                                    ..default()
                                },
                                PillarProgressFill { pillar },
                            ));
                        });
                    });
            }

            // === CURRENT PHASE (only visible during trial) ===
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(6.0),
                        margin: UiRect::top(Val::Px(10.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.12, 0.06, 0.18, 0.85)),
                    border_color: BorderColor(Color::srgb(0.7, 0.6, 0.9)),
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                PhaseContainer,
            ))
            .with_children(|phase_section| {
                phase_section.spawn((
                    TextBundle {
                        text: Text::from_section(
                            "PHASE: THE RECKONING",
                            TextStyle {
                                font_size: 16.0,
                                color: Color::srgb(0.95, 0.75, 0.6),
                                ..default()
                            },
                        ),
                        ..default()
                    },
                    PhaseIndicator,
                ));

                phase_section.spawn(TextBundle {
                    text: Text::from_section(
                        "Confront your past actions and the server’s shadow.",
                        TextStyle {
                            font_size: 11.0,
                            color: Color::srgb(0.75, 0.75, 0.8),
                            ..default()
                        },
                    ),
                    ..default()
                });

                // Mercy Score
                phase_section
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(8.0),
                            margin: UiRect::top(Val::Px(6.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|score_row| {
                        score_row.spawn(TextBundle {
                            text: Text::from_section(
                                "Mercy Score:",
                                TextStyle {
                                    font_size: 12.0,
                                    color: Color::srgb(0.85, 0.85, 0.9),
                                    ..default()
                                },
                            ),
                            ..default()
                        });

                        score_row
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(160.0),
                                        height: Val::Px(16.0),
                                        border: UiRect::all(Val::Px(1.0)),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::srgba(0.2, 0.15, 0.3, 0.9)),
                                    border_color: BorderColor(Color::srgb(0.5, 0.45, 0.6)),
                                    border_radius: BorderRadius::all(Val::Px(4.0)),
                                    ..default()
                                },
                            ))
                            .with_children(|bar| {
                                bar.spawn((
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        background_color: BackgroundColor(Color::srgb(0.3, 0.85, 0.5)),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        ..default()
                                    },
                                    MercyScoreFill,
                                ));
                            });

                        score_row.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    "92%",
                                    TextStyle {
                                        font_size: 13.0,
                                        color: Color::srgb(0.6, 0.95, 0.7),
                                        ..default()
                                    },
                                ),
                                ..default()
                            },
                            MercyScoreText,
                        ));
                    });
            });

            // === CURRENT OBJECTIVES (header) ===
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "CURRENT OBJECTIVES",
                        TextStyle {
                            font_size: 12.0,
                            color: Color::srgb(0.65, 0.8, 0.95),
                            ..default()
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Objectives Header"),
            ));

            // Dynamic objectives container (populated by rebuild_objectives_system)
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(4.0),
                        margin: UiRect::left(Val::Px(8.0)),
                        ..default()
                    },
                    ..default()
                },
                ObjectivesList,
            ));

            // === GROUP STATUS (placeholder ready for real party query) ===
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Group Resonance: 3/4 members aligned",
                        TextStyle {
                            font_size: 11.0,
                            color: Color::srgb(0.6, 0.85, 0.75),
                            ..default()
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Group Status"),
            ));

            // === FOOTER ===
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "“Ascension is not given. It is remembered.” — TOLC 8",
                    TextStyle {
                        font_size: 10.0,
                        color: Color::srgb(0.55, 0.6, 0.7),
                        font_style: FontStyle::Italic,
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::top(Val::Px(12.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                ..default()
            });
        });
}

/// Updates the Mercy Ascent Trial UI based on real player state.
/// Fully wired to AscensionProgress for live pillar progress + dynamic phase objectives.
pub fn update_mercy_ascent_trial_ui(
    mut ui_query: Query<&mut Visibility, With<PhaseContainer>>,
    trial_query: Query<&InMercyAscentTrial>,
    progress_query: Query<&AscensionProgress>,
    mercy_query: Query<&MercyAlignment>,
    mut phase_text: Query<&mut Text, With<PhaseIndicator>>,
    mut score_fill: Query<&mut Style, With<MercyScoreFill>>,
    mut score_text: Query<&mut Text, With<MercyScoreText>>,
    mut pillar_fills: Query<(&mut Style, &PillarProgressFill)>,
) {
    let in_trial = trial_query.get_single().ok();

    // Show/hide phase section based on whether player is currently in a Mercy Ascent Trial
    for mut vis in ui_query.iter_mut() {
        *vis = if in_trial.is_some() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    // === REAL ASCENSION PROGRESS WIRING ===
    if let Ok(progress) = progress_query.get_single() {
        for (mut style, fill) in pillar_fills.iter_mut() {
            let percent = match fill.pillar {
                PillarType::Council => {
                    let p = (progress.council_participations as f32 / 30.0
                        + progress.successful_council_blooms as f32 / 10.0)
                        / 2.0;
                    (p.clamp(0.0, 1.0) * 100.0)
                }
                PillarType::Epiphany => {
                    let p = (progress.total_epiphanies as f32 / 75.0) * 0.7
                        + progress.average_epiphany_intensity.clamp(0.0, 1.0) * 0.3;
                    (p.clamp(0.0, 1.0) * 100.0)
                }
                PillarType::Abundance => {
                    let p = (progress.total_abundance_contributed as f32 / 50_000.0).min(1.0);
                    (p * 100.0)
                }
                PillarType::Resonance => {
                    (progress.resonance_attunement.clamp(0.0, 1.0) * 100.0)
                }
            };
            style.width = Val::Percent(percent);
        }
    }

    if let Some(trial) = in_trial {
        // Update Phase Indicator text and color
        if let Ok(mut text) = phase_text.get_single_mut() {
            let phase_name = match trial.phase {
                TrialPhase::Reckoning => "THE RECKONING",
                TrialPhase::Alignment => "THE ALIGNMENT",
                TrialPhase::Bloom => "THE BLOOM",
            };
            text.sections[0].value = format!("PHASE: {}", phase_name);
            text.sections[0].style.color = match trial.phase {
                TrialPhase::Reckoning => Color::srgb(0.95, 0.65, 0.55),
                TrialPhase::Alignment => Color::srgb(0.7, 0.85, 0.95),
                TrialPhase::Bloom => Color::srgb(0.85, 0.75, 0.95),
            };
        }

        // Update Mercy Score bar and text (with smart color feedback)
        let mercy_score = mercy_query.get_single().map(|m| m.score).unwrap_or(trial.mercy_score);
        let percent = (mercy_score * 100.0).clamp(0.0, 100.0);

        if let Ok(mut style) = score_fill.get_single_mut() {
            style.width = Val::Percent(percent);
        }

        if let Ok(mut text) = score_text.get_single_mut() {
            text.sections[0].value = format!("{:.0}%", percent);
            text.sections[0].style.color = if percent > 70.0 {
                Color::srgb(0.5, 0.95, 0.6)
            } else {
                Color::srgb(0.95, 0.6, 0.5)
            };
        }
    }
}

/// Rebuilds the dynamic objectives list based on the current TrialPhase.
/// This system runs every frame while in trial — lightweight and sacred.
pub fn rebuild_objectives_system(
    mut commands: Commands,
    trial_query: Query<&InMercyAscentTrial>,
    list_query: Query<Entity, With<ObjectivesList>>,
    existing_objectives: Query<Entity, With<ObjectiveLine>>,
) {
    let in_trial = trial_query.get_single().ok();

    for list_entity in list_query.iter() {
        // Despawn previous objective lines (safe per-frame for small UI)
        for obj_entity in existing_objectives.iter() {
            commands.entity(obj_entity).despawn_recursive();
        }

        if let Some(trial) = in_trial {
            let objectives = get_objectives_for_phase(trial.phase);

            for obj_text in objectives {
                commands.entity(list_entity).with_children(|list| {
                    list.spawn((
                        TextBundle {
                            text: Text::from_section(
                                format!("• {}", obj_text),
                                TextStyle {
                                    font_size: 11.0,
                                    color: Color::srgb(0.8, 0.82, 0.88),
                                    ..default()
                                },
                            ),
                            style: Style {
                                margin: UiRect::left(Val::Px(8.0)),
                                ..default()
                            },
                            ..default()
                        },
                        ObjectiveLine,
                    ));
                });
            }
        }
    }
}
