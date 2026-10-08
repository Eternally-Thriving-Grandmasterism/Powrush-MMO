//! CARD HUD-PRESETS-UI-1 — step 3c of the UI layout epic.
//! Step 3b ([`HudLayoutPlugin`]) still skips `Style` writes while
//! [`ActiveHudPreset`] is `None`. Yield memory still drains in that state, so a
//! hide cannot leave a slab `Hidden` after the id returns to `None`.
//! Boot copies `hud_preset` from `powrush_settings.json` and applies it.
//! Missing, unreadable, or unknown loads `classic`.
//!
//! Ruled: Q1, Q2, Q3, Q5, Q6, Q7, Q8, Q9, Q11, Q12, Q16, Q19, Q20, and Q22.
//! Q16: the saved layout is `data/powrush_hud_layout.json` (step 4a, data only).
//! Q14 touch is culled while edit mode is up. Q17 the yard keeps running in
//! edit mode; Use, the build wheel, and Esc-as-pause are dead there.
//! Open: Q4, Q10, Q13, Q15, Q18, and Q21.
//!
//! Q1 Reset restores classic. Q2 push is accepted in a shared panel slot.
//! Q3 cover is accepted, and a toast may expire while it is hidden.
//! Q5 modal yield is accepted. Q6 the ledger band stays coded and does not yield.
//! Q7 Comfort stays a modal-band plate and is not an R5 trigger.
//! Q8 parked rows 14 and 15 keep spawning, with no anchor, and are not hidden.
//! Q9 rows 16 and 17 (Redemption, Hybrid) are in the preset tables.
//! Q11 place-name stays a showing candidate.
//! Q12 climate state stays independent of every other slab.
//! Q19 rank stays fixed and does not pick an R3 winner.
//! Q20 the gap is 8 px, the margin is 16 px, the touch clears are `right 76`
//! and `right 80`, and the column under the touch buttons starts at y 180.
//! Q22 ledger, satchel, and touch stay coded.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use bevy::render::view::VisibilitySystems;
use bevy::ui::UiSystem;
use bevy::window::PrimaryWindow;

use crate::abundance_journey_echo::AbundanceJourneyEcho;
use crate::coop_voice::VoiceYard;
use crate::hex_travel::PlacesPlate;
use crate::hud_anchor_registry::{
    slab_rect, r2_yields_to, HudAnchor, HudCorner, HudOccupant, HudOffset, HudRect, HudShare, HudSlab, HudZBand,
    SlabPlace, ID_ALLOCATE, ID_CARE_PROMPT, ID_CARE_STRIP, ID_CLIMATE_STATE, ID_COMPASS,
    ID_EMBASSY, ID_FAB, ID_FACTORY, ID_GUIDANCE, ID_HYBRID, ID_JOURNEY, ID_MERCY, ID_PEER,
    ID_PICKUP, ID_PLACE_NAME, ID_PRACTICE, ID_PULSE, ID_REALM, ID_REDEMPTION, ID_SOVEREIGN,
    ID_SPILL, ID_THRIVING, ID_VOICE, ID_WATCH, ID_WELCOME, ID_WELL, ID_WHISPER,
};
use crate::hud_anchor_registry::{clamp_hud_width, write_hud_anchor_style, HUD_REGISTRY_REV};
use shared::hud_layout::{HudLayoutAnchor, HudLayoutCorner};
use crate::human_soft_panels::HumanSoftPanels;
use crate::rbe_allocate_choice::RbeAllocateChoice;
use crate::title_screen::{HouseLabel, LaunchDoor, PersonaCreatorState};

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
        hidden: false,
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

/// One preset: a list of anchors. [`HudLayoutPlugin`] applies it while that id is active.
/// The lifetime lets a pure override check borrow a local anchor list.
#[derive(Clone, Copy, Debug)]
pub struct HudPreset<'a> {
    pub id: HudPresetId,
    pub name: &'static str,
    pub anchors: &'a [HudAnchor],
}

impl HudPreset<'_> {
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

pub const CLASSIC: HudPreset<'static> = HudPreset {
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

pub const MINIMAL: HudPreset<'static> = HudPreset {
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

pub const MANAGEMENT: HudPreset<'static> = HudPreset {
    id: HudPresetId::Management,
    name: "management",
    anchors: &MANAGEMENT_ANCHORS,
};

pub const PRESETS: &[HudPreset<'static>] = &[CLASSIC, MINIMAL, MANAGEMENT];

pub fn preset(id: HudPresetId) -> &'static HudPreset<'static> {
    match id {
        HudPresetId::Classic => &CLASSIC,
        HudPresetId::Minimal => &MINIMAL,
        HudPresetId::Management => &MANAGEMENT,
    }
}

/// Q1. The preset Reset restores.
pub fn reset_preset() -> &'static HudPreset<'static> {
    preset(RESET_PRESET)
}

pub fn slab_metrics(id: &str) -> &'static HudSlabMetrics {
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

/// R3 (design §2.3, Q2). Class-1 panels on a [`HudShare::Push`] anchor never
/// yield. Opening `opened` closes every other class-1 occupant of this anchor.
/// Returns false unless the anchor's share is `Push`. Rank does not pick the
/// winner (Q19). Panics if `opened` or `panel` is not an occupant
/// ([`HudAnchor::occupant`]). This does not write a panel's close flag.
pub fn r3_pushes_closed(anchor: &HudAnchor, opened: &str, panel: &str) -> bool {
    if anchor.share != HudShare::Push || opened == panel {
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

/// Which R5 modals are open. Empty does not yield. Comfort is not a flag (Q7).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HudModalsOpen {
    pub pause: bool,
    pub places: bool,
    pub title: bool,
    pub name_house: bool,
    pub house_dress: bool,
    pub persona: bool,
}

/// R5 (design §2.3, Q5). The HUD band yields while one of these modals is open.
/// The ledger band and the touch band do not yield (Q6, Q22).
pub fn r5_band_yields(open: HudModalsOpen, band: HudBand) -> bool {
    if band != HudBand::Hud {
        return false;
    }
    open.pause
        || open.places
        || open.title
        || open.name_house
        || open.house_dress
        || open.persona
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
    preset: &HudPreset<'_>,
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
    preset: &HudPreset<'_>,
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

fn find_occupant<'a>(preset: &'a HudPreset<'_>, id: &str) -> (&'a HudAnchor, &'a HudOccupant) {
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

/// Result of [`apply_overrides`]. F5 returns [`HudOverrideApply::Refused`]
/// with nothing written. F4 returns [`HudOverrideApply::StaleRev`].
#[derive(Clone, Debug, PartialEq)]
pub enum HudOverrideApply {
    Applied(Vec<HudAnchor>),
    /// F4. `registry_rev` is not [`HUD_REGISTRY_REV`]. Every override is dropped.
    StaleRev,
    /// F5. Unknown id, non-finite number, width outside §6.2, `hidden` on a
    /// class 1–3 anchor, or a duplicate id. No partial apply.
    Refused,
}

/// What to keep after apply plus the §6.4 save check. F6 is [`HudLayoutChoice::Fallback`].
#[derive(Clone, Debug, PartialEq)]
pub enum HudLayoutChoice {
    Applied(Vec<HudAnchor>),
    Fallback(HudPresetId),
}

/// §5 / §6.3. Apply every override, or none. A stale [`HUD_REGISTRY_REV`] drops
/// every override (F4). Unknown ids, non-finite numbers, widths outside the
/// §6.2 clamp at this window and at both proof sizes, and `hidden` on class
/// 1–3 anchors are F5.
pub fn apply_overrides(
    base: &HudPreset<'_>,
    registry_rev: u32,
    overrides: &[HudLayoutAnchor],
    view_w: f32,
    view_h: f32,
) -> HudOverrideApply {
    if registry_rev != HUD_REGISTRY_REV {
        return HudOverrideApply::StaleRev;
    }
    if overrides.is_empty() {
        return HudOverrideApply::Applied(base.anchors.iter().copied().collect());
    }
    if !view_w.is_finite() || !view_h.is_finite() {
        return HudOverrideApply::Refused;
    }
    let mut anchors: Vec<HudAnchor> = base.anchors.iter().copied().collect();
    let mut seen = Vec::with_capacity(overrides.len());
    let mut pending = Vec::with_capacity(overrides.len());
    for over in overrides {
        if !over.x.is_finite() || !over.y.is_finite() || !over.width.is_finite() {
            return HudOverrideApply::Refused;
        }
        let Some(index) = anchors.iter().position(|anchor| anchor.id == over.id) else {
            return HudOverrideApply::Refused;
        };
        if seen.contains(&index) {
            return HudOverrideApply::Refused;
        }
        seen.push(index);
        let anchor = &anchors[index];
        if over.hidden && anchor_blocks_hide(anchor) {
            return HudOverrideApply::Refused;
        }
        if width_outside_clamp(over.width, anchor.width, view_w) {
            return HudOverrideApply::Refused;
        }
        pending.push((index, over));
    }
    for (index, over) in pending {
        let anchor = &mut anchors[index];
        anchor.corner = layout_corner(over.corner);
        anchor.offset = HudOffset {
            x: over.x,
            y: over.y,
        };
        anchor.width = over.width;
        anchor.hidden = over.hidden;
    }
    HudOverrideApply::Applied(anchors)
}

/// §4.2 / §6.4. S1–S6 at `view_w` × `view_h` and at both proof windows.
pub fn hud_save_allowed(layout: &HudPreset<'_>, view_w: f32, view_h: f32) -> bool {
    let views = [(view_w, view_h), PROOF_VIEWS[0], PROOF_VIEWS[1]];
    views.iter().all(|(w, h)| save_allowed_at(layout, *w, *h))
}

/// §5 F7. S2–S4 at the current window only. A failure applies `base` for the
/// session and does not write the save.
pub fn hud_resize_holds(layout: &HudPreset<'_>, view_w: f32, view_h: f32) -> bool {
    view_w.is_finite()
        && view_h.is_finite()
        && s2_inside_margin(layout, view_w, view_h)
        && s3_coded_fit(layout)
        && resize_s4_clear(layout, view_w, view_h)
}

fn resize_s4_clear(layout: &HudPreset<'_>, view_w: f32, view_h: f32) -> bool {
    let mut shown: Vec<HudAnchor> = layout.anchors.iter().copied().collect();
    for anchor in &mut shown {
        if anchor.hidden {
            anchor.occupants = &[];
        }
    }
    let shown_layout = HudPreset {
        id: layout.id,
        name: layout.name,
        anchors: &shown,
    };
    [HudHeightModel::A, HudHeightModel::B]
        .into_iter()
        .all(|model| {
            overlap_census(&shown_layout, view_w, view_h, model)
                .visible_together
                .is_empty()
        })
}

/// Apply overrides, then F6: a failed save check falls back to `base`, or to
/// classic when `base` itself fails S1–S6.
pub fn resolve_hud_layout(
    base: &HudPreset<'_>,
    registry_rev: u32,
    overrides: &[HudLayoutAnchor],
    view_w: f32,
    view_h: f32,
) -> HudLayoutChoice {
    match apply_overrides(base, registry_rev, overrides, view_w, view_h) {
        HudOverrideApply::Applied(anchors) => {
            let layout = HudPreset {
                id: base.id,
                name: base.name,
                anchors: &anchors,
            };
            if hud_save_allowed(&layout, view_w, view_h) {
                HudLayoutChoice::Applied(anchors)
            } else {
                fallback_base(base, view_w, view_h)
            }
        }
        HudOverrideApply::StaleRev | HudOverrideApply::Refused => {
            fallback_base(base, view_w, view_h)
        }
    }
}

fn fallback_base(base: &HudPreset<'_>, view_w: f32, view_h: f32) -> HudLayoutChoice {
    if hud_save_allowed(base, view_w, view_h) {
        HudLayoutChoice::Fallback(base.id)
    } else {
        HudLayoutChoice::Fallback(RESET_PRESET)
    }
}

fn anchor_blocks_hide(anchor: &HudAnchor) -> bool {
    anchor.class <= CLASS_TUTOR
        || anchor
            .occupants
            .iter()
            .any(|occupant| occupant.class <= CLASS_TUTOR)
}

fn width_outside_clamp(width: f32, coded: f32, view_w: f32) -> bool {
    let views = [view_w, PROOF_VIEWS[0].0, PROOF_VIEWS[1].0];
    views
        .iter()
        .any(|view| clamp_hud_width(width, coded, *view) != width)
}

fn layout_corner(corner: HudLayoutCorner) -> HudCorner {
    match corner {
        HudLayoutCorner::TopLeft => HudCorner::TopLeft,
        HudLayoutCorner::TopCentre => HudCorner::TopCentre,
        HudLayoutCorner::TopRight => HudCorner::TopRight,
        HudLayoutCorner::BottomLeft => HudCorner::BottomLeft,
        HudLayoutCorner::BottomCentre => HudCorner::BottomCentre,
        HudLayoutCorner::BottomRight => HudCorner::BottomRight,
    }
}

fn save_allowed_at(layout: &HudPreset<'_>, view_w: f32, view_h: f32) -> bool {
    s1_each_slab_once(layout)
        && s2_inside_margin(layout, view_w, view_h)
        && s3_coded_fit(layout)
        && s4_s5_no_visible_together(layout, view_w, view_h)
        && s6_class_limits(layout, view_w, view_h)
}

fn s1_each_slab_once(layout: &HudPreset<'_>) -> bool {
    let mut seen = Vec::new();
    for anchor in layout.anchors {
        for occupant in anchor.occupants {
            if seen.contains(&occupant.id) {
                return false;
            }
            let Some(metrics) = SLAB_METRICS
                .iter()
                .find(|metrics| metrics.id == occupant.id)
            else {
                return false;
            };
            if matches!(metrics.row, 14 | 15 | 45 | 46) {
                return false;
            }
            seen.push(occupant.id);
        }
    }
    seen.len() == SLAB_METRICS.len()
        && SLAB_METRICS
            .iter()
            .all(|metrics| seen.contains(&metrics.id))
}

fn s2_inside_margin(layout: &HudPreset<'_>, view_w: f32, view_h: f32) -> bool {
    layout
        .anchors
        .iter()
        .all(|anchor| rect_inside_margin(anchor_rect(anchor, view_w, view_h), view_w, view_h))
}

fn s3_coded_fit(layout: &HudPreset<'_>) -> bool {
    layout.anchors.iter().all(|anchor| {
        anchor.occupants.iter().all(|occupant| {
            let metrics = slab_metrics(occupant.id);
            metrics.width <= anchor.width && metrics.height_b <= anchor.height_budget
        })
    })
}

fn s4_s5_no_visible_together(layout: &HudPreset<'_>, view_w: f32, view_h: f32) -> bool {
    // A hidden anchor is not showing (§6.3), so its occupants are not a visible pair.
    let mut shown: Vec<HudAnchor> = layout.anchors.iter().copied().collect();
    for anchor in &mut shown {
        if anchor.hidden {
            anchor.occupants = &[];
        }
    }
    let shown_layout = HudPreset {
        id: layout.id,
        name: layout.name,
        anchors: &shown,
    };
    let no_visible = [HudHeightModel::A, HudHeightModel::B]
        .into_iter()
        .all(|model| {
            overlap_census(&shown_layout, view_w, view_h, model)
                .visible_together
                .is_empty()
        });
    no_visible && s5_anchors_clear_fixed(layout, view_w, view_h)
}

/// S5. The anchor's budget rect misses every fixed rect. R1's fixed pairs are
/// fixed-to-fixed (touch stick / ledger, touch stick / satchel), not HUD anchors.
fn s5_anchors_clear_fixed(layout: &HudPreset<'_>, view_w: f32, view_h: f32) -> bool {
    layout.anchors.iter().all(|anchor| {
        let budget = anchor_rect(anchor, view_w, view_h);
        HudFixedId::ALL.iter().all(|fixed| {
            [HudHeightModel::A, HudHeightModel::B]
                .into_iter()
                .all(|model| budget.overlap_area(fixed_rect(*fixed, model, view_w, view_h)) == 0)
        })
    })
}

fn s6_class_limits(layout: &HudPreset<'_>, view_w: f32, view_h: f32) -> bool {
    for anchor in layout.anchors {
        if anchor.hidden && anchor_blocks_hide(anchor) {
            return false;
        }
        if anchor.share == HudShare::Push {
            if anchor.class != CLASS_PANEL
                || anchor
                    .occupants
                    .iter()
                    .any(|occupant| occupant.class != CLASS_PANEL)
            {
                return false;
            }
        } else {
            let class1 = anchor
                .occupants
                .iter()
                .filter(|occupant| occupant.class == CLASS_PANEL)
                .count();
            if class1 > 1 {
                return false;
            }
        }
    }
    let guarded: Vec<&str> = layout
        .anchors
        .iter()
        .filter(|anchor| anchor.class <= CLASS_TUTOR)
        .flat_map(|anchor| anchor.occupants.iter())
        .filter(|occupant| occupant.class <= CLASS_TUTOR)
        .map(|occupant| occupant.id)
        .collect();
    for left_index in 0..guarded.len() {
        for right_index in (left_index + 1)..guarded.len() {
            let left_id = guarded[left_index];
            let right_id = guarded[right_index];
            let (left_anchor, _) = find_occupant(layout, left_id);
            let (right_anchor, _) = find_occupant(layout, right_id);
            if left_anchor.id == right_anchor.id {
                continue;
            }
            for model in [HudHeightModel::A, HudHeightModel::B] {
                if slab_overlap(layout, left_id, right_id, view_w, view_h, model).is_some() {
                    return false;
                }
            }
        }
    }
    true
}

/// Occupant width stays coded unless this anchor's override changed the width.
fn slab_width_for(
    using_overrides: bool,
    base: &HudPreset<'_>,
    anchor: &HudAnchor,
    occupant: &HudOccupant,
) -> f32 {
    if !using_overrides {
        return occupant.width;
    }
    match base.anchors.iter().find(|item| item.id == anchor.id) {
        Some(original) if (original.width - anchor.width).abs() <= 0.5 => occupant.width,
        _ => anchor.width,
    }
}

/// Saved base plus overrides, or the active preset table when no override is applied.
fn session_layout<'a>(
    id: HudPresetId,
    session: &'a HudSessionLayout,
    base: &'a HudPreset<'static>,
) -> HudPreset<'a> {
    if let Some(anchors) = session.anchors.as_deref() {
        HudPreset {
            id: session.base.unwrap_or(id),
            name: base.name,
            anchors,
        }
    } else {
        *base
    }
}

fn slab_on<'a>(preset: &'a HudPreset<'_>, id: &str) -> Option<(&'a HudAnchor, &'a HudOccupant)> {
    for anchor in preset.anchors {
        if let Some(occupant) = anchor.occupants.iter().find(|occupant| occupant.id == id) {
            return Some((anchor, occupant));
        }
    }
    None
}

/// Chosen preset. `None` is the dark default: layout systems return before
/// any `Style` or `Visibility` write.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveHudPreset {
    pub id: Option<HudPresetId>,
}

/// Applied anchor list when a save or an edit session differs from the preset table.
/// `anchors: None` keeps the preset table, which is the no-file visual path.
#[derive(Resource, Clone, Debug, Default)]
pub struct HudSessionLayout {
    pub base: Option<HudPresetId>,
    pub anchors: Option<Vec<HudAnchor>>,
}

impl Default for ActiveHudPreset {
    fn default() -> Self {
        Self { id: None }
    }
}

/// Sets [`ActiveHudPreset`]. Reset always targets [`RESET_PRESET`].
/// There is no command that returns the preset to `None`.
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudLayoutCommand {
    Reset,
    Apply(HudPresetId),
}

/// Own `Visibility` captured when a yield, cover, or modal hid the slab.
#[derive(Resource, Default)]
struct HudYieldMemory {
    own: HashMap<Entity, Visibility>,
}

/// Previous-frame open flags for R3. Not a panel flag.
#[derive(Clone, Copy, Default)]
struct PushEdges {
    voice: bool,
    allocate: bool,
    mercy: bool,
    realm: bool,
    journey: bool,
}

/// Dark until a preset is chosen. R3 push uses `.after` on the four panel
/// toggles. Plugin registration order does not set system order.
pub struct HudLayoutPlugin;

impl Plugin for HudLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveHudPreset>()
            .init_resource::<HudSessionLayout>()
            .init_resource::<HudYieldMemory>()
            .add_event::<HudLayoutCommand>()
            .add_systems(PreUpdate, restore_yielded_visibility)
            .add_systems(
                Update,
                (
                    apply_hud_layout_commands,
                    push_shared_panels
                        .after(crate::coop_voice::handle_voice)
                        .after(crate::rbe_allocate_choice::toggle_allocate_panel)
                        .after(crate::human_soft_panels::toggle_soft_panels)
                        .after(crate::abundance_journey_echo::toggle_echo_panel),
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                apply_active_preset
                    .before(UiSystem::Layout)
                    .before(VisibilitySystems::VisibilityPropagate),
            );
    }
}

pub(crate) fn apply_hud_layout_commands(
    mut commands: EventReader<HudLayoutCommand>,
    mut active: ResMut<ActiveHudPreset>,
    mut session: ResMut<HudSessionLayout>,
    edit: Option<Res<crate::hud_edit_mode::HudEditMode>>,
) {
    let mut chosen = None;
    for command in commands.read() {
        chosen = Some(match *command {
            HudLayoutCommand::Reset => RESET_PRESET,
            HudLayoutCommand::Apply(id) => id,
        });
    }
    if edit.as_ref().is_some_and(|mode| mode.active) {
        return;
    }
    let Some(id) = chosen else {
        return;
    };
    session.anchors = None;
    session.base = None;
    if active.id != Some(id) {
        active.id = Some(id);
    }
}

fn restore_yielded_visibility(
    mut memory: ResMut<HudYieldMemory>,
    mut vis: Query<&mut Visibility>,
) {
    // Drain when the id is `None` too. A hide stores the slab's own
    // visibility; skipping the drain strands that slab at Hidden.
    let drained: Vec<(Entity, Visibility)> = memory.own.drain().collect();
    for (entity, own) in drained {
        let Ok(mut current) = vis.get_mut(entity) else {
            continue;
        };
        if *current != own {
            *current = own;
        }
    }
}

fn push_shared_panels(
    active: Res<ActiveHudPreset>,
    session: Res<HudSessionLayout>,
    mut edges: Local<PushEdges>,
    mut voice: Option<ResMut<VoiceYard>>,
    mut allocate: Option<ResMut<RbeAllocateChoice>>,
    mut soft: Option<ResMut<HumanSoftPanels>>,
    mut journey: Option<ResMut<AbundanceJourneyEcho>>,
) {
    if active.id.is_none() {
        *edges = PushEdges::default();
        return;
    }
    let id = active.id.expect("preset id");
    let base = preset(id);
    let layout = session_layout(id, &session, base);
    let voice_open = voice.as_ref().is_some_and(|yard| yard.sash_open);
    let allocate_open = allocate.as_ref().is_some_and(|choice| choice.panel_open);
    let mercy_open = soft.as_ref().is_some_and(|panels| panels.mercy_open);
    let realm_open = soft.as_ref().is_some_and(|panels| panels.realm_open);
    let journey_open = journey.as_ref().is_some_and(|echo| echo.panel_open);

    struct Slot {
        id: &'static str,
        rank: u8,
        anchor_id: &'static str,
        open: bool,
        rose: bool,
    }
    let mut slots = Vec::new();
    for anchor in layout.anchors {
        if anchor.share != HudShare::Push {
            continue;
        }
        for occupant in anchor.occupants {
            let (open, was_open) = match occupant.id {
                ID_VOICE => (voice_open, edges.voice),
                ID_ALLOCATE => (allocate_open, edges.allocate),
                ID_MERCY => (mercy_open, edges.mercy),
                ID_REALM => (realm_open, edges.realm),
                ID_JOURNEY => (journey_open, edges.journey),
                _ => continue,
            };
            slots.push(Slot {
                id: occupant.id,
                rank: occupant.rank,
                anchor_id: anchor.id,
                open,
                rose: open && !was_open,
            });
        }
    }

    let risers: Vec<&Slot> = slots.iter().filter(|slot| slot.rose).collect();
    let winner = if risers.len() == 1 {
        Some(risers[0].id)
    } else if risers.len() > 1 {
        risers.iter().min_by_key(|slot| slot.rank).map(|slot| slot.id)
    } else {
        let open: Vec<&Slot> = slots.iter().filter(|slot| slot.open).collect();
        if open.len() > 1 {
            open.iter().min_by_key(|slot| slot.rank).map(|slot| slot.id)
        } else {
            None
        }
    };

    if let Some(winner_id) = winner {
        let winner_anchor = slots
            .iter()
            .find(|slot| slot.id == winner_id)
            .map(|slot| slot.anchor_id)
            .expect("r3 winner");
        let anchor = layout.anchor(winner_anchor);
        let mut close_voice = false;
        let mut close_allocate = false;
        let mut close_mercy = false;
        let mut close_realm = false;
        let mut close_journey = false;
        for slot in &slots {
            if slot.anchor_id != winner_anchor || !slot.open {
                continue;
            }
            if r3_pushes_closed(anchor, winner_id, slot.id) {
                match slot.id {
                    ID_VOICE => close_voice = true,
                    ID_ALLOCATE => close_allocate = true,
                    ID_MERCY => close_mercy = true,
                    ID_REALM => close_realm = true,
                    ID_JOURNEY => close_journey = true,
                    _ => {}
                }
            }
        }
        if close_voice && voice.as_ref().is_some_and(|yard| yard.sash_open) {
            if let Some(yard) = voice.as_mut() {
                yard.sash_open = false;
            }
        }
        if close_allocate && allocate.as_ref().is_some_and(|choice| choice.panel_open) {
            if let Some(choice) = allocate.as_mut() {
                choice.panel_open = false;
            }
        }
        if close_mercy && soft.as_ref().is_some_and(|panels| panels.mercy_open) {
            if let Some(panels) = soft.as_mut() {
                panels.mercy_open = false;
            }
        }
        if close_realm && soft.as_ref().is_some_and(|panels| panels.realm_open) {
            if let Some(panels) = soft.as_mut() {
                panels.realm_open = false;
            }
        }
        if close_journey && journey.as_ref().is_some_and(|echo| echo.panel_open) {
            if let Some(echo) = journey.as_mut() {
                echo.panel_open = false;
            }
        }
    }

    edges.voice = voice.as_ref().is_some_and(|yard| yard.sash_open);
    edges.allocate = allocate.as_ref().is_some_and(|choice| choice.panel_open);
    edges.mercy = soft.as_ref().is_some_and(|panels| panels.mercy_open);
    edges.realm = soft.as_ref().is_some_and(|panels| panels.realm_open);
    edges.journey = journey.as_ref().is_some_and(|echo| echo.panel_open);
}

struct LiveSlab {
    entity: Entity,
    id: &'static str,
    anchor_id: &'static str,
    class: u8,
    wants: bool,
    rect: Option<HudRect>,
}

fn apply_active_preset(
    active: Res<ActiveHudPreset>,
    session: Res<HudSessionLayout>,
    edit: Option<Res<crate::hud_edit_mode::HudEditMode>>,
    mut memory: ResMut<HudYieldMemory>,
    mut styles: Query<(&HudSlab, &mut Style)>,
    mut vis_q: Query<(Entity, &HudSlab, &mut Visibility)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    door: Option<Res<LaunchDoor>>,
    places: Option<Res<PlacesPlate>>,
    house: Option<Res<HouseLabel>>,
    persona: Option<Res<PersonaCreatorState>>,
) {
    let Some(id) = active.id else {
        return;
    };
    let base = preset(id);
    let using_overrides = session.anchors.is_some();
    let layout = session_layout(id, &session, base);
    let view = windows
        .iter()
        .next()
        .map(|window| (window.width(), window.height()));
    let door = door.as_deref().copied();
    let modals = HudModalsOpen {
        pause: house.as_ref().is_some_and(|label| label.settings_open),
        places: places.as_ref().is_some_and(|plate| plate.open),
        title: door == Some(LaunchDoor::Title),
        name_house: door == Some(LaunchDoor::NameHouse),
        house_dress: door == Some(LaunchDoor::HouseDress),
        persona: persona.as_ref().is_some_and(|state| state.open),
    };
    // §6.1. R5 stops hiding the HUD band while edit mode is up.
    let yield_hud = !edit.as_ref().is_some_and(|mode| mode.active)
        && r5_band_yields(modals, HudBand::Hud);

    for (slab, mut style) in &mut styles {
        if let Some((anchor, occupant)) = slab_on(&layout, slab.0) {
            let width = slab_width_for(using_overrides, base, anchor, occupant);
            write_hud_anchor_style(&mut style, anchor.corner, anchor.offset, width);
        }
    }

    let mut live = Vec::new();
    for (entity, slab, vis) in vis_q.iter() {
        let wants = *vis == Visibility::Visible;
        if let Some((anchor, occupant)) = slab_on(&layout, slab.0) {
            let rect = view.map(|(view_w, view_h)| {
                occupant_rect(anchor, occupant, HudHeightModel::B, view_w, view_h)
            });
            live.push(LiveSlab {
                entity,
                id: slab.0,
                anchor_id: anchor.id,
                class: occupant.class,
                wants,
                rect,
            });
        } else {
            live.push(LiveSlab {
                entity,
                id: slab.0,
                anchor_id: "",
                class: 0,
                wants,
                rect: None,
            });
        }
    }

    let mut hide = HashSet::new();
    if yield_hud {
        for item in &live {
            hide.insert(item.entity);
        }
    } else {
        // §6.3. A hidden class 4 or 5 anchor hides its occupants. Keys keep running.
        for anchor in layout.anchors {
            if !anchor.hidden {
                continue;
            }
            for item in &live {
                if item.anchor_id == anchor.id {
                    hide.insert(item.entity);
                }
            }
        }
        for anchor in layout.anchors {
            if anchor.share != HudShare::Yield {
                continue;
            }
            let showing: Vec<(&str, bool)> = anchor
                .occupants
                .iter()
                .map(|occupant| {
                    let on = live.iter().any(|item| {
                        item.id == occupant.id && item.anchor_id == anchor.id && item.wants
                    });
                    (occupant.id, on)
                })
                .collect();
            for item in &live {
                if item.anchor_id == anchor.id && r2_yields_to(anchor, item.id, &showing) {
                    hide.insert(item.entity);
                }
            }
        }
        if view.is_some() {
            for item in &live {
                let Some(item_rect) = item.rect else {
                    continue;
                };
                let covered = live.iter().any(|panel| {
                    let Some(panel_rect) = panel.rect else {
                        return false;
                    };
                    panel.wants
                        && panel.anchor_id != item.anchor_id
                        && r4_covers(true, panel.class, panel_rect, item.class, item_rect)
                });
                if covered {
                    hide.insert(item.entity);
                }
            }
        }
    }

    memory.own.clear();
    for (entity, _slab, mut vis) in &mut vis_q {
        if !hide.contains(&entity) {
            continue;
        }
        let own = *vis;
        memory.own.insert(entity, own);
        if own != Visibility::Hidden {
            *vis = Visibility::Hidden;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ui::FocusPolicy;
    use shared::house_name::HouseName;

    use crate::abundance_journey_echo::AbundanceJourneyEcho;
    use crate::coop_voice::VoiceYard;
    use crate::first_session_guidance::FirstSessionGuidance;
    use crate::hex_travel::{PlacesPlate, PLACES_PLATE_Z};
    use crate::ui_above_world::{
        LivedUiPlate, LIVED_UI_Z_LEDGER, LIVED_UI_Z_PAUSE, LIVED_UI_Z_TITLE,
    };
    use crate::hud_anchor_registry::{
        coded_joiner, r2_yields_to, HudSlab, ACTION_BAR, ALLOCATE_DOCK, CODED_JOINERS, VOICE,
    };
    use crate::human_soft_panels::HumanSoftPanels;
    use crate::rbe_allocate_choice::RbeAllocateChoice;
    use crate::title_screen::{HouseLabel, LaunchDoor, PersonaCreatorState, TITLE_SAFE_INSET};
    use crate::touch_controls::TOUCH_HIT_MIN;

    fn rect(x0: i32, y0: i32, x1: i32, y1: i32) -> HudRect {
        HudRect { x0, y0, x1, y1 }
    }

    const YIELD_TWO_PANELS: [HudOccupant; 2] = [occ(ID_ALLOCATE, 1), occ(ID_MERCY, 2)];

    fn yield_anchor_two_class1() -> HudAnchor {
        anchor(AnchorFields {
            id: "YIELD_TWO",
            corner: HudCorner::TopRight,
            x: 16.0,
            y: 16.0,
            width: 520.0,
            height_budget: 320.0,
            class: CLASS_PANEL,
            occupants: &YIELD_TWO_PANELS,
            share: HudShare::Yield,
        })
    }

    /// Pinned kinds for the seven accepted pairs. `None` is area 0.
    /// Cover becoming `None` fails this helper's caller on its own.
    fn seven_pair_kind(
        preset: HudPresetId,
        view_w: f32,
        left: &str,
        right: &str,
    ) -> Option<HudOverlapKind> {
        let same = |a: &str, b: &str| (left == a && right == b) || (left == b && right == a);
        let wide = view_w < 1100.0;
        match preset {
            HudPresetId::Classic => {
                if same(ID_ALLOCATE, ID_PICKUP) || same(ID_ALLOCATE, ID_WHISPER) {
                    Some(HudOverlapKind::Cover)
                } else if same(ID_ALLOCATE, ID_MERCY) {
                    Some(HudOverlapKind::SameAnchor)
                } else {
                    None
                }
            }
            HudPresetId::Minimal => {
                if same(ID_ALLOCATE, ID_PICKUP) || same(ID_ALLOCATE, ID_WHISPER) {
                    Some(HudOverlapKind::Cover)
                } else if same(ID_MERCY, ID_VOICE) || same(ID_ALLOCATE, ID_MERCY) {
                    Some(HudOverlapKind::SameAnchor)
                } else {
                    None
                }
            }
            HudPresetId::Management => {
                if wide && (same(ID_ALLOCATE, ID_HYBRID) || same(ID_ALLOCATE, ID_REDEMPTION)) {
                    Some(HudOverlapKind::Cover)
                } else if same(ID_MERCY, ID_VOICE) || same(ID_ALLOCATE, ID_MERCY) {
                    Some(HudOverlapKind::SameAnchor)
                } else {
                    None
                }
            }
        }
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

    fn assert_s2_s3_s5(preset: &HudPreset<'_>, view_w: f32, view_h: f32) {
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

    fn assert_model_a_inside_model_b(preset: &HudPreset<'_>, view_w: f32, view_h: f32) {
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

    fn assert_anchor_gap_or_cover(preset: &HudPreset<'_>, view_w: f32, view_h: f32) {
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
                        let got = slab_overlap(preset, left, right, view_w, view_h, model)
                            .map(|pair| pair.kind);
                        let expect = seven_pair_kind(preset.id, view_w, left, right);
                        assert_eq!(
                            got, expect,
                            "{} {left} × {right} at {view_w}x{view_h} {model:?}",
                            preset.name
                        );
                        if let Some(kind) = got {
                            assert_ne!(kind, HudOverlapKind::VisibleTogether);
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
                    let class1 = anchor
                        .occupants
                        .iter()
                        .filter(|occupant| occupant.class == CLASS_PANEL)
                        .count();
                    assert!(
                        class1 <= 1,
                        "{} share {:?} holds {class1} class-1 panels",
                        anchor.id,
                        anchor.share
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

        let yield_two = yield_anchor_two_class1();
        assert_eq!(yield_two.share, HudShare::Yield);
        assert!(yield_two.occupants.len() > 1);
        assert!(yield_two
            .occupants
            .iter()
            .all(|occupant| occupant.class == CLASS_PANEL));
        assert!(!r3_pushes_closed(&yield_two, ID_ALLOCATE, ID_MERCY));
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
        let none = HudModalsOpen {
            pause: false,
            places: false,
            title: false,
            name_house: false,
            house_dress: false,
            persona: false,
        };
        assert!(!r5_band_yields(none, HudBand::Hud));
        assert!(!r5_band_yields(none, HudBand::Ledger));
        assert!(!r5_band_yields(none, HudBand::Touch));

        let pause = HudModalsOpen {
            pause: true,
            places: false,
            title: false,
            name_house: false,
            house_dress: false,
            persona: false,
        };
        let places = HudModalsOpen {
            pause: false,
            places: true,
            title: false,
            name_house: false,
            house_dress: false,
            persona: false,
        };
        let title = HudModalsOpen {
            pause: false,
            places: false,
            title: true,
            name_house: false,
            house_dress: false,
            persona: false,
        };
        let name_house = HudModalsOpen {
            pause: false,
            places: false,
            title: false,
            name_house: true,
            house_dress: false,
            persona: false,
        };
        let house_dress = HudModalsOpen {
            pause: false,
            places: false,
            title: false,
            name_house: false,
            house_dress: true,
            persona: false,
        };
        let persona = HudModalsOpen {
            pause: false,
            places: false,
            title: false,
            name_house: false,
            house_dress: false,
            persona: true,
        };
        for open in [pause, places, title, name_house, house_dress, persona] {
            assert!(r5_band_yields(open, HudBand::Hud), "{open:?}");
            assert!(!r5_band_yields(open, HudBand::Ledger), "{open:?}");
            assert!(!r5_band_yields(open, HudBand::Touch), "{open:?}");
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
        let cases: &[(&HudPreset<'static>, &[(&str, HudRect, HudRect)])] = &[
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

    #[derive(Resource, Default)]
    struct StyleChangeTape {
        any_style: bool,
        any_vis: bool,
    }

    fn tape_style_and_vis(
        styles: Query<Ref<Style>, With<HudSlab>>,
        vis: Query<Ref<Visibility>, With<HudSlab>>,
        mut tape: ResMut<StyleChangeTape>,
    ) {
        tape.any_style = styles.iter().any(|style| style.is_changed());
        tape.any_vis = vis.iter().any(|visibility| visibility.is_changed());
    }

    fn layout_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(HudLayoutPlugin)
            .init_resource::<StyleChangeTape>()
            .add_systems(Last, tape_style_and_vis);
        app
    }

    fn sentinel_style() -> Style {
        Style {
            position_type: PositionType::Absolute,
            top: Val::Px(1.0),
            bottom: Val::Px(2.0),
            left: Val::Px(3.0),
            right: Val::Px(4.0),
            width: Val::Px(5.0),
            margin: UiRect {
                left: Val::Px(6.0),
                right: Val::Px(7.0),
                top: Val::Px(8.0),
                bottom: Val::Px(9.0),
            },
            padding: UiRect::all(Val::Px(11.0)),
            border: UiRect::all(Val::Px(12.0)),
            ..default()
        }
    }

    /// Hand literals for classic. Not `preset_edges`.
    fn classic_six(id: &str) -> (Val, Val, Val, Val, Val, Val) {
        let auto = Val::Auto;
        let top_left = |y, x, w| (Val::Px(y), auto, Val::Px(x), auto, auto, Val::Px(w));
        let top_right = |y, x, w| (Val::Px(y), auto, auto, Val::Px(x), auto, Val::Px(w));
        let bottom_right = |y, x, w| (auto, Val::Px(y), auto, Val::Px(x), auto, Val::Px(w));
        let centre = |w: f32| {
            (
                Val::Px(16.0),
                auto,
                Val::Percent(50.0),
                auto,
                Val::Px(-w / 2.0),
                Val::Px(w),
            )
        };
        match id {
            ID_FACTORY | ID_SPILL | ID_FAB => top_left(93.0, 16.0, 520.0),
            ID_CLIMATE_STATE | ID_WELL => top_left(153.0, 16.0, 420.0),
            ID_EMBASSY => top_right(180.0, 16.0, 420.0),
            ID_REDEMPTION => top_right(240.0, 16.0, 420.0),
            ID_HYBRID => top_right(300.0, 16.0, 420.0),
            ID_COMPASS => top_right(360.0, 16.0, 420.0),
            ID_VOICE => bottom_right(221.0, 16.0, 560.0),
            ID_GUIDANCE => bottom_right(144.0, 16.0, 520.0),
            ID_CARE_PROMPT => bottom_right(144.0, 16.0, 460.0),
            ID_CARE_STRIP => bottom_right(144.0, 16.0, 560.0),
            ID_PRACTICE => bottom_right(144.0, 16.0, 640.0),
            ID_PULSE => centre(560.0),
            ID_WELCOME => centre(380.0),
            ID_PICKUP => centre(360.0),
            ID_SOVEREIGN => centre(520.0),
            ID_THRIVING => centre(620.0),
            ID_WHISPER => centre(420.0),
            ID_ALLOCATE => top_right(16.0, 76.0, 520.0),
            ID_MERCY => top_right(16.0, 76.0, 360.0),
            ID_JOURNEY => top_right(16.0, 76.0, 360.0),
            ID_REALM => top_right(16.0, 76.0, 300.0),
            ID_WATCH => bottom_right(76.0, 80.0, 340.0),
            ID_PEER => bottom_right(16.0, 80.0, 280.0),
            ID_PLACE_NAME => top_right(93.0, 76.0, 280.0),
            _ => panic!("no classic edges for {id}"),
        }
    }

    fn house_label(settings_open: bool) -> HouseLabel {
        HouseLabel {
            house: HouseName::default(),
            persist_present: false,
            hour_two_held: false,
            book_held: false,
            settings_open,
            draft: String::new(),
            naming_offered: false,
            seals_offered: false,
        }
    }

    #[test]
    fn t4_reset_restores_classic_and_second_reset_is_quiet() {
        let mut app = layout_app();
        let child_style = Style {
            width: Val::Px(4.0),
            height: Val::Px(5.0),
            ..default()
        };
        let mut roots = Vec::new();
        for metrics in SLAB_METRICS {
            let root = app
                .world_mut()
                .spawn((
                    NodeBundle {
                        style: sentinel_style(),
                        z_index: ZIndex::Global(77),
                        focus_policy: FocusPolicy::Pass,
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    HudSlab(metrics.id),
                ))
                .id();
            let child = app
                .world_mut()
                .spawn((
                    TextBundle {
                        style: child_style.clone(),
                        text: Text::from_section(
                            "slab",
                            TextStyle {
                                font_size: 9.0,
                                ..default()
                            },
                        ),
                        ..default()
                    },
                ))
                .id();
            app.world_mut().entity_mut(root).add_child(child);
            roots.push((metrics.id, root, child));
        }
        app.update();
        app.world_mut()
            .send_event(HudLayoutCommand::Apply(HudPresetId::Minimal));
        app.update();
        {
            let factory = roots
                .iter()
                .find(|(id, _, _)| *id == ID_FACTORY)
                .expect("factory")
                .1;
            let style = app.world().get::<Style>(factory).expect("factory style");
            assert_eq!(style.top, Val::Px(180.0));
            assert_eq!(style.right, Val::Px(16.0));
            assert_eq!(style.left, Val::Auto);
            assert_eq!(style.bottom, Val::Auto);
            assert_eq!(style.margin.left, Val::Auto);
            assert_eq!(style.width, Val::Px(520.0));
            assert_ne!(style.top, Val::Px(93.0));
        }
        app.world_mut().send_event(HudLayoutCommand::Reset);
        app.update();
        for (id, root, child) in &roots {
            let style = app.world().get::<Style>(*root).expect(id);
            let (top, bottom, left, right, margin_left, width) = classic_six(id);
            assert_eq!(style.top, top, "{id} top");
            assert_eq!(style.bottom, bottom, "{id} bottom");
            assert_eq!(style.left, left, "{id} left");
            assert_eq!(style.right, right, "{id} right");
            assert_eq!(style.margin.left, margin_left, "{id} margin.left");
            assert_eq!(style.width, width, "{id} width");
            assert_eq!(style.margin.right, Val::Px(7.0), "{id}");
            assert_eq!(style.margin.top, Val::Px(8.0), "{id}");
            assert_eq!(style.margin.bottom, Val::Px(9.0), "{id}");
            assert_eq!(style.padding, UiRect::all(Val::Px(11.0)), "{id}");
            assert_eq!(style.border, UiRect::all(Val::Px(12.0)), "{id}");
            assert_eq!(style.position_type, PositionType::Absolute, "{id}");
            assert_eq!(
                *app.world().get::<ZIndex>(*root).expect(id),
                ZIndex::Global(77)
            );
            assert_eq!(
                *app.world().get::<FocusPolicy>(*root).expect(id),
                FocusPolicy::Pass
            );
            let child_now = app.world().get::<Style>(*child).expect("child");
            assert_eq!(child_now.width, Val::Px(4.0));
            assert_eq!(child_now.height, Val::Px(5.0));
        }
        let before: Vec<Style> = roots
            .iter()
            .map(|(_, root, _)| app.world().get::<Style>(*root).expect("style").clone())
            .collect();
        app.world_mut().send_event(HudLayoutCommand::Reset);
        app.update();
        for ((_, root, _), previous) in roots.iter().zip(before) {
            assert_eq!(
                app.world().get::<Style>(*root).expect("style").clone(),
                previous
            );
        }
        assert!(!app.world().resource::<StyleChangeTape>().any_style);
    }

    #[test]
    fn none_preset_writes_no_style_or_visibility() {
        let mut app = layout_app();
        app.insert_resource(LaunchDoor::Title);
        app.insert_resource(VoiceYard {
            sash_open: true,
            ..default()
        });
        let mut allocate = RbeAllocateChoice::default();
        allocate.panel_open = true;
        allocate.eligible = true;
        allocate.choices_made = 4;
        allocate.surplus_signal = 3.5;
        app.insert_resource(allocate);
        app.insert_resource(HumanSoftPanels {
            mercy_open: true,
            realm_open: true,
        });
        let mut journey = AbundanceJourneyEcho::default();
        journey.panel_open = true;
        journey.last_choices_seen = 7;
        journey.dirty = true;
        app.insert_resource(journey);
        let sentinel = sentinel_style();
        let pulse = app
            .world_mut()
            .spawn((
                NodeBundle {
                    style: sentinel.clone(),
                    visibility: Visibility::Visible,
                    ..default()
                },
                HudSlab(ID_PULSE),
            ))
            .id();
        let welcome = app
            .world_mut()
            .spawn((
                NodeBundle {
                    style: sentinel.clone(),
                    visibility: Visibility::Visible,
                    ..default()
                },
                HudSlab(ID_WELCOME),
            ))
            .id();
        let plain = app
            .world_mut()
            .spawn(NodeBundle {
                style: sentinel.clone(),
                visibility: Visibility::Visible,
                ..default()
            })
            .id();
        app.update();
        app.update();
        for entity in [pulse, welcome, plain] {
            assert_eq!(app.world().get::<Style>(entity).expect("style").clone(), sentinel);
            assert_eq!(
                *app.world().get::<Visibility>(entity).expect("vis"),
                Visibility::Visible
            );
        }
        let tape = app.world().resource::<StyleChangeTape>();
        assert!(!tape.any_style);
        assert!(!tape.any_vis);
        assert!(app.world().resource::<ActiveHudPreset>().id.is_none());
        let voice = app.world().resource::<VoiceYard>();
        assert!(voice.sash_open);
        let allocate = app.world().resource::<RbeAllocateChoice>();
        assert!(allocate.panel_open);
        assert!(allocate.eligible);
        assert_eq!(allocate.choices_made, 4);
        assert!((allocate.surplus_signal - 3.5).abs() < f32::EPSILON);
        let soft = app.world().resource::<HumanSoftPanels>();
        assert!(soft.mercy_open);
        assert!(soft.realm_open);
        let journey = app.world().resource::<AbundanceJourneyEcho>();
        assert!(journey.panel_open);
        assert_eq!(journey.last_choices_seen, 7);
        assert!(journey.dirty);
    }

    #[derive(Component)]
    struct OwnStamp(u32);

    fn force_visible(app: &mut App, by_id: &std::collections::HashMap<&str, (Entity, u32)>) {
        for (entity, _) in by_id.values() {
            *app.world_mut().get_mut::<Visibility>(*entity).expect("vis") = Visibility::Visible;
        }
    }

    fn assert_vis(
        app: &App,
        by_id: &std::collections::HashMap<&str, (Entity, u32)>,
        id: &str,
        want: Visibility,
        anchor: &str,
    ) {
        let entity = by_id[id].0;
        assert_eq!(
            *app.world().get::<Visibility>(entity).expect(id),
            want,
            "{anchor} {id}"
        );
    }

    fn assert_stamps(app: &App, by_id: &std::collections::HashMap<&str, (Entity, u32)>) {
        for (entity, stamp) in by_id.values() {
            assert_eq!(app.world().get::<OwnStamp>(*entity).expect("stamp").0, *stamp);
        }
    }

    fn assert_panel_state(app: &App) {
        let guidance = app.world().resource::<FirstSessionGuidance>();
        assert!(guidance.active);
        assert!(!guidance.dismissed);
        assert_eq!(guidance.shown_at_seconds, 12.5);
        let allocate = app.world().resource::<RbeAllocateChoice>();
        assert!(allocate.panel_open);
        assert_eq!(allocate.choices_made, 4);
    }

    #[test]
    fn t6_shared_yield_shows_only_the_top_occupant() {
        let mut app = layout_app();
        let mut guidance = FirstSessionGuidance::default();
        guidance.shown_at_seconds = 12.5;
        app.insert_resource(guidance);
        let mut allocate = RbeAllocateChoice::default();
        allocate.panel_open = true;
        allocate.choices_made = 4;
        app.insert_resource(allocate);

        let mut by_id = std::collections::HashMap::new();
        for (index, metrics) in SLAB_METRICS.iter().enumerate() {
            let entity = app
                .world_mut()
                .spawn((
                    NodeBundle {
                        style: sentinel_style(),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    HudSlab(metrics.id),
                    OwnStamp(index as u32),
                ))
                .id();
            by_id.insert(metrics.id, (entity, index as u32));
        }

        for layout in PRESETS {
            app.world_mut()
                .send_event(HudLayoutCommand::Apply(layout.id));
            for anchor in layout.anchors {
                if anchor.share != HudShare::Yield || anchor.occupants.len() < 2 {
                    continue;
                }
                force_visible(&mut app, &by_id);
                app.update();
                let mut ranked = anchor.occupants.to_vec();
                ranked.sort_by_key(|occupant| (occupant.class, occupant.rank));
                assert_vis(&app, &by_id, ranked[0].id, Visibility::Visible, anchor.id);
                for occupant in ranked.iter().skip(1) {
                    assert_vis(&app, &by_id, occupant.id, Visibility::Hidden, anchor.id);
                }
                assert_stamps(&app, &by_id);
                assert_panel_state(&app);

                let winner = by_id[ranked[0].id].0;
                *app.world_mut().get_mut::<Visibility>(winner).expect("winner") = Visibility::Hidden;
                app.update();
                assert_vis(&app, &by_id, ranked[0].id, Visibility::Hidden, anchor.id);
                assert_vis(&app, &by_id, ranked[1].id, Visibility::Visible, anchor.id);
                for occupant in ranked.iter().skip(2) {
                    assert_vis(&app, &by_id, occupant.id, Visibility::Hidden, anchor.id);
                }
                assert_stamps(&app, &by_id);
                assert_panel_state(&app);
            }
        }
    }

    fn fixed_style_rows(app: &mut App) -> Vec<(Entity, Style)> {
        let mut query = app
            .world_mut()
            .query_filtered::<(Entity, &Style), Without<HudSlab>>();
        let mut rows: Vec<(Entity, Style)> = query
            .iter(app.world())
            .map(|(entity, style)| (entity, style.clone()))
            .collect();
        rows.sort_by_key(|(entity, _)| entity.index());
        rows
    }

    fn global_z(z: Option<&ZIndex>, want: i32) -> bool {
        matches!(z, Some(ZIndex::Global(n)) if *n == want)
    }

    /// Rows 1–7, 11, 20–25, and 33, located on the entities the real spawn
    /// systems created. The match is a locator, not a copied `Style`.
    fn assert_real_fixed_rows_spawned(app: &mut App) {
        let mut query = app.world_mut().query_filtered::<(
            &Style,
            Option<&ZIndex>,
        ), Without<HudSlab>>();
        let rows: Vec<(Style, Option<ZIndex>)> = query
            .iter(app.world())
            .map(|(style, z)| (style.clone(), z.copied()))
            .collect();
        let hit = |pred: &dyn Fn(&Style, Option<ZIndex>) -> bool, name: &str| {
            assert!(
                rows.iter().any(|(style, z)| pred(style, *z)),
                "real fixed row missing: {name}"
            );
        };
        hit(
            &|style, z| {
                style.width == Val::Percent(100.0)
                    && style.height == Val::Percent(100.0)
                    && style.padding == UiRect::all(Val::Px(TITLE_SAFE_INSET))
                    && global_z(z.as_ref(), LIVED_UI_Z_TITLE)
            },
            "title",
        );
        hit(
            &|style, z| {
                style.top == Val::Px(10.0)
                    && style.width == Val::Px(520.0)
                    && style.margin.left == Val::Px(-260.0)
                    && global_z(z.as_ref(), LIVED_UI_Z_PAUSE + 1)
            },
            "comfort banner",
        );
        hit(
            &|style, z| {
                style.top == Val::Percent(1.0)
                    && style.width == Val::Px(420.0)
                    && style.margin.left == Val::Px(-210.0)
                    && global_z(z.as_ref(), LIVED_UI_Z_PAUSE)
            },
            "settings",
        );
        hit(
            &|style, z| {
                style.width == Val::Percent(100.0)
                    && style.height == Val::Percent(100.0)
                    && global_z(z.as_ref(), 140)
            },
            "name house",
        );
        hit(
            &|style, z| {
                style.width == Val::Percent(100.0)
                    && style.height == Val::Percent(100.0)
                    && global_z(z.as_ref(), 141)
            },
            "house dress",
        );
        hit(
            &|style, z| {
                style.width == Val::Percent(100.0)
                    && style.height == Val::Percent(100.0)
                    && global_z(z.as_ref(), 142)
            },
            "persona",
        );
        hit(
            &|style, z| {
                style.top == Val::Percent(18.0)
                    && style.width == Val::Px(400.0)
                    && style.margin.left == Val::Px(-200.0)
                    && global_z(z.as_ref(), PLACES_PLATE_Z)
            },
            "places",
        );
        hit(
            &|style, z| {
                style.bottom == Val::Px(16.0)
                    && style.left == Val::Px(16.0)
                    && style.width == Val::Px(560.0)
                    && global_z(z.as_ref(), LIVED_UI_Z_LEDGER)
            },
            "ledger",
        );
        hit(
            &|style, z| {
                style.width == Val::Percent(100.0)
                    && style.height == Val::Percent(100.0)
                    && style.padding == UiRect::DEFAULT
                    && global_z(z.as_ref(), LIVED_UI_Z_LEDGER - 1)
            },
            "touch overlay",
        );
        hit(
            &|style, _| {
                style.left == Val::Px(24.0)
                    && style.bottom == Val::Px(24.0)
                    && style.width == Val::Px(120.0)
                    && style.height == Val::Px(120.0)
            },
            "touch stick",
        );
        hit(
            &|style, _| {
                style.right == Val::Px(28.0)
                    && style.bottom == Val::Px(36.0)
                    && style.width == Val::Px(TOUCH_HIT_MIN)
                    && style.height == Val::Px(TOUCH_HIT_MIN)
            },
            "touch use",
        );
        hit(
            &|style, _| {
                style.right == Val::Px(24.0)
                    && style.top == Val::Px(24.0)
                    && style.width == Val::Px(TOUCH_HIT_MIN)
            },
            "touch pause",
        );
        hit(
            &|style, _| {
                style.right == Val::Px(24.0)
                    && style.top == Val::Px(24.0 + TOUCH_HIT_MIN + 8.0)
                    && style.width == Val::Px(TOUCH_HIT_MIN)
            },
            "touch q",
        );
        hit(
            &|style, _| {
                style.right == Val::Px(24.0)
                    && style.top == Val::Px(24.0 + 2.0 * (TOUCH_HIT_MIN + 8.0))
                    && style.width == Val::Px(TOUCH_HIT_MIN)
            },
            "touch l",
        );
        hit(
            &|style, z| {
                style.bottom == Val::Percent(22.0)
                    && style.left == Val::Px(16.0)
                    && style.width == Val::Px(300.0)
                    && global_z(z.as_ref(), LIVED_UI_Z_LEDGER)
            },
            "satchel",
        );
        let plates = {
            let mut plates = app.world_mut().query::<&LivedUiPlate>();
            plates.iter(app.world()).count()
        };
        assert!(plates >= 10, "real LivedUiPlate roots, counted {plates}");
    }

    #[test]
    fn t7_fixed_rows_keep_coded_style() {
        let mut app = layout_app();
        app.add_plugins((
            crate::title_screen::TitleScreenPlugin,
            crate::hex_travel::HexTravelPlugin,
            crate::ledger_bind::LedgerBindPlugin,
            crate::touch_controls::TouchControlsPlugin,
            crate::human_inventory::HumanInventoryPlugin,
        ));
        app.world_mut().run_schedule(Startup);
        let mover = app
            .world_mut()
            .spawn((
                NodeBundle {
                    style: Style {
                        top: Val::Px(1.0),
                        ..default()
                    },
                    ..default()
                },
                HudSlab(ID_FACTORY),
            ))
            .id();
        assert_real_fixed_rows_spawned(&mut app);
        let fixed = fixed_style_rows(&mut app);
        assert!(
            fixed.len() > 15,
            "real spawn tree, not 15 hand-copied rows ({})",
            fixed.len()
        );
        let assert_fixed = |app: &App, fixed: &[(Entity, Style)]| {
            for (entity, style) in fixed {
                assert_eq!(
                    app.world().get::<Style>(*entity).expect("fixed").clone(),
                    *style
                );
            }
        };
        app.world_mut().run_schedule(PostUpdate);
        assert_fixed(&app, &fixed);
        app.world_mut().resource_mut::<ActiveHudPreset>().id = Some(HudPresetId::Minimal);
        app.world_mut().run_schedule(PostUpdate);
        assert_fixed(&app, &fixed);
        assert_eq!(
            app.world().get::<Style>(mover).expect("mover").top,
            Val::Px(180.0)
        );
        app.world_mut().resource_mut::<ActiveHudPreset>().id = Some(HudPresetId::Management);
        app.world_mut().run_schedule(PostUpdate);
        assert_fixed(&app, &fixed);
        app.world_mut().resource_mut::<ActiveHudPreset>().id = Some(HudPresetId::Classic);
        app.world_mut().run_schedule(PostUpdate);
        assert_fixed(&app, &fixed);
        assert_eq!(
            app.world().get::<Style>(mover).expect("mover").top,
            Val::Px(93.0)
        );
        assert_eq!(
            app.world().get::<Style>(mover).expect("mover").left,
            Val::Px(16.0)
        );
    }

    #[test]
    fn r3_allocate_rise_closes_mercy_through_its_flag() {
        let mut app = layout_app();
        app.insert_resource(HumanSoftPanels {
            mercy_open: true,
            realm_open: false,
        });
        let mut allocate = RbeAllocateChoice::default();
        allocate.choices_made = 2;
        app.insert_resource(allocate);
        app.world_mut()
            .send_event(HudLayoutCommand::Apply(HudPresetId::Classic));
        app.update();
        assert!(app.world().resource::<HumanSoftPanels>().mercy_open);
        assert!(!app.world().resource::<HumanSoftPanels>().realm_open);
        assert!(!app.world().resource::<RbeAllocateChoice>().panel_open);
        app.world_mut()
            .resource_mut::<RbeAllocateChoice>()
            .panel_open = true;
        app.update();
        assert!(!app.world().resource::<HumanSoftPanels>().mercy_open);
        assert!(!app.world().resource::<HumanSoftPanels>().realm_open);
        let allocate = app.world().resource::<RbeAllocateChoice>();
        assert!(allocate.panel_open);
        assert_eq!(allocate.choices_made, 2);
    }

    #[test]
    fn r5_modals_hide_hud_slabs_and_restore_own_visibility() {
        let mut app = layout_app();
        let slab = app
            .world_mut()
            .spawn((
                NodeBundle {
                    visibility: Visibility::Visible,
                    ..default()
                },
                HudSlab(ID_FACTORY),
            ))
            .id();
        let plain = app
            .world_mut()
            .spawn(NodeBundle {
                visibility: Visibility::Visible,
                ..default()
            })
            .id();
        let vis = |app: &App, entity: Entity| *app.world().get::<Visibility>(entity).expect("vis");
        app.world_mut()
            .send_event(HudLayoutCommand::Apply(HudPresetId::Classic));
        app.update();
        assert_eq!(vis(&app, slab), Visibility::Visible);
        assert_eq!(vis(&app, plain), Visibility::Visible);

        app.insert_resource(LaunchDoor::InYard);
        app.update();
        assert_eq!(vis(&app, slab), Visibility::Visible);

        *app.world_mut().resource_mut::<LaunchDoor>() = LaunchDoor::Title;
        app.update();
        assert_eq!(vis(&app, slab), Visibility::Hidden);
        assert_eq!(vis(&app, plain), Visibility::Visible);

        *app.world_mut().resource_mut::<LaunchDoor>() = LaunchDoor::NameHouse;
        app.update();
        assert_eq!(vis(&app, slab), Visibility::Hidden);

        *app.world_mut().resource_mut::<LaunchDoor>() = LaunchDoor::HouseDress;
        app.update();
        assert_eq!(vis(&app, slab), Visibility::Hidden);

        *app.world_mut().resource_mut::<LaunchDoor>() = LaunchDoor::InYard;
        app.update();
        assert_eq!(vis(&app, slab), Visibility::Visible);
        assert_eq!(vis(&app, plain), Visibility::Visible);

        let mut places = PlacesPlate::default();
        places.open = true;
        app.insert_resource(places);
        app.update();
        assert_eq!(vis(&app, slab), Visibility::Hidden);
        app.world_mut().resource_mut::<PlacesPlate>().open = false;
        app.update();
        assert_eq!(vis(&app, slab), Visibility::Visible);

        app.insert_resource(house_label(true));
        app.update();
        assert_eq!(vis(&app, slab), Visibility::Hidden);
        app.world_mut().resource_mut::<HouseLabel>().settings_open = false;
        app.update();
        assert_eq!(vis(&app, slab), Visibility::Visible);

        let mut persona = PersonaCreatorState::default();
        persona.open = true;
        app.insert_resource(persona);
        app.update();
        assert_eq!(vis(&app, slab), Visibility::Hidden);
        app.world_mut().resource_mut::<PersonaCreatorState>().open = false;
        app.update();
        assert_eq!(vis(&app, slab), Visibility::Visible);
        assert_eq!(vis(&app, plain), Visibility::Visible);
    }

    #[test]
    fn none_after_hide_drains_yield_memory() {
        let mut app = layout_app();
        let slab = app
            .world_mut()
            .spawn((
                NodeBundle {
                    visibility: Visibility::Visible,
                    ..default()
                },
                HudSlab(ID_FACTORY),
            ))
            .id();
        app.insert_resource(LaunchDoor::Title);
        app.world_mut()
            .send_event(HudLayoutCommand::Apply(HudPresetId::Classic));
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(slab).expect("vis"),
            Visibility::Hidden
        );
        assert!(
            !app.world().resource::<HudYieldMemory>().own.is_empty(),
            "hide stores the slab's own visibility"
        );
        app.world_mut().resource_mut::<ActiveHudPreset>().id = None;
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(slab).expect("vis"),
            Visibility::Visible
        );
        assert!(app.world().resource::<HudYieldMemory>().own.is_empty());
        assert!(app.world().resource::<ActiveHudPreset>().id.is_none());
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(slab).expect("vis"),
            Visibility::Visible
        );
    }

    fn over(
        id: &str,
        corner: HudLayoutCorner,
        x: f32,
        y: f32,
        width: f32,
        hidden: bool,
    ) -> HudLayoutAnchor {
        HudLayoutAnchor {
            id: id.to_string(),
            corner,
            x,
            y,
            width,
            hidden,
        }
    }

    #[test]
    fn zero_overrides_apply_exactly_the_preset() {
        for layout in PRESETS {
            let applied = apply_overrides(layout, HUD_REGISTRY_REV, &[], 1024.0, 640.0);
            match applied {
                HudOverrideApply::Applied(anchors) => assert_eq!(anchors, layout.anchors),
                other => panic!("{} zero overrides became {other:?}", layout.name),
            }
            assert!(hud_save_allowed(layout, 1024.0, 640.0), "{}", layout.name);
        }
    }

    #[test]
    fn hidden_refused_on_class_1_2_and_3_anchors() {
        let class1 = over(
            "VOICE",
            HudLayoutCorner::BottomRight,
            16.0,
            221.0,
            560.0,
            true,
        );
        let class2 = over(
            "ACTION_BAR",
            HudLayoutCorner::BottomRight,
            16.0,
            144.0,
            640.0,
            true,
        );
        assert!(matches!(
            apply_overrides(&CLASSIC, HUD_REGISTRY_REV, &[class1], 1024.0, 640.0),
            HudOverrideApply::Refused
        ));
        assert!(matches!(
            apply_overrides(&CLASSIC, HUD_REGISTRY_REV, &[class2], 1280.0, 800.0),
            HudOverrideApply::Refused
        ));

        const TUTOR_ONLY: [HudOccupant; 1] = [occ(ID_GUIDANCE, 1)];
        let tutor_anchors = [anchor(AnchorFields {
            id: "TUTOR",
            corner: HudCorner::TopLeft,
            x: 16.0,
            y: 16.0,
            width: 520.0,
            height_budget: 69.0,
            class: CLASS_TUTOR,
            occupants: &TUTOR_ONLY,
            share: HudShare::Solo,
        })];
        let tutor = HudPreset {
            id: HudPresetId::Classic,
            name: "classic",
            anchors: &tutor_anchors,
        };
        let class3 = over("TUTOR", HudLayoutCorner::TopLeft, 16.0, 16.0, 520.0, true);
        assert!(matches!(
            apply_overrides(&tutor, HUD_REGISTRY_REV, &[class3], 1024.0, 640.0),
            HudOverrideApply::Refused
        ));

        let class4 = over(
            "TOP_TOAST",
            HudLayoutCorner::TopCentre,
            0.0,
            16.0,
            620.0,
            true,
        );
        match apply_overrides(&CLASSIC, HUD_REGISTRY_REV, &[class4], 1024.0, 640.0) {
            HudOverrideApply::Applied(anchors) => {
                let toast = anchors
                    .iter()
                    .find(|anchor| anchor.id == "TOP_TOAST")
                    .unwrap();
                assert!(toast.hidden);
                assert!(!anchors
                    .iter()
                    .any(|anchor| anchor.id != "TOP_TOAST" && anchor.hidden));
            }
            other => panic!("class 4 hide should apply, got {other:?}"),
        }
    }

    #[test]
    fn apply_overrides_refuses_f5_without_a_partial_apply() {
        let bad_width = over(
            "PLACE_NAME",
            HudLayoutCorner::TopRight,
            76.0,
            93.0,
            279.0,
            false,
        );
        let too_wide = over(
            "PLACE_NAME",
            HudLayoutCorner::TopRight,
            76.0,
            93.0,
            641.0,
            false,
        );
        let unknown = over(
            "NO_SUCH",
            HudLayoutCorner::TopLeft,
            16.0,
            16.0,
            280.0,
            false,
        );
        let non_finite = over(
            "PLACE_NAME",
            HudLayoutCorner::TopRight,
            f32::NAN,
            93.0,
            280.0,
            false,
        );
        for (view_w, view_h) in PROOF_VIEWS {
            assert!(matches!(
                apply_overrides(
                    &CLASSIC,
                    HUD_REGISTRY_REV,
                    &[bad_width.clone()],
                    view_w,
                    view_h
                ),
                HudOverrideApply::Refused
            ));
            assert!(matches!(
                apply_overrides(
                    &CLASSIC,
                    HUD_REGISTRY_REV,
                    &[too_wide.clone()],
                    view_w,
                    view_h
                ),
                HudOverrideApply::Refused
            ));
        }
        let legal = over(
            "TOP_TOAST",
            HudLayoutCorner::TopCentre,
            0.0,
            16.0,
            620.0,
            true,
        );
        assert!(matches!(
            apply_overrides(
                &CLASSIC,
                HUD_REGISTRY_REV,
                &[legal.clone(), unknown],
                1024.0,
                640.0
            ),
            HudOverrideApply::Refused
        ));
        assert!(matches!(
            apply_overrides(&CLASSIC, HUD_REGISTRY_REV, &[non_finite], 1024.0, 640.0),
            HudOverrideApply::Refused
        ));
        assert!(matches!(
            apply_overrides(&CLASSIC, HUD_REGISTRY_REV + 1, &[legal], 1024.0, 640.0),
            HudOverrideApply::StaleRev
        ));
    }

    #[test]
    fn overlap_override_blocks_save_and_falls_back_to_base() {
        for (view_w, view_h) in PROOF_VIEWS {
            assert!(hud_save_allowed(&MINIMAL, view_w, view_h));
            let moved = over(
                "CORNER",
                HudLayoutCorner::TopRight,
                16.0,
                180.0,
                340.0,
                false,
            );
            let HudOverrideApply::Applied(anchors) =
                apply_overrides(&MINIMAL, HUD_REGISTRY_REV, &[moved.clone()], view_w, view_h)
            else {
                panic!("overlap case must be a legal override");
            };
            let layout = HudPreset {
                id: MINIMAL.id,
                name: MINIMAL.name,
                anchors: &anchors,
            };
            let census = overlap_census(&layout, view_w, view_h, HudHeightModel::B);
            assert!(
                census.visible_together.iter().any(|pair| {
                    (pair.left == ID_WATCH || pair.right == ID_WATCH)
                        && (pair.left == ID_FACTORY || pair.right == ID_FACTORY)
                }),
                "CORNER over EDGE_STATUS at {view_w}x{view_h}: {:?}",
                census.visible_together
            );
            assert!(!hud_save_allowed(&layout, view_w, view_h));
            assert_eq!(
                resolve_hud_layout(&MINIMAL, HUD_REGISTRY_REV, &[moved], view_w, view_h),
                HudLayoutChoice::Fallback(HudPresetId::Minimal)
            );
        }

        let broken = HudPreset {
            id: HudPresetId::Minimal,
            name: "minimal",
            anchors: &[],
        };
        assert_eq!(
            resolve_hud_layout(&broken, HUD_REGISTRY_REV, &[], 1024.0, 640.0),
            HudLayoutChoice::Fallback(HudPresetId::Classic),
            "F6 uses classic when base itself fails"
        );
    }
}
