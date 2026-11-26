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
use crate::first_session_guidance::{
    signpost_showing, FirstSessionGuidance, GuidanceObjective, Signpost,
};
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
        let ready = guidance.dismissed && guidance.harvests_completed >= 1;
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
                crate::ui_frame::plate_frame(),
                BorderColor::all(TITLE_BORDER),
                Visibility::Hidden,
            ),
            LivingPracticeStrip,
            HudSlab(ID_PRACTICE),
        ))
        .with_children(|parent| {
            parent.spawn((
                (
Text::new(PracticeSurface::SanctuaryCap.prompt()),
TextFont { font_size: FontSize::Px(15.5 / 1.2), ..default() },
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
    signpost: Option<Res<Signpost>>,
    care: Option<Res<CareCycleOffer>>,
    epi: Option<Res<FirstHarvestEpiphany>>,
    nearby: Option<Res<NearbyMercyNode>>,
    bind: Option<Res<LivedHourBind>>,
    time: Res<Time>,
    mut query: Query<&mut Visibility, With<LivingPracticeStrip>>,
) {
    let guidance_showing = signpost
        .as_deref()
        .is_some_and(signpost_showing)
        || (guidance.active && !guidance.dismissed);
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
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
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
pub(crate) fn soft_interact_harvest_credit(
    keyboard: Res<ButtonInput<KeyCode>>,
    epiphany: Option<Res<FirstHarvestEpiphany>>,
    mut practice: ResMut<LivingPracticeLoop>,
    mut moments: ResMut<ThrivingMoments>,
    soft_realm: Res<SoftPlayerRealm>,
    time: Res<Time>,
    wood: Option<Res<crate::wood_expedition::WoodClaim>>,
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
    if wood.as_ref().is_some_and(|claim| claim.blocks()) {
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

    /// CARD PLAYTEST-HUD-COPY-1 — the strip's three colours (primary, gold flash,
    /// secondary) each ride on their own words, so no state is colour-only.
    #[test]
    fn practice_strip_colour_states_each_carry_words() {
        let mut loop_ = LivingPracticeLoop::default();
        loop_.active = true;
        let working = practice_strip_body(&loop_, 0.0);
        assert!(working.contains("Caps Across Climates"), "{working}");
        assert!(working.contains(&format!("(0/{})", loop_.harvests_needed)), "{working}");
        loop_.celebrate_until = 10.0;
        let flash = practice_strip_body(&loop_, 1.0);
        assert!(flash.starts_with("Surface cleared"), "{flash}");
        loop_.celebrate_until = 0.0;
        loop_.principle_sealed = true;
        loop_.surface = PracticeSurface::PrincipleSealed;
        let sealed = practice_strip_body(&loop_, 1.0);
        assert!(sealed.contains("same principle across three climates"), "{sealed}");
        assert_ne!(working, flash);
        assert_ne!(flash, sealed);
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
        assert_eq!(text.single(app.world()).unwrap().font_size, FontSize::Px(15.5 / 1.2));
    }

    #[test]
    fn hud_clean_hc2_offline_door_practice_and_signpost() {
        use crate::first_session_guidance::Signpost;

        let mut practice = LivingPracticeLoop::default();
        practice.try_activate_from_guidance(&FirstSessionGuidance {
            objective: GuidanceObjective::FreeExploration,
            active: true,
            dismissed: false,
            ..Default::default()
        });
        assert!(!practice.active);

        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .init_resource::<LivingPracticeLoop>()
            .init_resource::<FirstSessionGuidance>()
            .init_resource::<Signpost>()
            .init_resource::<FirstHarvestEpiphany>()
            .init_resource::<NearbyMercyNode>()
            .init_resource::<CareCycleOffer>()
            .insert_resource(crate::title_screen::LaunchDoor::InYard)
            .add_systems(Update, (handoff_from_first_session, update_practice_visibility));
        app.world_mut()
            .spawn((LivingPracticeStrip, Visibility::Hidden));
        {
            let mut guidance = app.world_mut().resource_mut::<FirstSessionGuidance>();
            guidance.active = true;
            guidance.dismissed = true;
            guidance.harvests_completed = 1;
            guidance.objective = GuidanceObjective::FreeExploration;
            let mut sign = app.world_mut().resource_mut::<Signpost>();
            sign.seen = false;
            sign.until = Some(120.0);
        }
        app.update();
        assert_eq!(
            *app.world_mut()
                .query_filtered::<&Visibility, With<LivingPracticeStrip>>()
                .single(app.world())
                .unwrap(),
            Visibility::Hidden
        );
    }

    #[test] // GHOST-TESTS-1: restore the attribute dropped by #1024.
    fn wood_claim_does_not_credit_practice() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .init_resource::<LivingPracticeLoop>()
            .init_resource::<ThrivingMoments>()
            .init_resource::<SoftPlayerRealm>()
            .init_resource::<ButtonInput<KeyCode>>()
            .insert_resource(crate::wood_expedition::WoodClaim::default())
            .add_systems(Update, soft_interact_harvest_credit);
        app.world_mut().resource_mut::<SoftPlayerRealm>().current = Some(0);
        app.world_mut().resource_mut::<LivingPracticeLoop>().active = true;
        app.world_mut().resource_mut::<crate::wood_expedition::WoodClaim>().set_held(true);
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(crate::soft_play_bindings::INTERACT);
        app.update();
        assert_eq!(app.world().resource::<LivingPracticeLoop>().mercy_harvests_on_surface, 0);
        app.world_mut().resource_mut::<crate::wood_expedition::WoodClaim>().set_held(false);
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().release(crate::soft_play_bindings::INTERACT);
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(crate::soft_play_bindings::INTERACT);
        app.update();
        assert_eq!(app.world().resource::<LivingPracticeLoop>().mercy_harvests_on_surface, 1);
    }
    fn hp2_ui(w: u32, h: u32, s: f32) -> App {
        let mut app = App::new(); app.add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default(), bevy::input::InputPlugin, bevy::text::TextPlugin, bevy::ui::UiPlugin, crate::ui_font::UiFontPlugin)).init_asset::<Image>().init_asset::<TextureAtlasLayout>().add_message::<bevy::picking::pointer::PointerInput>().add_message::<bevy::picking::backend::PointerHits>().init_resource::<bevy::picking::hover::HoverMap>().init_resource::<bevy::picking::events::PointerState>().add_systems(Startup, spawn_practice_strip);
        app.world_mut().spawn((Camera2d, bevy::ui::IsDefaultUiCamera, Camera { computed: bevy::camera::ComputedCameraValues { target_info: Some(bevy::camera::RenderTargetInfo { physical_size: UVec2::new(w, h), scale_factor: s }), ..default() }, ..default() })); app
    }
    fn hp2_root(app: &mut App) -> Entity { let w = app.world_mut(); w.query_filtered::<Entity, With<LivingPracticeStrip>>().single(w).unwrap() }
    #[test] fn hud_plate_hp2_a_practice_carries_sliced_frame() {
        let mut app = App::new(); app.add_plugins(MinimalPlugins).add_systems(Startup, spawn_practice_strip).update(); let root = hp2_root(&mut app);
        assert_eq!(app.world().get::<ImageNode>(root).cloned().map(|i| (i.image.id(), i.visual_box, i.image_mode)), Some((crate::ui_frame::FRAME_IMAGE.id(), VisualBox::PaddingBox, NodeImageMode::Sliced(crate::ui_frame::frame_slicer()))), "HP2-a practice frame missing");
    }
    #[test] fn hud_plate_hp2_b_practice_text_has_no_image() {
        let mut app = App::new(); app.add_plugins(MinimalPlugins).add_systems(Startup, spawn_practice_strip).update(); let w = app.world_mut();
        assert_eq!(w.query_filtered::<Option<&ImageNode>, With<LivingPracticeText>>().iter(w).map(|i| i.is_some()).collect::<Vec<_>>(), vec![false], "HP2-b practice");
    }
    #[test] fn hud_plate_hp2_c_practice_layout_matches_without_image() {
        for (w, h, s) in [(1280u32, 720u32, 1f32), (2560, 1440, 2.)] {
            let mut app = hp2_ui(w, h, s); (app.update(), app.update()); let root = hp2_root(&mut app);
            let at = |app: &App| (app.world().get::<ComputedNode>(root).unwrap().size(), app.world().get::<bevy::ui::UiGlobalTransform>(root).unwrap().translation); let framed = at(&app);
            app.world_mut().entity_mut(root).remove::<ImageNode>(); app.update(); assert_eq!(at(&app), framed, "HP2-c practice {w}x{h}");
        }
    }
    #[test] fn hud_plate_hp2_d_practice_corner_is_one_at_spawned_font() { let mut app = hp2_ui(1280, 720, 1.); (app.update(), app.update()); let root = hp2_root(&mut app); let n = app.world().get::<ComputedNode>(root).unwrap(); let pad = n.padding_box().size() * n.inverse_scale_factor;
        assert!(pad.x >= 24. && pad.y >= 24. && (pad / crate::ui_frame::FRAME_PX as f32).min_element().min(crate::ui_frame::frame_slicer().max_corner_scale) == 1., "HP2-d practice {pad}");
    }

    /// PRACTICE-NEQ-1 PN1. Real HudLayoutPlugin + LivingPracticeLoopPlugin (Classic, yard): a steady strip stays unmarked; R5/R2 yields still hide and churn, and restore wins (steps 5, 9).
    #[derive(Resource, Default)] struct PnTape(Vec<(Visibility, bool)>);
    fn pn_tape(q: Query<Ref<Visibility>, With<LivingPracticeStrip>>, mut t: ResMut<PnTape>) { t.0.extend(q.iter().map(|v| (*v, v.is_changed()))); }
    #[test]
    fn practice_neq_1_pn1_steady_strip_unmarked_yield_and_restore_still_win() {
        use crate::{hex_travel::PlacesPlate, hud_presets::{ActiveHudPreset, HudLayoutPlugin, HudPresetId, HudYielded}, title_screen::LaunchDoor};
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, HudLayoutPlugin, LivingPracticeLoopPlugin)).insert_resource(ActiveHudPreset { id: Some(HudPresetId::Classic) }).insert_resource(LaunchDoor::InYard).insert_resource(ButtonInput::<KeyCode>::default()).init_resource::<FirstSessionGuidance>().init_resource::<ThrivingMoments>().init_resource::<RbeUiSync>().init_resource::<PnTape>().add_systems(Last, pn_tape);
        { let mut g = app.world_mut().resource_mut::<FirstSessionGuidance>(); g.dismissed = true; g.harvests_completed = 1; }
        app.update(); app.update(); app.update(); app.world_mut().resource_mut::<PnTape>().0.clear();
        let vis = |app: &mut App| *app.world_mut().query_filtered::<&Visibility, With<LivingPracticeStrip>>().single(app.world()).unwrap();
        assert!(app.world().resource::<LivingPracticeLoop>().active && vis(&mut app) == Visibility::Visible, "PN1 warm-up: practice active through the real handoff, strip Visible");
        let (v, h) = (Visibility::Visible, Visibility::Hidden); let mut care = None;
        let want = [(v, false), (v, false), (h, true), (h, true), (v, true), (v, false), (h, true), (h, true), (v, true), (v, false), (h, true), (h, false), (h, false), (v, true), (v, false)];
        for (i, exp) in want.iter().enumerate() {
            match i + 1 {
                3 => { app.insert_resource(PlacesPlate::default()); app.world_mut().resource_mut::<PlacesPlate>().open = true; }
                5 => app.world_mut().resource_mut::<PlacesPlate>().open = false,
                7 => care = Some(app.world_mut().spawn((HudSlab(ID_CARE_STRIP), Visibility::Visible, Node::default())).id()),
                9 => { app.world_mut().despawn(care.unwrap()); }
                11 => app.world_mut().resource_mut::<LivingPracticeLoop>().dismissed = true,
                13 => app.world_mut().resource_mut::<PlacesPlate>().open = true,
                14 => { app.world_mut().resource_mut::<PlacesPlate>().open = false; app.world_mut().resource_mut::<LivingPracticeLoop>().dismissed = false; }
                _ => {}
            }
            app.update();
            assert_eq!(vis(&mut app), exp.0, "PN1 final Visibility, step {}", i + 1);
            match (i + 1, &app.world().resource::<HudYielded>().0) { (7, y) => assert!(y.contains(&ID_PRACTICE), "PN1 R2 yield, step 7"), (9, y) => assert!(y.is_empty(), "PN1 R2 released, step 9"), _ => {} }
        }
        assert_eq!(app.world().resource::<PnTape>().0, want, "PN1 tape (final Visibility, is_changed) per tick");
    }
}
