/*!
 * Living Practice Loop — Human playability layer (post first-session)
 * v21.97.0 — Soft harvest on **E** (interact), never Space (jump)
 *
 * Soft interact remains demo path. Real harvest feedback credits practice.
 *
 * AG-SML v1.0 | Contact: info@Rathor.ai | Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::first_session_guidance::{FirstSessionGuidance, GuidanceObjective};
use crate::lived_hour_support::RbeUiSync;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

/// Client-side soft mirror of current realm (no hard sim crate dep).
#[derive(Resource, Debug, Default, Clone)]
pub struct SoftPlayerRealm {
    pub current: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PracticeSurface {
    SanctuaryCap,
    VerdantSurge,
    HorizonScarcity,
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
                "Caps Across Climates · Sanctuary: harvest with restraint — leave the node thriving (E)"
            }
            PracticeSurface::VerdantSurge => {
                "Caps Across Climates · Verdant: abundance is flooding — allocate without collapse (E)"
            }
            PracticeSurface::HorizonScarcity => {
                "Caps Across Climates · Horizon: sparse yields — choose carefully under uncertainty (E)"
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
    pub mercy_harvests_on_surface: u32,
    pub harvests_needed: u32,
    pub surfaces_cleared: u32,
    pub principle_sealed: bool,
    pub celebrate_until: f64,
    pub realm_aware: bool,
    pub last_realm_mismatch_hint_at: f64,
    pub last_bridged_feedback: Option<String>,
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
            realm_aware: true,
            last_realm_mismatch_hint_at: -999.0,
            last_bridged_feedback: None,
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
        let ready = matches!(guidance.objective, GuidanceObjective::FreeExploration)
            || (guidance.dismissed && guidance.harvests_completed >= 1);
        if ready {
            self.active = true;
            self.surface = PracticeSurface::SanctuaryCap;
            self.mercy_harvests_on_surface = 0;
        }
    }

    pub fn credit_mercy_harvest(&mut self, now_secs: f64) -> bool {
        if !self.active || self.dismissed || self.principle_sealed {
            return false;
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
        true
    }

    pub fn allows_credit(&self, player_realm: Option<u8>) -> bool {
        if !self.realm_aware {
            return true;
        }
        match (self.surface.realm_id(), player_realm) {
            (Some(need), Some(have)) => need == have,
            (Some(_), None) => true,
            (None, _) => false,
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
            .init_resource::<SoftPlayerRealm>()
            .add_systems(Startup, spawn_practice_strip)
            .add_systems(
                Update,
                (
                    handoff_from_first_session,
                    handle_practice_toggle,
                    update_practice_visibility,
                    update_practice_text,
                    soft_interact_harvest_credit,
                    bridge_rbe_feedback_to_practice,
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
    if keyboard.just_pressed(KeyCode::KeyP) {
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            practice.dismiss();
        } else if practice.principle_sealed {
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

fn apply_practice_credit(
    practice: &mut LivingPracticeLoop,
    moments: &mut ThrivingMoments,
    now: f64,
    player_realm: Option<u8>,
) -> bool {
    if !practice.allows_credit(player_realm) {
        return false;
    }
    let before_surface = practice.surface;
    let before_sealed = practice.principle_sealed;
    if !practice.credit_mercy_harvest(now) {
        return false;
    }
    fire_thriving(moments, ThrivingKind::FirstMercyHarvest, now);
    if practice.surface != before_surface {
        fire_thriving(moments, ThrivingKind::SurfaceCleared, now);
    }
    if practice.principle_sealed && !before_sealed {
        fire_thriving(moments, ThrivingKind::PrincipleSealed, now);
        fire_thriving(moments, ThrivingKind::CouncilInvite, now);
    }
    true
}

/// Soft demo harvest — **E** interact (never Space; Space is jump).
fn soft_interact_harvest_credit(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut practice: ResMut<LivingPracticeLoop>,
    mut moments: ResMut<ThrivingMoments>,
    soft_realm: Res<SoftPlayerRealm>,
    time: Res<Time>,
) {
    if !practice.active || practice.dismissed || practice.principle_sealed {
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::INTERACT) {
        return;
    }

    let player_realm = soft_realm.current;
    if !practice.allows_credit(player_realm) {
        let now = time.elapsed_seconds_f64();
        if now - practice.last_realm_mismatch_hint_at > 6.0 {
            practice.last_realm_mismatch_hint_at = now;
            if let Some(need) = practice.surface.realm_id() {
                info!(
                    target: "powrush::practice",
                    need_realm = need,
                    have = ?player_realm,
                    "Practice surface wants another climate — travel when ready"
                );
            }
        }
        return;
    }

    apply_practice_credit(
        &mut practice,
        &mut moments,
        time.elapsed_seconds_f64(),
        player_realm,
    );
}

fn bridge_rbe_feedback_to_practice(
    rbe_ui: Res<RbeUiSync>,
    mut practice: ResMut<LivingPracticeLoop>,
    mut moments: ResMut<ThrivingMoments>,
    soft_realm: Res<SoftPlayerRealm>,
    time: Res<Time>,
) {
    if !practice.active || practice.dismissed || practice.principle_sealed {
        return;
    }
    let Some(ref fb) = rbe_ui.last_harvest_feedback else {
        return;
    };
    if practice.last_bridged_feedback.as_ref() == Some(fb) {
        return;
    }

    let mercy_aligned = fb.contains("Sustainable")
        || fb.contains("mercy")
        || fb.contains("Mercy")
        || fb.contains("Epiphany")
        || fb.contains("harmony")
        || fb.contains("Council")
        || fb.contains("joy increased");

    if !mercy_aligned {
        return;
    }

    practice.last_bridged_feedback = Some(fb.clone());
    let now = time.elapsed_seconds_f64();
    let player_realm = soft_realm.current;
    if !practice.allows_credit(player_realm) {
        if now - practice.last_realm_mismatch_hint_at > 6.0 {
            practice.last_realm_mismatch_hint_at = now;
            info!(
                target: "powrush::practice",
                "Mercy harvest in another climate — travel to match practice surface"
            );
        }
        return;
    }
    apply_practice_credit(&mut practice, &mut moments, now, player_realm);
}

pub fn credit_practice_mercy_harvest(
    practice: &mut LivingPracticeLoop,
    moments: &mut ThrivingMoments,
    now_secs: f64,
    player_realm: Option<u8>,
) -> bool {
    apply_practice_credit(practice, moments, now_secs, player_realm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surfaces_progress_to_sealed() {
        let mut loop_ = LivingPracticeLoop::default();
        loop_.active = true;
        assert!(loop_.credit_mercy_harvest(1.0));
        assert!(loop_.credit_mercy_harvest(2.0));
        assert_eq!(loop_.surface, PracticeSurface::VerdantSurge);
        assert!(loop_.credit_mercy_harvest(3.0));
        assert!(loop_.credit_mercy_harvest(4.0));
        assert_eq!(loop_.surface, PracticeSurface::HorizonScarcity);
        assert!(loop_.credit_mercy_harvest(5.0));
        assert!(loop_.credit_mercy_harvest(6.0));
        assert!(loop_.principle_sealed);
    }

    #[test]
    fn realm_aware_blocks_mismatch() {
        let loop_ = LivingPracticeLoop {
            active: true,
            surface: PracticeSurface::SanctuaryCap,
            realm_aware: true,
            ..Default::default()
        };
        assert!(loop_.allows_credit(None));
        assert!(loop_.allows_credit(Some(0)));
        assert!(!loop_.allows_credit(Some(2)));
    }
}
