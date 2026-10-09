/*!
 * Living Practice Loop — Human playability layer (post first-session)
 * v21.97.0 — Soft harvest on **E** (interact), never Space (jump)
 *
 * Soft interact remains demo path. Real harvest feedback credits practice.
 *
 * AG-SML v1.0 | Contact: info@Rathor.ai | Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hud_anchor_registry::{
    action_bar_prompts_showing, r2_yields_to, HudSlab, ACTION_BAR, ID_CARE_PROMPT, ID_CARE_STRIP,
    ID_PRACTICE,
};
use crate::lived_hour_bind::LivedHourBind;
use crate::mercy_harvest_nodes::{CareCycleOffer, NearbyMercyNode};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY, TITLE_TEXT_SECONDARY};
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
                "Caps Across Climates · Verdant: abundance is flooding — take only what the node can spare (E)"
            }
            PracticeSurface::HorizonScarcity => {
                "Caps Across Climates · Horizon: sparse yields — choose carefully under uncertainty (E)"
            }
            PracticeSurface::PrincipleSealed => {
                "You carried the same principle across three climates. Sovereign exploration continues."
            }
        }
    }

    /// Yard line under the three-climate seal. The seal sentence stays in `prompt`.
    pub fn after_seal(&self) -> Option<&'static str> {
        match self {
            PracticeSurface::PrincipleSealed => Some("E tend the well · the principle holds"),
            PracticeSurface::SanctuaryCap
            | PracticeSurface::VerdantSurge
            | PracticeSurface::HorizonScarcity => None,
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
                    update_practice_visibility
                        .after(crate::first_harvest_epiphany::update_world_care_prompt)
                        .after(crate::mercy_harvest_nodes::update_care_cycle_strip),
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
            (
                Node {
                    position_type: PositionType::Absolute,
                    bottom: ACTION_BAR.bottom(),
                    right: ACTION_BAR.right(),
                    width: Val::Px(640.0),
                    padding: UiRect::axes(Val::Px(18.0), Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.5)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG),
                BorderColor(TITLE_BORDER),
                Visibility::Hidden,
            ),
            LivingPracticeStrip,
            HudSlab(ID_PRACTICE),
        ))
        .with_children(|parent| {
            parent.spawn((
                (
Text::new(PracticeSurface::SanctuaryCap.prompt()),
TextFont { font_size: 15.5 / 1.2, ..default() },
TextColor(TITLE_TEXT_PRIMARY),
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

pub(crate) fn handle_practice_toggle(
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

pub(crate) fn update_practice_visibility(
    practice: Res<LivingPracticeLoop>,
    guidance: Res<FirstSessionGuidance>,
    care: Option<Res<CareCycleOffer>>,
    epi: Option<Res<FirstHarvestEpiphany>>,
    nearby: Option<Res<NearbyMercyNode>>,
    bind: Option<Res<LivedHourBind>>,
    time: Res<Time>,
    mut query: Query<&mut Visibility, With<LivingPracticeStrip>>,
) {
    let guidance_showing = guidance.active && !guidance.dismissed;
    let guidance_hidden = bind.as_ref().is_some_and(|bind| bind.guidance_hidden);
    let (care_strip, care_prompt) = action_bar_prompts_showing(
        care.as_deref(),
        epi.as_deref(),
        nearby.as_deref(),
        &guidance,
        time.elapsed_secs_f64(),
        guidance_hidden,
    );
    let show = practice.active && !practice.dismissed && !guidance_showing
        && !r2_yields_to(
            &ACTION_BAR,
            ID_PRACTICE,
            &[(ID_CARE_STRIP, care_strip), (ID_CARE_PROMPT, care_prompt)],
        );
    for mut vis in &mut query {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Strip copy. After the seal, the next line names the well. No fourth climate.
fn practice_strip_body(practice: &LivingPracticeLoop, now: f64) -> String {
    let celebrating = now < practice.celebrate_until;
    if practice.principle_sealed {
        let seal = practice.surface.prompt();
        match practice.surface.after_seal() {
            Some(well) => format!("{seal}\n{well}"),
            None => seal.to_string(),
        }
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
    }
}

fn update_practice_text(
    practice: Res<LivingPracticeLoop>,
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut TextColor), With<LivingPracticeText>>,
) {
    if !practice.is_changed() && practice.celebrate_until <= 0.0 {
        return;
    }
    let now = time.elapsed_secs_f64();
    let celebrating = now < practice.celebrate_until;
    let body = practice_strip_body(&practice, now);
    for (mut text, mut color) in &mut query {
        **text = body.clone();
        color.0 = if celebrating {
            // state: celebrate flash — warm gold, one breath, then back to the palette.
            Color::srgb(1.0, 0.95, 0.55)
        } else if practice.principle_sealed {
            TITLE_TEXT_SECONDARY
        } else {
            TITLE_TEXT_PRIMARY
        };
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
    epiphany: Option<Res<FirstHarvestEpiphany>>,
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
    // A door already claimed this Use, so it took nothing — crediting it here
    // would speak the harvest line over someone else's verb.
    if epiphany.is_some_and(|e| e.harvest_use_is_claimed()) {
        return;
    }

    let player_realm = soft_realm.current;
    if !practice.allows_credit(player_realm) {
        let now = time.elapsed_secs_f64();
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
        time.elapsed_secs_f64(),
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
    let now = time.elapsed_secs_f64();
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
        assert_eq!(loop_.surface.next(), PracticeSurface::PrincipleSealed);
    }

    #[test]
    fn after_seal_line_names_the_well_and_keeps_the_seal() {
        let seal =
            "You carried the same principle across three climates. Sovereign exploration continues.";
        assert_eq!(PracticeSurface::PrincipleSealed.prompt(), seal);
        assert_eq!(
            PracticeSurface::PrincipleSealed.after_seal(),
            Some("E tend the well · the principle holds")
        );
        assert!(PracticeSurface::SanctuaryCap.after_seal().is_none());
        assert!(PracticeSurface::VerdantSurge.after_seal().is_none());
        assert!(PracticeSurface::HorizonScarcity.after_seal().is_none());

        let mut loop_ = LivingPracticeLoop::default();
        loop_.active = true;
        for t in 1..=6 {
            assert!(loop_.credit_mercy_harvest(t as f64));
        }
        assert!(loop_.principle_sealed);
        assert_eq!(loop_.surface, PracticeSurface::PrincipleSealed);
        let body = practice_strip_body(&loop_, 6.0);
        assert_eq!(
            body,
            format!("{seal}\nE tend the well · the principle holds")
        );
        assert!(!body.contains("next climate"));
    }

    #[test]
    fn verdant_surge_prompt_ends_with_interact_and_not_allocate() {
        let prompt = PracticeSurface::VerdantSurge.prompt();
        assert!(prompt.ends_with("(E)"));
        assert!(!prompt.contains("allocate"));
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

    /// CARD HUD-ANCHOR-REGISTRY-1 — Practice sits on ACTION_BAR.
    #[test]
    fn practice_strip_lands_on_action_bar() {
        use crate::hud_anchor_registry::{ACTION_BAR, ID_PRACTICE};

        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_practice_strip);
        app.update();
        let mut query = app
            .world_mut()
            .query_filtered::<&Node, With<LivingPracticeStrip>>();
        let style = query.single(app.world()).unwrap().clone();
        assert_eq!(style.bottom, ACTION_BAR.bottom());
        assert_eq!(style.right, ACTION_BAR.right());
        assert_eq!(style.left, Val::Auto);
        assert_eq!(style.width, Val::Px(ACTION_BAR.occupant(ID_PRACTICE).width));
        assert_eq!(style.margin, UiRect::default());
        assert_eq!(style.padding, UiRect::axes(Val::Px(18.0), Val::Px(12.0)));
        assert_eq!(style.border, UiRect::all(Val::Px(1.5)));
        let mut text = app
            .world_mut()
            .query_filtered::<&TextFont, With<LivingPracticeText>>();
        assert_eq!(text.single(app.world()).unwrap().font_size, 15.5 / 1.2);
    }
}
