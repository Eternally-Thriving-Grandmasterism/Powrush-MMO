//! CARD HUD-ANCHOR-REGISTRY-1 — step 2a of the UI layout epic.
//! CARD HUD-ANCHOR-REGISTRY-2B — the 19 as-is joiners from design §2.4,
//! plus rows 16 and 17 (Redemption, Hybrid) as position-only joiners.
//!
//! Anchor records follow design §2.1. Step 2a fills three of them, from §2.5:
//! `ACTION_BAR`, `VOICE`, and `ALLOCATE_DOCK`. Step 2b moves each as-is
//! joiner's coded position into [`HudCodedPlace`]. Spawn sites read those
//! Vals. Each centred joiner keeps its own `margin_left`. No plugin: slabs
//! read the anchors when they spawn, and the R2 yield runs inside the slabs'
//! own visibility systems. Preset tables and the R3, R4, and R5 predicates live
//! in `hud_presets`. `HudLayoutPlugin` applies them only while a preset is active.

use bevy::prelude::{Component, Mut};
use bevy::ui::{Node, UiRect, Val};

/// Marker on a HUD slab root. The string is the slab id (`Factory`, `Voice`).
/// The marker does not write `Style` or `Visibility`.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct HudSlab(pub &'static str);

use crate::first_harvest_epiphany::{world_care_prompt_visible, FirstHarvestEpiphany};
use crate::first_session_guidance::{FirstSessionGuidance, GuidanceObjective};
use crate::mercy_harvest_nodes::{CareCycleOffer, NearbyMercyNode};

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

/// Design §2.1 share.
/// `Solo` is one occupant.
/// `CodeExclusive` is R1: the code already keeps the pair apart.
/// `Yield` is R2: a lower occupant hides while a higher one shows.
/// `Push` is R3: opening one class-1 panel closes the others through their own flags.
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
    /// §6.3. Class 4 and 5 may be hidden. Class 1–3 stay visible.
    pub hidden: bool,
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
    hidden: false,
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
    hidden: false,
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
    hidden: false,
};

pub const ANCHORS: &[HudAnchor] = &[ACTION_BAR, VOICE, ALLOCATE_DOCK];

/// Bumped when slab ids, classes, or preset tables change. Step 4a is rev 1.
pub const HUD_REGISTRY_REV: u32 = 1;

/// §6.2 / Q20. Snap onto this screen margin when an edge is within it.
const SNAP_MARGIN_PX: i32 = 16;
/// §6.2 / Q20. Snap onto another anchor's facing edge plus this gap.
const SNAP_GAP_PX: i32 = 8;
/// §6.2. Both screen margins, subtracted from the window width.
const WIDTH_WINDOW_INSET_PX: f32 = 32.0;

pub const ID_FACTORY: &str = "Factory";
pub const ID_SPILL: &str = "Spill";
pub const ID_FAB: &str = "Fab";
pub const ID_EMBASSY: &str = "Embassy";
pub const ID_REDEMPTION: &str = "Redemption";
pub const ID_HYBRID: &str = "Hybrid";
pub const ID_COMPASS: &str = "Compass";
pub const ID_WELL: &str = "Well";
pub const ID_PULSE: &str = "Pulse";
pub const ID_WELCOME: &str = "Welcome";
pub const ID_CLIMATE_STATE: &str = "ClimateState";
pub const ID_WATCH: &str = "Watch";
pub const ID_PICKUP: &str = "Pickup";
pub const ID_SOVEREIGN: &str = "Sovereign";
pub const ID_THRIVING: &str = "Thriving";
pub const ID_MERCY: &str = "Mercy";
pub const ID_REALM: &str = "Realm";
pub const ID_WHISPER: &str = "Whisper";
pub const ID_JOURNEY: &str = "Journey";
pub const ID_PEER: &str = "Peer";
pub const ID_PLACE_NAME: &str = "PlaceName";

/// One as-is joiner (§2.4). Position Vals are that slab's coded literals.
/// `margin_left` is set only for a centred slab, and it is that slab's own
/// half-width margin. It is never a shared constant.
#[derive(Clone, Copy, Debug)]
pub struct HudCodedPlace {
    pub id: &'static str,
    /// Map section 2 row. Rows 16 and 17 are position-only joiners.
    pub row: u8,
    /// Design §2.3 class. Class 1 is the highest priority.
    pub class: u8,
    pub width: f32,
    /// Model B height from design §2.3. Not written onto `Style`.
    pub height_b: f32,
    pub top: Val,
    pub right: Val,
    pub bottom: Val,
    pub left: Val,
    pub margin_left: Option<f32>,
}

/// Named fields for [`coded`].
struct CodedFields {
    id: &'static str,
    row: u8,
    class: u8,
    width: f32,
    height_b: f32,
    top: Val,
    right: Val,
    bottom: Val,
    left: Val,
    margin_left: Option<f32>,
}

const fn coded(fields: CodedFields) -> HudCodedPlace {
    HudCodedPlace {
        id: fields.id,
        row: fields.row,
        class: fields.class,
        width: fields.width,
        height_b: fields.height_b,
        top: fields.top,
        right: fields.right,
        bottom: fields.bottom,
        left: fields.left,
        margin_left: fields.margin_left,
    }
}

/// Row 8. Top 16, centred, margin −260, width 520.
pub const FACTORY: HudCodedPlace = coded(CodedFields {
    id: ID_FACTORY,
    row: 8,
    class: 5,
    width: 520.0,
    height_b: 52.0,
    top: Val::Px(16.0),
    right: Val::Auto,
    bottom: Val::Auto,
    left: Val::Percent(50.0),
    margin_left: Some(-260.0),
});
/// Row 10. Top 52, left 16, width 520.
pub const SPILL: HudCodedPlace = coded(CodedFields {
    id: ID_SPILL,
    row: 10,
    class: 5,
    width: 520.0,
    height_b: 52.0,
    top: Val::Px(52.0),
    right: Val::Auto,
    bottom: Val::Auto,
    left: Val::Px(16.0),
    margin_left: None,
});
/// Row 12. Top 88, centred, margin −260, width 520.
pub const FAB: HudCodedPlace = coded(CodedFields {
    id: ID_FAB,
    row: 12,
    class: 5,
    width: 520.0,
    height_b: 52.0,
    top: Val::Px(88.0),
    right: Val::Auto,
    bottom: Val::Auto,
    left: Val::Percent(50.0),
    margin_left: Some(-260.0),
});
/// Row 13. Top 124, right 16, width 420.
pub const EMBASSY: HudCodedPlace = coded(CodedFields {
    id: ID_EMBASSY,
    row: 13,
    class: 5,
    width: 420.0,
    height_b: 52.0,
    top: Val::Px(124.0),
    right: Val::Px(16.0),
    bottom: Val::Auto,
    left: Val::Auto,
    margin_left: None,
});
/// Row 16. Top 204, right 16. Position only. Width stays the coded literal at the spawn.
pub const REDEMPTION: HudCodedPlace = coded(CodedFields {
    id: ID_REDEMPTION,
    row: 16,
    class: 5,
    width: 420.0,
    height_b: 52.0,
    top: Val::Px(204.0),
    right: Val::Px(16.0),
    bottom: Val::Auto,
    left: Val::Auto,
    margin_left: None,
});
/// Row 17. Top 244, right 16. Position only. Width stays the coded literal at the spawn.
pub const HYBRID: HudCodedPlace = coded(CodedFields {
    id: ID_HYBRID,
    row: 17,
    class: 5,
    width: 420.0,
    height_b: 52.0,
    top: Val::Px(244.0),
    right: Val::Px(16.0),
    bottom: Val::Auto,
    left: Val::Auto,
    margin_left: None,
});
/// Row 18. Bottom 92, right 16, width 420.
pub const COMPASS: HudCodedPlace = coded(CodedFields {
    id: ID_COMPASS,
    row: 18,
    class: 5,
    width: 420.0,
    height_b: 52.0,
    top: Val::Auto,
    right: Val::Px(16.0),
    bottom: Val::Px(92.0),
    left: Val::Auto,
    margin_left: None,
});
/// Row 19. Bottom 132, left 16, width 420.
pub const WELL: HudCodedPlace = coded(CodedFields {
    id: ID_WELL,
    row: 19,
    class: 5,
    width: 420.0,
    height_b: 52.0,
    top: Val::Auto,
    right: Val::Auto,
    bottom: Val::Px(132.0),
    left: Val::Px(16.0),
    margin_left: None,
});
/// Row 28. Top 118, centred, margin −280, width 560.
pub const PULSE: HudCodedPlace = coded(CodedFields {
    id: ID_PULSE,
    row: 28,
    class: 4,
    width: 560.0,
    height_b: 60.8,
    top: Val::Px(118.0),
    right: Val::Auto,
    bottom: Val::Auto,
    left: Val::Percent(50.0),
    margin_left: Some(-280.0),
});
/// Row 29. Top 16, left 16, width 380.
pub const WELCOME: HudCodedPlace = coded(CodedFields {
    id: ID_WELCOME,
    row: 29,
    class: 4,
    width: 380.0,
    height_b: 58.4,
    top: Val::Px(16.0),
    right: Val::Auto,
    bottom: Val::Auto,
    left: Val::Px(16.0),
    margin_left: None,
});
/// Row 31. Bottom 176, left 16, width 420.
pub const CLIMATE_STATE: HudCodedPlace = coded(CodedFields {
    id: ID_CLIMATE_STATE,
    row: 31,
    class: 5,
    width: 420.0,
    height_b: 52.0,
    top: Val::Auto,
    right: Val::Auto,
    bottom: Val::Px(176.0),
    left: Val::Px(16.0),
    margin_left: None,
});
/// Row 32. Bottom 16, left 16, width 340. Satchel (row 33) stays coded.
pub const WATCH: HudCodedPlace = coded(CodedFields {
    id: ID_WATCH,
    row: 32,
    class: 5,
    width: 340.0,
    height_b: 53.2,
    top: Val::Auto,
    right: Val::Auto,
    bottom: Val::Px(16.0),
    left: Val::Px(16.0),
    margin_left: None,
});
/// Row 34. Top 38%, centred, margin −180, width 360.
pub const PICKUP: HudCodedPlace = coded(CodedFields {
    id: ID_PICKUP,
    row: 34,
    class: 4,
    width: 360.0,
    height_b: 56.4,
    top: Val::Percent(38.0),
    right: Val::Auto,
    bottom: Val::Auto,
    left: Val::Percent(50.0),
    margin_left: Some(-180.0),
});
/// Row 35. Top 52, centred, margin −260, width 520.
pub const SOVEREIGN: HudCodedPlace = coded(CodedFields {
    id: ID_SOVEREIGN,
    row: 35,
    class: 4,
    width: 520.0,
    height_b: 56.0,
    top: Val::Px(52.0),
    right: Val::Auto,
    bottom: Val::Auto,
    left: Val::Percent(50.0),
    margin_left: Some(-260.0),
});
/// Row 38. Top 48, centred, margin −310, width 620.
pub const THRIVING: HudCodedPlace = coded(CodedFields {
    id: ID_THRIVING,
    row: 38,
    class: 4,
    width: 620.0,
    height_b: 58.0,
    top: Val::Px(48.0),
    right: Val::Auto,
    bottom: Val::Auto,
    left: Val::Percent(50.0),
    margin_left: Some(-310.0),
});
/// Row 39. Top 10%, right 2%, width 360.
pub const MERCY: HudCodedPlace = coded(CodedFields {
    id: ID_MERCY,
    row: 39,
    class: 1,
    width: 360.0,
    height_b: 320.0,
    top: Val::Percent(10.0),
    right: Val::Percent(2.0),
    bottom: Val::Auto,
    left: Val::Auto,
    margin_left: None,
});
/// Row 40. Top 18%, left 2%, width 300.
pub const REALM: HudCodedPlace = coded(CodedFields {
    id: ID_REALM,
    row: 40,
    class: 1,
    width: 300.0,
    height_b: 170.0,
    top: Val::Percent(18.0),
    right: Val::Auto,
    bottom: Val::Auto,
    left: Val::Percent(2.0),
    margin_left: None,
});
/// Row 41. Top 28%, centred, margin −210, width 420.
pub const WHISPER: HudCodedPlace = coded(CodedFields {
    id: ID_WHISPER,
    row: 41,
    class: 4,
    width: 420.0,
    height_b: 69.0,
    top: Val::Percent(28.0),
    right: Val::Auto,
    bottom: Val::Auto,
    left: Val::Percent(50.0),
    margin_left: Some(-210.0),
});
/// Row 42. Top 12%, left 2%, width 360.
pub const JOURNEY: HudCodedPlace = coded(CodedFields {
    id: ID_JOURNEY,
    row: 42,
    class: 1,
    width: 360.0,
    height_b: 280.0,
    top: Val::Percent(12.0),
    right: Val::Auto,
    bottom: Val::Auto,
    left: Val::Percent(2.0),
    margin_left: None,
});
/// Row 43. Bottom 16, right 16, width 280.
pub const PEER: HudCodedPlace = coded(CodedFields {
    id: ID_PEER,
    row: 43,
    class: 5,
    width: 280.0,
    height_b: 52.0,
    top: Val::Auto,
    right: Val::Px(16.0),
    bottom: Val::Px(16.0),
    left: Val::Auto,
    margin_left: None,
});
/// Row 44. Top 18, centred, margin −140, width 280.
pub const PLACE_NAME: HudCodedPlace = coded(CodedFields {
    id: ID_PLACE_NAME,
    row: 44,
    class: 5,
    width: 280.0,
    height_b: 31.0,
    top: Val::Px(18.0),
    right: Val::Auto,
    bottom: Val::Auto,
    left: Val::Percent(50.0),
    margin_left: Some(-140.0),
});

/// The 21 coded joiners. Rows 16 and 17 are position only. Satchel is not here.
pub const CODED_JOINERS: &[HudCodedPlace] = &[
    FACTORY,
    SPILL,
    FAB,
    EMBASSY,
    REDEMPTION,
    HYBRID,
    COMPASS,
    WELL,
    PULSE,
    WELCOME,
    CLIMATE_STATE,
    WATCH,
    PICKUP,
    SOVEREIGN,
    THRIVING,
    MERCY,
    REALM,
    WHISPER,
    JOURNEY,
    PEER,
    PLACE_NAME,
];

impl HudCodedPlace {
    pub const fn top(self) -> Val {
        self.top
    }

    pub const fn right(self) -> Val {
        self.right
    }

    pub const fn bottom(self) -> Val {
        self.bottom
    }

    pub const fn left(self) -> Val {
        self.left
    }

    /// This slab's own coded centring margin (`UiRect::left`).
    ///
    /// Panics when the slab is not centred (`margin_left` is `None`).
    pub fn margin(self) -> UiRect {
        let px = self
            .margin_left
            .unwrap_or_else(|| panic!("hud place {} is not centred", self.id));
        UiRect::left(Val::Px(px))
    }

    pub fn rect(self, view_w: f32, view_h: f32) -> HudRect {
        slab_rect(
            self.slab_place(),
            self.width,
            self.height_b,
            view_w,
            view_h,
        )
    }

    fn slab_place(self) -> SlabPlace {
        if let Some(margin_left) = self.margin_left {
            return match self.top {
                Val::Px(top) => SlabPlace::TopCentre { top, margin_left },
                Val::Percent(pct) => SlabPlace::TopPercentCentre {
                    top_fraction: pct / 100.0,
                    margin_left,
                },
                _ => panic!("centred joiner {} has no top", self.id),
            };
        }
        match (self.top, self.bottom, self.left, self.right) {
            (Val::Px(top), Val::Auto, Val::Px(left), Val::Auto) => SlabPlace::TopLeft { top, left },
            (Val::Auto, Val::Px(bottom), Val::Px(left), Val::Auto) => {
                SlabPlace::BottomLeft { bottom, left }
            }
            (Val::Auto, Val::Px(bottom), Val::Auto, Val::Px(right)) => {
                SlabPlace::BottomRight { bottom, right }
            }
            (Val::Px(top), Val::Auto, Val::Auto, Val::Px(right)) => {
                SlabPlace::TopRight { top, right }
            }
            (Val::Percent(top), Val::Auto, Val::Auto, Val::Percent(right)) => {
                SlabPlace::TopPercentRightPercent {
                    top_fraction: top / 100.0,
                    right_fraction: right / 100.0,
                }
            }
            (Val::Percent(top), Val::Auto, Val::Percent(left), Val::Auto) => {
                SlabPlace::TopPercentLeftPercent {
                    top_fraction: top / 100.0,
                    left_fraction: left / 100.0,
                }
            }
            _ => panic!("no slab place for {}", self.id),
        }
    }
}

pub fn coded_joiner(id: &str) -> &'static HudCodedPlace {
    CODED_JOINERS
        .iter()
        .find(|place| place.id == id)
        .unwrap_or_else(|| panic!("no coded joiner {id}"))
}

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

/// ACTION_BAR CareStrip and CarePrompt, as the slabs' visibility systems see them.
/// CareStrip wins over CarePrompt. Hour 1: while Guidance is active on
/// MoveAround and no harvest node is in range, CarePrompt is not showing,
/// so Guidance keeps this anchor. `guidance_hidden` is the lived-hour bind:
/// the player chose to hide the slabs, so MoveAround does not keep the bar.
/// The caller writes `Visibility` only.
pub(crate) fn action_bar_prompts_showing(
    care: Option<&CareCycleOffer>,
    epi: Option<&FirstHarvestEpiphany>,
    nearby: Option<&NearbyMercyNode>,
    guidance: &FirstSessionGuidance,
    now: f64,
    guidance_hidden: bool,
) -> (bool, bool) {
    let care_strip = care.is_some_and(|offer| offer.active);
    let move_around_owns = guidance.active
        && !guidance.dismissed
        && guidance.objective == GuidanceObjective::MoveAround
        && !nearby.is_some_and(|node| node.in_range)
        && !guidance_hidden;
    let care_prompt = !move_around_owns
        && epi.zip(nearby).is_some_and(|(epi, nearby)| {
            world_care_prompt_visible(
                nearby.in_range,
                nearby.nodes_exist,
                epi.first_harvest_lived,
                guidance.dismissed,
                epi.prompt_visible(now, guidance),
            )
        })
        && !care_strip;
    (care_strip, care_prompt)
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
    TopLeft { top: f32, left: f32 },
    BottomPercentLeft { bottom_fraction: f32, left: f32 },
    TopPercentRightPercent { top_fraction: f32, right_fraction: f32 },
    TopPercentLeftPercent { top_fraction: f32, left_fraction: f32 },
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
        SlabPlace::TopLeft { top, left } => (left, top, left + width, top + height),
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
        SlabPlace::TopPercentLeftPercent {
            top_fraction,
            left_fraction,
        } => {
            let top = view_h * top_fraction;
            let left = view_w * left_fraction;
            (left, top, left + width, top + height)
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

/// Corner and offsets after §6.2 re-picks the screen third.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HudCornerPick {
    pub corner: HudCorner,
    pub offset: HudOffset,
}

/// §6.2. Translate `rect` onto the 16 px screen margin when an edge is within
/// 16 px of that screen edge. Otherwise translate it onto a neighbour's facing
/// edge plus the 8 px gap when that edge is within 8 px. Width and height stay.
pub fn snap_hud_rect(rect: HudRect, view_w: f32, view_h: f32, neighbours: &[HudRect]) -> HudRect {
    let view_w_px = view_w.round() as i32;
    let view_h_px = view_h.round() as i32;
    let width = rect.x1 - rect.x0;
    let height = rect.y1 - rect.y0;
    let mut x = snap_to_screen_margin(rect.x0, width, view_w_px);
    let mut y = snap_to_screen_margin(rect.y0, height, view_h_px);
    if !near_screen_edge(rect.x0, width, view_w_px) {
        x = snap_to_neighbour_gap(
            x,
            width,
            neighbours.iter().map(|other| (other.x0, other.x1)),
        );
    }
    if !near_screen_edge(rect.y0, height, view_h_px) {
        y = snap_to_neighbour_gap(
            y,
            height,
            neighbours.iter().map(|other| (other.y0, other.y1)),
        );
    }
    HudRect {
        x0: x,
        y0: y,
        x1: x + width,
        y1: y + height,
    }
}

/// §6.2. Width stays between the anchor's coded width and `min(640, window − 32)`.
pub fn clamp_hud_width(width: f32, coded_width: f32, view_w: f32) -> f32 {
    if !width.is_finite() || !coded_width.is_finite() || !view_w.is_finite() {
        return coded_width;
    }
    let max = (view_w - WIDTH_WINDOW_INSET_PX).min(shared::hud_layout::HUD_LAYOUT_WIDTH_MAX);
    if max < coded_width {
        return coded_width;
    }
    width.clamp(coded_width, max)
}

/// §6.2. The horizontal screen third of the anchor's centre picks left, centre,
/// or right. The vertical half picks top or bottom. Offsets are re-measured
/// from those edges. A centred corner leaves `offset.x` unused (`0`).
pub fn repick_hud_corner(rect: HudRect, view_w: f32, view_h: f32) -> HudCornerPick {
    let centre_x = (rect.x0 + rect.x1) as f32 * 0.5;
    let centre_y = (rect.y0 + rect.y1) as f32 * 0.5;
    let column = if centre_x < view_w / 3.0 {
        0
    } else if centre_x < view_w * 2.0 / 3.0 {
        1
    } else {
        2
    };
    let top = centre_y < view_h * 0.5;
    let corner = match (top, column) {
        (true, 0) => HudCorner::TopLeft,
        (true, 1) => HudCorner::TopCentre,
        (true, _) => HudCorner::TopRight,
        (false, 0) => HudCorner::BottomLeft,
        (false, 1) => HudCorner::BottomCentre,
        (false, _) => HudCorner::BottomRight,
    };
    let offset = match corner {
        HudCorner::TopLeft => HudOffset {
            x: rect.x0 as f32,
            y: rect.y0 as f32,
        },
        HudCorner::TopCentre => HudOffset {
            x: 0.0,
            y: rect.y0 as f32,
        },
        HudCorner::TopRight => HudOffset {
            x: view_w - rect.x1 as f32,
            y: rect.y0 as f32,
        },
        HudCorner::BottomLeft => HudOffset {
            x: rect.x0 as f32,
            y: view_h - rect.y1 as f32,
        },
        HudCorner::BottomCentre => HudOffset {
            x: 0.0,
            y: view_h - rect.y1 as f32,
        },
        HudCorner::BottomRight => HudOffset {
            x: view_w - rect.x1 as f32,
            y: view_h - rect.y1 as f32,
        },
    };
    HudCornerPick { corner, offset }
}

fn near_screen_edge(start: i32, span: i32, view: i32) -> bool {
    let end = start + span;
    start.abs() <= SNAP_MARGIN_PX || (view - end).abs() <= SNAP_MARGIN_PX
}

fn snap_to_screen_margin(start: i32, span: i32, view: i32) -> i32 {
    let end = start + span;
    let to_start = start.abs();
    let to_end = (view - end).abs();
    let start_near = to_start <= SNAP_MARGIN_PX;
    let end_near = to_end <= SNAP_MARGIN_PX;
    match (start_near, end_near) {
        (true, false) => SNAP_MARGIN_PX,
        (false, true) => view - SNAP_MARGIN_PX - span,
        (true, true) if to_start <= to_end => SNAP_MARGIN_PX,
        (true, true) => view - SNAP_MARGIN_PX - span,
        (false, false) => start,
    }
}

fn snap_to_neighbour_gap(start: i32, span: i32, edges: impl Iterator<Item = (i32, i32)>) -> i32 {
    let end = start + span;
    let mut best: Option<(i32, i32)> = None;
    for (other_start, other_end) in edges {
        let candidates = [
            ((start - other_end).abs(), other_end + SNAP_GAP_PX),
            ((end - other_start).abs(), other_start - SNAP_GAP_PX - span),
        ];
        for (distance, new_start) in candidates {
            if distance > SNAP_GAP_PX {
                continue;
            }
            let shift = (new_start - start).abs();
            match best {
                Some((best_shift, _)) if best_shift <= shift => {}
                _ => best = Some((shift, new_start)),
            }
        }
    }
    best.map(|(_, new_start)| new_start).unwrap_or(start)
}

/// §6.5. Writes only `top`, `bottom`, `left`, `right`, `margin.left`, and `width`.
///
/// Equal values are left untouched so a matching layout does not mark `Style` changed.
/// Padding, border, the other margin edges, font, and `text_scale` are not fields here.
pub fn write_hud_anchor_style(
    style: &mut Mut<'_, Node>,
    corner: HudCorner,
    offset: HudOffset,
    width: f32,
) {
    let auto = Val::Auto;
    let width_val = Val::Px(width);
    let (top, bottom, left, right, margin_left) = match corner {
        HudCorner::TopLeft => (Val::Px(offset.y), auto, Val::Px(offset.x), auto, auto),
        HudCorner::TopCentre => (
            Val::Px(offset.y),
            auto,
            Val::Percent(50.0),
            auto,
            Val::Px(-width / 2.0),
        ),
        HudCorner::TopRight => (Val::Px(offset.y), auto, auto, Val::Px(offset.x), auto),
        HudCorner::BottomLeft => (auto, Val::Px(offset.y), Val::Px(offset.x), auto, auto),
        HudCorner::BottomCentre => (
            auto,
            Val::Px(offset.y),
            Val::Percent(50.0),
            auto,
            Val::Px(-width / 2.0),
        ),
        HudCorner::BottomRight => (auto, Val::Px(offset.y), auto, Val::Px(offset.x), auto),
    };
    if style.top != top {
        style.top = top;
    }
    if style.bottom != bottom {
        style.bottom = bottom;
    }
    if style.left != left {
        style.left = left;
    }
    if style.right != right {
        style.right = right;
    }
    if style.margin.left != margin_left {
        style.margin.left = margin_left;
    }
    if style.width != width_val {
        style.width = width_val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    use crate::first_harvest_epiphany::FirstHarvestEpiphany;
    use crate::first_session_guidance::{
        handle_guidance_dismiss_input, update_guidance_visibility, FirstSessionGuidance,
        FirstSessionGuidanceStrip, GuidanceObjective,
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
    fn model_b_overlaps_zero_or_accepted_both_sizes() {
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

        // Seven pairs at 1024x640 Model B. Accepted; step 3 folds them.
        let added_1024 = [
            (ID_ALLOCATE, "Hybrid", 21840),
            (ID_ALLOCATE, "Pickup", 11628),
            (ID_ALLOCATE, "Redemption", 13440),
            (ID_ALLOCATE, "Whisper", 5616),
            ("ClimateState", ID_PRACTICE, 2176),
            ("Mercy", ID_VOICE, 6120),
            (ID_ALLOCATE, "Mercy", 48600),
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

    /// CARD HUD-ANCHOR-REGISTRY-2B. Each centred joiner's margin is its own
    /// coded value. Factory, Fab and Sovereign are each −260. The other five
    /// centred margins are −280, −180, −310, −210 and −140.
    #[test]
    fn centred_joiner_margins_equal_coded() {
        let coded = [
            (ID_FACTORY, -260.0),
            (ID_FAB, -260.0),
            (ID_PULSE, -280.0),
            (ID_PICKUP, -180.0),
            (ID_SOVEREIGN, -260.0),
            (ID_THRIVING, -310.0),
            (ID_WHISPER, -210.0),
            (ID_PLACE_NAME, -140.0),
        ];
        assert_eq!(coded.len(), 8);
        let mut margins = Vec::new();
        for (id, want) in coded {
            let place = coded_joiner(id);
            assert_eq!(
                place.margin_left,
                Some(want),
                "{id} stores its own margin {want}"
            );
            assert_eq!(place.margin(), UiRect::left(Val::Px(want)), "{id}");
            assert_eq!(place.left(), Val::Percent(50.0), "{id}");
            margins.push(want);
        }
        let mut unique = margins.clone();
        unique.sort_by(|a, b| a.partial_cmp(b).unwrap());
        unique.dedup();
        assert_eq!(
            unique,
            vec![-310.0, -280.0, -260.0, -210.0, -180.0, -140.0],
            "six distinct half-width margins"
        );
        let centred: Vec<_> = CODED_JOINERS
            .iter()
            .filter(|place| place.margin_left.is_some())
            .map(|place| place.id)
            .collect();
        assert_eq!(centred.len(), coded.len());
        for place in CODED_JOINERS {
            if place.margin_left.is_none() {
                assert!(
                    !coded.iter().any(|(id, _)| *id == place.id),
                    "{} is not centred",
                    place.id
                );
            }
        }
    }

    /// The 21 coded joiners equal the coded Style literals. Satchel is absent.
    /// Rows 16 and 17 are position only: top and right come from the record.
    /// These literals are the code at `8a79ce3d`, which matches design §2.4
    /// for the original 19, and the coded spawn positions for rows 16 and 17.
    #[test]
    fn as_is_joiners_match_coded_style_literals() {
        assert_eq!(CODED_JOINERS.len(), 21);
        assert!(CODED_JOINERS.iter().all(|place| place.id != "Satchel"));
        let rows: Vec<u8> = CODED_JOINERS.iter().map(|place| place.row).collect();
        assert_eq!(
            rows,
            vec![
                8, 10, 12, 13, 16, 17, 18, 19, 28, 29, 31, 32, 34, 35, 38, 39, 40, 41, 42, 43, 44
            ]
        );

        let literals = [
            (ID_FACTORY, Val::Px(16.0), Val::Auto, Val::Auto, Val::Percent(50.0), 520.0, 5, 52.0),
            (ID_SPILL, Val::Px(52.0), Val::Auto, Val::Auto, Val::Px(16.0), 520.0, 5, 52.0),
            (ID_FAB, Val::Px(88.0), Val::Auto, Val::Auto, Val::Percent(50.0), 520.0, 5, 52.0),
            (ID_EMBASSY, Val::Px(124.0), Val::Px(16.0), Val::Auto, Val::Auto, 420.0, 5, 52.0),
            (ID_REDEMPTION, Val::Px(204.0), Val::Px(16.0), Val::Auto, Val::Auto, 420.0, 5, 52.0),
            (ID_HYBRID, Val::Px(244.0), Val::Px(16.0), Val::Auto, Val::Auto, 420.0, 5, 52.0),
            (ID_COMPASS, Val::Auto, Val::Px(16.0), Val::Px(92.0), Val::Auto, 420.0, 5, 52.0),
            (ID_WELL, Val::Auto, Val::Auto, Val::Px(132.0), Val::Px(16.0), 420.0, 5, 52.0),
            (ID_PULSE, Val::Px(118.0), Val::Auto, Val::Auto, Val::Percent(50.0), 560.0, 4, 60.8),
            (ID_WELCOME, Val::Px(16.0), Val::Auto, Val::Auto, Val::Px(16.0), 380.0, 4, 58.4),
            (ID_CLIMATE_STATE, Val::Auto, Val::Auto, Val::Px(176.0), Val::Px(16.0), 420.0, 5, 52.0),
            (ID_WATCH, Val::Auto, Val::Auto, Val::Px(16.0), Val::Px(16.0), 340.0, 5, 53.2),
            (ID_PICKUP, Val::Percent(38.0), Val::Auto, Val::Auto, Val::Percent(50.0), 360.0, 4, 56.4),
            (ID_SOVEREIGN, Val::Px(52.0), Val::Auto, Val::Auto, Val::Percent(50.0), 520.0, 4, 56.0),
            (ID_THRIVING, Val::Px(48.0), Val::Auto, Val::Auto, Val::Percent(50.0), 620.0, 4, 58.0),
            (ID_MERCY, Val::Percent(10.0), Val::Percent(2.0), Val::Auto, Val::Auto, 360.0, 1, 320.0),
            (ID_REALM, Val::Percent(18.0), Val::Auto, Val::Auto, Val::Percent(2.0), 300.0, 1, 170.0),
            (ID_WHISPER, Val::Percent(28.0), Val::Auto, Val::Auto, Val::Percent(50.0), 420.0, 4, 69.0),
            (ID_JOURNEY, Val::Percent(12.0), Val::Auto, Val::Auto, Val::Percent(2.0), 360.0, 1, 280.0),
            (ID_PEER, Val::Auto, Val::Px(16.0), Val::Px(16.0), Val::Auto, 280.0, 5, 52.0),
            (ID_PLACE_NAME, Val::Px(18.0), Val::Auto, Val::Auto, Val::Percent(50.0), 280.0, 5, 31.0),
        ];
        assert_eq!(literals.len(), 21);
        for id in [ID_REDEMPTION, ID_HYBRID] {
            let place = coded_joiner(id);
            assert_eq!(place.margin_left, None, "{id} is not centred");
        }
        for (id, top, right, bottom, left, width, class, height_b) in literals {
            let place = coded_joiner(id);
            assert_eq!(place.top(), top, "{id} top");
            assert_eq!(place.right(), right, "{id} right");
            assert_eq!(place.bottom(), bottom, "{id} bottom");
            assert_eq!(place.left(), left, "{id} left");
            assert_eq!(place.width, width, "{id} width");
            assert_eq!(place.class, class, "{id} class");
            assert_eq!(place.height_b, height_b, "{id} height_b");
        }
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
            "Well" => WELL.rect(view_w, view_h),
            "ClimateState" => CLIMATE_STATE.rect(view_w, view_h),
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
            "Compass" => COMPASS.rect(view_w, view_h),
            "Hybrid" => HYBRID.rect(view_w, view_h),
            "Redemption" => REDEMPTION.rect(view_w, view_h),
            "Pickup" => PICKUP.rect(view_w, view_h),
            "Whisper" => WHISPER.rect(view_w, view_h),
            "Mercy" => MERCY.rect(view_w, view_h),
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

    #[test]
    fn move_around_yields_the_care_prompt_only_when_guidance_is_hidden() {
        let guidance = FirstSessionGuidance::default();
        assert!(guidance.active);
        assert!(!guidance.dismissed);
        assert_eq!(guidance.objective, GuidanceObjective::MoveAround);
        let epi = FirstHarvestEpiphany::default();
        assert!(!epi.first_harvest_lived);
        assert_eq!(epi.prompt_until, 9999.0);
        let mut nearby = NearbyMercyNode::default();
        nearby.nodes_exist = true;
        nearby.in_range = false;
        let (_strip, prompt) = action_bar_prompts_showing(
            None,
            Some(&epi),
            Some(&nearby),
            &guidance,
            0.0,
            false,
        );
        assert!(!prompt);
        let (_strip, prompt) = action_bar_prompts_showing(
            None,
            Some(&epi),
            Some(&nearby),
            &guidance,
            0.0,
            true,
        );
        assert!(prompt);
    }

    fn strip_vis<T: Component>(app: &mut App) -> Visibility {
        let mut query = app.world_mut().query_filtered::<&Visibility, With<T>>();
        *query
            .iter(app.world())
            .next()
            .expect("strip")
    }

    #[test]
    fn snap_to_sixteen_px_margin_on_all_four_screen_edges() {
        assert_eq!(HUD_REGISTRY_REV, 1);
        for (view_w, view_h) in [(1024.0_f32, 640.0_f32), (1280.0, 800.0)] {
            let view_w_px = view_w.round() as i32;
            let view_h_px = view_h.round() as i32;

            let left = snap_hud_rect(
                HudRect {
                    x0: 4,
                    y0: 200,
                    x1: 204,
                    y1: 260,
                },
                view_w,
                view_h,
                &[],
            );
            assert_eq!(left.x0, SNAP_MARGIN_PX, "left edge at {view_w}x{view_h}");
            assert_eq!(left.x1 - left.x0, 200);
            assert_eq!(left.y0, 200);

            let right = snap_hud_rect(
                HudRect {
                    x0: view_w_px - 204,
                    y0: 200,
                    x1: view_w_px - 4,
                    y1: 260,
                },
                view_w,
                view_h,
                &[],
            );
            assert_eq!(right.x1, view_w_px - SNAP_MARGIN_PX, "right edge");
            assert_eq!(right.x1 - right.x0, 200);

            let top = snap_hud_rect(
                HudRect {
                    x0: 200,
                    y0: 3,
                    x1: 400,
                    y1: 63,
                },
                view_w,
                view_h,
                &[],
            );
            assert_eq!(top.y0, SNAP_MARGIN_PX, "top edge");
            assert_eq!(top.x0, 200);

            let bottom = snap_hud_rect(
                HudRect {
                    x0: 200,
                    y0: view_h_px - 63,
                    x1: 400,
                    y1: view_h_px - 3,
                },
                view_w,
                view_h,
                &[],
            );
            assert_eq!(bottom.y1, view_h_px - SNAP_MARGIN_PX, "bottom edge");
            assert_eq!(bottom.x0, 200);
        }
    }

    #[test]
    fn snap_to_neighbour_anchor_edge_plus_gap() {
        let neighbour = HudRect {
            x0: 400,
            y0: 120,
            x1: 600,
            y1: 180,
        };
        let beside = snap_hud_rect(
            HudRect {
                x0: 604,
                y0: 120,
                x1: 804,
                y1: 180,
            },
            1024.0,
            640.0,
            &[neighbour],
        );
        assert_eq!(beside.x0, neighbour.x1 + SNAP_GAP_PX);
        assert_eq!(beside.x1 - beside.x0, 200);
        assert_eq!(beside.y0, 120);

        let before = snap_hud_rect(
            HudRect {
                x0: 180,
                y0: 120,
                x1: 396,
                y1: 180,
            },
            1024.0,
            640.0,
            &[neighbour],
        );
        assert_eq!(before.x1, neighbour.x0 - SNAP_GAP_PX);
        assert_eq!(before.x1 - before.x0, 216);
    }

    #[test]
    fn clamp_width_both_ends_at_1024x640_and_1280x800() {
        for (view_w, _view_h) in [(1024.0_f32, 640.0_f32), (1280.0, 800.0)] {
            assert_eq!(
                clamp_hud_width(10.0, 520.0, view_w),
                520.0,
                "min at {view_w}"
            );
            assert_eq!(
                clamp_hud_width(900.0, 520.0, view_w),
                640.0,
                "max at {view_w}"
            );
            assert_eq!(
                clamp_hud_width(580.0, 520.0, view_w),
                580.0,
                "inside at {view_w}"
            );
            assert_eq!(clamp_hud_width(100.0, 640.0, view_w), 640.0);
            assert_eq!(clamp_hud_width(900.0, 280.0, view_w), 640.0);
            assert_eq!(clamp_hud_width(280.0, 280.0, view_w), 280.0);
        }
    }

    #[test]
    fn repick_corner_for_each_screen_third() {
        for (view_w, view_h) in [(1024.0_f32, 640.0_f32), (1280.0, 800.0)] {
            let spots = [
                (view_w / 6.0, view_h / 4.0, HudCorner::TopLeft),
                (view_w / 2.0, view_h / 4.0, HudCorner::TopCentre),
                (view_w * 5.0 / 6.0, view_h / 4.0, HudCorner::TopRight),
                (view_w / 6.0, view_h * 3.0 / 4.0, HudCorner::BottomLeft),
                (view_w / 2.0, view_h * 3.0 / 4.0, HudCorner::BottomCentre),
                (
                    view_w * 5.0 / 6.0,
                    view_h * 3.0 / 4.0,
                    HudCorner::BottomRight,
                ),
            ];
            for (spot_x, spot_y, expect) in spots {
                let centre_x = spot_x.round() as i32;
                let centre_y = spot_y.round() as i32;
                let rect = HudRect {
                    x0: centre_x - 20,
                    y0: centre_y - 10,
                    x1: centre_x + 20,
                    y1: centre_y + 10,
                };
                let pick = repick_hud_corner(rect, view_w, view_h);
                assert_eq!(
                    pick.corner, expect,
                    "{view_w}x{view_h} centre {centre_x},{centre_y}"
                );
                let expect_x = match expect {
                    HudCorner::TopLeft | HudCorner::BottomLeft => rect.x0 as f32,
                    HudCorner::TopRight | HudCorner::BottomRight => view_w - rect.x1 as f32,
                    HudCorner::TopCentre | HudCorner::BottomCentre => 0.0,
                };
                let expect_y = match expect {
                    HudCorner::TopLeft | HudCorner::TopCentre | HudCorner::TopRight => {
                        rect.y0 as f32
                    }
                    HudCorner::BottomLeft | HudCorner::BottomCentre | HudCorner::BottomRight => {
                        view_h - rect.y1 as f32
                    }
                };
                assert_eq!(
                    pick.offset,
                    HudOffset {
                        x: expect_x,
                        y: expect_y
                    }
                );
            }
        }
    }

    /// §6.5. An override write touches only the six Style fields.
    #[test]
    fn override_write_touches_only_six_style_fields() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        let entity = app
            .world_mut()
            .spawn(Node {
                padding: UiRect::all(Val::Px(7.0)),
                border: UiRect::all(Val::Px(8.0)),
                margin: UiRect {
                    left: Val::Px(5.0),
                    right: Val::Px(6.0),
                    top: Val::Px(3.0),
                    bottom: Val::Px(4.0),
                },
                top: Val::Px(1.0),
                bottom: Val::Px(2.0),
                left: Val::Px(9.0),
                right: Val::Px(10.0),
                width: Val::Px(11.0),
                ..default()
            })
            .id();
        app.add_systems(Update, move |mut styles: Query<&mut Node>| {
            let mut style = styles.single_mut().unwrap();
            write_hud_anchor_style(
                &mut style,
                HudCorner::TopLeft,
                HudOffset { x: 40.0, y: 200.0 },
                520.0,
            );
        });
        app.update();
        let style = app.world().get::<Node>(entity).expect("style");
        assert_eq!(style.top, Val::Px(200.0));
        assert_eq!(style.left, Val::Px(40.0));
        assert_eq!(style.width, Val::Px(520.0));
        assert_eq!(style.bottom, Val::Auto);
        assert_eq!(style.right, Val::Auto);
        assert_eq!(style.margin.left, Val::Auto);
        assert_eq!(style.padding.left, Val::Px(7.0));
        assert_eq!(style.border.left, Val::Px(8.0));
        assert_eq!(style.margin.right, Val::Px(6.0));
        assert_eq!(style.margin.top, Val::Px(3.0));
        assert_eq!(style.margin.bottom, Val::Px(4.0));
    }
}
