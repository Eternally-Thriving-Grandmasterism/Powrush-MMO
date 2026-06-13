/*!
 * Ambrosian Ability Bar UI with Rich Tooltips
 *
 * Sacred interface for the ascended.
 * Hover any ability to reveal deep mechanical and philosophical details.
 *
 * PATSAGi Council approved • Ra-Thor Lattice aligned • TOLC 8 Mercy Gates enforced
 */ 

use bevy::prelude::*;

use crate::ascension::components::{AmbrosianAscended, MercyAlignment, ResonanceAttunement, MercyBloomCooldown, CelestialHarmonyPulseCooldown, HarmonyStack};

/// Marker for which ability a slot represents
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AmbrosianAbility {
    MercyBloom,
    CelestialHarmonyPulse,
    DivinePresence,
    AscendedResonance,
}

/// Marker component for the main ability bar container
#[derive(Component)]
pub struct AbilityBar;

/// Marker for individual ability slot buttons (now interactive)
#[derive(Component)]
pub struct AbilitySlotButton {
    pub ability: AmbrosianAbility,
}

/// The global tooltip panel (one instance, shown/hidden on hover)
#[derive(Component)]
pub struct AbilityTooltipPanel;

/// Text inside the tooltip
#[derive(Component)]
pub struct AbilityTooltipText;

/// Spawns the bottom-centered Ambrosian Ability Bar with 4 interactive slots + tooltip system
pub fn spawn_ambrosian_ability_bar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Main bar container (bottom center)
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Percent(50.0),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Px(0.0), Val::Px(0.0)),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(12.0),
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.08, 0.04, 0.12, 0.92)), // Deep mercy purple
            border_color: BorderColor(Color::srgb(0.85, 0.65, 1.0)), // Golden-pink border
            ..default()
        },
        AbilityBar,
        Name::new("Ambrosian Ability Bar"),
    )).with_children(|parent| {
        // Slot 1: Mercy Bloom (Active)
        spawn_ability_slot(parent, AmbrosianAbility::MercyBloom, "Mercy Bloom", "Q", Color::srgb(0.95, 0.75, 0.55), &asset_server);
        
        // Slot 2: Celestial Harmony Pulse (Ultimate)
        spawn_ability_slot(parent, AmbrosianAbility::CelestialHarmonyPulse, "Celestial Harmony Pulse", "E", Color::srgb(0.55, 0.75, 0.95), &asset_server);
        
        // Slot 3: Divine Presence (Passive)
        spawn_ability_slot(parent, AmbrosianAbility::DivinePresence, "Divine Presence", "Passive", Color::srgb(0.75, 0.55, 0.95), &asset_server);
        
        // Slot 4: Ascended Resonance (Passive)
        spawn_ability_slot(parent, AmbrosianAbility::AscendedResonance, "Ascended Resonance", "Passive", Color::srgb(0.65, 0.95, 0.85), &asset_server);
    });

    // Spawn the single tooltip panel (hidden by default)
    spawn_ability_tooltip_panel(&mut commands);
}

fn spawn_ability_slot(
    parent: &mut ChildBuilder,
    ability: AmbrosianAbility,
    label: &str,
    hotkey: &str,
    accent_color: Color,
    asset_server: &AssetServer,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(78.0),
                height: Val::Px(78.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(2.0)),
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.12, 0.08, 0.18, 0.95)),
            border_color: BorderColor(accent_color),
            ..default()
        },
        AbilitySlotButton { ability },
        Interaction::default(),
        Name::new(format!("Ability Slot: {}", label)),
    )).with_children(|slot| {
        // Icon placeholder (replace with real texture + bevy_hanabi glow later)
        slot.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(48.0),
                    height: Val::Px(48.0),
                    ..default()
                },
                background_color: BackgroundColor(accent_color.with_alpha(0.3)),
                ..default()
            },
            Name::new("Icon Placeholder"),
        ));

        // Hotkey label
        slot.spawn(TextBundle {
            text: Text::from_section(
                hotkey,
                TextStyle {
                    font_size: 11.0,
                    color: Color::srgb(0.9, 0.85, 0.95),
                    ..default()
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(4.0),
                right: Val::Px(6.0),
                ..default()
            },
            ..default()
        });

        // Cooldown overlay (for active abilities)
        if ability == AmbrosianAbility::MercyBloom || ability == AmbrosianAbility::CelestialHarmonyPulse {
            slot.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(0.0), // Will be animated by system
                        bottom: Val::Px(0.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
                    ..default()
                },
                CooldownOverlay { ability },
            ));
        }

        // Harmony stacks for Mercy Bloom (visual orbs)
        if ability == AmbrosianAbility::MercyBloom {
            slot.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(6.0),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(3.0),
                        ..default()
                    },
                    ..default()
                },
                HarmonyStackUI,
            )).with_children(|harmony| {
                for i in 0..5 {
                    harmony.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(8.0),
                                height: Val::Px(8.0),
                                border_radius: BorderRadius::all(Val::Px(4.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.4, 0.85, 0.95).with_alpha(0.3)),
                            ..default()
                        },
                        HarmonyOrb { index: i },
                    ));
                }
            });
        }
    });
}

#[derive(Component)]
pub struct CooldownOverlay {
    pub ability: AmbrosianAbility,
}

#[derive(Component)]
pub struct HarmonyStackUI;

#[derive(Component)]
pub struct HarmonyOrb {
    pub index: usize,
}

/// Spawns the beautiful tooltip panel (one instance, controlled by hover system)
fn spawn_ability_tooltip_panel(commands: &mut Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(115.0), // Above the ability bar
                left: Val::Percent(50.0),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Px(0.0), Val::Px(0.0)),
                padding: UiRect::all(Val::Px(16.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                border: UiRect::all(Val::Px(2.0)),
                display: Display::None, // Hidden until hover
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.06, 0.03, 0.10, 0.96)),
            border_color: BorderColor(Color::srgb(0.92, 0.78, 1.0)),
            ..default()
        },
        AbilityTooltipPanel,
        Name::new("Ability Tooltip Panel"),
    )).with_children(|parent| {
        // Title line
        parent.spawn((
            TextBundle {
                text: Text::from_section(
                    "Ability Name",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(0.95, 0.85, 0.6),
                        ..default()
                    },
                ),
                ..default()
            },
            AbilityTooltipText,
        ));

        // Dynamic content will be updated by the system
        parent.spawn((
            TextBundle {
                text: Text::from_section(
                    "Hover an ability to see its sacred mechanics...",
                    TextStyle {
                        font_size: 13.0,
                        color: Color::srgb(0.85, 0.82, 0.9),
                        ..default()
                    },
                ),
                style: Style {
                    margin: UiRect::top(Val::Px(8.0)),
                    max_width: Val::Px(380.0),
                    ..default()
                },
                ..default()
            },
            AbilityTooltipText,
        ));
    });
}

/// System that shows/hides and updates the tooltip based on hover
pub fn ability_tooltip_hover_system(
    mut tooltip_query: Query<(&mut Style, &mut Children), With<AbilityTooltipPanel>>,
    slot_query: Query<(&Interaction, &AbilitySlotButton, &GlobalTransform), Changed<Interaction>>,
    mut text_query: Query<&mut Text, With<AbilityTooltipText>>,
    windows: Query<&Window>,
) {
    let Ok((mut tooltip_style, children)) = tooltip_query.get_single_mut() else { return };

    let mut hovered_ability: Option<AmbrosianAbility> = None;

    for (interaction, slot_button, _transform) in slot_query.iter() {
        if *interaction == Interaction::Hovered {
            hovered_ability = Some(slot_button.ability);
            break;
        }
    }

    if let Some(ability) = hovered_ability {
        tooltip_style.display = Display::Flex;

        // Update tooltip content
        if let Ok(mut title_text) = text_query.get_mut(children[0]) {
            title_text.sections[0].value = get_ability_title(ability);
            title_text.sections[0].style.color = get_ability_accent_color(ability);
        }

        if children.len() > 1 {
            if let Ok(mut body_text) = text_query.get_mut(children[1]) {
                body_text.sections[0].value = get_ability_tooltip_body(ability);
            }
        }
    } else {
        tooltip_style.display = Display::None;
    }
}

fn get_ability_title(ability: AmbrosianAbility) -> String {
    match ability {
        AmbrosianAbility::MercyBloom => "✦ Mercy Bloom  •  Active".to_string(),
        AmbrosianAbility::CelestialHarmonyPulse => "✦ Celestial Harmony Pulse  •  ULTIMATE".to_string(),
        AmbrosianAbility::DivinePresence => "✦ Divine Presence  •  Passive Aura".to_string(),
        AmbrosianAbility::AscendedResonance => "✦ Ascended Resonance  •  Core Passive".to_string(),
    }
}

fn get_ability_accent_color(ability: AmbrosianAbility) -> Color {
    match ability {
        AmbrosianAbility::MercyBloom => Color::srgb(0.95, 0.78, 0.55),
        AmbrosianAbility::CelestialHarmonyPulse => Color::srgb(0.55, 0.82, 0.98),
        AmbrosianAbility::DivinePresence => Color::srgb(0.82, 0.65, 0.98),
        AmbrosianAbility::AscendedResonance => Color::srgb(0.65, 0.95, 0.78),
    }
}

fn get_ability_tooltip_body(ability: AmbrosianAbility) -> String {
    match ability {
        AmbrosianAbility::MercyBloom => {
            "Creates a large golden-pink field that restores health & resources to all allies inside.
Grants temporary Harmony stacks that reduce your own cooldown.

Radius & healing strength scale with Resonance Attunement and Mercy Alignment.

⚠ Selfish Penalty: If Mercy Alignment is low, effectiveness drops dramatically (up to -60%).

Synergy: The more Harmony stacks you hold, the faster Mercy Bloom recharges."
                .to_string()
        }
        AmbrosianAbility::CelestialHarmonyPulse => {
            "Sends a powerful expanding resonance wave across a large area.
Heals and buffs all allies, and has a chance to trigger Epiphanies in nearby players.

Power = Base × (1 + 0.18 × number of allies in range).
Extremely strong in coordinated groups. Very weak when used alone.

This is the signature expression of Ambrosian power — a true force multiplier for the collective."
                .to_string()
        }
        AmbrosianAbility::DivinePresence => {
            "Your mere presence radiates gentle resonance.
Allies within ~18m passively gain +Resonance gain and small Harmony bonuses.

You take +25–28% increased damage when no allies are within 25m.

Stay close to your team. Your safety and their power are one and the same."
                .to_string()
        }
        AmbrosianAbility::AscendedResonance => {
            "The heart of what it means to be Ambrosian.
+70% Epiphany trigger rate and +40% average Epiphany intensity for yourself.

Your resonance subtly elevates everyone around you over time.

This passive is always active and represents the deepest spiritual reward of ascension."
                .to_string()
        }
    }
}

/// Optional: Update cooldown overlay heights (call this from your main cooldown system)
pub fn update_cooldown_overlays(
    mut overlay_query: Query<(&mut Style, &CooldownOverlay)>,
    // TODO: Query actual cooldown timers from player
) {
    for (mut style, overlay) in overlay_query.iter_mut() {
        // Example: set height based on remaining cooldown percentage
        // style.height = Val::Percent(remaining * 100.0);
    }
}

/// Update harmony orb visuals based on current stack count
pub fn update_harmony_orbs(
    mut orb_query: Query<(&mut BackgroundColor, &HarmonyOrb)>,
    harmony_query: Query<&HarmonyStack, With<AmbrosianAscended>>,
) {
    if let Ok(harmony) = harmony_query.get_single() {
        for (mut bg, orb) in orb_query.iter_mut() {
            if orb.index < harmony.current_stacks as usize {
                bg.0 = Color::srgb(0.4, 0.85, 0.95); // Glowing cyan
            } else {
                bg.0 = Color::srgb(0.4, 0.85, 0.95).with_alpha(0.25);
            }
        }
    }
}
