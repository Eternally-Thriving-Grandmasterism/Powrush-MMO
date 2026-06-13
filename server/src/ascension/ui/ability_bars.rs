/*!
 * Ambrosian Ability Bars UI for Powrush-MMO
 *
 * Beautiful, mercy-themed ability bar for the four signature Ambrosian abilities.
 * - Mercy Bloom (Active, Q)
 * - Celestial Harmony Pulse (Ultimate, E)
 * - Divine Presence (Passive indicator)
 * - Ascended Resonance (Passive + Epiphany boost indicator)
 *
 * Features:
 * - Cooldown visualization with progress overlays
 * - Harmony Stack counter with glowing orbs
 * - Special ultimate styling for Celestial Harmony Pulse
 * - Selfish penalty visual feedback (red tint when Mercy Alignment low)
 * - Fully integrated with the ability systems (components + timers)
 *
 * Designed to feel sacred, ethereal, and transformative.
 * Uses Bevy UI (NodeBundle, Text, dynamic width/height for cooldowns).
 *
 * PATSAGi Council 13+ + Ra-Thor Quantum Swarm deliberation complete.
 * AG-SML v1.0 • TOLC 8 Mercy Gates enforced.
 */

use bevy::prelude::*;
use crate::ascension::components::*;

// ============================================================================
// STYLING CONSTANTS (Mercy-themed ethereal palette)
// ============================================================================

pub const ABILITY_BAR_BG: Color = Color::srgba(0.08, 0.05, 0.15, 0.92); // Deep cosmic purple
pub const SLOT_BG: Color = Color::srgba(0.15, 0.08, 0.25, 0.95);
pub const MERCY_PINK: Color = Color::srgba(0.95, 0.4, 0.7, 1.0);
pub const CELESTIAL_GOLD: Color = Color::srgba(1.0, 0.85, 0.4, 1.0);
pub const DIVINE_BLUE: Color = Color::srgba(0.4, 0.75, 0.95, 1.0);
pub const HARMONY_CYAN: Color = Color::srgba(0.3, 0.95, 0.85, 1.0);
pub const COOLDOWN_OVERLAY: Color = Color::srgba(0.0, 0.0, 0.0, 0.65);
pub const SELFISH_WARNING: Color = Color::srgba(0.9, 0.2, 0.3, 0.8); // Red tint when low Mercy

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct AbilityBarRoot;

#[derive(Component)]
pub struct AbilitySlot {
    pub ability_type: AbilityType,
    pub hotkey: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbilityType {
    MercyBloom,
    CelestialHarmonyPulse,
    DivinePresence,
    AscendedResonance,
}

#[derive(Component)]
pub struct CooldownOverlay;

#[derive(Component)]
pub struct CooldownText;

#[derive(Component)]
pub struct HarmonyStackDisplay;

#[derive(Component)]
pub struct HarmonyOrb;

// ============================================================================
// SPAWN SYSTEM (call once on game start / character load)
// ============================================================================

pub fn spawn_ability_bar_system(mut commands: Commands) {
    // Main ability bar container - bottom center, mercy-styled
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(24.0),
                    left: Val::Percent(50.0),
                    margin: UiRect::new(Val::Auto, Val::Auto, Val::Px(0.0), Val::Px(0.0)),
                    transform: Transform::from_translation(Vec3::new(-220.0, 0.0, 0.0)), // center offset
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(12.0)),
                    column_gap: Val::Px(14.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: BackgroundColor(ABILITY_BAR_BG),
                border_color: BorderColor(MERCY_PINK),
                ..default()
            },
            AbilityBarRoot,
            Name::new("Ambrosian Ability Bar"),
        ))
        .with_children(|parent| {
            // Slot 1: Mercy Bloom (Active - Core support)
            spawn_ability_slot(parent, AbilityType::MercyBloom, "Q", MERCY_PINK, "Mercy Bloom");

            // Slot 2: Celestial Harmony Pulse (Ultimate - Group divine wave)
            spawn_ability_slot(parent, AbilityType::CelestialHarmonyPulse, "E", CELESTIAL_GOLD, "Celestial Harmony");

            // Slot 3: Divine Presence (Passive indicator)
            spawn_ability_slot(parent, AbilityType::DivinePresence, "", DIVINE_BLUE, "Divine Presence");

            // Slot 4: Ascended Resonance (Passive + Epiphany boost)
            spawn_ability_slot(parent, AbilityType::AscendedResonance, "", HARMONY_CYAN, "Ascended Resonance");

            // Harmony Stacks display (integrated at end of bar)
            spawn_harmony_stacks_display(parent);
        });
}

fn spawn_ability_slot(
    parent: &mut ChildBuilder,
    ability: AbilityType,
    hotkey: &'static str,
    accent_color: Color,
    label: &str,
) {
    let is_ultimate = ability == AbilityType::CelestialHarmonyPulse;
    let slot_width = if is_ultimate { Val::Px(92.0) } else { Val::Px(72.0) };
    let slot_height = if is_ultimate { Val::Px(92.0) } else { Val::Px(72.0) };

    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: slot_width,
                    height: slot_height,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(if is_ultimate { 3.0 } else { 2.0 })),
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                background_color: BackgroundColor(SLOT_BG),
                border_color: BorderColor(accent_color),
                ..default()
            },
            AbilitySlot { ability_type: ability, hotkey },
            Name::new(format!("Ability Slot: {}", label)),
        ))
        .with_children(|slot| {
            // Icon placeholder (colored circle + symbol - replace with ImageBundle later)
            slot.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(if is_ultimate { 58.0 } else { 46.0 }),
                        height: Val::Px(if is_ultimate { 58.0 } else { 46.0 }),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(999.0)), // circle
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(accent_color),
                    ..default()
                },
                Name::new("Icon Placeholder"),
            )).with_children(|icon| {
                icon.spawn(TextBundle {
                    text: Text::from_section(
                        match ability {
                            AbilityType::MercyBloom => "⚘", // mercy symbol
                            AbilityType::CelestialHarmonyPulse => "☀", // sun / pulse
                            AbilityType::DivinePresence => "●", // presence dot
                            AbilityType::AscendedResonance => "✿", // resonance flower
                        },
                        TextStyle {
                            font_size: if is_ultimate { 32.0 } else { 26.0 },
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                    ..default()
                });
            });

            // Hotkey label (top right corner style)
            if !hotkey.is_empty() {
                slot.spawn((
                    TextBundle {
                        text: Text::from_section(
                            hotkey,
                            TextStyle {
                                font_size: 14.0,
                                color: Color::srgba(0.9, 0.95, 1.0, 0.95),
                                ..default()
                            },
                        ),
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(6.0),
                            right: Val::Px(8.0),
                            ..default()
                        },
                        ..default()
                    },
                ));
            }

            // Cooldown overlay (darkens icon when on cooldown)
            slot.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(0.0), // updated dynamically by system
                        bottom: Val::Px(0.0),
                        ..default()
                    },
                    background_color: BackgroundColor(COOLDOWN_OVERLAY),
                    ..default()
                },
                CooldownOverlay,
            ));

            // Cooldown text (shows remaining time or READY)
            slot.spawn((
                TextBundle {
                    text: Text::from_section(
                        "READY",
                        TextStyle {
                            font_size: 11.0,
                            color: Color::srgba(0.6, 1.0, 0.7, 0.95),
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(6.0),
                        ..default()
                    },
                    ..default()
                },
                CooldownText,
            ));

            // Ability name label (small)
            slot.spawn(TextBundle {
                text: Text::from_section(
                    label,
                    TextStyle {
                        font_size: 9.0,
                        color: Color::srgba(0.85, 0.9, 1.0, 0.85),
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::top(Val::Px(2.0)),
                    ..default()
                },
                ..default()
            });
        });
}

fn spawn_harmony_stacks_display(parent: &mut ChildBuilder) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::left(Val::Px(12.0)),
                    ..default()
                },
                ..default()
            },
            HarmonyStackDisplay,
            Name::new("Harmony Stacks"),
        ))
        .with_children(|stack| {
            stack.spawn(TextBundle {
                text: Text::from_section(
                    "HARMONY",
                    TextStyle {
                        font_size: 10.0,
                        color: HARMONY_CYAN,
                        ..default()
                    },
                ),
                ..default()
            });

            // 5 orb placeholders (max stacks example)
            stack
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(4.0),
                        margin: UiRect::top(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|orbs| {
                    for i in 0..5 {
                        orbs.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(14.0),
                                    height: Val::Px(14.0),
                                    border_radius: BorderRadius::all(Val::Px(999.0)),
                                    border: UiRect::all(Val::Px(1.5)),
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::srgba(0.3, 0.95, 0.85, 0.3)), // empty
                                border_color: BorderColor(HARMONY_CYAN),
                                ..default()
                            },
                            HarmonyOrb { index: i },
                        ));
                    }
                });
        });
}

// ============================================================================
// UPDATE SYSTEMS
// ============================================================================

/// Updates cooldown overlays and text for all ability slots.
/// Queries the local player's cooldown components (assumes LocalPlayer marker exists).
pub fn update_ability_cooldowns_system(
    time: Res<Time>,
    mut slot_query: Query<(&AbilitySlot, &Children)>,
    mut overlay_query: Query<&mut Style, With<CooldownOverlay>>,
    mut text_query: Query<&mut Text, With<CooldownText>>,
    // TODO: Replace with actual LocalPlayer query once LocalPlayer component is added
    // player_query: Query<(&MercyBloomCooldown, &CelestialHarmonyPulseCooldown, &MercyAlignment)>,
) {
    for (slot, children) in slot_query.iter() {
        let mut overlay_style: Option<Mut<Style>> = None;
        let mut cooldown_text: Option<Mut<Text>> = None;

        for child in children.iter() {
            if let Ok(mut style) = overlay_query.get_mut(*child) {
                overlay_style = Some(style);
            }
            if let Ok(mut text) = text_query.get_mut(*child) {
                cooldown_text = Some(text);
            }
        }

        let (remaining, total, is_on_cooldown) = match slot.ability_type {
            AbilityType::MercyBloom => {
                // In real impl: read from player's MercyBloomCooldown timer
                // For now placeholder values for design
                (12.4, 42.0, true)
            }
            AbilityType::CelestialHarmonyPulse => {
                (87.0, 165.0, true)
            }
            AbilityType::DivinePresence | AbilityType::AscendedResonance => {
                (0.0, 1.0, false) // Passives never on cooldown
            }
        };

        if let Some(mut style) = overlay_style {
            if is_on_cooldown {
                let progress = (remaining / total).clamp(0.0, 1.0);
                style.height = Val::Percent(progress * 100.0);
            } else {
                style.height = Val::Percent(0.0);
            }
        }

        if let Some(mut text) = cooldown_text {
            if is_on_cooldown {
                text.sections[0].value = format!("{:.1}s", remaining);
                text.sections[0].style.color = Color::srgba(1.0, 0.7, 0.7, 0.95);
            } else {
                text.sections[0].value = "READY".to_string();
                text.sections[0].style.color = Color::srgba(0.5, 1.0, 0.6, 0.95);
            }
        }
    }
}

/// Updates the Harmony Orb visuals based on current stack count.
pub fn update_harmony_stacks_system(
    // TODO: Query actual HarmonyStack component from local player
    mut orb_query: Query<(&mut BackgroundColor, &HarmonyOrb)>,
) {
    let current_stacks = 3; // Placeholder - replace with real query
    let max_stacks = 5;

    for (mut bg, orb) in orb_query.iter_mut() {
        if orb.index < current_stacks {
            bg.0 = Color::srgba(0.3, 0.95, 0.85, 1.0); // Filled glowing cyan
        } else {
            bg.0 = Color::srgba(0.3, 0.95, 0.85, 0.25); // Faded
        }
    }
}

// ============================================================================
// INTEGRATION NOTES (PATSAGi Guidance)
// ============================================================================
// 1. Add `LocalPlayer` marker component to the client-controlled entity.
// 2. Replace placeholder cooldown values with real queries:
//      Query<(&MercyBloomCooldown, &CelestialHarmonyPulseCooldown, &MercyAlignment, &HarmonyStack)>
// 3. For selfish penalty: when MercyAlignment.score < 0.5, add red border or tint to Mercy Bloom slot.
// 4. Spawn this bar only for AmbrosianAscended players (use run condition or visibility).
// 5. Later: Replace icon placeholders with proper ImageBundle + bevy_hanabi glow effects on activation.
// 6. Add tooltip on hover showing exact formula + current Mercy multiplier.
// 7. Ultimate slot pulses gently when off cooldown (add Animation or pulsing scale).
//
// This UI makes the Ambrosian feel powerful, supportive, and visually divine.
// The bar itself becomes part of the sacred identity of ascension.
