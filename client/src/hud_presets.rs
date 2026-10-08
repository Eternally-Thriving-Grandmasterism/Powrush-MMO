//! CARD HUD-PRESETS-1 — step 3a of the UI layout epic.
//!
//! Three preset tables from design §3.2–§3.4, plus the R3, R4, and R5
//! predicates. Pure data. No plugin, no system, and no resource reads a
//! preset, so nothing on screen moves. Q10 (renders) and Q16 (where a layout
//! is saved) are later cards.
//!
//! Reset restores `classic` (Q1). Push is accepted in a shared panel slot
//! (Q2). Cover is accepted, and a toast may expire while it is hidden (Q3).
//! Modal yield is accepted (Q5). Place-name stays a showing candidate (Q11).
//! Climate state stays independent of every other slab (Q12). Rank stays
//! fixed (Q19). The gap is 8 px, the margin is 16 px, the touch clears are
//! `right 76` and `right 80`, and the column under the touch buttons starts
//! at y 180 (Q20). Ledger, satchel, touch, and the modal plates stay on their
//! coded anchors (Q6, Q7, Q22).

use crate::hud_anchor_registry::{
    slab_rect, HudAnchor, HudCorner, HudOccupant, HudOffset, HudRect, HudShare, HudZBand,
    SlabPlace, ID_ALLOCATE, ID_CARE_PROMPT, ID_CARE_STRIP, ID_CLIMATE_STATE, ID_COMPASS,
    ID_EMBASSY, ID_FAB, ID_FACTORY, ID_GUIDANCE, ID_HYBRID, ID_JOURNEY, ID_MERCY, ID_PEER,
    ID_PICKUP, ID_PLACE_NAME, ID_PRACTICE, ID_PULSE, ID_REALM, ID_REDEMPTION, ID_SOVEREIGN,
    ID_SPILL, ID_THRIVING, ID_VOICE, ID_WATCH, ID_WELCOME, ID_WELL, ID_WHISPER,
};

/// Q20. Edge margin, in window px.
pub const EDGE_MARGIN_PX: i32 = 16;
/// Q20. Gap between anchors that are not in an R4 cover, in window px.
pub const ANCHOR_GAP_PX: i32 = 8;
/// Q20. Right offset above y 180: touch column (right 24 + 44) plus the 8 px gap.
pub const RIGHT_CLEAR_COLUMN_PX: f32 = 76.0;
/// Q20. Right offset beside TouchUse: right 28 + 44 plus the 8 px gap.
pub const RIGHT_CLEAR_USE_PX: f32 = 80.0;
/// Q20. First top-right row under the touch column.
pub const TOUCH_COLUMN_CLEAR_Y: f32 = 180.0;

const CLASS_PANEL: u8 = 1;
const CLASS_PROMPT: u8 = 2;
const CLASS_TUTOR: u8 = 3;
const CLASS_TOAST: u8 = 4;
const CLASS_STATUS: u8 = 5;

/// Design §2.3. Model A is the forced-line height. Model B adds one wrapped line.
/// Mercy, Journey, Realm, and PlaceName use one height for both models (max-height,
/// or one line).
#[derive(Clone, Copy, Debug)]
pub struct HudSlabMetrics {
    pub id: &'static str,
    pub row: u8,
    pub class: u8,
    pub width: f32,
    pub height_a: f32,
    pub height_b: f32,
}

const SLAB_METRICS: [HudSlabMetrics; 27] = [
    m(ID_FACTORY, 8, CLASS_STATUS, 520.0, 35.0, 52.0),
    m(ID_VOICE, 9, CLASS_PANEL, 560.0, 35.0, 52.0),
    m(ID_SPILL, 10, CLASS_STATUS, 520.0, 35.0, 52.0),
    m(ID_FAB, 12, CLASS_STATUS, 520.0, 35.0, 52.0),
    m(ID_EMBASSY, 13, CLASS_STATUS, 420.0, 35.0, 52.0),
    m(ID_REDEMPTION, 16, CLASS_STATUS, 420.0, 35.0, 52.0),
    m(ID_HYBRID, 17, CLASS_STATUS, 420.0, 35.0, 52.0),
    m(ID_COMPASS, 18, CLASS_STATUS, 420.0, 35.0, 52.0),
    m(ID_WELL, 19, CLASS_STATUS, 420.0, 35.0, 52.0),
    m(ID_GUIDANCE, 26, CLASS_TUTOR, 520.0, 48.0, 69.0),
    m(ID_CARE_PROMPT, 27, CLASS_PROMPT, 460.0, 37.2, 56.4),
    m(ID_PULSE, 28, CLASS_TOAST, 560.0, 41.6, 60.8),
    m(ID_WELCOME, 29, CLASS_TOAST, 380.0, 42.2, 58.4),
    m(ID_CARE_STRIP, 30, CLASS_PROMPT, 560.0, 43.0, 62.0),
    m(ID_CLIMATE_STATE, 31, CLASS_STATUS, 420.0, 35.0, 52.0),
    m(ID_WATCH, 32, CLASS_STATUS, 340.0, 37.6, 53.2),
    m(ID_PICKUP, 34, CLASS_TOAST, 360.0, 37.2, 56.4),
    m(ID_SOVEREIGN, 35, CLASS_TOAST, 520.0, 39.0, 56.0),
    m(ID_PRACTICE, 36, CLASS_TUTOR, 640.0, 46.0, 64.0),
    m(ID_ALLOCATE, 37, CLASS_PANEL, 520.0, 117.0, 135.0),
    m(ID_THRIVING, 38, CLASS_TOAST, 620.0, 40.0, 58.0),
    m(ID_MERCY, 39, CLASS_PANEL, 360.0, 320.0, 320.0),
    m(ID_REALM, 40, CLASS_PANEL, 300.0, 170.0, 170.0),
    m(ID_WHISPER, 41, CLASS_TOAST, 420.0, 48.0, 69.0),
    m(ID_JOURNEY, 42, CLASS_PANEL, 360.0, 280.0, 280.0),
    m(ID_PEER, 43, CLASS_STATUS, 280.0, 37.0, 52.0),
    m(ID_PLACE_NAME, 44, CLASS_STATUS, 280.0, 31.0, 31.0),
];

const fn m(
    id: &'static str,
    row: u8,
    class: u8,
    width: f32,
    height_a: f32,
    height_b: f32,
) -> HudSlabMetrics {
    HudSlabMetrics {
        id,
        row,
        class,
        width,
        height_a,
        height_b,
    }
}

const fn bytes_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut index = 0;
    while index < left.len() {
        if left[index] != right[index] {
            return false;
        }
        index += 1;
    }
    true
}

const fn metrics_of(id: &str) -> HudSlabMetrics {
    let mut index = 0;
    while index < SLAB_METRICS.len() {
        if bytes_eq(SLAB_METRICS[index].id.as_bytes(), id.as_bytes()) {
            return SLAB_METRICS[index];
        }
        index += 1;
    }
    panic!("no HUD slab metrics for this id");
}

const fn occ(id: &'static str, rank: u8) -> HudOccupant {
    let metrics = metrics_of(id);
    HudOccupant {
        id,
        rank,
        class: metrics.class,
        width: metrics.width,
        height_b: metrics.height_b,
    }
}

struct AnchorFields {
    id: &'static str,
    corner: HudCorner,
    x: f32,
    y: f32,
    width: f32,
    height_budget: f32,
    class: u8,
    occupants: &'static [HudOccupant],
    share: HudShare,
}

const fn anchor(fields: AnchorFields) -> HudAnchor {
    HudAnchor {
        id: fields.id,
        corner: fields.corner,
        offset: HudOffset {
            x: fields.x,
            y: fields.y,
        },
        width: fields.width,
        height_budget: fields.height_budget,
        z_band: HudZBand::Hud,
        class: fields.class,
        occupants: fields.occupants,
        share: fields.share,
    }
}

/// The three named presets. `classic` is the Reset target (Q1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudPresetId {
    Classic,
    Minimal,
    Management,
}

impl HudPresetId {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Classic => "classic",
            Self::Minimal => "minimal",
            Self::Management => "management",
        }
    }
}

/// One preset: a list of anchors. Applying it is a later card.
#[derive(Clone, Copy, Debug)]
pub struct HudPreset {
    pub id: HudPresetId,
    pub name: &'static str,
    pub anchors: &'static [HudAnchor],
}

impl HudPreset {
    pub fn anchor(&self, id: &str) -> &HudAnchor {
        self.anchors
            .iter()
            .find(|anchor| anchor.id == id)
            .unwrap_or_else(|| panic!("preset {} has no anchor {id}", self.name))
    }
}

/// Q1. Reset restores this preset and does not read a save.
pub const RESET_PRESET: HudPresetId = HudPresetId::Classic;

/// Height model for the overlap proof. Design §2.3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudHeightModel {
    A,
    B,
}

/// Proof windows from design §4.2.
pub const PROOF_VIEWS: [(f32, f32); 2] = [(1024.0, 640.0), (1280.0, 800.0)];

// --- classic (§3.2) -------------------------------------------------------

const CLASSIC_TOP_TOAST: [HudOccupant; 6] = [
    occ(ID_PULSE, 1),
    occ(ID_PICKUP, 2),
    occ(ID_THRIVING, 3),
    occ(ID_SOVEREIGN, 4),
    occ(ID_WHISPER, 5),
    occ(ID_WELCOME, 6),
];
const CLASSIC_LEFT_STATUS_1: [HudOccupant; 3] =
    [occ(ID_FACTORY, 1), occ(ID_FAB, 2), occ(ID_SPILL, 3)];
const CLASSIC_LEFT_STATUS_2: [HudOccupant; 2] = [occ(ID_CLIMATE_STATE, 1), occ(ID_WELL, 2)];
const CLASSIC_PLACE_NAME: [HudOccupant; 1] = [occ(ID_PLACE_NAME, 1)];
const CLASSIC_TRACKER_1: [HudOccupant; 1] = [occ(ID_EMBASSY, 1)];
const CLASSIC_TRACKER_2: [HudOccupant; 1] = [occ(ID_REDEMPTION, 1)];
const CLASSIC_TRACKER_3: [HudOccupant; 1] = [occ(ID_HYBRID, 1)];
const CLASSIC_TRACKER_4: [HudOccupant; 1] = [occ(ID_COMPASS, 1)];
const CLASSIC_VOICE: [HudOccupant; 1] = [occ(ID_VOICE, 1)];
const CLASSIC_ACTION_BAR: [HudOccupant; 4] = [
    occ(ID_CARE_STRIP, 1),
    occ(ID_CARE_PROMPT, 2),
    occ(ID_GUIDANCE, 3),
    occ(ID_PRACTICE, 4),
];
const CLASSIC_WINDOW: [HudOccupant; 4] = [
    occ(ID_ALLOCATE, 1),
    occ(ID_MERCY, 2),
    occ(ID_JOURNEY, 3),
    occ(ID_REALM, 4),
];
const CLASSIC_CORNER_WATCH: [HudOccupant; 1] = [occ(ID_WATCH, 1)];
const CLASSIC_CORNER_PEER: [HudOccupant; 1] = [occ(ID_PEER, 1)];

const CLASSIC_ANCHORS: [HudAnchor; 13] = [
    anchor(AnchorFields {
        id: "TOP_TOAST",
        corner: HudCorner::TopCentre,
        x: 0.0,
        y: 16.0,
        width: 620.0,
        height_budget: 69.0,
        class: CLASS_TOAST,
        occupants: &CLASSIC_TOP_TOAST,
        share: HudShare::Yield,
    }),
    anchor(AnchorFields {
        id: "LEFT_STATUS_1",
        corner: HudCorner::TopLeft,
        x: 16.0,
        y: 93.0,
        width: 520.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &CLASSIC_LEFT_STATUS_1,
        share: HudShare::Yield,
    }),
    anchor(AnchorFields {
        id: "LEFT_STATUS_2",
        corner: HudCorner::TopLeft,
        x: 16.0,
        y: 153.0,
        width: 420.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &CLASSIC_LEFT_STATUS_2,
        share: HudShare::Yield,
    }),
    anchor(AnchorFields {
        id: "PLACE_NAME",
        corner: HudCorner::TopRight,
        x: RIGHT_CLEAR_COLUMN_PX,
        y: 93.0,
        width: 280.0,
        height_budget: 31.0,
        class: CLASS_STATUS,
        occupants: &CLASSIC_PLACE_NAME,
        share: HudShare::Solo,
    }),
    anchor(AnchorFields {
        id: "TRACKER_1",
        corner: HudCorner::TopRight,
        x: 16.0,
        y: TOUCH_COLUMN_CLEAR_Y,
        width: 420.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &CLASSIC_TRACKER_1,
        share: HudShare::Solo,
    }),
    anchor(AnchorFields {
        id: "TRACKER_2",
        corner: HudCorner::TopRight,
        x: 16.0,
        y: 240.0,
        width: 420.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &CLASSIC_TRACKER_2,
        share: HudShare::Solo,
    }),
    anchor(AnchorFields {
        id: "TRACKER_3",
        corner: HudCorner::TopRight,
        x: 16.0,
        y: 300.0,
        width: 420.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &CLASSIC_TRACKER_3,
        share: HudShare::Solo,
    }),
    anchor(AnchorFields {
        id: "TRACKER_4",
        corner: HudCorner::TopRight,
        x: 16.0,
        y: 360.0,
        width: 420.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &CLASSIC_TRACKER_4,
        share: HudShare::Solo,
    }),
    anchor(AnchorFields {
        id: "VOICE",
        corner: HudCorner::BottomRight,
        x: 16.0,
        y: 221.0,
        width: 560.0,
        height_budget: 52.0,
        class: CLASS_PANEL,
        occupants: &CLASSIC_VOICE,
        share: HudShare::Solo,
    }),
    anchor(AnchorFields {
        id: "ACTION_BAR",
        corner: HudCorner::BottomRight,
        x: 16.0,
        y: 144.0,
        width: 640.0,
        height_budget: 69.0,
        class: CLASS_PROMPT,
        occupants: &CLASSIC_ACTION_BAR,
        share: HudShare::Yield,
    }),
    anchor(AnchorFields {
        id: "WINDOW",
        corner: HudCorner::TopRight,
        x: RIGHT_CLEAR_COLUMN_PX,
        y: 16.0,
        width: 520.0,
        height_budget: 320.0,
        class: CLASS_PANEL,
        occupants: &CLASSIC_WINDOW,
        share: HudShare::Push,
    }),
    anchor(AnchorFields {
        id: "CORNER_WATCH",
        corner: HudCorner::BottomRight,
        x: RIGHT_CLEAR_USE_PX,
        y: 76.0,
        width: 340.0,
        height_budget: 54.0,
        class: CLASS_STATUS,
        occupants: &CLASSIC_CORNER_WATCH,
        share: HudShare::Solo,
    }),
    anchor(AnchorFields {
        id: "CORNER_PEER",
        corner: HudCorner::BottomRight,
        x: RIGHT_CLEAR_USE_PX,
        y: 16.0,
        width: 280.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &CLASSIC_CORNER_PEER,
        share: HudShare::Solo,
    }),
];

pub const CLASSIC: HudPreset = HudPreset {
    id: HudPresetId::Classic,
    name: "classic",
    anchors: &CLASSIC_ANCHORS,
};

// --- minimal (§3.3) -------------------------------------------------------

const MINIMAL_TOP_TOAST: [HudOccupant; 7] = [
    occ(ID_PULSE, 1),
    occ(ID_PICKUP, 2),
    occ(ID_THRIVING, 3),
    occ(ID_SOVEREIGN, 4),
    occ(ID_WHISPER, 5),
    occ(ID_WELCOME, 6),
    occ(ID_PLACE_NAME, 7),
];
const MINIMAL_EDGE_STATUS: [HudOccupant; 9] = [
    occ(ID_FACTORY, 1),
    occ(ID_FAB, 2),
    occ(ID_SPILL, 3),
    occ(ID_EMBASSY, 4),
    occ(ID_REDEMPTION, 5),
    occ(ID_HYBRID, 6),
    occ(ID_COMPASS, 7),
    occ(ID_WELL, 8),
    occ(ID_CLIMATE_STATE, 9),
];
const MINIMAL_ACTION_BAR: [HudOccupant; 4] = [
    occ(ID_CARE_STRIP, 1),
    occ(ID_CARE_PROMPT, 2),
    occ(ID_GUIDANCE, 3),
    occ(ID_PRACTICE, 4),
];
const MINIMAL_WINDOW: [HudOccupant; 5] = [
    occ(ID_VOICE, 1),
    occ(ID_ALLOCATE, 2),
    occ(ID_MERCY, 3),
    occ(ID_JOURNEY, 4),
    occ(ID_REALM, 5),
];
const MINIMAL_CORNER: [HudOccupant; 2] = [occ(ID_WATCH, 1), occ(ID_PEER, 2)];

const MINIMAL_ANCHORS: [HudAnchor; 5] = [
    anchor(AnchorFields {
        id: "TOP_TOAST",
        corner: HudCorner::TopCentre,
        x: 0.0,
        y: 16.0,
        width: 620.0,
        height_budget: 69.0,
        class: CLASS_TOAST,
        occupants: &MINIMAL_TOP_TOAST,
        share: HudShare::Yield,
    }),
    anchor(AnchorFields {
        id: "EDGE_STATUS",
        corner: HudCorner::TopRight,
        x: 16.0,
        y: TOUCH_COLUMN_CLEAR_Y,
        width: 520.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &MINIMAL_EDGE_STATUS,
        share: HudShare::Yield,
    }),
    anchor(AnchorFields {
        id: "ACTION_BAR",
        corner: HudCorner::BottomRight,
        x: 16.0,
        y: 144.0,
        width: 640.0,
        height_budget: 69.0,
        class: CLASS_PROMPT,
        occupants: &MINIMAL_ACTION_BAR,
        share: HudShare::Yield,
    }),
    anchor(AnchorFields {
        id: "WINDOW",
        corner: HudCorner::TopRight,
        x: RIGHT_CLEAR_COLUMN_PX,
        y: 16.0,
        width: 560.0,
        height_budget: 320.0,
        class: CLASS_PANEL,
        occupants: &MINIMAL_WINDOW,
        share: HudShare::Push,
    }),
    anchor(AnchorFields {
        id: "CORNER",
        corner: HudCorner::BottomRight,
        x: RIGHT_CLEAR_USE_PX,
        y: 16.0,
        width: 340.0,
        height_budget: 54.0,
        class: CLASS_STATUS,
        occupants: &MINIMAL_CORNER,
        share: HudShare::Yield,
    }),
];

pub const MINIMAL: HudPreset = HudPreset {
    id: HudPresetId::Minimal,
    name: "minimal",
    anchors: &MINIMAL_ANCHORS,
};

// --- management (§3.4) ----------------------------------------------------

const MANAGEMENT_PLACE: [HudOccupant; 1] = [occ(ID_PLACE_NAME, 1)];
const MANAGEMENT_WATCH: [HudOccupant; 1] = [occ(ID_WATCH, 1)];
const MANAGEMENT_PEER: [HudOccupant; 1] = [occ(ID_PEER, 1)];
const MANAGEMENT_ADVISOR: [HudOccupant; 4] = [
    occ(ID_CARE_STRIP, 1),
    occ(ID_CARE_PROMPT, 2),
    occ(ID_GUIDANCE, 3),
    occ(ID_PRACTICE, 4),
];
const MANAGEMENT_LIST_1: [HudOccupant; 3] = [occ(ID_FACTORY, 1), occ(ID_FAB, 2), occ(ID_SPILL, 3)];
const MANAGEMENT_LIST_2: [HudOccupant; 3] =
    [occ(ID_EMBASSY, 1), occ(ID_REDEMPTION, 2), occ(ID_HYBRID, 3)];
const MANAGEMENT_LIST_3: [HudOccupant; 1] = [occ(ID_COMPASS, 1)];
const MANAGEMENT_LIST_4: [HudOccupant; 2] = [occ(ID_WELL, 1), occ(ID_CLIMATE_STATE, 2)];
const MANAGEMENT_WINDOW: [HudOccupant; 5] = [
    occ(ID_VOICE, 1),
    occ(ID_ALLOCATE, 2),
    occ(ID_MERCY, 3),
    occ(ID_JOURNEY, 4),
    occ(ID_REALM, 5),
];
const MANAGEMENT_NEWS: [HudOccupant; 6] = [
    occ(ID_PULSE, 1),
    occ(ID_PICKUP, 2),
    occ(ID_THRIVING, 3),
    occ(ID_SOVEREIGN, 4),
    occ(ID_WHISPER, 5),
    occ(ID_WELCOME, 6),
];

const MANAGEMENT_ANCHORS: [HudAnchor; 10] = [
    anchor(AnchorFields {
        id: "TOOLBAR_PLACE",
        corner: HudCorner::TopLeft,
        x: 16.0,
        y: 16.0,
        width: 280.0,
        height_budget: 31.0,
        class: CLASS_STATUS,
        occupants: &MANAGEMENT_PLACE,
        share: HudShare::Solo,
    }),
    anchor(AnchorFields {
        id: "TOOLBAR_WATCH",
        corner: HudCorner::TopLeft,
        x: 304.0,
        y: 16.0,
        width: 340.0,
        height_budget: 54.0,
        class: CLASS_STATUS,
        occupants: &MANAGEMENT_WATCH,
        share: HudShare::Solo,
    }),
    anchor(AnchorFields {
        id: "TOOLBAR_PEER",
        corner: HudCorner::TopLeft,
        x: 652.0,
        y: 16.0,
        width: 280.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &MANAGEMENT_PEER,
        share: HudShare::Solo,
    }),
    anchor(AnchorFields {
        id: "ADVISOR",
        corner: HudCorner::TopCentre,
        x: 0.0,
        y: 78.0,
        width: 640.0,
        height_budget: 69.0,
        class: CLASS_PROMPT,
        occupants: &MANAGEMENT_ADVISOR,
        share: HudShare::Yield,
    }),
    anchor(AnchorFields {
        id: "LIST_1",
        corner: HudCorner::TopRight,
        x: 16.0,
        y: TOUCH_COLUMN_CLEAR_Y,
        width: 520.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &MANAGEMENT_LIST_1,
        share: HudShare::Yield,
    }),
    anchor(AnchorFields {
        id: "LIST_2",
        corner: HudCorner::TopRight,
        x: 16.0,
        y: 240.0,
        width: 420.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &MANAGEMENT_LIST_2,
        share: HudShare::Yield,
    }),
    anchor(AnchorFields {
        id: "LIST_3",
        corner: HudCorner::TopRight,
        x: 16.0,
        y: 300.0,
        width: 420.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &MANAGEMENT_LIST_3,
        share: HudShare::Solo,
    }),
    anchor(AnchorFields {
        id: "LIST_4",
        corner: HudCorner::TopRight,
        x: 16.0,
        y: 360.0,
        width: 420.0,
        height_budget: 52.0,
        class: CLASS_STATUS,
        occupants: &MANAGEMENT_LIST_4,
        share: HudShare::Yield,
    }),
    anchor(AnchorFields {
        id: "WINDOW",
        corner: HudCorner::TopLeft,
        x: 324.0,
        y: 155.0,
        width: 560.0,
        height_budget: 320.0,
        class: CLASS_PANEL,
        occupants: &MANAGEMENT_WINDOW,
        share: HudShare::Push,
    }),
    anchor(AnchorFields {
        id: "NEWS_TICKER",
        corner: HudCorner::BottomRight,
        x: 16.0,
        y: 144.0,
        width: 620.0,
        height_budget: 69.0,
        class: CLASS_TOAST,
        occupants: &MANAGEMENT_NEWS,
        share: HudShare::Yield,
    }),
];

pub const MANAGEMENT: HudPreset = HudPreset {
    id: HudPresetId::Management,
    name: "management",
    anchors: &MANAGEMENT_ANCHORS,
};

pub const PRESETS: &[HudPreset] = &[CLASSIC, MINIMAL, MANAGEMENT];

pub fn preset(id: HudPresetId) -> &'static HudPreset {
    match id {
        HudPresetId::Classic => &CLASSIC,
        HudPresetId::Minimal => &MINIMAL,
        HudPresetId::Management => &MANAGEMENT,
    }
}

/// Q1. The preset Reset restores.
pub fn reset_preset() -> &'static HudPreset {
    preset(RESET_PRESET)
}

pub fn slab_metrics(id: &str) -> &HudSlabMetrics {
    SLAB_METRICS
        .iter()
        .find(|metrics| metrics.id == id)
        .unwrap_or_else(|| panic!("no HUD slab metrics for {id}"))
}

/// Design §1.4. Each edge is rounded to a whole pixel on its own.
/// A centred anchor ignores `offset.x` and centres the passed width.
pub fn placed_rect(
    corner: HudCorner,
    offset: HudOffset,
    width: f32,
    height: f32,
    view_w: f32,
    view_h: f32,
) -> HudRect {
    let place = match corner {
        HudCorner::TopLeft => SlabPlace::TopLeft {
            top: offset.y,
            left: offset.x,
        },
        HudCorner::TopCentre => SlabPlace::TopCentre {
            top: offset.y,
            margin_left: -width / 2.0,
        },
        HudCorner::TopRight => SlabPlace::TopRight {
            top: offset.y,
            right: offset.x,
        },
        HudCorner::BottomLeft => SlabPlace::BottomLeft {
            bottom: offset.y,
            left: offset.x,
        },
        HudCorner::BottomCentre => {
            let x0 = view_w * 0.5 - width / 2.0;
            let y1 = view_h - offset.y;
            return rounded_rect(x0, y1 - height, x0 + width, y1);
        }
        HudCorner::BottomRight => SlabPlace::BottomRight {
            bottom: offset.y,
            right: offset.x,
        },
    };
    slab_rect(place, width, height, view_w, view_h)
}

fn rounded_rect(x0: f32, y0: f32, x1: f32, y1: f32) -> HudRect {
    HudRect {
        x0: x0.round() as i32,
        y0: y0.round() as i32,
        x1: x1.round() as i32,
        y1: y1.round() as i32,
    }
}

pub fn anchor_rect(anchor: &HudAnchor, view_w: f32, view_h: f32) -> HudRect {
    placed_rect(
        anchor.corner,
        anchor.offset,
        anchor.width,
        anchor.height_budget,
        view_w,
        view_h,
    )
}

pub fn occupant_rect(
    anchor: &HudAnchor,
    occupant: &HudOccupant,
    model: HudHeightModel,
    view_w: f32,
    view_h: f32,
) -> HudRect {
    let metrics = slab_metrics(occupant.id);
    let height = match model {
        HudHeightModel::A => metrics.height_a,
        HudHeightModel::B => occupant.height_b,
    };
    placed_rect(
        anchor.corner,
        anchor.offset,
        occupant.width,
        height,
        view_w,
        view_h,
    )
}

/// Fixed rects from design §2.3. The same in every preset. Model A uses the
/// map's forced-line heights for the ledger (102) and the satchel (219).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudFixedId {
    Ledger,
    Satchel,
    TouchStick,
    TouchUse,
    TouchPause,
    TouchQ,
    TouchL,
}

impl HudFixedId {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Ledger => "Ledger",
            Self::Satchel => "Satchel",
            Self::TouchStick => "TouchStick",
            Self::TouchUse => "TouchUse",
            Self::TouchPause => "TouchPause",
            Self::TouchQ => "TouchQ",
            Self::TouchL => "TouchL",
        }
    }

    pub const ALL: [HudFixedId; 7] = [
        Self::Ledger,
        Self::Satchel,
        Self::TouchStick,
        Self::TouchUse,
        Self::TouchPause,
        Self::TouchQ,
        Self::TouchL,
    ];
}

pub fn fixed_rect(id: HudFixedId, model: HudHeightModel, view_w: f32, view_h: f32) -> HudRect {
    match id {
        HudFixedId::Ledger => {
            let height = match model {
                HudHeightModel::A => 102.0,
                HudHeightModel::B => 119.0,
            };
            slab_rect(
                SlabPlace::BottomLeft {
                    bottom: 16.0,
                    left: 16.0,
                },
                560.0,
                height,
                view_w,
                view_h,
            )
        }
        HudFixedId::Satchel => {
            let height = match model {
                HudHeightModel::A => 219.0,
                HudHeightModel::B => 235.0,
            };
            slab_rect(
                SlabPlace::BottomPercentLeft {
                    bottom_fraction: 0.22,
                    left: 16.0,
                },
                300.0,
                height,
                view_w,
                view_h,
            )
        }
        HudFixedId::TouchStick => slab_rect(
            SlabPlace::BottomLeft {
                bottom: 24.0,
                left: 24.0,
            },
            120.0,
            120.0,
            view_w,
            view_h,
        ),
        HudFixedId::TouchUse => slab_rect(
            SlabPlace::BottomRight {
                bottom: 36.0,
                right: 28.0,
            },
            44.0,
            44.0,
            view_w,
            view_h,
        ),
        HudFixedId::TouchPause => slab_rect(
            SlabPlace::TopRight {
                top: 24.0,
                right: 24.0,
            },
            44.0,
            44.0,
            view_w,
            view_h,
        ),
        HudFixedId::TouchQ => slab_rect(
            SlabPlace::TopRight {
                top: 76.0,
                right: 24.0,
            },
            44.0,
            44.0,
            view_w,
            view_h,
        ),
        HudFixedId::TouchL => slab_rect(
            SlabPlace::TopRight {
                top: 128.0,
                right: 24.0,
            },
            44.0,
            44.0,
            view_w,
            view_h,
        ),
    }
}

/// Plates the §3.5 R5 column measures against. Settings is the Pause plate.
/// Places is row 7. Comfort is not an R5 trigger: the map shows it on the
/// Title door or while Pause is open, and both of those already yield the HUD.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudModalGeom {
    Settings,
    Places,
    Comfort,
}

impl HudModalGeom {
    pub const ALL: [HudModalGeom; 3] = [Self::Settings, Self::Places, Self::Comfort];
}

pub fn modal_rect(id: HudModalGeom, model: HudHeightModel, view_w: f32, view_h: f32) -> HudRect {
    match id {
        HudModalGeom::Settings => slab_rect(
            SlabPlace::TopPercentCentre {
                top_fraction: 0.01,
                margin_left: -210.0,
            },
            420.0,
            view_h * 0.98,
            view_w,
            view_h,
        ),
        HudModalGeom::Places => slab_rect(
            SlabPlace::TopPercentCentre {
                top_fraction: 0.18,
                margin_left: -200.0,
            },
            400.0,
            400.0,
            view_w,
            view_h,
        ),
        HudModalGeom::Comfort => {
            let height = match model {
                HudHeightModel::A => 34.0,
                HudHeightModel::B => 49.0,
            };
            slab_rect(
                SlabPlace::TopCentre {
                    top: 10.0,
                    margin_left: -260.0,
                },
                520.0,
                height,
                view_w,
                view_h,
            )
        }
    }
}

/// R5 plates (design §2.3). Comfort is not one of them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudModal {
    /// Row 3. Esc pause / Settings.
    Pause,
    /// Row 7. Places plate.
    Places,
    /// Row 1. Launch door is Title.
    Title,
    /// Row 4. Launch door is NameHouse.
    NameHouse,
    /// Row 5. Launch door is HouseDress.
    HouseDress,
    /// Row 6. Persona is open.
    Persona,
}

impl HudModal {
    pub const ALL: [HudModal; 6] = [
        Self::Pause,
        Self::Places,
        Self::Title,
        Self::NameHouse,
        Self::HouseDress,
        Self::Persona,
    ];
}

/// Bands R5 can speak about. Ledger and touch stay up (Q6, Q22).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudBand {
    Hud,
    Ledger,
    Touch,
}

/// R1 (design §2.3). Pairs the code already excludes.
/// Guidance / Practice, CarePrompt / CareStrip (#678), touch stick / Ledger,
/// touch stick / Satchel, Settings / Places.
pub fn r1_code_excludes(left: &str, right: &str) -> bool {
    pair_is(left, right, ID_GUIDANCE, ID_PRACTICE)
        || pair_is(left, right, ID_CARE_PROMPT, ID_CARE_STRIP)
        || pair_is(left, right, "TouchStick", "Ledger")
        || pair_is(left, right, "TouchStick", "Satchel")
        || pair_is(left, right, "Settings", "Places")
}

fn pair_is(left: &str, right: &str, a: &str, b: &str) -> bool {
    (left == a && right == b) || (left == b && right == a)
}

/// R3 (design §2.3, Q2). Class-1 panels never yield. Opening `opened` closes
/// every other class-1 occupant of this anchor. Rank does not pick the winner
/// (Q19 keeps rank for yield). Classes 2, 3, 4, and 5 never push. This does
/// not write a panel's close flag.
pub fn r3_pushes_closed(anchor: &HudAnchor, opened: &str, panel: &str) -> bool {
    if opened == panel {
        return false;
    }
    let opener = anchor.occupant(opened);
    let other = anchor.occupant(panel);
    opener.class == CLASS_PANEL && other.class == CLASS_PANEL
}

/// R4 (design §2.3, Q3). A class-4 or class-5 slab yields while a visible
/// class-1 slab's rectangle overlaps it. Classes 2 and 3 are never covered.
/// Cover only hides. A toast's timer keeps running, so the toast can expire
/// while it is hidden.
pub fn r4_covers(
    panel_open: bool,
    panel_class: u8,
    panel_rect: HudRect,
    other_class: u8,
    other_rect: HudRect,
) -> bool {
    panel_open
        && panel_class == CLASS_PANEL
        && (other_class == CLASS_TOAST || other_class == CLASS_STATUS)
        && panel_rect.overlap_area(other_rect) > 0
}

/// R5 (design §2.3, Q5). The HUD band yields while one of [`HudModal`] is open.
/// The ledger band and the touch band do not yield (Q6, Q22).
pub fn r5_band_yields(modal: HudModal, band: HudBand) -> bool {
    let yields_hud = matches!(
        modal,
        HudModal::Pause
            | HudModal::Places
            | HudModal::Title
            | HudModal::NameHouse
            | HudModal::HouseDress
            | HudModal::Persona
    );
    yields_hud && band == HudBand::Hud
}

/// How an overlapping pair is excluded. `VisibleTogether` is the §3.5 zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudOverlapKind {
    Code,
    SameAnchor,
    Cover,
    VisibleTogether,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HudOverlapPair {
    pub left: &'static str,
    pub right: &'static str,
    pub kind: HudOverlapKind,
    pub area: i32,
    pub left_rect: HudRect,
    pub right_rect: HudRect,
}

/// Counts for one preset, one window, one height model.
///
/// `hud_vs_modal_r5` is design §3.5's last column: each of the 27 HUD slabs
/// against Settings, Places, and Comfort. Comfort counts because it is only
/// on screen while Title or Pause is up, and R5 has already yielded the HUD.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HudOverlapCensus {
    pub visible_together: Vec<HudOverlapPair>,
    pub code: usize,
    pub same_anchor: usize,
    pub cover: usize,
    pub hud_vs_modal_r5: usize,
}

struct CensusNode {
    id: &'static str,
    class: Option<u8>,
    anchor_id: Option<&'static str>,
    rect: HudRect,
}

/// Design §3.5 / §1.4. HUD slabs, the ledger band, and the touch rects.
/// Exclusions, in order: R1, then same anchor (R2 / R3), then R4 cover.
/// Modal plates are counted only in [`HudOverlapCensus::hud_vs_modal_r5`].
pub fn overlap_census(
    preset: &HudPreset,
    view_w: f32,
    view_h: f32,
    model: HudHeightModel,
) -> HudOverlapCensus {
    let mut nodes = Vec::with_capacity(SLAB_METRICS.len() + HudFixedId::ALL.len());
    for anchor in preset.anchors {
        for occupant in anchor.occupants {
            nodes.push(CensusNode {
                id: occupant.id,
                class: Some(occupant.class),
                anchor_id: Some(anchor.id),
                rect: occupant_rect(anchor, occupant, model, view_w, view_h),
            });
        }
    }
    for id in HudFixedId::ALL {
        nodes.push(CensusNode {
            id: id.name(),
            class: None,
            anchor_id: None,
            rect: fixed_rect(id, model, view_w, view_h),
        });
    }

    let mut census = HudOverlapCensus {
        visible_together: Vec::new(),
        code: 0,
        same_anchor: 0,
        cover: 0,
        hud_vs_modal_r5: 0,
    };
    for left_index in 0..nodes.len() {
        for right_index in (left_index + 1)..nodes.len() {
            let left = &nodes[left_index];
            let right = &nodes[right_index];
            let area = left.rect.overlap_area(right.rect);
            if area <= 0 {
                continue;
            }
            let kind = classify(left, right);
            match kind {
                HudOverlapKind::Code => census.code += 1,
                HudOverlapKind::SameAnchor => census.same_anchor += 1,
                HudOverlapKind::Cover => census.cover += 1,
                HudOverlapKind::VisibleTogether => census.visible_together.push(HudOverlapPair {
                    left: left.id,
                    right: right.id,
                    kind,
                    area,
                    left_rect: left.rect,
                    right_rect: right.rect,
                }),
            }
        }
    }
    for anchor in preset.anchors {
        for occupant in anchor.occupants {
            let rect = occupant_rect(anchor, occupant, model, view_w, view_h);
            for plate in HudModalGeom::ALL {
                if rect.overlap_area(modal_rect(plate, model, view_w, view_h)) > 0 {
                    census.hud_vs_modal_r5 += 1;
                }
            }
        }
    }
    census
}

fn classify(left: &CensusNode, right: &CensusNode) -> HudOverlapKind {
    if r1_code_excludes(left.id, right.id) {
        return HudOverlapKind::Code;
    }
    if let (Some(left_anchor), Some(right_anchor)) = (left.anchor_id, right.anchor_id) {
        if left_anchor == right_anchor {
            return HudOverlapKind::SameAnchor;
        }
    }
    if let (Some(left_class), Some(right_class)) = (left.class, right.class) {
        let covered = r4_covers(true, left_class, left.rect, right_class, right.rect)
            || r4_covers(true, right_class, right.rect, left_class, left.rect);
        if covered {
            return HudOverlapKind::Cover;
        }
    }
    HudOverlapKind::VisibleTogether
}

/// Kind of one HUD-slab pair inside a preset. `None` when the rectangles do
/// not overlap. Both ids must be occupants of `preset`.
pub fn slab_overlap(
    preset: &HudPreset,
    left_id: &str,
    right_id: &str,
    view_w: f32,
    view_h: f32,
    model: HudHeightModel,
) -> Option<HudOverlapPair> {
    let (left_anchor, left) = find_occupant(preset, left_id);
    let (right_anchor, right) = find_occupant(preset, right_id);
    let left_rect = occupant_rect(left_anchor, left, model, view_w, view_h);
    let right_rect = occupant_rect(right_anchor, right, model, view_w, view_h);
    let area = left_rect.overlap_area(right_rect);
    if area <= 0 {
        return None;
    }
    let left_node = CensusNode {
        id: left.id,
        class: Some(left.class),
        anchor_id: Some(left_anchor.id),
        rect: left_rect,
    };
    let right_node = CensusNode {
        id: right.id,
        class: Some(right.class),
        anchor_id: Some(right_anchor.id),
        rect: right_rect,
    };
    Some(HudOverlapPair {
        left: left.id,
        right: right.id,
        kind: classify(&left_node, &right_node),
        area,
        left_rect,
        right_rect,
    })
}

fn find_occupant<'a>(preset: &'a HudPreset, id: &str) -> (&'a HudAnchor, &'a HudOccupant) {
    for anchor in preset.anchors {
        if let Some(occupant) = anchor.occupants.iter().find(|occupant| occupant.id == id) {
            return (anchor, occupant);
        }
    }
    panic!("preset {} has no slab {id}", preset.name);
}

pub fn rect_inside_margin(rect: HudRect, view_w: f32, view_h: f32) -> bool {
    let right_limit = (view_w.round() as i32) - EDGE_MARGIN_PX;
    let bottom_limit = (view_h.round() as i32) - EDGE_MARGIN_PX;
    rect.x0 >= EDGE_MARGIN_PX
        && rect.y0 >= EDGE_MARGIN_PX
        && rect.x1 <= right_limit
        && rect.y1 <= bottom_limit
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hud_anchor_registry::{
        coded_joiner, r2_yields_to, ACTION_BAR, ALLOCATE_DOCK, CODED_JOINERS, VOICE,
    };

    fn rect(x0: i32, y0: i32, x1: i32, y1: i32) -> HudRect {
        HudRect { x0, y0, x1, y1 }
    }

    #[test]
    fn t2_reset_target_is_classic() {
        assert_eq!(RESET_PRESET, HudPresetId::Classic);
        assert_eq!(reset_preset().id, HudPresetId::Classic);
        assert_eq!(reset_preset().name, "classic");
        assert!(PRESETS.iter().any(|preset| preset.id == RESET_PRESET));
        assert_eq!(preset(RESET_PRESET).anchors.len(), CLASSIC.anchors.len());
        assert_eq!(HudPresetId::Classic.name(), "classic");
        assert_eq!(HudPresetId::Minimal.name(), "minimal");
        assert_eq!(HudPresetId::Management.name(), "management");
    }

    #[test]
    fn t1_s2_through_s5_zero_visible_together() {
        // §3.5 cells: code / same anchor / cover / HUD-vs-modal R5.
        // Management at 1024 splits cover by model: A 40, B 41.
        let expect = [
            (
                HudPresetId::Classic,
                1024.0,
                640.0,
                HudHeightModel::A,
                4,
                29,
                38,
                56,
            ),
            (
                HudPresetId::Classic,
                1024.0,
                640.0,
                HudHeightModel::B,
                4,
                29,
                38,
                57,
            ),
            (
                HudPresetId::Classic,
                1280.0,
                800.0,
                HudHeightModel::A,
                3,
                29,
                26,
                32,
            ),
            (
                HudPresetId::Classic,
                1280.0,
                800.0,
                HudHeightModel::B,
                3,
                29,
                26,
                37,
            ),
            (
                HudPresetId::Minimal,
                1024.0,
                640.0,
                HudHeightModel::A,
                4,
                72,
                62,
                56,
            ),
            (
                HudPresetId::Minimal,
                1024.0,
                640.0,
                HudHeightModel::B,
                4,
                72,
                62,
                56,
            ),
            (
                HudPresetId::Minimal,
                1280.0,
                800.0,
                HudHeightModel::A,
                3,
                72,
                51,
                38,
            ),
            (
                HudPresetId::Minimal,
                1280.0,
                800.0,
                HudHeightModel::B,
                3,
                72,
                51,
                39,
            ),
            (
                HudPresetId::Management,
                1024.0,
                640.0,
                HudHeightModel::A,
                4,
                36,
                40,
                52,
            ),
            (
                HudPresetId::Management,
                1024.0,
                640.0,
                HudHeightModel::B,
                4,
                36,
                41,
                53,
            ),
            (
                HudPresetId::Management,
                1280.0,
                800.0,
                HudHeightModel::A,
                3,
                36,
                6,
                34,
            ),
            (
                HudPresetId::Management,
                1280.0,
                800.0,
                HudHeightModel::B,
                3,
                36,
                6,
                35,
            ),
        ];
        assert_eq!(expect.len(), PRESETS.len() * PROOF_VIEWS.len() * 2);
        for (id, view_w, view_h, model, code, same, cover, r5) in expect {
            let preset = preset(id);
            let census = overlap_census(preset, view_w, view_h, model);
            assert!(
                census.visible_together.is_empty(),
                "{id:?} {view_w}x{view_h} {model:?} visible-together pairs for Core to rule: {:?}",
                census.visible_together
            );
            assert_eq!(
                (
                    census.code,
                    census.same_anchor,
                    census.cover,
                    census.hud_vs_modal_r5
                ),
                (code, same, cover, r5),
                "{id:?} {view_w}x{view_h} {model:?} §3.5 cell"
            );
            assert_s2_s3_s5(preset, view_w, view_h);
            assert_model_a_inside_model_b(preset, view_w, view_h);
            assert_anchor_gap_or_cover(preset, view_w, view_h);
        }
    }

    fn assert_s2_s3_s5(preset: &HudPreset, view_w: f32, view_h: f32) {
        for anchor in preset.anchors {
            let budget = anchor_rect(anchor, view_w, view_h);
            assert!(
                rect_inside_margin(budget, view_w, view_h),
                "{} {} fails the 16 px margin at {view_w}x{view_h}: {budget:?}",
                preset.name,
                anchor.id
            );
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
            let highest = anchor
                .occupants
                .iter()
                .map(|occupant| occupant.class)
                .min()
                .expect("occupants");
            assert_eq!(anchor.width, widest, "{}", anchor.id);
            assert_eq!(anchor.height_budget, tallest, "{}", anchor.id);
            assert_eq!(anchor.class, highest, "{}", anchor.id);
            assert_eq!(anchor.z_band, HudZBand::Hud);
            for occupant in anchor.occupants {
                assert!(occupant.width <= anchor.width, "{}", occupant.id);
                assert!(occupant.height_b <= anchor.height_budget, "{}", occupant.id);
                let metrics = slab_metrics(occupant.id);
                assert_eq!(occupant.class, metrics.class);
                assert_eq!(occupant.width, metrics.width);
                assert_eq!(occupant.height_b, metrics.height_b);
            }
            for fixed in HudFixedId::ALL {
                let fixed_rect = fixed_rect(fixed, HudHeightModel::B, view_w, view_h);
                let area = budget.overlap_area(fixed_rect);
                assert_eq!(
                    area,
                    0,
                    "{} {} meets {} at {view_w}x{view_h} ({budget:?} × {fixed_rect:?})",
                    preset.name,
                    anchor.id,
                    fixed.name()
                );
            }
        }
    }

    fn assert_model_a_inside_model_b(preset: &HudPreset, view_w: f32, view_h: f32) {
        for anchor in preset.anchors {
            for occupant in anchor.occupants {
                let model_a = occupant_rect(anchor, occupant, HudHeightModel::A, view_w, view_h);
                let model_b = occupant_rect(anchor, occupant, HudHeightModel::B, view_w, view_h);
                assert!(
                    model_a.x0 >= model_b.x0
                        && model_a.y0 >= model_b.y0
                        && model_a.x1 <= model_b.x1
                        && model_a.y1 <= model_b.y1,
                    "{} Model A {:?} is outside Model B {:?}",
                    occupant.id,
                    model_a,
                    model_b
                );
            }
        }
    }

    fn assert_anchor_gap_or_cover(preset: &HudPreset, view_w: f32, view_h: f32) {
        let anchors = preset.anchors;
        for left_index in 0..anchors.len() {
            for right_index in (left_index + 1)..anchors.len() {
                let left = &anchors[left_index];
                let right = &anchors[right_index];
                let left_rect = anchor_rect(left, view_w, view_h);
                let right_rect = anchor_rect(right, view_w, view_h);
                let sep_x = (right_rect.x0 - left_rect.x1).max(left_rect.x0 - right_rect.x1);
                let sep_y = (right_rect.y0 - left_rect.y1).max(left_rect.y0 - right_rect.y1);
                if sep_x < 0 && sep_y < 0 {
                    let cover = (left.class == CLASS_PANEL
                        && (right.class == CLASS_TOAST || right.class == CLASS_STATUS))
                        || (right.class == CLASS_PANEL
                            && (left.class == CLASS_TOAST || left.class == CLASS_STATUS));
                    assert!(
                        cover,
                        "{} {} × {} overlap and are not an R4 cover ({left_rect:?} × {right_rect:?})",
                        preset.name, left.id, right.id
                    );
                    continue;
                }
                let gap = if sep_x < 0 {
                    sep_y
                } else if sep_y < 0 {
                    sep_x
                } else {
                    continue;
                };
                assert!(
                    gap >= ANCHOR_GAP_PX,
                    "{} {} × {} gap {gap} px is under {ANCHOR_GAP_PX}",
                    preset.name,
                    left.id,
                    right.id
                );
            }
        }
    }

    #[test]
    fn t1_seven_accepted_pairs_are_not_visible_together() {
        let seven = [
            (ID_ALLOCATE, ID_HYBRID),
            (ID_ALLOCATE, ID_PICKUP),
            (ID_ALLOCATE, ID_REDEMPTION),
            (ID_ALLOCATE, ID_WHISPER),
            (ID_CLIMATE_STATE, ID_PRACTICE),
            (ID_MERCY, ID_VOICE),
            (ID_ALLOCATE, ID_MERCY),
        ];
        assert_eq!(seven.len(), 7);
        for preset in PRESETS {
            for (view_w, view_h) in PROOF_VIEWS {
                for model in [HudHeightModel::A, HudHeightModel::B] {
                    for (left, right) in seven {
                        match slab_overlap(preset, left, right, view_w, view_h, model) {
                            None => {}
                            Some(pair) => assert_ne!(
                                pair.kind,
                                HudOverlapKind::VisibleTogether,
                                "{} {left} × {right} at {view_w}x{view_h} {model:?} area {} rects {:?} {:?}",
                                preset.name,
                                pair.area,
                                pair.left_rect,
                                pair.right_rect
                            ),
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn t3_s1_each_hud_slab_once_and_s6_cover_and_push() {
        let rows = [
            8, 9, 10, 12, 13, 16, 17, 18, 19, 26, 27, 28, 29, 30, 31, 32, 34, 35, 36, 37, 38, 39,
            40, 41, 42, 43, 44,
        ];
        assert_eq!(rows.len(), 27);
        assert_eq!(SLAB_METRICS.len(), 27);
        for row in [14_u8, 15, 45, 46] {
            assert!(SLAB_METRICS.iter().all(|metrics| metrics.row != row));
        }
        for preset in PRESETS {
            let mut seen = Vec::new();
            for anchor in preset.anchors {
                assert_ne!(anchor.share, HudShare::CodeExclusive);
                if anchor.share == HudShare::Push {
                    assert_eq!(anchor.class, CLASS_PANEL, "{}", anchor.id);
                    assert!(anchor.occupants.len() > 1, "{}", anchor.id);
                    assert!(
                        anchor
                            .occupants
                            .iter()
                            .all(|occupant| occupant.class == CLASS_PANEL),
                        "{}",
                        anchor.id
                    );
                } else {
                    assert!(
                        anchor
                            .occupants
                            .iter()
                            .any(|occupant| occupant.class != CLASS_PANEL)
                            || anchor.occupants.len() == 1,
                        "{} pushes without HudShare::Push",
                        anchor.id
                    );
                }
                for (index, occupant) in anchor.occupants.iter().enumerate() {
                    assert_eq!(occupant.rank, (index + 1) as u8, "{}", occupant.id);
                    assert!(!seen.contains(&occupant.id), "{} twice", occupant.id);
                    seen.push(occupant.id);
                    assert!(rows.contains(&slab_metrics(occupant.id).row));
                }
                if anchor.class == CLASS_PANEL
                    || anchor.class == CLASS_PROMPT
                    || anchor.class == CLASS_TUTOR
                {
                    for other in preset.anchors {
                        if other.id == anchor.id {
                            continue;
                        }
                        if other.class == CLASS_PANEL
                            || other.class == CLASS_PROMPT
                            || other.class == CLASS_TUTOR
                        {
                            for (view_w, view_h) in PROOF_VIEWS {
                                let area = anchor_rect(anchor, view_w, view_h)
                                    .overlap_area(anchor_rect(other, view_w, view_h));
                                assert_eq!(
                                    area, 0,
                                    "{} class {} {} overlaps class {} {}",
                                    preset.name, anchor.class, anchor.id, other.class, other.id
                                );
                            }
                        }
                    }
                }
            }
            assert_eq!(seen.len(), 27, "{}", preset.name);
            for metrics in SLAB_METRICS {
                assert!(
                    seen.contains(&metrics.id),
                    "{} missing {}",
                    preset.name,
                    metrics.id
                );
            }
        }
        assert_eq!(CLASSIC.anchor("PLACE_NAME").offset.x, RIGHT_CLEAR_COLUMN_PX);
        assert_eq!(CLASSIC.anchor("WINDOW").offset.x, RIGHT_CLEAR_COLUMN_PX);
        assert_eq!(MINIMAL.anchor("WINDOW").offset.x, RIGHT_CLEAR_COLUMN_PX);
        assert_eq!(CLASSIC.anchor("CORNER_WATCH").offset.x, RIGHT_CLEAR_USE_PX);
        assert_eq!(CLASSIC.anchor("CORNER_PEER").offset.x, RIGHT_CLEAR_USE_PX);
        assert_eq!(MINIMAL.anchor("CORNER").offset.x, RIGHT_CLEAR_USE_PX);
        assert_eq!(CLASSIC.anchor("TRACKER_1").offset.y, TOUCH_COLUMN_CLEAR_Y);
        assert_eq!(MINIMAL.anchor("EDGE_STATUS").offset.y, TOUCH_COLUMN_CLEAR_Y);
        assert_eq!(MANAGEMENT.anchor("LIST_1").offset.y, TOUCH_COLUMN_CLEAR_Y);
        assert_eq!(EDGE_MARGIN_PX, 16);
        assert_eq!(ANCHOR_GAP_PX, 8);
    }

    #[test]
    fn r3_push_closes_other_class1_panels_only() {
        let window = CLASSIC.anchor("WINDOW");
        assert!(r3_pushes_closed(window, ID_ALLOCATE, ID_MERCY));
        assert!(r3_pushes_closed(window, ID_ALLOCATE, ID_JOURNEY));
        assert!(r3_pushes_closed(window, ID_ALLOCATE, ID_REALM));
        assert!(r3_pushes_closed(window, ID_MERCY, ID_ALLOCATE));
        assert!(!r3_pushes_closed(window, ID_ALLOCATE, ID_ALLOCATE));

        let minimal_window = MINIMAL.anchor("WINDOW");
        assert!(r3_pushes_closed(minimal_window, ID_VOICE, ID_ALLOCATE));
        assert!(r3_pushes_closed(minimal_window, ID_REALM, ID_VOICE));

        let action_bar = CLASSIC.anchor("ACTION_BAR");
        assert!(!r3_pushes_closed(action_bar, ID_CARE_STRIP, ID_GUIDANCE));
        assert!(!r3_pushes_closed(action_bar, ID_GUIDANCE, ID_PRACTICE));
        assert!(!r3_pushes_closed(action_bar, ID_CARE_PROMPT, ID_CARE_STRIP));

        let voice = CLASSIC.anchor("VOICE");
        assert!(!r3_pushes_closed(voice, ID_VOICE, ID_VOICE));
    }

    #[test]
    fn r4_cover_is_only_class1_over_class4_or_5() {
        let window = occupant_rect(
            CLASSIC.anchor("WINDOW"),
            CLASSIC.anchor("WINDOW").occupant(ID_ALLOCATE),
            HudHeightModel::B,
            1024.0,
            640.0,
        );
        let pulse = occupant_rect(
            CLASSIC.anchor("TOP_TOAST"),
            CLASSIC.anchor("TOP_TOAST").occupant(ID_PULSE),
            HudHeightModel::B,
            1024.0,
            640.0,
        );
        assert!(r4_covers(true, CLASS_PANEL, window, CLASS_TOAST, pulse));
        assert!(!r4_covers(false, CLASS_PANEL, window, CLASS_TOAST, pulse));
        assert!(!r4_covers(true, CLASS_TOAST, pulse, CLASS_PANEL, window));
        assert!(!r4_covers(true, CLASS_PANEL, window, CLASS_PROMPT, pulse));
        assert!(!r4_covers(true, CLASS_PANEL, window, CLASS_TUTOR, pulse));
        assert!(!r4_covers(true, CLASS_STATUS, pulse, CLASS_TOAST, pulse));

        let apart = rect(0, 0, 10, 10);
        let other = rect(20, 20, 30, 30);
        assert!(!r4_covers(true, CLASS_PANEL, apart, CLASS_STATUS, other));

        // Classes 2 and 3 on the action bar are not covered by WINDOW's class 1
        // when the rectangles do not overlap, and the predicate refuses them
        // even if a caller passes overlapping rectangles.
        assert!(!r4_covers(true, CLASS_PANEL, window, CLASS_PROMPT, window));
        assert!(!r4_covers(true, CLASS_PANEL, window, CLASS_TUTOR, window));
    }

    #[test]
    fn r5_modal_yields_the_hud_band_only() {
        for modal in HudModal::ALL {
            assert!(r5_band_yields(modal, HudBand::Hud), "{modal:?}");
            assert!(!r5_band_yields(modal, HudBand::Ledger), "{modal:?}");
            assert!(!r5_band_yields(modal, HudBand::Touch), "{modal:?}");
        }
    }

    #[test]
    fn q19_rank_stays_fixed_inside_a_shared_anchor() {
        let toast = CLASSIC.anchor("TOP_TOAST");
        assert!(r2_yields_to(toast, ID_WELCOME, &[(ID_PULSE, true)]));
        assert!(!r2_yields_to(toast, ID_PULSE, &[(ID_WELCOME, true)]));
        assert!(r2_yields_to(toast, ID_SOVEREIGN, &[(ID_PICKUP, true)]));
        assert!(!r2_yields_to(toast, ID_PICKUP, &[(ID_SOVEREIGN, true)]));

        let minimal_toast = MINIMAL.anchor("TOP_TOAST");
        assert!(r2_yields_to(
            minimal_toast,
            ID_PLACE_NAME,
            &[(ID_PULSE, true)]
        ));
        assert!(!r2_yields_to(
            minimal_toast,
            ID_PULSE,
            &[(ID_PLACE_NAME, true)]
        ));

        let action_bar = CLASSIC.anchor("ACTION_BAR");
        assert!(r2_yields_to(
            action_bar,
            ID_GUIDANCE,
            &[(ID_CARE_STRIP, true)]
        ));
        assert!(!r2_yields_to(
            action_bar,
            ID_CARE_STRIP,
            &[(ID_GUIDANCE, true)]
        ));
        assert!(r2_yields_to(
            action_bar,
            ID_PRACTICE,
            &[(ID_GUIDANCE, true)]
        ));
    }

    #[test]
    fn q11_place_name_is_a_candidate_and_q12_climate_is_independent() {
        for preset in PRESETS {
            assert!(preset.anchors.iter().any(|anchor| anchor
                .occupants
                .iter()
                .any(|occupant| occupant.id == ID_PLACE_NAME)));
            assert!(preset.anchors.iter().any(|anchor| {
                anchor
                    .occupants
                    .iter()
                    .any(|occupant| occupant.id == ID_CLIMATE_STATE)
            }));
        }
        for metrics in SLAB_METRICS {
            if metrics.id != ID_PLACE_NAME {
                assert!(!r1_code_excludes(ID_PLACE_NAME, metrics.id));
            }
            if metrics.id != ID_CLIMATE_STATE {
                assert!(!r1_code_excludes(ID_CLIMATE_STATE, metrics.id));
            }
        }
        for name in [
            "Ledger",
            "Satchel",
            "TouchStick",
            "Settings",
            "Places",
            "Comfort",
        ] {
            assert!(!r1_code_excludes(ID_PLACE_NAME, name));
            assert!(!r1_code_excludes(ID_CLIMATE_STATE, name));
        }
    }

    #[test]
    fn metrics_match_registry_model_b() {
        for place in CODED_JOINERS {
            let metrics = slab_metrics(place.id);
            assert_eq!(metrics.row, place.row, "{}", place.id);
            assert_eq!(metrics.class, place.class, "{}", place.id);
            assert_eq!(metrics.width, place.width, "{}", place.id);
            assert_eq!(metrics.height_b, place.height_b, "{}", place.id);
            assert_eq!(coded_joiner(place.id).height_b, metrics.height_b);
        }
        for anchor in [ACTION_BAR, VOICE, ALLOCATE_DOCK] {
            for occupant in anchor.occupants {
                let metrics = slab_metrics(occupant.id);
                assert_eq!(metrics.class, occupant.class, "{}", occupant.id);
                assert_eq!(metrics.width, occupant.width, "{}", occupant.id);
                assert_eq!(metrics.height_b, occupant.height_b, "{}", occupant.id);
            }
        }
        assert_eq!(CLASSIC.anchor("ACTION_BAR"), &ACTION_BAR);
        assert_eq!(MINIMAL.anchor("ACTION_BAR"), &ACTION_BAR);
        assert_eq!(CLASSIC.anchor("VOICE"), &VOICE);
    }

    #[test]
    fn fixed_and_modal_rects_match_design() {
        assert_eq!(
            fixed_rect(HudFixedId::Ledger, HudHeightModel::B, 1024.0, 640.0),
            rect(16, 505, 576, 624)
        );
        assert_eq!(
            fixed_rect(HudFixedId::Ledger, HudHeightModel::B, 1280.0, 800.0),
            rect(16, 665, 576, 784)
        );
        assert_eq!(
            fixed_rect(HudFixedId::Ledger, HudHeightModel::A, 1024.0, 640.0),
            rect(16, 522, 576, 624)
        );
        assert_eq!(
            fixed_rect(HudFixedId::Satchel, HudHeightModel::B, 1024.0, 640.0),
            rect(16, 264, 316, 499)
        );
        assert_eq!(
            fixed_rect(HudFixedId::Satchel, HudHeightModel::B, 1280.0, 800.0),
            rect(16, 389, 316, 624)
        );
        assert_eq!(
            fixed_rect(HudFixedId::TouchStick, HudHeightModel::B, 1024.0, 640.0),
            rect(24, 496, 144, 616)
        );
        assert_eq!(
            fixed_rect(HudFixedId::TouchStick, HudHeightModel::B, 1280.0, 800.0),
            rect(24, 656, 144, 776)
        );
        assert_eq!(
            fixed_rect(HudFixedId::TouchUse, HudHeightModel::B, 1024.0, 640.0),
            rect(952, 560, 996, 604)
        );
        assert_eq!(
            fixed_rect(HudFixedId::TouchUse, HudHeightModel::B, 1280.0, 800.0),
            rect(1208, 720, 1252, 764)
        );
        assert_eq!(
            fixed_rect(HudFixedId::TouchPause, HudHeightModel::A, 1024.0, 640.0),
            rect(956, 24, 1000, 68)
        );
        assert_eq!(
            fixed_rect(HudFixedId::TouchQ, HudHeightModel::A, 1024.0, 640.0),
            rect(956, 76, 1000, 120)
        );
        assert_eq!(
            fixed_rect(HudFixedId::TouchL, HudHeightModel::A, 1024.0, 640.0),
            rect(956, 128, 1000, 172)
        );
        assert_eq!(
            fixed_rect(HudFixedId::TouchPause, HudHeightModel::B, 1280.0, 800.0),
            rect(1212, 24, 1256, 68)
        );
        assert_eq!(
            modal_rect(HudModalGeom::Settings, HudHeightModel::A, 1024.0, 640.0),
            rect(302, 6, 722, 634)
        );
        assert_eq!(
            modal_rect(HudModalGeom::Settings, HudHeightModel::B, 1280.0, 800.0),
            rect(430, 8, 850, 792)
        );
        assert_eq!(
            modal_rect(HudModalGeom::Places, HudHeightModel::A, 1024.0, 640.0),
            rect(312, 115, 712, 515)
        );
        assert_eq!(
            modal_rect(HudModalGeom::Places, HudHeightModel::B, 1280.0, 800.0),
            rect(440, 144, 840, 544)
        );
        assert_eq!(
            modal_rect(HudModalGeom::Comfort, HudHeightModel::A, 1024.0, 640.0),
            rect(252, 10, 772, 44)
        );
        assert_eq!(
            modal_rect(HudModalGeom::Comfort, HudHeightModel::B, 1024.0, 640.0),
            rect(252, 10, 772, 59)
        );
    }

    #[test]
    fn model_b_slab_rects_match_design_3_2_to_3_4() {
        let cases: &[(&HudPreset, &[(&str, HudRect, HudRect)])] = &[
            (
                &CLASSIC,
                &[
                    (ID_FACTORY, rect(16, 93, 536, 145), rect(16, 93, 536, 145)),
                    (
                        ID_VOICE,
                        rect(448, 367, 1008, 419),
                        rect(704, 527, 1264, 579),
                    ),
                    (ID_SPILL, rect(16, 93, 536, 145), rect(16, 93, 536, 145)),
                    (ID_FAB, rect(16, 93, 536, 145), rect(16, 93, 536, 145)),
                    (
                        ID_EMBASSY,
                        rect(588, 180, 1008, 232),
                        rect(844, 180, 1264, 232),
                    ),
                    (
                        ID_REDEMPTION,
                        rect(588, 240, 1008, 292),
                        rect(844, 240, 1264, 292),
                    ),
                    (
                        ID_HYBRID,
                        rect(588, 300, 1008, 352),
                        rect(844, 300, 1264, 352),
                    ),
                    (
                        ID_COMPASS,
                        rect(588, 360, 1008, 412),
                        rect(844, 360, 1264, 412),
                    ),
                    (ID_WELL, rect(16, 153, 436, 205), rect(16, 153, 436, 205)),
                    (
                        ID_GUIDANCE,
                        rect(488, 427, 1008, 496),
                        rect(744, 587, 1264, 656),
                    ),
                    (
                        ID_CARE_PROMPT,
                        rect(548, 440, 1008, 496),
                        rect(804, 600, 1264, 656),
                    ),
                    (ID_PULSE, rect(232, 16, 792, 77), rect(360, 16, 920, 77)),
                    (ID_WELCOME, rect(322, 16, 702, 74), rect(450, 16, 830, 74)),
                    (
                        ID_CARE_STRIP,
                        rect(448, 434, 1008, 496),
                        rect(704, 594, 1264, 656),
                    ),
                    (
                        ID_CLIMATE_STATE,
                        rect(16, 153, 436, 205),
                        rect(16, 153, 436, 205),
                    ),
                    (
                        ID_WATCH,
                        rect(604, 511, 944, 564),
                        rect(860, 671, 1200, 724),
                    ),
                    (ID_PICKUP, rect(332, 16, 692, 72), rect(460, 16, 820, 72)),
                    (ID_SOVEREIGN, rect(252, 16, 772, 72), rect(380, 16, 900, 72)),
                    (
                        ID_PRACTICE,
                        rect(368, 432, 1008, 496),
                        rect(624, 592, 1264, 656),
                    ),
                    (
                        ID_ALLOCATE,
                        rect(428, 16, 948, 151),
                        rect(684, 16, 1204, 151),
                    ),
                    (ID_THRIVING, rect(202, 16, 822, 74), rect(330, 16, 950, 74)),
                    (ID_MERCY, rect(588, 16, 948, 336), rect(844, 16, 1204, 336)),
                    (ID_REALM, rect(648, 16, 948, 186), rect(904, 16, 1204, 186)),
                    (ID_WHISPER, rect(302, 16, 722, 85), rect(430, 16, 850, 85)),
                    (
                        ID_JOURNEY,
                        rect(588, 16, 948, 296),
                        rect(844, 16, 1204, 296),
                    ),
                    (ID_PEER, rect(664, 572, 944, 624), rect(920, 732, 1200, 784)),
                    (
                        ID_PLACE_NAME,
                        rect(668, 93, 948, 124),
                        rect(924, 93, 1204, 124),
                    ),
                ],
            ),
            (
                &MINIMAL,
                &[
                    (
                        ID_FACTORY,
                        rect(488, 180, 1008, 232),
                        rect(744, 180, 1264, 232),
                    ),
                    (ID_VOICE, rect(388, 16, 948, 68), rect(644, 16, 1204, 68)),
                    (
                        ID_SPILL,
                        rect(488, 180, 1008, 232),
                        rect(744, 180, 1264, 232),
                    ),
                    (ID_FAB, rect(488, 180, 1008, 232), rect(744, 180, 1264, 232)),
                    (
                        ID_EMBASSY,
                        rect(588, 180, 1008, 232),
                        rect(844, 180, 1264, 232),
                    ),
                    (
                        ID_REDEMPTION,
                        rect(588, 180, 1008, 232),
                        rect(844, 180, 1264, 232),
                    ),
                    (
                        ID_HYBRID,
                        rect(588, 180, 1008, 232),
                        rect(844, 180, 1264, 232),
                    ),
                    (
                        ID_COMPASS,
                        rect(588, 180, 1008, 232),
                        rect(844, 180, 1264, 232),
                    ),
                    (
                        ID_WELL,
                        rect(588, 180, 1008, 232),
                        rect(844, 180, 1264, 232),
                    ),
                    (
                        ID_GUIDANCE,
                        rect(488, 427, 1008, 496),
                        rect(744, 587, 1264, 656),
                    ),
                    (
                        ID_CARE_PROMPT,
                        rect(548, 440, 1008, 496),
                        rect(804, 600, 1264, 656),
                    ),
                    (ID_PULSE, rect(232, 16, 792, 77), rect(360, 16, 920, 77)),
                    (ID_WELCOME, rect(322, 16, 702, 74), rect(450, 16, 830, 74)),
                    (
                        ID_CARE_STRIP,
                        rect(448, 434, 1008, 496),
                        rect(704, 594, 1264, 656),
                    ),
                    (
                        ID_CLIMATE_STATE,
                        rect(588, 180, 1008, 232),
                        rect(844, 180, 1264, 232),
                    ),
                    (
                        ID_WATCH,
                        rect(604, 571, 944, 624),
                        rect(860, 731, 1200, 784),
                    ),
                    (ID_PICKUP, rect(332, 16, 692, 72), rect(460, 16, 820, 72)),
                    (ID_SOVEREIGN, rect(252, 16, 772, 72), rect(380, 16, 900, 72)),
                    (
                        ID_PRACTICE,
                        rect(368, 432, 1008, 496),
                        rect(624, 592, 1264, 656),
                    ),
                    (
                        ID_ALLOCATE,
                        rect(428, 16, 948, 151),
                        rect(684, 16, 1204, 151),
                    ),
                    (ID_THRIVING, rect(202, 16, 822, 74), rect(330, 16, 950, 74)),
                    (ID_MERCY, rect(588, 16, 948, 336), rect(844, 16, 1204, 336)),
                    (ID_REALM, rect(648, 16, 948, 186), rect(904, 16, 1204, 186)),
                    (ID_WHISPER, rect(302, 16, 722, 85), rect(430, 16, 850, 85)),
                    (
                        ID_JOURNEY,
                        rect(588, 16, 948, 296),
                        rect(844, 16, 1204, 296),
                    ),
                    (ID_PEER, rect(664, 572, 944, 624), rect(920, 732, 1200, 784)),
                    (
                        ID_PLACE_NAME,
                        rect(372, 16, 652, 47),
                        rect(500, 16, 780, 47),
                    ),
                ],
            ),
            (
                &MANAGEMENT,
                &[
                    (
                        ID_FACTORY,
                        rect(488, 180, 1008, 232),
                        rect(744, 180, 1264, 232),
                    ),
                    (ID_VOICE, rect(324, 155, 884, 207), rect(324, 155, 884, 207)),
                    (
                        ID_SPILL,
                        rect(488, 180, 1008, 232),
                        rect(744, 180, 1264, 232),
                    ),
                    (ID_FAB, rect(488, 180, 1008, 232), rect(744, 180, 1264, 232)),
                    (
                        ID_EMBASSY,
                        rect(588, 240, 1008, 292),
                        rect(844, 240, 1264, 292),
                    ),
                    (
                        ID_REDEMPTION,
                        rect(588, 240, 1008, 292),
                        rect(844, 240, 1264, 292),
                    ),
                    (
                        ID_HYBRID,
                        rect(588, 240, 1008, 292),
                        rect(844, 240, 1264, 292),
                    ),
                    (
                        ID_COMPASS,
                        rect(588, 300, 1008, 352),
                        rect(844, 300, 1264, 352),
                    ),
                    (
                        ID_WELL,
                        rect(588, 360, 1008, 412),
                        rect(844, 360, 1264, 412),
                    ),
                    (
                        ID_GUIDANCE,
                        rect(252, 78, 772, 147),
                        rect(380, 78, 900, 147),
                    ),
                    (
                        ID_CARE_PROMPT,
                        rect(282, 78, 742, 134),
                        rect(410, 78, 870, 134),
                    ),
                    (
                        ID_PULSE,
                        rect(448, 435, 1008, 496),
                        rect(704, 595, 1264, 656),
                    ),
                    (
                        ID_WELCOME,
                        rect(628, 438, 1008, 496),
                        rect(884, 598, 1264, 656),
                    ),
                    (
                        ID_CARE_STRIP,
                        rect(232, 78, 792, 140),
                        rect(360, 78, 920, 140),
                    ),
                    (
                        ID_CLIMATE_STATE,
                        rect(588, 360, 1008, 412),
                        rect(844, 360, 1264, 412),
                    ),
                    (ID_WATCH, rect(304, 16, 644, 69), rect(304, 16, 644, 69)),
                    (
                        ID_PICKUP,
                        rect(648, 440, 1008, 496),
                        rect(904, 600, 1264, 656),
                    ),
                    (
                        ID_SOVEREIGN,
                        rect(488, 440, 1008, 496),
                        rect(744, 600, 1264, 656),
                    ),
                    (
                        ID_PRACTICE,
                        rect(192, 78, 832, 142),
                        rect(320, 78, 960, 142),
                    ),
                    (
                        ID_ALLOCATE,
                        rect(324, 155, 844, 290),
                        rect(324, 155, 844, 290),
                    ),
                    (
                        ID_THRIVING,
                        rect(388, 438, 1008, 496),
                        rect(644, 598, 1264, 656),
                    ),
                    (ID_MERCY, rect(324, 155, 684, 475), rect(324, 155, 684, 475)),
                    (ID_REALM, rect(324, 155, 624, 325), rect(324, 155, 624, 325)),
                    (
                        ID_WHISPER,
                        rect(588, 427, 1008, 496),
                        rect(844, 587, 1264, 656),
                    ),
                    (
                        ID_JOURNEY,
                        rect(324, 155, 684, 435),
                        rect(324, 155, 684, 435),
                    ),
                    (ID_PEER, rect(652, 16, 932, 68), rect(652, 16, 932, 68)),
                    (ID_PLACE_NAME, rect(16, 16, 296, 47), rect(16, 16, 296, 47)),
                ],
            ),
        ];
        for &(preset, rows) in cases {
            assert_eq!(rows.len(), 27, "{}", preset.name);
            for &(id, at_1024, at_1280) in rows {
                let (anchor, occupant) = find_occupant(preset, id);
                assert_eq!(
                    occupant_rect(anchor, occupant, HudHeightModel::B, 1024.0, 640.0),
                    at_1024,
                    "{} {id} at 1024",
                    preset.name
                );
                assert_eq!(
                    occupant_rect(anchor, occupant, HudHeightModel::B, 1280.0, 800.0),
                    at_1280,
                    "{} {id} at 1280",
                    preset.name
                );
            }
        }
    }
}
