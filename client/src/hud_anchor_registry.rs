//! CARD HUD-ANCHOR-REGISTRY-1 — step 2a of the UI layout epic.
//!
//! Anchor records follow design §2.1. This card fills three of them, from §2.5:
//! `ACTION_BAR`, `VOICE`, and `ALLOCATE_DOCK`. No plugin: slabs read the
//! anchors when they spawn, and the R2 yield runs inside the slabs' own
//! visibility systems. Presets, push, cover, and edit mode stay later cards.

use bevy::ui::Val;

/// Design §2.1. Edge midpoints use `Centre` on the free axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudCorner {
    TopLeft,
    TopCentre,
    TopRight,
    BottomLeft,
    BottomCentre,
    BottomRight,
}

/// Px from the named edges. `x` is from the left or the right. `y` is from
/// the top or the bottom. Centred anchors leave `x` unused.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HudOffset {
    pub x: f32,
    pub y: f32,
}

/// Design §2.2. `Hud` is the map's default: no `ZIndex`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudZBand {
    Hud,
}

/// Design §2.1 share. This card uses `Solo` and `Yield`. `Push` is step 3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudShare {
    Solo,
    CodeExclusive,
    Yield,
    Push,
}

/// One slab on an anchor. Rank 1 is the highest rank. Class 1 is the highest class.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HudOccupant {
    pub id: &'static str,
    pub rank: u8,
    pub class: u8,
    pub width: f32,
    /// Model B height from design §2.3. Edges round when a rect is built.
    pub height_b: f32,
}

/// Design §2.1 anchor record.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HudAnchor {
    pub id: &'static str,
    pub corner: HudCorner,
    pub offset: HudOffset,
    /// Widest occupant's coded width. No occupant is resized.
    pub width: f32,
    /// Tallest occupant's Model B height, rounded up to a whole px.
    pub height_budget: f32,
    pub z_band: HudZBand,
    /// Highest-priority (lowest-numbered) occupant class.
    pub class: u8,
    pub occupants: &'static [HudOccupant],
    pub share: HudShare,
}

pub const ID_CARE_STRIP: &str = "CareStrip";
pub const ID_CARE_PROMPT: &str = "CarePrompt";
pub const ID_GUIDANCE: &str = "Guidance";
pub const ID_PRACTICE: &str = "Practice";
pub const ID_VOICE: &str = "Voice";
pub const ID_ALLOCATE: &str = "Allocate";

const ACTION_BAR_OCCUPANTS: [HudOccupant; 4] = [
    HudOccupant {
        id: ID_CARE_STRIP,
        rank: 1,
        class: 2,
        width: 560.0,
        height_b: 62.0,
    },
    HudOccupant {
        id: ID_CARE_PROMPT,
        rank: 2,
        class: 2,
        width: 460.0,
        height_b: 56.4,
    },
    HudOccupant {
        id: ID_GUIDANCE,
        rank: 3,
        class: 3,
        width: 520.0,
        height_b: 69.0,
    },
    HudOccupant {
        id: ID_PRACTICE,
        rank: 4,
        class: 3,
        width: 640.0,
        height_b: 64.0,
    },
];

const VOICE_OCCUPANTS: [HudOccupant; 1] = [HudOccupant {
    id: ID_VOICE,
    rank: 1,
    class: 1,
    width: 560.0,
    height_b: 52.0,
}];

const ALLOCATE_DOCK_OCCUPANTS: [HudOccupant; 1] = [HudOccupant {
    id: ID_ALLOCATE,
    rank: 1,
    class: 1,
    width: 520.0,
    height_b: 135.0,
}];

/// Design §2.5. Bottom 144, right 16. CareStrip, CarePrompt, Guidance, Practice.
pub const ACTION_BAR: HudAnchor = HudAnchor {
    id: "ACTION_BAR",
    corner: HudCorner::BottomRight,
    offset: HudOffset { x: 16.0, y: 144.0 },
    width: 640.0,
    height_budget: 69.0,
    z_band: HudZBand::Hud,
    class: 2,
    occupants: &ACTION_BAR_OCCUPANTS,
    share: HudShare::Yield,
};

/// Design §2.5. Bottom 221, right 16. Voice.
pub const VOICE: HudAnchor = HudAnchor {
    id: "VOICE",
    corner: HudCorner::BottomRight,
    offset: HudOffset { x: 16.0, y: 221.0 },
    width: 560.0,
    height_budget: 52.0,
    z_band: HudZBand::Hud,
    class: 1,
    occupants: &VOICE_OCCUPANTS,
    share: HudShare::Solo,
};

/// Design §2.5. Bottom 281, right 16. Allocate. Not the classic `WINDOW` slot.
pub const ALLOCATE_DOCK: HudAnchor = HudAnchor {
    id: "ALLOCATE_DOCK",
    corner: HudCorner::BottomRight,
    offset: HudOffset { x: 16.0, y: 281.0 },
    width: 520.0,
    height_budget: 135.0,
    z_band: HudZBand::Hud,
    class: 1,
    occupants: &ALLOCATE_DOCK_OCCUPANTS,
    share: HudShare::Solo,
};

pub const ANCHORS: &[HudAnchor] = &[ACTION_BAR, VOICE, ALLOCATE_DOCK];

impl HudAnchor {
    pub fn occupant(&self, id: &str) -> &HudOccupant {
        self.occupants
            .iter()
            .find(|occupant| occupant.id == id)
            .unwrap_or_else(|| panic!("hud anchor {} has no occupant {id}", self.id))
    }

    /// Bottom edge offset. These three anchors are bottom-right.
    pub const fn bottom(self) -> Val {
        Val::Px(self.offset.y)
    }

    /// Right edge offset. These three anchors are bottom-right.
    pub const fn right(self) -> Val {
        Val::Px(self.offset.x)
    }

    pub fn occupant_rect(&self, id: &str, view_w: f32, view_h: f32) -> HudRect {
        let occupant = self.occupant(id);
        assert_eq!(
            self.corner,
            HudCorner::BottomRight,
            "occupant_rect is the step 2a bottom-right dock"
        );
        slab_rect(
            SlabPlace::BottomRight {
                bottom: self.offset.y,
                right: self.offset.x,
            },
            occupant.width,
            occupant.height_b,
            view_w,
            view_h,
        )
    }
}

/// R2 (design §2.3). True when `occupant` hides because a higher-priority
/// occupant on the same anchor is showing. Lower class number wins, then
/// lower rank. The caller changes visibility only.
pub fn r2_yields_to(anchor: &HudAnchor, occupant: &str, showing: &[(&str, bool)]) -> bool {
    let me = anchor.occupant(occupant);
    showing.iter().any(|(id, on)| {
        if !on {
            return false;
        }
        let other = anchor.occupant(id);
        other.class < me.class || (other.class == me.class && other.rank < me.rank)
    })
}

/// Window rectangle in px, origin top-left. `x1` / `y1` are the exclusive edges.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HudRect {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

impl HudRect {
    /// Shared area. Zero when the rectangles only touch an edge.
    pub fn overlap_area(self, other: HudRect) -> i32 {
        let width = (self.x1.min(other.x1) - self.x0.max(other.x0)).max(0);
        let height = (self.y1.min(other.y1) - self.y0.max(other.y0)).max(0);
        width.saturating_mul(height)
    }
}

/// Coded placement for the overlap helper. Percent fields are fractions.
#[derive(Clone, Copy, Debug)]
pub enum SlabPlace {
    BottomRight { bottom: f32, right: f32 },
    BottomLeft { bottom: f32, left: f32 },
    TopRight { top: f32, right: f32 },
    TopCentre { top: f32, margin_left: f32 },
    BottomPercentLeft { bottom_fraction: f32, left: f32 },
    TopPercentRightPercent { top_fraction: f32, right_fraction: f32 },
    TopPercentCentre { top_fraction: f32, margin_left: f32 },
}

fn round_px(value: f32) -> i32 {
    value.round() as i32
}

/// Design §1.4. Each edge is rounded to a whole pixel on its own.
pub fn slab_rect(place: SlabPlace, width: f32, height: f32, view_w: f32, view_h: f32) -> HudRect {
    let (x0, y0, x1, y1) = match place {
        SlabPlace::BottomRight { bottom, right } => {
            let x1 = view_w - right;
            let y1 = view_h - bottom;
            (x1 - width, y1 - height, x1, y1)
        }
        SlabPlace::BottomLeft { bottom, left } => {
            let y1 = view_h - bottom;
            (left, y1 - height, left + width, y1)
        }
        SlabPlace::TopRight { top, right } => {
            let x1 = view_w - right;
            (x1 - width, top, x1, top + height)
        }
        SlabPlace::TopCentre { top, margin_left } => {
            let x0 = view_w * 0.5 + margin_left;
            (x0, top, x0 + width, top + height)
        }
        SlabPlace::BottomPercentLeft {
            bottom_fraction,
            left,
        } => {
            let y1 = view_h - view_h * bottom_fraction;
            (left, y1 - height, left + width, y1)
        }
        SlabPlace::TopPercentRightPercent {
            top_fraction,
            right_fraction,
        } => {
            let top = view_h * top_fraction;
            let x1 = view_w - view_w * right_fraction;
            (x1 - width, top, x1, top + height)
        }
        SlabPlace::TopPercentCentre {
            top_fraction,
            margin_left,
        } => {
            let top = view_h * top_fraction;
            let x0 = view_w * 0.5 + margin_left;
            (x0, top, x0 + width, top + height)
        }
    };
    HudRect {
        x0: round_px(x0),
        y0: round_px(y0),
        x1: round_px(x1),
        y1: round_px(y1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    use crate::first_harvest_epiphany::FirstHarvestEpiphany;
    use crate::first_session_guidance::{
        handle_guidance_dismiss_input, update_guidance_visibility, FirstSessionGuidance,
        FirstSessionGuidanceStrip,
    };
    use crate::living_practice_loop::{
        handle_practice_toggle, update_practice_visibility, LivingPracticeLoop, LivingPracticeStrip,
    };
    use crate::mercy_harvest_nodes::{CareCycleOffer, NearbyMercyNode};
    use crate::title_screen::LaunchDoor;

    #[test]
    fn anchor_numbers_match_design_2_5() {
        assert_eq!(
            ACTION_BAR.offset,
            HudOffset { x: 16.0, y: 144.0 },
            "ACTION_BAR bottom 144, right 16"
        );
        assert_eq!(
            VOICE.offset,
            HudOffset { x: 16.0, y: 221.0 },
            "VOICE bottom 221, right 16"
        );
        assert_eq!(
            ALLOCATE_DOCK.offset,
            HudOffset { x: 16.0, y: 281.0 },
            "ALLOCATE_DOCK bottom 281, right 16"
        );
        assert_eq!(ACTION_BAR.bottom(), Val::Px(144.0));
        assert_eq!(ACTION_BAR.right(), Val::Px(16.0));
        assert_eq!(VOICE.bottom(), Val::Px(221.0));
        assert_eq!(VOICE.right(), Val::Px(16.0));
        assert_eq!(ALLOCATE_DOCK.bottom(), Val::Px(281.0));
        assert_eq!(ALLOCATE_DOCK.right(), Val::Px(16.0));

        for anchor in ANCHORS {
            assert_eq!(anchor.corner, HudCorner::BottomRight);
            assert_eq!(anchor.z_band, HudZBand::Hud);
            let widest = anchor
                .occupants
                .iter()
                .map(|occupant| occupant.width)
                .fold(0.0, f32::max);
            let tallest = anchor
                .occupants
                .iter()
                .map(|occupant| occupant.height_b.ceil())
                .fold(0.0, f32::max);
            let highest_class = anchor
                .occupants
                .iter()
                .map(|occupant| occupant.class)
                .min()
                .expect("occupants");
            assert_eq!(anchor.width, widest, "{}", anchor.id);
            assert_eq!(anchor.height_budget, tallest, "{}", anchor.id);
            assert_eq!(anchor.class, highest_class, "{}", anchor.id);
        }

        assert_eq!(ACTION_BAR.width, 640.0);
        assert_eq!(ACTION_BAR.height_budget, 69.0);
        assert_eq!(ACTION_BAR.class, 2);
        assert_eq!(ACTION_BAR.share, HudShare::Yield);
        assert_eq!(
            ACTION_BAR
                .occupants
                .iter()
                .map(|occupant| (occupant.id, occupant.rank, occupant.class))
                .collect::<Vec<_>>(),
            vec![
                (ID_CARE_STRIP, 1, 2),
                (ID_CARE_PROMPT, 2, 2),
                (ID_GUIDANCE, 3, 3),
                (ID_PRACTICE, 4, 3),
            ]
        );
        assert_eq!(ACTION_BAR.occupant(ID_CARE_STRIP).width, 560.0);
        assert_eq!(ACTION_BAR.occupant(ID_CARE_STRIP).height_b, 62.0);
        assert_eq!(ACTION_BAR.occupant(ID_CARE_PROMPT).width, 460.0);
        assert_eq!(ACTION_BAR.occupant(ID_CARE_PROMPT).height_b, 56.4);
        assert_eq!(ACTION_BAR.occupant(ID_GUIDANCE).width, 520.0);
        assert_eq!(ACTION_BAR.occupant(ID_GUIDANCE).height_b, 69.0);
        assert_eq!(ACTION_BAR.occupant(ID_PRACTICE).width, 640.0);
        assert_eq!(ACTION_BAR.occupant(ID_PRACTICE).height_b, 64.0);

        assert_eq!(VOICE.width, 560.0);
        assert_eq!(VOICE.height_budget, 52.0);
        assert_eq!(VOICE.class, 1);
        assert_eq!(VOICE.share, HudShare::Solo);
        assert_eq!(VOICE.occupant(ID_VOICE).height_b, 52.0);

        assert_eq!(ALLOCATE_DOCK.width, 520.0);
        assert_eq!(ALLOCATE_DOCK.height_budget, 135.0);
        assert_eq!(ALLOCATE_DOCK.class, 1);
        assert_eq!(ALLOCATE_DOCK.share, HudShare::Solo);
        assert_eq!(ALLOCATE_DOCK.occupant(ID_ALLOCATE).height_b, 135.0);
    }

    #[test]
    fn six_moves_land_on_design_2_5_rects() {
        let cases = [
            (
                ID_GUIDANCE,
                &ACTION_BAR,
                rect(488, 427, 1008, 496),
                rect(744, 587, 1264, 656),
            ),
            (
                ID_PRACTICE,
                &ACTION_BAR,
                rect(368, 432, 1008, 496),
                rect(624, 592, 1264, 656),
            ),
            (
                ID_CARE_STRIP,
                &ACTION_BAR,
                rect(448, 434, 1008, 496),
                rect(704, 594, 1264, 656),
            ),
            (
                ID_CARE_PROMPT,
                &ACTION_BAR,
                rect(548, 440, 1008, 496),
                rect(804, 600, 1264, 656),
            ),
            (
                ID_VOICE,
                &VOICE,
                rect(448, 367, 1008, 419),
                rect(704, 527, 1264, 579),
            ),
            (
                ID_ALLOCATE,
                &ALLOCATE_DOCK,
                rect(488, 224, 1008, 359),
                rect(744, 384, 1264, 519),
            ),
        ];
        for (id, anchor, at_1024, at_1280) in cases {
            assert_eq!(
                anchor.occupant_rect(id, 1024.0, 640.0),
                at_1024,
                "{id} at 1024x640"
            );
            assert_eq!(
                anchor.occupant_rect(id, 1280.0, 800.0),
                at_1280,
                "{id} at 1280x800"
            );
        }
    }

    #[test]
    fn banked_pairs_model_b_overlap_zero_both_sizes() {
        for (view_w, view_h) in [(1024.0, 640.0), (1280.0, 800.0)] {
            let area = |a: &str, b: &str| model_b(a, view_w, view_h).overlap_area(model_b(b, view_w, view_h));
            for (left, right) in [
                (ID_VOICE, ID_CARE_STRIP),
                (ID_VOICE, ID_CARE_PROMPT),
                (ID_GUIDANCE, "Ledger"),
                (ID_PRACTICE, "Ledger"),
                (ID_ALLOCATE, ID_CARE_STRIP),
                (ID_ALLOCATE, ID_CARE_PROMPT),
                (ID_ALLOCATE, ID_VOICE),
                (ID_ALLOCATE, "Well"),
                (ID_ALLOCATE, "ClimateState"),
                (ID_ALLOCATE, "Satchel"),
                (ID_ALLOCATE, "Compass"),
                (ID_ALLOCATE, ID_GUIDANCE),
            ] {
                assert_eq!(
                    area(left, right),
                    0,
                    "{left} × {right} at {view_w}x{view_h}"
                );
            }
        }

        // Six pairs §2.5 adds at 1024x640 Model B. Accepted; step 3 folds them.
        let added_1024 = [
            (ID_ALLOCATE, "Hybrid", 21840),
            (ID_ALLOCATE, "Pickup", 11628),
            (ID_ALLOCATE, "Redemption", 13440),
            (ID_ALLOCATE, "Whisper", 5616),
            ("ClimateState", ID_PRACTICE, 2176),
            ("Mercy", ID_VOICE, 6120),
        ];
        for (left, right, want) in added_1024 {
            assert_eq!(
                model_b(left, 1024.0, 640.0).overlap_area(model_b(right, 1024.0, 640.0)),
                want,
                "{left} × {right} stays at 1024"
            );
        }
        // §2.5 also adds these two at 1280x800 Model B.
        assert_eq!(
            model_b(ID_ALLOCATE, 1280.0, 800.0).overlap_area(model_b("Mercy", 1280.0, 800.0)),
            5760
        );
        assert_eq!(
            model_b("Places", 1280.0, 800.0).overlap_area(model_b(ID_VOICE, 1280.0, 800.0)),
            2312
        );
        // Named under Model A in the banked table. The move does not clear it.
        assert_eq!(
            model_b(ID_ALLOCATE, 1024.0, 640.0).overlap_area(model_b("Mercy", 1024.0, 640.0)),
            48600
        );
    }

    #[test]
    fn action_bar_yield_hides_guidance_and_practice_state_untouched() {
        assert!(r2_yields_to(
            &ACTION_BAR,
            ID_GUIDANCE,
            &[(ID_CARE_STRIP, true), (ID_CARE_PROMPT, false)]
        ));
        assert!(r2_yields_to(
            &ACTION_BAR,
            ID_PRACTICE,
            &[(ID_CARE_STRIP, false), (ID_CARE_PROMPT, true)]
        ));
        assert!(!r2_yields_to(
            &ACTION_BAR,
            ID_GUIDANCE,
            &[(ID_CARE_STRIP, false), (ID_CARE_PROMPT, false)]
        ));
        assert!(!r2_yields_to(
            &ACTION_BAR,
            ID_CARE_STRIP,
            &[(ID_CARE_PROMPT, true)]
        ));

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<FirstSessionGuidance>()
            .init_resource::<LivingPracticeLoop>()
            .init_resource::<FirstHarvestEpiphany>()
            .init_resource::<NearbyMercyNode>()
            .init_resource::<CareCycleOffer>()
            .insert_resource(LaunchDoor::InYard)
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(
                Update,
                (
                    handle_guidance_dismiss_input,
                    handle_practice_toggle,
                    update_guidance_visibility,
                    update_practice_visibility,
                ),
            );
        app.world_mut()
            .spawn((FirstSessionGuidanceStrip, Visibility::Hidden));
        app.world_mut()
            .spawn((LivingPracticeStrip, Visibility::Hidden));

        {
            let mut guidance = app.world_mut().resource_mut::<FirstSessionGuidance>();
            guidance.active = true;
            guidance.dismissed = false;
            guidance.shown_at_seconds = 12.5;
            let mut practice = app.world_mut().resource_mut::<LivingPracticeLoop>();
            practice.active = true;
            practice.dismissed = false;
            practice.celebrate_until = 40.0;
        }
        quiet_prompts(&mut app);

        let guidance_stamp = stamp_guidance(&app);
        let practice_stamp = stamp_practice(&app);
        app.update();
        assert_eq!(strip_vis::<FirstSessionGuidanceStrip>(&mut app), Visibility::Visible);
        assert_eq!(strip_vis::<LivingPracticeStrip>(&mut app), Visibility::Hidden);
        assert_eq!(stamp_guidance(&app), guidance_stamp);
        assert_eq!(stamp_practice(&app), practice_stamp);

        app.world_mut().resource_mut::<CareCycleOffer>().active = true;
        show_prompt_gate(&mut app);
        app.update();
        assert_eq!(strip_vis::<FirstSessionGuidanceStrip>(&mut app), Visibility::Hidden);
        assert_eq!(strip_vis::<LivingPracticeStrip>(&mut app), Visibility::Hidden);
        assert_eq!(stamp_guidance(&app), guidance_stamp);
        assert_eq!(stamp_practice(&app), practice_stamp);

        app.world_mut().resource_mut::<CareCycleOffer>().active = false;
        app.update();
        assert_eq!(strip_vis::<FirstSessionGuidanceStrip>(&mut app), Visibility::Hidden);
        assert_eq!(strip_vis::<LivingPracticeStrip>(&mut app), Visibility::Hidden);
        assert_eq!(stamp_guidance(&app), guidance_stamp);
        assert_eq!(stamp_practice(&app), practice_stamp);

        quiet_prompts(&mut app);
        app.world_mut()
            .resource_mut::<FirstSessionGuidance>()
            .active = false;
        let guidance_quiet = stamp_guidance(&app);
        let practice_quiet = stamp_practice(&app);
        app.update();
        assert_eq!(strip_vis::<FirstSessionGuidanceStrip>(&mut app), Visibility::Hidden);
        assert_eq!(strip_vis::<LivingPracticeStrip>(&mut app), Visibility::Visible);
        assert_eq!(stamp_guidance(&app), guidance_quiet);
        assert_eq!(stamp_practice(&app), practice_quiet);

        app.world_mut().resource_mut::<CareCycleOffer>().active = true;
        app.update();
        assert_eq!(strip_vis::<LivingPracticeStrip>(&mut app), Visibility::Hidden);
        assert_eq!(stamp_guidance(&app), guidance_quiet);
        assert_eq!(stamp_practice(&app), practice_quiet);

        app.world_mut().resource_mut::<CareCycleOffer>().active = false;
        show_prompt_gate(&mut app);
        app.update();
        assert_eq!(strip_vis::<LivingPracticeStrip>(&mut app), Visibility::Hidden);
        assert_eq!(stamp_practice(&app), practice_quiet);
        assert_eq!(app.world().resource::<LivingPracticeLoop>().celebrate_until, 40.0);
        assert_eq!(app.world().resource::<FirstSessionGuidance>().shown_at_seconds, 12.5);
        assert!(app.world().resource::<LivingPracticeLoop>().active);
        assert!(!app.world().resource::<FirstSessionGuidance>().dismissed);

        // P still toggles while the strip is hidden. The timer stays.
        press(&mut app, KeyCode::KeyP);
        assert!(!app.world().resource::<LivingPracticeLoop>().active);
        assert_eq!(app.world().resource::<LivingPracticeLoop>().celebrate_until, 40.0);
        assert_eq!(strip_vis::<LivingPracticeStrip>(&mut app), Visibility::Hidden);

        // H still dismisses while Guidance is hidden by the care strip.
        {
            let mut guidance = app.world_mut().resource_mut::<FirstSessionGuidance>();
            guidance.active = true;
            guidance.dismissed = false;
        }
        app.world_mut().resource_mut::<CareCycleOffer>().active = true;
        press(&mut app, KeyCode::KeyH);
        let guidance = app.world().resource::<FirstSessionGuidance>();
        assert!(guidance.dismissed);
        assert!(!guidance.active);
        assert_eq!(guidance.shown_at_seconds, 12.5);
        assert_eq!(strip_vis::<FirstSessionGuidanceStrip>(&mut app), Visibility::Hidden);
    }

    fn press(app: &mut App, key: KeyCode) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(key);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();
    }

    fn rect(x0: i32, y0: i32, x1: i32, y1: i32) -> HudRect {
        HudRect { x0, y0, x1, y1 }
    }

    fn model_b(id: &str, view_w: f32, view_h: f32) -> HudRect {
        match id {
            ID_GUIDANCE | ID_PRACTICE | ID_CARE_STRIP | ID_CARE_PROMPT => {
                ACTION_BAR.occupant_rect(id, view_w, view_h)
            }
            ID_VOICE => VOICE.occupant_rect(id, view_w, view_h),
            ID_ALLOCATE => ALLOCATE_DOCK.occupant_rect(id, view_w, view_h),
            "Ledger" => slab_rect(
                SlabPlace::BottomLeft { bottom: 16.0, left: 16.0 },
                560.0,
                119.0,
                view_w,
                view_h,
            ),
            "Well" => slab_rect(
                SlabPlace::BottomLeft { bottom: 132.0, left: 16.0 },
                420.0,
                52.0,
                view_w,
                view_h,
            ),
            "ClimateState" => slab_rect(
                SlabPlace::BottomLeft { bottom: 176.0, left: 16.0 },
                420.0,
                52.0,
                view_w,
                view_h,
            ),
            "Satchel" => slab_rect(
                SlabPlace::BottomPercentLeft {
                    bottom_fraction: 0.22,
                    left: 16.0,
                },
                300.0,
                235.0,
                view_w,
                view_h,
            ),
            "Compass" => slab_rect(
                SlabPlace::BottomRight { bottom: 92.0, right: 16.0 },
                420.0,
                52.0,
                view_w,
                view_h,
            ),
            "Hybrid" => slab_rect(
                SlabPlace::TopRight { top: 244.0, right: 16.0 },
                420.0,
                52.0,
                view_w,
                view_h,
            ),
            "Redemption" => slab_rect(
                SlabPlace::TopRight { top: 204.0, right: 16.0 },
                420.0,
                52.0,
                view_w,
                view_h,
            ),
            "Pickup" => slab_rect(
                SlabPlace::TopPercentCentre {
                    top_fraction: 0.38,
                    margin_left: -180.0,
                },
                360.0,
                56.4,
                view_w,
                view_h,
            ),
            "Whisper" => slab_rect(
                SlabPlace::TopPercentCentre {
                    top_fraction: 0.28,
                    margin_left: -210.0,
                },
                420.0,
                69.0,
                view_w,
                view_h,
            ),
            "Mercy" => slab_rect(
                SlabPlace::TopPercentRightPercent {
                    top_fraction: 0.10,
                    right_fraction: 0.02,
                },
                360.0,
                320.0,
                view_w,
                view_h,
            ),
            "Places" => slab_rect(
                SlabPlace::TopPercentCentre {
                    top_fraction: 0.18,
                    margin_left: -200.0,
                },
                400.0,
                400.0,
                view_w,
                view_h,
            ),
            _ => panic!("no model B rect for {id}"),
        }
    }

    fn quiet_prompts(app: &mut App) {
        {
            let mut epi = app.world_mut().resource_mut::<FirstHarvestEpiphany>();
            epi.first_harvest_lived = true;
            epi.prompt_until = -1.0;
        }
        let mut nearby = app.world_mut().resource_mut::<NearbyMercyNode>();
        nearby.in_range = false;
        nearby.nodes_exist = false;
        app.world_mut().resource_mut::<CareCycleOffer>().active = false;
    }

    fn show_prompt_gate(app: &mut App) {
        let mut nearby = app.world_mut().resource_mut::<NearbyMercyNode>();
        nearby.in_range = true;
        nearby.nodes_exist = true;
    }

    fn stamp_guidance(app: &App) -> (bool, bool, f64, u32) {
        let guidance = app.world().resource::<FirstSessionGuidance>();
        (
            guidance.active,
            guidance.dismissed,
            guidance.shown_at_seconds,
            guidance.harvests_completed,
        )
    }

    fn stamp_practice(app: &App) -> (bool, bool, f64, u32) {
        let practice = app.world().resource::<LivingPracticeLoop>();
        (
            practice.active,
            practice.dismissed,
            practice.celebrate_until,
            practice.mercy_harvests_on_surface,
        )
    }

    fn strip_vis<T: Component>(app: &mut App) -> Visibility {
        let mut query = app.world_mut().query_filtered::<&Visibility, With<T>>();
        *query
            .iter(app.world())
            .next()
            .expect("strip")
    }
}
