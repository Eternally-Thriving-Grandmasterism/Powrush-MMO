/*!
 * Living Practice Loop — Human playability layer (post first-session)
 *
 * After the soft first-session strip reaches Free Exploration, this loop
 * invites the player into *Caps Across Climates* — the same high-road
 * principle (resource allocation under uncertainty) across three different
 * realm surfaces. Designed for deeper gameplay without extractive
 * gamification.
 *
 * Principles (PATSAGi + TOLC 8):
 * - Voluntary, dismissible (H still respects FirstSession; P toggles practice)
 * - No FOMO timers, no paywalls, no punitive failure
 * - Progress is presence + mercy-aligned action, not grind counters
 * - Mirrors simulation CrossRealmChallenge id=1 for dual-repo coherence
 *
 * AG-SML v1.0 | Contact: info@Rathor.ai
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::first_session_guidance::{FirstSessionGuidance, GuidanceObjective};

/// Which climate-surface of Caps Across Climates the player is practicing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PracticeSurface {
    /// Realm 0 — stable yields, soft harvest cap without starving growth.
    SanctuaryCap,
    /// Realm 2 — surplus flood, allocate without sustainability collapse.
    VerdantSurge,
    /// Realm 4 — thin horizon, allocate under incomplete scouting.
    HorizonScarcity,
    /// All three practiced — principle internalized.
    PrincipleSealed,
}

impl PracticeSurface {
    pub fn title(&self) -> &'static str {
        match self {
            PracticeSurface::SanctuaryCap => "Sanctuary soft cap",
            PracticeSurface::VerdantSurge => "Verdant surge",
            PracticeSurface::HorizonScarcity => "Horizon scarcity",
            PracticeSurface::PrincipleSealed => "Principle sealed",
        }
    }

    pub fn prompt(&self) -> &'static str {
        match self {
            PracticeSurface::SanctuaryCap => {
                "Caps Across Climates · Sanctuary: harvest with restraint — leave the node thriving"
            }
            PracticeSurface::VerdantSurge => {
                "Caps Across Climates · Verdant: abundance is flooding — allocate without collapse"
            }
            PracticeSurface::HorizonScarcity => {
                "Caps Across Climates · Horizon: sparse yields — choose carefully under uncertainty"
            }
            PracticeSurface::PrincipleSealed => {
                "You carried the same principle across three climates. Sovereign exploration continues."
            }
        }
    }

    pub fn next(&self) -> Self {
        match self {
            PracticeSurface::SanctuaryCap => PracticeSurface::VerdantSurge,
            PracticeSurface::VerdantSurge => PracticeSurface::HorizonScarcity,
            PracticeSurface::HorizonScarcity => PracticeSurface::PrincipleSealed,
            PracticeSurface::PrincipleSealed => PracticeSurface::PrincipleSealed,
        }
    }

    /// Soft mapping to multi-realm ids (simulation CrossRealmChallenge id=1).
    pub fn realm_id(&self) -> Option<u8> {
        match self {
            PracticeSurface::SanctuaryCap => Some(0),
            PracticeSurface::VerdantSurge => Some(2),
            PracticeSurface::HorizonScarcity => Some(4),
            PracticeSurface::PrincipleSealed => None,
        }
    }
}

#[derive(Resource, Debug)]
pub struct LivingPracticeLoop {
    pub active: bool,
    pub dismissed: bool,
    pub surface: PracticeSurface,
    /// Mercy-aligned harvests credited on current surface.
    pub mercy_harvests_on_surface: u32,
    /// Harvests needed per surface (gentle, not grindy).
    pub harvests_needed: u32,
    pub surfaces_cleared: u32,
    pub principle_sealed: bool,
    /// Soft celebration pulse when a surface clears.
    pub celebrate_until: f64,
}

impl Default for LivingPracticeLoop {
    fn default() -> Self {
        Self {
            active: false,
            dismissed: false,
            surface: PracticeSurface::SanctuaryCap,
            mercy_harvests_on_surface: 0,
            harvests_needed: 2,
            surfaces_cleared: 0,
            principle_sealed: false,
            celebrate_until: 0.0,
        }
    }
}

impl LivingPracticeLoop {
    pub fn dismiss(&mut self) {
        self.dismissed = true;
        self.active = false;
    }

    pub fn try_activate_from_guidance(&mut self, guidance: &FirstSessionGuidance) {
        if self.dismissed || self.active || self.principle_sealed {
            return;
        }
        // Hand-off only once first-session reaches free exploration (or was dismissed after real play)
        let ready = matches!(guidance.objective, GuidanceObjective::FreeExploration)
            || (guidance.dismissed && guidance.harvests_completed >= 1);
        if ready {
            self.active = true;
            self.surface = PracticeSurface::SanctuaryCap;
            self.mercy_harvests_on_surface = 0;
        }
    }

    /// Credit a harvest that was taken with mercy (caller decides mercy).
    pub fn credit_mercy_harvest(&mut self, now_secs: f64) {
        if !self.active || self.dismissed || self.principle_sealed {
            return;
        }
        self.mercy_harvests_on_surface = self.mercy_harvests_on_surface.saturating_add(1);
        if self.mercy_harvests_on_surface >= self.harvests_needed {
            self.surfaces_cleared = self.surfaces_cleared.saturating_add(1);
            self.mercy_harvests_on_surface = 0;
            self.celebrate_until = now_secs + 4.0;
            self.surface = self.surface.next();
            if matches!(self.surface, PracticeSurface::PrincipleSealed) {
                self.principle_sealed = true;
            }
        }
    }
}

#[derive(Component)]
pub struct LivingPracticeStrip;

#[derive(Component)]
pub struct LivingPracticeText;

pub struct LivingPracticeLoopPlugin;

impl Plugin for LivingPracticeLoopPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LivingPracticeLoop>()
            .add_systems(Startup, spawn_practice_strip)
            .add_systems(
                Update,
                (
                    handoff_from_first_session,
                    handle_practice_toggle,
                    update_practice_visibility,
                    update_practice_text,
                    soft_space_harvest_credit,
                ),
            );
    }
}

fn spawn_practice_strip(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(72.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(640.0),
                    margin: UiRect::left(Val::Px(-320.0)),
                    padding: UiRect::axes(Val::Px(18.0), Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.5)),
                    border_radius: BorderRadius::all(Val::Px(14.0)),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.09, 0.08, 0.90).into(),
                border_color: Color::srgba(0.45, 0.92, 0.72, 0.55).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            LivingPracticeStrip,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    PracticeSurface::SanctuaryCap.prompt(),
                    TextStyle {
                        font_size: 15.5,
                        color: Color::srgb(0.86, 0.98, 0.92),
                        ..default()
                    },
                ),
                LivingPracticeText,
            ));
        });
}

fn handoff_from_first_session(
    guidance: Res<FirstSessionGuidance>,
    mut practice: ResMut<LivingPracticeLoop>,
) {
    practice.try_activate_from_guidance(&guidance);
}

fn handle_practice_toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut practice: ResMut<LivingPracticeLoop>,
) {
    // P toggles practice strip; Shift+P dismisses for the session
    if keyboard.just_pressed(KeyCode::KeyP) {
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            practice.dismiss();
        } else if practice.principle_sealed {
            // Already sealed — soft re-show sealed message
            practice.active = !practice.active;
        } else if !practice.dismissed {
            practice.active = !practice.active;
        }
    }
}

fn update_practice_visibility(
    practice: Res<LivingPracticeLoop>,
    guidance: Res<FirstSessionGuidance>,
    mut query: Query<&mut Visibility, With<LivingPracticeStrip>>,
) {
    // Prefer not to fight the first-session strip
    let guidance_showing = guidance.active && !guidance.dismissed;
    let show = practice.active && !practice.dismissed && !guidance_showing;
    for mut vis in &mut query {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn update_practice_text(
    practice: Res<LivingPracticeLoop>,
    time: Res<Time>,
    mut query: Query<&mut Text, With<LivingPracticeText>>,
) {
    if !practice.is_changed() && practice.celebrate_until <= 0.0 {
        return;
    }
    let now = time.elapsed_seconds_f64();
    let celebrating = now < practice.celebrate_until;
    let body = if practice.principle_sealed {
        practice.surface.prompt().to_string()
    } else if celebrating {
        format!(
            "Surface cleared · {} → next climate",
            practice.surface.title()
        )
    } else {
        format!(
            "{}  ({}/{})",
            practice.surface.prompt(),
            practice.mercy_harvests_on_surface,
            practice.harvests_needed
        )
    };
    for mut text in &mut query {
        if let Some(section) = text.sections.get_mut(0) {
            section.value = body.clone();
            section.style.color = if celebrating {
                Color::srgb(1.0, 0.95, 0.55)
            } else if practice.principle_sealed {
                Color::srgb(0.75, 1.0, 0.88)
            } else {
                Color::srgb(0.86, 0.98, 0.92)
            };
        }
    }
}

/// Soft credit path when Space is used during an active practice surface.
/// Real harvest systems should call `credit_mercy_harvest` for authoritative progress.
fn soft_space_harvest_credit(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut practice: ResMut<LivingPracticeLoop>,
    time: Res<Time>,
) {
    if !practice.active || practice.dismissed || practice.principle_sealed {
        return;
    }
    if keyboard.just_pressed(KeyCode::Space) {
        practice.credit_mercy_harvest(time.elapsed_seconds_f64());
    }
}

/// Call from real harvest success when the action was mercy-aligned.
pub fn credit_practice_mercy_harvest(practice: &mut LivingPracticeLoop, now_secs: f64) {
    practice.credit_mercy_harvest(now_secs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surfaces_progress_to_sealed() {
        let mut loop_ = LivingPracticeLoop::default();
        loop_.active = true;
        assert_eq!(loop_.surface, PracticeSurface::SanctuaryCap);
        loop_.credit_mercy_harvest(1.0);
        loop_.credit_mercy_harvest(2.0);
        assert_eq!(loop_.surface, PracticeSurface::VerdantSurge);
        loop_.credit_mercy_harvest(3.0);
        loop_.credit_mercy_harvest(4.0);
        assert_eq!(loop_.surface, PracticeSurface::HorizonScarcity);
        loop_.credit_mercy_harvest(5.0);
        loop_.credit_mercy_harvest(6.0);
        assert!(loop_.principle_sealed);
        assert_eq!(loop_.surface, PracticeSurface::PrincipleSealed);
    }

    #[test]
    fn dismiss_blocks_credit() {
        let mut loop_ = LivingPracticeLoop::default();
        loop_.active = true;
        loop_.dismiss();
        loop_.credit_mercy_harvest(1.0);
        assert_eq!(loop_.mercy_harvests_on_surface, 0);
    }
}
