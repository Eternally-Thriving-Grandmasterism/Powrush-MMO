/*!
 * First Session Guidance — single onboarding card (v23.2.24)
 *
 * One sentence at a time: walk · tend · satchel · allocate.
 * H hides. World still teaches. Not a second HUD.
 * Does not rewrite harvest_feel or rbe_allocate_choice.
 *
 * Contact: info@Rathor.ai | Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::lived_hour_bind::LivedHourBind;
use crate::mercy_harvest_nodes::NearbyMercyNode;

/// Soft objective the player is gently invited to try next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuidanceObjective {
    MoveAround,
    ApproachGlowingNode,
    HarvestWithInteract,
    OpenInventory,
    ShareAbundance,
    FeelFirstEpiphany,
    MeetCouncilWhisper,
    FreeExploration,
}

impl GuidanceObjective {
    /// One sentence. Not a manifesto.
    pub fn prompt(&self) -> &'static str {
        match self {
            GuidanceObjective::MoveAround => "WASD walk · Space jump · Shift sprint",
            GuidanceObjective::ApproachGlowingNode => "Walk to a glow",
            GuidanceObjective::HarvestWithInteract => "E tend the glow",
            GuidanceObjective::OpenInventory => "I opens the satchel",
            GuidanceObjective::ShareAbundance => "R then 1 flow · 2 reserve",
            GuidanceObjective::FeelFirstEpiphany => "The field answers",
            GuidanceObjective::MeetCouncilWhisper => "The field answers",
            GuidanceObjective::FreeExploration => "The field keeps teaching",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            GuidanceObjective::MoveAround => GuidanceObjective::ApproachGlowingNode,
            GuidanceObjective::ApproachGlowingNode => GuidanceObjective::HarvestWithInteract,
            GuidanceObjective::HarvestWithInteract => GuidanceObjective::OpenInventory,
            GuidanceObjective::OpenInventory => GuidanceObjective::ShareAbundance,
            GuidanceObjective::ShareAbundance => GuidanceObjective::FreeExploration,
            GuidanceObjective::FeelFirstEpiphany => GuidanceObjective::FreeExploration,
            GuidanceObjective::MeetCouncilWhisper => GuidanceObjective::FreeExploration,
            GuidanceObjective::FreeExploration => GuidanceObjective::FreeExploration,
        }
    }
}

#[derive(Resource)]
pub struct FirstSessionGuidance {
    pub active: bool,
    pub dismissed: bool,
    pub objective: GuidanceObjective,
    pub harvests_completed: u32,
    pub moved_distance: f32,
    pub inventory_opened: bool,
    pub shared_abundance: bool,
    pub epiphany_felt: bool,
    pub shown_at_seconds: f64,
    pub near_glow: bool,
    pub free_since: f32,
}

impl Default for FirstSessionGuidance {
    fn default() -> Self {
        Self {
            active: true,
            dismissed: false,
            objective: GuidanceObjective::MoveAround,
            harvests_completed: 0,
            moved_distance: 0.0,
            inventory_opened: false,
            shared_abundance: false,
            epiphany_felt: false,
            shown_at_seconds: 0.0,
            near_glow: false,
            free_since: 0.0,
        }
    }
}

impl FirstSessionGuidance {
    pub fn dismiss(&mut self) {
        self.dismissed = true;
        self.active = false;
    }

    pub fn advance_if_ready(&mut self) {
        if self.dismissed {
            return;
        }
        let should_advance = match self.objective {
            GuidanceObjective::MoveAround => self.moved_distance > 4.0,
            GuidanceObjective::ApproachGlowingNode => self.near_glow || self.moved_distance > 12.0,
            GuidanceObjective::HarvestWithInteract => self.harvests_completed >= 1,
            GuidanceObjective::OpenInventory => self.inventory_opened,
            GuidanceObjective::ShareAbundance => self.shared_abundance,
            GuidanceObjective::FeelFirstEpiphany => self.epiphany_felt,
            GuidanceObjective::MeetCouncilWhisper => {
                self.epiphany_felt && self.harvests_completed >= 1
            }
            GuidanceObjective::FreeExploration => false,
        };
        if should_advance {
            self.objective = self.objective.next();
        }
    }
}

#[derive(Component)]
pub struct FirstSessionGuidanceStrip;

#[derive(Component)]
pub struct FirstSessionGuidanceText;

pub struct FirstSessionGuidancePlugin;

impl Plugin for FirstSessionGuidancePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FirstSessionGuidance>()
            .add_systems(Startup, spawn_guidance_strip)
            .add_systems(
                Update,
                (
                    handle_guidance_dismiss_input,
                    track_simple_progress_signals,
                    update_guidance_visibility,
                    update_guidance_text,
                )
                    .chain(),
            );
    }
}

fn spawn_guidance_strip(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(72.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(520.0),
                    margin: UiRect::left(Val::Px(-260.0)),
                    padding: UiRect::axes(Val::Px(18.0), Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: Color::srgba(0.02, 0.03, 0.04, 0.94).into(),
                border_color: Color::srgba(0.92, 0.96, 0.78, 0.82).into(),
                visibility: Visibility::Visible,
                ..default()
            },
            FirstSessionGuidanceStrip,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    card_line(GuidanceObjective::MoveAround.prompt()),
                    TextStyle {
                        font_size: 17.0,
                        color: Color::srgb(0.96, 0.98, 0.88),
                        ..default()
                    },
                ),
                FirstSessionGuidanceText,
            ));
        });
}

fn card_line(prompt: &str) -> String {
    format!("{prompt}  · H hides")
}

fn update_guidance_visibility(
    guidance: Res<FirstSessionGuidance>,
    bind: Option<Res<LivedHourBind>>,
    mut query: Query<&mut Visibility, With<FirstSessionGuidanceStrip>>,
) {
    let hidden_by_bind = bind.map(|b| b.guidance_hidden).unwrap_or(false);
    let show = guidance.active && !guidance.dismissed && !hidden_by_bind;
    for mut vis in &mut query {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn update_guidance_text(
    guidance: Res<FirstSessionGuidance>,
    mut query: Query<&mut Text, With<FirstSessionGuidanceText>>,
) {
    if !guidance.is_changed() {
        return;
    }
    let prompt = if guidance.dismissed {
        String::new()
    } else {
        card_line(guidance.objective.prompt())
    };
    for mut text in &mut query {
        if let Some(section) = text.sections.get_mut(0) {
            if section.value != prompt {
                section.value = prompt.clone();
            }
        }
    }
}

fn handle_guidance_dismiss_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut guidance: ResMut<FirstSessionGuidance>,
    bind: Option<ResMut<LivedHourBind>>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) && guidance.active {
        guidance.dismiss();
        if let Some(mut bind) = bind {
            bind.guidance_hidden = true;
        }
    }
}

fn track_simple_progress_signals(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut guidance: ResMut<FirstSessionGuidance>,
    time: Res<Time>,
    nearby: Option<Res<NearbyMercyNode>>,
    bind: Option<Res<LivedHourBind>>,
) {
    if guidance.dismissed {
        return;
    }

    let moving = keyboard.pressed(KeyCode::KeyW)
        || keyboard.pressed(KeyCode::KeyA)
        || keyboard.pressed(KeyCode::KeyS)
        || keyboard.pressed(KeyCode::KeyD)
        || keyboard.pressed(KeyCode::ArrowUp)
        || keyboard.pressed(KeyCode::ArrowDown)
        || keyboard.pressed(KeyCode::ArrowLeft)
        || keyboard.pressed(KeyCode::ArrowRight);

    if moving {
        guidance.moved_distance += time.delta_seconds() * 6.0;
    }

    if keyboard.just_pressed(KeyCode::KeyI) {
        guidance.inventory_opened = true;
    }

    if let Some(near) = nearby {
        guidance.near_glow = near.in_range;
    }

    if let Some(bind) = bind {
        let taken = bind.satchel_count() as u32
            + bind.hour.allocation.flow
            + bind.hour.allocation.reserve;
        if taken > guidance.harvests_completed {
            guidance.harvests_completed = taken;
        }
        if bind.hour.allocation.flow + bind.hour.allocation.reserve > 0 {
            guidance.shared_abundance = true;
        }
    }

    guidance.advance_if_ready();

    if guidance.objective == GuidanceObjective::FreeExploration {
        guidance.free_since += time.delta_seconds();
        if guidance.free_since > 8.0 {
            guidance.dismiss();
        }
    }
}

pub fn credit_harvest(guidance: &mut FirstSessionGuidance) {
    guidance.harvests_completed = guidance.harvests_completed.saturating_add(1);
    guidance.advance_if_ready();
}

pub fn credit_epiphany(guidance: &mut FirstSessionGuidance) {
    guidance.epiphany_felt = true;
    guidance.advance_if_ready();
}

pub fn credit_share(guidance: &mut FirstSessionGuidance) {
    guidance.shared_abundance = true;
    guidance.advance_if_ready();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hour_is_four_hands() {
        let mut g = FirstSessionGuidance::default();
        assert_eq!(g.objective, GuidanceObjective::MoveAround);
        g.moved_distance = 5.0;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::ApproachGlowingNode);
        g.near_glow = true;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::HarvestWithInteract);
        credit_harvest(&mut g);
        assert_eq!(g.objective, GuidanceObjective::OpenInventory);
        g.inventory_opened = true;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::ShareAbundance);
        credit_share(&mut g);
        assert_eq!(g.objective, GuidanceObjective::FreeExploration);
    }

    #[test]
    fn satchel_then_share_then_free() {
        let mut g = FirstSessionGuidance::default();
        g.objective = GuidanceObjective::OpenInventory;
        g.inventory_opened = true;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::ShareAbundance);
        credit_share(&mut g);
        assert_eq!(g.objective, GuidanceObjective::FreeExploration);
    }

    #[test]
    fn prompts_are_one_sentence() {
        for obj in [
            GuidanceObjective::MoveAround,
            GuidanceObjective::ApproachGlowingNode,
            GuidanceObjective::HarvestWithInteract,
            GuidanceObjective::OpenInventory,
            GuidanceObjective::ShareAbundance,
            GuidanceObjective::FreeExploration,
        ] {
            let p = obj.prompt();
            assert!(p.len() < 48, "{p} is a manifesto");
            assert!(!p.contains("Lattice"));
            assert!(!p.contains("Council"));
            assert!(!p.contains("Divine"));
        }
    }

    #[test]
    fn h_dismisses() {
        let mut g = FirstSessionGuidance::default();
        g.dismiss();
        assert!(g.dismissed);
        assert!(!g.active);
    }
}
