/*!
 * First Session Guidance — single onboarding card (v23.2.24 + hour two v23.2.31)
 *
 * One sentence at a time: walk · tend · satchel · allocate · Tab · Q · L · fabricator · Embassy.
 * Playable-loop polish: after harvest tip House; after Hour two name climate/week;
 * after book Ledger 3 is optional (never shouted on Peace boot).
 * Resume skips the walk when the hour pack is already held.
 * H hides. World still teaches. Not a second HUD.
 * Does not rewrite harvest_feel or rbe_allocate_choice.
 *
 * CARD L1 SANCTUARY-WANT — first minutes speak one People + one Want.
 * People = Human. Cite ART_BIBLE: Human | warm grey-gold | Sanctuary — cite only, no pack.
 * Want = the yard needs tending or the well goes quiet.
 * Cite PLACE_DRESS Sanctuary yard · DRIVE_LORE practices-after-House — cite, no pack.
 * H hush still works. 0 meshes · 0 new verbs · 0 Places.
 *
 * CARD L1 GARDEN-WANT — same People + Want retargeted onto the Garden / boot
 * plane (walkable title · God-plane, D0 EDEN-PLANE-LAW @ 2afff36).
 * Cite PLACE_DRESS Garden≠Sanctuary · ART_BIBLE / PLAYABLE_RACES (Human — cite only).
 * Want lives on Title before Play; Sanctuary dirt is not required.
 * H hush still works. Comfort Low is mesh LOD, not a text gate.
 * 0 meshes · 0 new verbs · 0 Places · Title stays Play / Continue / Settings · Online grey.
 *
 * CARD L2 HOUSE-PEOPLE-GATES — Q House offers five Peoples as God-plane doors
 * (Garden / boot · D0 @ 2afff36). Four Place landings only (PLAYABLE_RACES §1.1).
 * Doors ignite after one Tend. Crossing one-way this session.
 * Skip House = stay light / Peace default. C0 Cydruid = human-in-frame.
 * 0 meshes · ASSET_BUDGET_COURT cite only. Not the #459 dress-token prove-line.
 *
 * CARD L3 PEOPLE-DOOR-LAND — garden wrappers call L2 try_cross then
 * apply_place / lived bind. Call only. Cite L3_SPAWN_RESEARCH §3.
 *
 * CARD L5 TITLE-GARDEN-LAND — first-session People-door uses the same
 * L3/L4 wire (apply_people_landing). House + Tend → dressed Place.
 * Skip House / no Tend → none, PlaceId unchanged, Peace light-body.
 * Title chrome stays Play / Continue / Settings · Online grey.
 * 0 meshes · 0 portraits. Cite #465 #466 · D0 · C0.
 *
 * CARD L6 LANDING-WANT — after apply_people_landing, first-minutes Want
 * follows PlaceId. Garden boot (no land) / skip House keep GARDEN_WANT.
 * Copy already on tip (ART_BIBLE · PLACE_DRESS · Depths restore-not-Take).
 * Ambrosian shares Sanctuary Want. Quellorian shares Heartwood Want.
 * 0 meshes · 0 new verbs · no Title lobby · Online grey.
 * Cite #465 #466 #467 · D0 · C0 · L1 GARDEN-WANT ·
 * ACityGamesInc/status/2101247905218568248 stills only.
 *
 * CARD S1 AFTERMATH-EVIDENCE — after a sealed People-door crossing, the
 * landing Place shows the intro's aftermath as LOCAL EVIDENCE (well /
 * guidance / climate line). Not a trailer cutscene. Same war, five reads
 * keyed by PeopleLanding (Want stays L6 PlaceId). Cite lore already on
 * tip: DRIVE_LORE · DRAEK_ORIGIN · PLAYABLE_RACES · ART_BIBLE ·
 * FACTIONS_OVERVIEW · GDD_IMMERSION. Prefer existing Want strings;
 * extend only with cited lore phrasing. Fork A Ambrosian Want stays
 * Sanctuary. C0 not-treant. PlaceId stays 3. L7 arrival beat untouched.
 * 0 meshes · 0 new PlaceId · 0 Imagine pack import (cite only).
 * Title stays Play / Continue / Settings · Online grey.
 *
 * CARD S2 GATE-SEAL — new soul is light on the God-plane (garden). Doors
 * stay unsealed until existing E/Q confirm at landing seals People+Place
 * onto the existing hour-two disk. Decline / wrong door returns to garden
 * still light, not tied, PlaceId unchanged. Skip House stays light (L7).
 * Peace recall = vision home, not an unseal. S1 aftermath lines untouched.
 * PlaceId stays 3. Title chrome unchanged. Online grey.
 *
 * CARD F7 PLACE-AFTERMATH-VARIANTS — one extra local-evidence lore line
 * per People on the existing S1 aftermath plate (well / guidance Want).
 * S1 HUMAN/AMBROSIAN/CYDRUID/QUELLORIAN/DRAEK_AFTERMATH stay unread.
 * Cite-only (Clerk exact): docs/DRIVE_LORE_ADAPTATION.md ·
 * docs/PLAYABLE_RACES.md · docs/ART_BIBLE.md. Do not invent lore.
 * Not a trailer / cutscene / Imagine pack. PlaceId stays 3. 0 meshes.
 * S2 / S3 / F5 / F6 WRITE unread. Title chrome unchanged. Online grey.
 *
 * CARD FLESH-GUIDANCE-PLACE — the Hour-two week-bill sentence may name the
 * Place the player stands in (Sanctuary / Heartwood / Threshold-near / Depths).
 * Threshold-near is Heartwood plus existing shelf reach. No new PlaceId.
 * Same tend · week-bill · Continue. No new card. No Title chrome. Online grey.
 * Peak memory stays: walked · tended · week was the bill · yard remembered.
 *
 * Contact: info@Rathor.ai | Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::embassy::EmbassyYard;
use crate::fabricator::FabricatorYard;
use crate::hex_travel::{apply_people_landing, decline_people_door_land, HexTravelState};
use crate::hour_sacred::{
    confirm_gate_seal, doors_are_unsealed, god_plane_doors_ignited,
    offer_house_peoples, skip_house_stays_light, soul_is_light, try_cross_people_door,
    HousePeople, PeopleLanding, HourSacred, HOUSE_PEOPLES, L2_ASSET_BUDGET_CITE, L2_MESH_BUDGET,
};
use shared::hex_travel::PlaceId;
use shared::local_settings::PeaceKey;
use crate::human_presence::SoftPresence;
use crate::ledger_bind::LedgerYard;
use crate::lived_hour_bind::LivedHourBind;
use crate::mercy_harvest_nodes::NearbyMercyNode;
use crate::title_screen::LaunchDoor;
use shared::ledger_bind::ContractState;
use shared::persona::{house_dress_token, is_peace_default_dress, HOUSE_PEOPLE_TINT};
use shared::space_law::HexFlag;

/// CARD L1 SANCTUARY-WANT — first minutes: one People name.
/// Cite ART_BIBLE: Human | warm grey-gold | Sanctuary — cite only, no pack import.
pub const SANCTUARY_PEOPLE: &str = "Human";

/// CARD L1 SANCTUARY-WANT — first minutes: one Want.
/// Cite PLACE_DRESS Sanctuary yard · DRIVE_LORE practices-after-House — cite, no pack.
pub const SANCTUARY_WANT: &str = "the yard needs tending or the well goes quiet";

/// Spoken People + Want the stranger hears in the first minutes.
pub fn first_minutes_people_want_line() -> String {
    format!("{SANCTUARY_PEOPLE} · {SANCTUARY_WANT}")
}

/// CARD L1 GARDEN-WANT — retarget SANCTUARY-WANT People onto the Garden / boot plane.
/// People stays Human. Cite ART_BIBLE / PLAYABLE_RACES — cite only, no pack, no Title race lobby.
pub const GARDEN_PEOPLE: &str = SANCTUARY_PEOPLE;

/// CARD L1 GARDEN-WANT — retarget SANCTUARY-WANT onto the Garden / boot plane.
/// Same Want: tend or the well goes quiet. Cite PLACE_DRESS Garden≠Sanctuary · D0 EDEN-PLANE-LAW.
pub const GARDEN_WANT: &str = SANCTUARY_WANT;

/// CARD L1 GARDEN-WANT — Garden / walkable title · God-plane speaks People + Want.
/// `on_garden_boot` is Title (LaunchDoor::Title), not Sanctuary dirt / InYard.
/// H hush drops the line. Comfort Low does not gate these words.
/// CARD L6 — Garden boot (no land) keeps this GARDEN_WANT line.
pub fn garden_boot_want_line(on_garden_boot: bool, hush: bool) -> Option<String> {
    if !on_garden_boot || hush {
        None
    } else {
        Some(first_minutes_people_want_line())
    }
}

/// CARD L6 — Heartwood first-minutes Want. Existing tend line on tip
/// (`living_ecology` honest copy). Cite ART_BIBLE amber lamp · PLACE_DRESS
/// live/seal room. Cydruid + Quellorian share this PlaceId Want.
pub const HEARTWOOD_WANT: &str = "Heartwood Wards · Tend";

/// CARD L6 — Depths first-minutes Want. Existing Peace tend / restore-not-Take
/// line on tip (`depths_landing` · ART_BIBLE teal Peace). Not Take.
pub const DEPTHS_WANT: &str = "Depths Peace · restored";

/// CARD L6 — Want follows PlaceId after People-door land.
/// Sanctuary (Human + Ambrosian) keeps SANCTUARY_WANT.
/// Heartwood (Cydruid + Quellorian) uses the existing Heartwood tend line.
/// Depths (Draek) uses the existing restore-not-Take Peace tend line.
pub fn want_for_place(place: PlaceId) -> &'static str {
    match place {
        PlaceId::Sanctuary => SANCTUARY_WANT,
        PlaceId::Heartwood => HEARTWOOD_WANT,
        PlaceId::Depths => DEPTHS_WANT,
    }
}

/// CARD L6 — after apply_people_landing, Want is the landed PlaceId line.
/// Skip House / no land keeps GARDEN_WANT (same words as SANCTUARY_WANT).
pub fn want_after_people_landing(landing: Option<PeopleLanding>) -> &'static str {
    match landing {
        Some(land) => want_for_place(land.place_id()),
        None => GARDEN_WANT,
    }
}

/// Spoken People + Want after a People-door land. People prefix stays Human
/// (ART_BIBLE cite). Want names the Place the body is in.
pub fn first_minutes_people_want_line_for_place(place: PlaceId) -> String {
    format!("{SANCTUARY_PEOPLE} · {}", want_for_place(place))
}

/// CARD S1 — Human Sanctuary aftermath. Yard still teaching; war is rumor
/// at the well. Cite PLACE_DRESS / GDD teaching yard · DRIVE_LORE refuse
/// weekly wars as Sanctuary · GDD weekly wars parked after Online.
/// Want stays [`SANCTUARY_WANT`].
pub const HUMAN_AFTERMATH: &str = "yard still teaching · war is rumor at the well";

/// CARD S1 — Ambrosian Sanctuary lift (Fork A). Same disk, thinner fog,
/// no hull. Cite PLAYABLE_RACES same Sanctuary Place · ART_BIBLE Sanctuary
/// must-not Brood Spire / fleet · L7 Fork A thinner fog (cite only; do not
/// recook). Want stays Sanctuary.
pub const AMBROSIAN_AFTERMATH: &str = "same disk · thinner fog · no hull";

/// CARD S1 — Cydruid Heartwood aftermath. C0 human-in-frame · nature is
/// practice, not species. Cite PLAYABLE_RACES / DRIVE_LORE / ART_BIBLE.
/// NOT treant. Want stays [`HEARTWOOD_WANT`].
pub const CYDRUID_AFTERMATH: &str = "human-in-frame · nature is practice";

/// CARD S1 — Quellorian Threshold aftermath. Seam remembers the leaving.
/// Cite ART_BIBLE iron + tend seam · DRAEK_ORIGIN never-forgotten betrayal
/// / unfinished business · PLAYABLE_RACES Threshold landing.
pub const QUELLORIAN_AFTERMATH: &str = "seam remembers the leaving";

/// CARD S1 — Draek Depths aftermath. Consume-scar + teal way-home.
/// Cite DRAEK_ORIGIN consume / FACTIONS_OVERVIEW consumption · ART_BIBLE
/// teal Peace · PLAYABLE_RACES Depths (teal way-home). Want stays
/// [`DEPTHS_WANT`] (restore, not Take).
pub const DRAEK_AFTERMATH: &str = "consume-scar · teal way-home";

/// CARD S1 — Place-local aftermath evidence keyed by PeopleLanding.
/// Same war, five reads. Want stays L6 PlaceId ([`want_after_people_landing`]).
pub fn aftermath_evidence_for_landing(landing: PeopleLanding) -> &'static str {
    match landing {
        PeopleLanding::SanctuaryYard => HUMAN_AFTERMATH,
        PeopleLanding::SanctuaryWellFromAbove => AMBROSIAN_AFTERMATH,
        PeopleLanding::Heartwood => CYDRUID_AFTERMATH,
        PeopleLanding::Threshold => QUELLORIAN_AFTERMATH,
        PeopleLanding::DepthsTealWayHome => DRAEK_AFTERMATH,
    }
}

/// CARD S1 — after land, aftermath is the People-local evidence line.
/// Skip House / no land keeps GARDEN_WANT (same as L6 boot Want).
pub fn aftermath_after_people_landing(landing: Option<PeopleLanding>) -> &'static str {
    match landing {
        Some(land) => aftermath_evidence_for_landing(land),
        None => GARDEN_WANT,
    }
}

/// CARD S1 — spoken after land (or on Place Want): existing L6 Want plus
/// landing aftermath. Skip House keeps the Garden boot People + Want.
/// Not a trailer. People prefix stays Human (ART_BIBLE cite).
pub fn first_minutes_aftermath_line(landing: Option<PeopleLanding>) -> String {
    match landing {
        None => first_minutes_people_want_line(),
        Some(land) => format!(
            "{} · {}",
            first_minutes_people_want_line_for_place(land.place_id()),
            aftermath_evidence_for_landing(land)
        ),
    }
}

/// CARD S1 — Title garden / first-session guidance after land.
/// Garden boot (no land) keeps [`garden_boot_want_line`]. After a sealed
/// People-door land the stranger hears Place-local aftermath evidence.
/// H hush drops the line. Comfort Low is not a text gate.
pub fn garden_guidance_after_land(
    on_garden_boot: bool,
    hush: bool,
    landing: Option<PeopleLanding>,
) -> Option<String> {
    if hush {
        return None;
    }
    match landing {
        Some(land) => Some(first_minutes_aftermath_line(Some(land))),
        None => garden_boot_want_line(on_garden_boot, false),
    }
}

/// CARD F7 — Human extra aftermath variant. Local well evidence.
/// Cite-only `docs/ART_BIBLE.md` Sanctuary: warm gold well · one glow.
/// Cite-only `docs/DRIVE_LORE_ADAPTATION.md` refuse weekly wars as Sanctuary.
/// Not a trailer. Want stays [`SANCTUARY_WANT`]. S1 [`HUMAN_AFTERMATH`] unread.
pub const HUMAN_AFTERMATH_VARIANT: &str = "warm gold well · one glow";

/// CARD F7 — Ambrosian extra aftermath variant. Local well-from-above evidence.
/// Cite-only `docs/PLAYABLE_RACES.md` Sanctuary well-from-above.
/// Cite-only `docs/ART_BIBLE.md` prism cool. Same Place as Human.
/// Not a hull / fleet shot. S1 [`AMBROSIAN_AFTERMATH`] unread.
pub const AMBROSIAN_AFTERMATH_VARIANT: &str = "well-from-above · prism cool";

/// CARD F7 — Cydruid extra aftermath variant. Local Heartwood evidence.
/// Cite-only `docs/ART_BIBLE.md` amber lamp.
/// Cite-only `docs/PLAYABLE_RACES.md` living-wood Place, person not the tree.
/// Cite-only `docs/DRIVE_LORE_ADAPTATION.md` Cydruid = human-in-frame, not treant.
/// Want stays [`HEARTWOOD_WANT`]. S1 [`CYDRUID_AFTERMATH`] unread.
pub const CYDRUID_AFTERMATH_VARIANT: &str = "amber lamp · person not the tree";

/// CARD F7 — Quellorian extra aftermath variant. Local Threshold evidence.
/// Cite-only `docs/ART_BIBLE.md` iron + tend seam · E Tends the pipe.
/// Cite-only `docs/DRIVE_LORE_ADAPTATION.md` late mercy → Heartwood / Wards.
/// Not a Codex city. S1 [`QUELLORIAN_AFTERMATH`] unread.
pub const QUELLORIAN_AFTERMATH_VARIANT: &str = "iron + tend seam";

/// CARD F7 — Draek extra aftermath variant. Local Depths evidence.
/// Cite-only `docs/ART_BIBLE.md` teal Peace · Depths Peace plate.
/// Cite-only `docs/PLAYABLE_RACES.md` Depths (teal way-home).
/// Restore, not Take (word stays off this line so S1 Take-refuse holds).
/// S1 [`DRAEK_AFTERMATH`] unread.
pub const DRAEK_AFTERMATH_VARIANT: &str = "teal Peace plate";

/// CARD F7 — exactly one extra local-evidence line keyed by PeopleLanding.
/// S1 [`aftermath_evidence_for_landing`] stays unread.
pub fn f7_aftermath_variant_for_landing(landing: PeopleLanding) -> &'static str {
    match landing {
        PeopleLanding::SanctuaryYard => HUMAN_AFTERMATH_VARIANT,
        PeopleLanding::SanctuaryWellFromAbove => AMBROSIAN_AFTERMATH_VARIANT,
        PeopleLanding::Heartwood => CYDRUID_AFTERMATH_VARIANT,
        PeopleLanding::Threshold => QUELLORIAN_AFTERMATH_VARIANT,
        PeopleLanding::DepthsTealWayHome => DRAEK_AFTERMATH_VARIANT,
    }
}

/// CARD F7 — skip House / no land keeps the S1 garden boot Want.
/// After land, S1 plate plus exactly one extra variant.
pub fn f7_aftermath_variant_after_people_landing(
    landing: Option<PeopleLanding>,
) -> Option<&'static str> {
    landing.map(f7_aftermath_variant_for_landing)
}

/// CARD F7 — existing S1 aftermath plate plus exactly one extra local-evidence
/// line per People. Skip House keeps [`first_minutes_aftermath_line`] (Garden Want).
/// Not a trailer. People prefix stays Human (ART_BIBLE cite).
pub fn f7_first_minutes_aftermath_line(landing: Option<PeopleLanding>) -> String {
    match landing {
        None => first_minutes_aftermath_line(None),
        Some(land) => format!(
            "{} · {}",
            first_minutes_aftermath_line(Some(land)),
            f7_aftermath_variant_for_landing(land)
        ),
    }
}

/// CARD F7 — Title garden / guidance Want after land: S1 plate + one variant.
/// Garden boot (no land) keeps [`garden_boot_want_line`]. H hush drops the line.
pub fn f7_garden_guidance_after_land(
    on_garden_boot: bool,
    hush: bool,
    landing: Option<PeopleLanding>,
) -> Option<String> {
    if hush {
        return None;
    }
    match landing {
        Some(land) => Some(f7_first_minutes_aftermath_line(Some(land))),
        None => garden_boot_want_line(on_garden_boot, false),
    }
}

/// Soft objective the player is gently invited to try next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuidanceObjective {
    MoveAround,
    ApproachGlowingNode,
    HarvestWithInteract,
    OpenInventory,
    ShareAbundance,
    StepCharter,
    PlantHouse,
    OpenLedger,
    BindEscort,
    HourTwoHeld,
    PlantFabricator,
    EmbassySeat,
    HourThreeHeld,
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
            GuidanceObjective::OpenInventory => "I satchel · House after allocate",
            GuidanceObjective::ShareAbundance => "R then 1 flow · 2 reserve",
            GuidanceObjective::StepCharter => "Tab the ridge",
            GuidanceObjective::PlantHouse => "Q plant a House stake",
            GuidanceObjective::OpenLedger => "L opens the Ledger",
            GuidanceObjective::BindEscort => "E Bind then escort",
            GuidanceObjective::HourTwoHeld => "climate on slab · week tons+restored",
            GuidanceObjective::PlantFabricator => "Q after arrival — plant the fabricator",
            GuidanceObjective::EmbassySeat => "Embassy lamp · E Request seat",
            GuidanceObjective::HourThreeHeld => "Hour three · the book is yours",
            GuidanceObjective::FeelFirstEpiphany => "The field answers",
            GuidanceObjective::MeetCouncilWhisper => "The field answers",
            GuidanceObjective::FreeExploration => "this hex admits harm · optional",
        }
    }

    /// Walk · glow · first tend — the stranger-hour first minutes.
    /// Not a new verb. WASD / E stay the hands.
    pub fn is_first_minutes(&self) -> bool {
        matches!(
            self,
            GuidanceObjective::MoveAround
                | GuidanceObjective::ApproachGlowingNode
                | GuidanceObjective::HarvestWithInteract
        )
    }

    pub fn next(&self) -> Self {
        match self {
            GuidanceObjective::MoveAround => GuidanceObjective::ApproachGlowingNode,
            GuidanceObjective::ApproachGlowingNode => GuidanceObjective::HarvestWithInteract,
            GuidanceObjective::HarvestWithInteract => GuidanceObjective::OpenInventory,
            GuidanceObjective::OpenInventory => GuidanceObjective::ShareAbundance,
            GuidanceObjective::ShareAbundance => GuidanceObjective::StepCharter,
            GuidanceObjective::StepCharter => GuidanceObjective::PlantHouse,
            GuidanceObjective::PlantHouse => GuidanceObjective::OpenLedger,
            GuidanceObjective::OpenLedger => GuidanceObjective::BindEscort,
            GuidanceObjective::BindEscort => GuidanceObjective::HourTwoHeld,
            GuidanceObjective::HourTwoHeld => GuidanceObjective::PlantFabricator,
            GuidanceObjective::PlantFabricator => GuidanceObjective::EmbassySeat,
            GuidanceObjective::EmbassySeat => GuidanceObjective::HourThreeHeld,
            GuidanceObjective::HourThreeHeld => GuidanceObjective::FreeExploration,
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
    pub ridge_stepped: bool,
    pub house_live: bool,
    pub ledger_open: bool,
    pub hour_two_held: bool,
    pub proof_pack: bool,
    pub embassy_seated: bool,
    pub hour_three_held: bool,
    /// CARD S1 — People-door land this session. None = skip House /
    /// Garden boot. After land, Title garden + Place Want hear aftermath.
    /// CARD S2 — land is not a seal; [`gate_sealed`] is the E/Q confirm.
    pub people_landing: Option<PeopleLanding>,
    /// CARD S2 — E/Q confirm at landing ties this soul to People+Place.
    pub gate_sealed: bool,
    /// CARD S2 — sealed People after confirm. None while light / unsealed.
    pub sealed_people: Option<HousePeople>,
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
            ridge_stepped: false,
            house_live: false,
            ledger_open: false,
            hour_two_held: false,
            proof_pack: false,
            embassy_seated: false,
            hour_three_held: false,
            people_landing: None,
            gate_sealed: false,
            sealed_people: None,
        }
    }
}

impl FirstSessionGuidance {
    pub fn dismiss(&mut self) {
        self.dismissed = true;
        self.active = false;
    }

    /// First minutes speak People + Want until H hushes the card.
    pub fn speaks_people_want(&self) -> bool {
        self.active && !self.dismissed && self.objective.is_first_minutes()
    }

    /// CARD L2-REPLAY — dress token / people tint after House only.
    pub fn house_dress_token(&self) -> Option<&'static str> {
        house_dress_token(self.house_live)
    }

    /// True when the stranger still wears Peace default dress (no House).
    pub fn wears_peace_default_dress(&self) -> bool {
        is_peace_default_dress(self.house_live)
    }

    /// CARD L2 — Q House offers five Peoples. Skip House stays light / Peace.
    pub fn offers_five_peoples(&self) -> bool {
        self.house_live
    }

    /// Skip House: stay light-body / Peace default. No People offer.
    pub fn stays_light_peace(&self) -> bool {
        skip_house_stays_light(self.house_live)
    }

    /// Five God-plane doors ignite after one Tend (Hour-1 harvest).
    pub fn god_plane_doors_ignited(&self) -> bool {
        god_plane_doors_ignited(self.harvests_completed >= 1)
    }

    /// Five Peoples after House. None while the stranger is still light / Peace.
    pub fn house_people_offer(&self) -> Option<[HousePeople; 5]> {
        offer_house_peoples(self.house_live)
    }

    /// Cross one ignited God-plane door. One-way this session. Card stays Ledger.
    /// CARD L5 — same apply_people_landing wire as Title garden_cross_landing.
    /// CARD S1 — records the land so Title garden / Place Want hear aftermath.
    pub fn try_cross_people_door(
        &mut self,
        crossed: &mut Option<HousePeople>,
        people: HousePeople,
        travel: &mut HexTravelState,
        bind: &mut LivedHourBind,
        embassy: Option<&mut EmbassyYard>,
        presence: Option<&mut SoftPresence>,
    ) -> Option<PeopleLanding> {
        let land = garden_cross_people_land(
            self.house_live,
            self.harvests_completed >= 1,
            crossed,
            people,
            travel,
            bind,
            embassy,
            presence,
        );
        self.people_landing = land;
        land
    }

    /// CARD L3 — Garden wrapper: L2 cross then apply_place / lived bind. Call only.
    /// CARD S1 — records the land so aftermath evidence is Place-local.
    pub fn garden_cross_people_land(
        &mut self,
        crossed: &mut Option<HousePeople>,
        people: HousePeople,
        travel: &mut HexTravelState,
        bind: &mut LivedHourBind,
        embassy: Option<&mut EmbassyYard>,
        presence: Option<&mut SoftPresence>,
    ) -> Option<PeopleLanding> {
        self.try_cross_people_door(crossed, people, travel, bind, embassy, presence)
    }

    /// CARD S1 — after land, the stranger hears Place-local aftermath.
    /// Skip House keeps garden / Sanctuary boot Want.
    pub fn aftermath_evidence_line(&self) -> &'static str {
        aftermath_after_people_landing(self.people_landing)
    }

    /// CARD F7 — existing S1 plate plus exactly one extra local-evidence line.
    /// Skip House keeps Garden / Sanctuary boot Want. S1 evidence line unread.
    pub fn f7_aftermath_plate_line(&self) -> String {
        f7_first_minutes_aftermath_line(self.people_landing)
    }

    /// CARD S2 — new soul / skip House / declined door stays light.
    pub fn is_light(&self) -> bool {
        !self.gate_sealed && soul_is_light(self.sealed_pair())
    }

    /// CARD S2 — doors stay unsealed until E/Q confirm.
    pub fn doors_unsealed(&self) -> bool {
        !self.gate_sealed && doors_are_unsealed(self.sealed_pair())
    }

    fn sealed_pair(&self) -> Option<(HousePeople, PeopleLanding)> {
        match (self.sealed_people, self.people_landing) {
            (Some(people), Some(landing)) if self.gate_sealed => Some((people, landing)),
            _ => None,
        }
    }

    /// CARD S2 — existing E/Q at landing seals this People+Place for the soul.
    pub fn confirm_seal_at_landing(
        &mut self,
        key: PeaceKey,
        crossed: Option<HousePeople>,
    ) -> Option<(HousePeople, PeopleLanding)> {
        let pending = match (crossed, self.people_landing) {
            (Some(people), Some(landing)) => Some((people, landing)),
            _ => None,
        };
        let sealed = confirm_gate_seal(key, pending)?;
        self.gate_sealed = true;
        self.sealed_people = Some(sealed.0);
        Some(sealed)
    }

    /// CARD S2 — decline / wrong door: garden, still light, not tied.
    /// Restores `garden` PlaceId. No-op after seal.
    pub fn decline_or_wrong_door(
        &mut self,
        crossed: &mut Option<HousePeople>,
        travel: &mut HexTravelState,
        bind: &mut LivedHourBind,
        embassy: Option<&mut EmbassyYard>,
        garden: PlaceId,
    ) -> bool {
        let sealed = self.sealed_pair();
        if !crate::hour_sacred::decline_or_wrong_door(
            crossed,
            &mut self.people_landing,
            sealed,
        ) {
            return false;
        }
        decline_people_door_land(travel, bind, embassy, garden, crossed);
        self.gate_sealed = false;
        self.sealed_people = None;
        true
    }

    /// CARD S2 — Peace recall = vision home. Does not unseal.
    pub fn peace_recall(&self) -> &'static str {
        let (_, line) = crate::hour_sacred::peace_recall(self.sealed_pair());
        line
    }

    /// CARD L2 — 0 meshes · ASSET_BUDGET cite only. Five Peoples, not a dress token.
    pub fn l2_asset_budget_holds(&self) -> bool {
        L2_MESH_BUDGET == 0
            && L2_ASSET_BUDGET_CITE == "docs/ASSET_BUDGET_COURT.md"
            && HOUSE_PEOPLES.len() == 5
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
            GuidanceObjective::StepCharter => self.ridge_stepped || self.house_live,
            GuidanceObjective::PlantHouse => self.house_live,
            GuidanceObjective::OpenLedger => self.ledger_open || self.hour_two_held,
            GuidanceObjective::BindEscort => self.hour_two_held,
            GuidanceObjective::HourTwoHeld => false,
            GuidanceObjective::PlantFabricator => self.proof_pack,
            GuidanceObjective::EmbassySeat => self.embassy_seated,
            GuidanceObjective::HourThreeHeld => false,
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

    /// Quit/rerun: do not re-teach WASD if the pack already holds the yard.
    /// Same-session Settled keeps HourTwoHeld so the card can breathe before Hour three.
    pub fn resume_from_pack(&mut self) {
        if self.dismissed {
            return;
        }
        if self.hour_three_held {
            self.objective = GuidanceObjective::HourThreeHeld;
            return;
        }
        if self.hour_two_held {
            if self.embassy_seated {
                self.objective = GuidanceObjective::HourThreeHeld;
            } else if self.proof_pack {
                self.objective = GuidanceObjective::EmbassySeat;
            } else if matches!(
                self.objective,
                GuidanceObjective::MoveAround
                    | GuidanceObjective::ApproachGlowingNode
                    | GuidanceObjective::HarvestWithInteract
                    | GuidanceObjective::OpenInventory
                    | GuidanceObjective::ShareAbundance
                    | GuidanceObjective::StepCharter
                    | GuidanceObjective::PlantHouse
                    | GuidanceObjective::OpenLedger
            ) {
                // Cold Continue / quit-rerun only — skip walk→allocate when the pack is held.
                self.objective = GuidanceObjective::PlantFabricator;
            }
            // BindEscort / HourTwoHeld / PlantFabricator+: leave the live card alone.
            return;
        }
        if self.house_live {
            if matches!(
                self.objective,
                GuidanceObjective::MoveAround
                    | GuidanceObjective::ApproachGlowingNode
                    | GuidanceObjective::HarvestWithInteract
                    | GuidanceObjective::OpenInventory
                    | GuidanceObjective::ShareAbundance
                    | GuidanceObjective::StepCharter
                    | GuidanceObjective::PlantHouse
            ) {
                self.objective = GuidanceObjective::OpenLedger;
            }
            return;
        }
        if self.ridge_stepped {
            if matches!(
                self.objective,
                GuidanceObjective::MoveAround
                    | GuidanceObjective::ApproachGlowingNode
                    | GuidanceObjective::HarvestWithInteract
                    | GuidanceObjective::OpenInventory
                    | GuidanceObjective::ShareAbundance
                    | GuidanceObjective::StepCharter
            ) {
                self.objective = GuidanceObjective::PlantHouse;
            }
        }
    }
}

/// CARD L3 / L5 / L6 / S1 — Garden wrapper. Calls L2 try_cross then apply_people_landing.
/// After land, Want follows `landing.place_id()` via [`want_after_people_landing`].
/// Aftermath evidence follows PeopleLanding via [`aftermath_after_people_landing`].
pub fn garden_cross_people_land(
    house_live: bool,
    tended_once: bool,
    crossed: &mut Option<HousePeople>,
    people: HousePeople,
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    embassy: Option<&mut EmbassyYard>,
    presence: Option<&mut SoftPresence>,
) -> Option<PeopleLanding> {
    let landing = try_cross_people_door(house_live, tended_once, crossed, people)?;
    apply_people_landing(travel, bind, embassy, landing, presence);
    Some(landing)
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

/// CARD FLESH-GUIDANCE-PLACE — spoken room while standing.
/// Threshold-near is Heartwood plus existing shelf reach, not a PlaceId.
fn stood_place_name(place: PlaceId, threshold_near: bool) -> &'static str {
    match place {
        PlaceId::Sanctuary => "Sanctuary",
        PlaceId::Depths => "Depths",
        PlaceId::Heartwood if threshold_near => "Threshold-near",
        PlaceId::Heartwood => "Heartwood",
    }
}

/// Place label when travel is already in the world. None until that state exists.
fn stood_place_label(
    travel: Option<&HexTravelState>,
    presence: Option<&SoftPresence>,
) -> Option<&'static str> {
    let travel = travel?;
    let near = presence.is_some_and(|body| {
        shared::threshold_shelf::threshold_use_in_reach(
            travel.current,
            body.position.x,
            body.position.z,
        )
    });
    Some(stood_place_name(travel.current, near))
}

/// CARD FLESH-GUIDANCE-PLACE — one existing sentence (the week-bill card)
/// may prefix the Place. Tend, week tons+restored, and Continue stay.
fn spoken_guidance(objective: &GuidanceObjective, place: Option<&str>) -> String {
    let prompt = objective.prompt();
    if *objective == GuidanceObjective::HourTwoHeld {
        if let Some(place) = place {
            return format!("{place} · {prompt}");
        }
    }
    prompt.to_string()
}

fn update_guidance_visibility(
    guidance: Res<FirstSessionGuidance>,
    bind: Option<Res<LivedHourBind>>,
    door: Res<LaunchDoor>,
    mut query: Query<&mut Visibility, With<FirstSessionGuidanceStrip>>,
) {
    let hidden_by_bind = bind.map(|b| b.guidance_hidden).unwrap_or(false);
    let in_yard = *door == LaunchDoor::InYard;
    let show = in_yard && guidance.active && !guidance.dismissed && !hidden_by_bind;
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
    travel: Option<Res<HexTravelState>>,
    presence: Option<Res<SoftPresence>>,
    mut last_place: Local<Option<&'static str>>,
    mut query: Query<&mut Text, With<FirstSessionGuidanceText>>,
) {
    let place = stood_place_label(travel.as_deref(), presence.as_deref());
    let place_changed = *last_place != place;
    if !guidance.is_changed() && !place_changed {
        return;
    }
    *last_place = place;
    let prompt = if guidance.dismissed {
        String::new()
    } else {
        card_line(&spoken_guidance(&guidance.objective, place))
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
    hour: Option<Res<HourSacred>>,
    ledger: Option<Res<LedgerYard>>,
    fab: Option<Res<FabricatorYard>>,
    embassy: Option<Res<EmbassyYard>>,
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

    if let Some(hour) = hour {
        if hour.hex() != HexFlag::Peace {
            guidance.ridge_stepped = true;
        }
        if hour.charter_skin_live() {
            guidance.house_live = true;
        }
        if hour.complete {
            guidance.hour_two_held = true;
        }
        if hour.hour_three_complete {
            guidance.hour_three_held = true;
        }
    }

    if let Some(ledger) = ledger {
        if ledger.sash_open {
            guidance.ledger_open = true;
        }
        if ledger
            .board
            .open()
            .map(|c| c.state == ContractState::Settled)
            .unwrap_or(false)
        {
            guidance.hour_two_held = true;
        }
    }


    if let Some(fab) = fab {
        if fab.fab.pack.unlocked() {
            guidance.proof_pack = true;
        }
    }
    if let Some(embassy) = embassy {
        if embassy.embassy.seated {
            guidance.embassy_seated = true;
        }
    }
    guidance.resume_from_pack();
    guidance.advance_if_ready();

    if guidance.objective == GuidanceObjective::HourTwoHeld
        || guidance.objective == GuidanceObjective::HourThreeHeld
        || guidance.objective == GuidanceObjective::FreeExploration
    {
        guidance.free_since += time.delta_seconds();
        if guidance.objective == GuidanceObjective::HourTwoHeld && guidance.free_since > 6.0 {
            guidance.objective = GuidanceObjective::PlantFabricator;
            guidance.free_since = 0.0;
        } else if guidance.objective == GuidanceObjective::HourThreeHeld && guidance.free_since > 6.0 {
            guidance.objective = GuidanceObjective::FreeExploration;
            guidance.free_since = 0.0;
        } else if guidance.objective == GuidanceObjective::FreeExploration && guidance.free_since > 8.0
        {
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
    fn hour_is_four_hands_then_ridge() {
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
        assert_eq!(g.objective, GuidanceObjective::StepCharter);
    }

    /// Playtest H2-TAB: after allocate the card is *Tab the ridge*;
    /// Tab (ridge_stepped) advances to *Q plant a House stake*.
    #[test]
    fn h2_tab_after_allocate_card_is_plant_house() {
        let mut g = FirstSessionGuidance::default();
        g.objective = GuidanceObjective::ShareAbundance;
        g.shared_abundance = true;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::StepCharter);
        assert_eq!(g.objective.prompt(), "Tab the ridge");
        g.ridge_stepped = true;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::PlantHouse);
        assert_eq!(g.objective.prompt(), "Q plant a House stake");
    }

    /// Playtest H2-TAB: quit mid-ridge still skips WASD and names PlantHouse.
    #[test]
    fn h2_tab_resume_from_ridge_is_plant_house() {
        let mut g = FirstSessionGuidance::default();
        assert_eq!(g.objective, GuidanceObjective::MoveAround);
        g.ridge_stepped = true;
        g.resume_from_pack();
        assert_eq!(g.objective, GuidanceObjective::PlantHouse);
    }

    #[test]
    fn hour_two_card_walks_to_held() {
        let mut g = FirstSessionGuidance::default();
        g.objective = GuidanceObjective::StepCharter;
        g.ridge_stepped = true;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::PlantHouse);
        g.house_live = true;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::OpenLedger);
        g.ledger_open = true;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::BindEscort);
        g.hour_two_held = true;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::HourTwoHeld);
        // Same-session: resume must not skip the Hour-two held card.
        g.resume_from_pack();
        assert_eq!(g.objective, GuidanceObjective::HourTwoHeld);
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::HourTwoHeld);
    }

    #[test]
    fn prompts_are_one_sentence() {
        for obj in [
            GuidanceObjective::MoveAround,
            GuidanceObjective::ApproachGlowingNode,
            GuidanceObjective::HarvestWithInteract,
            GuidanceObjective::OpenInventory,
            GuidanceObjective::ShareAbundance,
            GuidanceObjective::StepCharter,
            GuidanceObjective::PlantHouse,
            GuidanceObjective::OpenLedger,
            GuidanceObjective::BindEscort,
            GuidanceObjective::HourTwoHeld,
            GuidanceObjective::PlantFabricator,
            GuidanceObjective::EmbassySeat,
            GuidanceObjective::HourThreeHeld,
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

    #[test]
    fn hour_three_card_after_proof_pack() {
        let mut g = FirstSessionGuidance::default();
        g.objective = GuidanceObjective::PlantFabricator;
        g.proof_pack = true;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::EmbassySeat);
        g.embassy_seated = true;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::HourThreeHeld);
    }

    #[test]
    fn resume_held_skips_walk() {
        let mut g = FirstSessionGuidance::default();
        g.hour_two_held = true;
        g.resume_from_pack();
        assert_eq!(g.objective, GuidanceObjective::PlantFabricator);
        g.proof_pack = true;
        g.resume_from_pack();
        assert_eq!(g.objective, GuidanceObjective::EmbassySeat);
        g.embassy_seated = true;
        g.hour_three_held = true;
        g.resume_from_pack();
        assert_eq!(g.objective, GuidanceObjective::HourThreeHeld);
    }

    /// Playtest H2-RESUME: rerun does not re-teach WASD when Hour two is held.
    #[test]
    fn h2_resume_does_not_reteach_wasd() {
        let mut g = FirstSessionGuidance::default();
        assert!(g.objective.prompt().contains("WASD"));
        g.hour_two_held = true;
        g.resume_from_pack();
        assert!(!g.objective.prompt().contains("WASD"));
        assert_eq!(g.objective, GuidanceObjective::PlantFabricator);
    }

    #[test]
    fn resume_house_skips_to_ledger() {
        let mut g = FirstSessionGuidance::default();
        g.house_live = true;
        g.resume_from_pack();
        assert_eq!(g.objective, GuidanceObjective::OpenLedger);
    }

    #[test]
    fn playable_loop_soft_cues_stay_quiet() {
        let harvest_next = GuidanceObjective::OpenInventory.prompt();
        assert!(harvest_next.contains("House"));
        assert!(harvest_next.len() < 48);

        let hour_two = GuidanceObjective::HourTwoHeld.prompt();
        assert!(hour_two.contains("climate") || hour_two.contains("week"));
        assert!(hour_two.contains("tons"));
        assert!(!hour_two.to_lowercase().contains("kill"));
        assert!(!hour_two.to_lowercase().contains("lethal"));
        assert!(hour_two.len() < 48);

        let after_book = GuidanceObjective::FreeExploration.prompt();
        assert!(after_book.contains("this hex admits harm") || after_book.contains("optional"));
        assert!(!after_book.to_lowercase().contains("combat"));
        assert!(after_book.len() < 48);

        let book = GuidanceObjective::HourThreeHeld.prompt();
        assert!(book.contains("book"));
        assert!(book.len() < 48);
    }

    /// CARD L1 SANCTUARY-WANT — prove-line: People + Want spoken in first minutes.
    #[test]
    fn first_minutes_speak_people_and_want() {
        let mut g = FirstSessionGuidance::default();
        assert!(g.objective.is_first_minutes());
        assert!(g.speaks_people_want());
        let line = first_minutes_people_want_line();
        assert!(line.contains(SANCTUARY_PEOPLE));
        assert!(line.contains(SANCTUARY_WANT));
        assert_eq!(line, "Human · the yard needs tending or the well goes quiet");
        // Hour 1 walk teaching stays on the card (0 new verbs).
        assert!(g.objective.prompt().contains("WASD"));
        g.moved_distance = 5.0;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::ApproachGlowingNode);
        assert!(g.speaks_people_want());
        g.near_glow = true;
        g.advance_if_ready();
        assert_eq!(g.objective, GuidanceObjective::HarvestWithInteract);
        assert!(g.speaks_people_want());
        credit_harvest(&mut g);
        assert_eq!(g.objective, GuidanceObjective::OpenInventory);
        assert!(!g.objective.is_first_minutes());
        assert!(!g.speaks_people_want());
    }

    /// CARD L2-REPLAY — stranger without House stays Peace default.
    #[test]
    fn stranger_without_house_stays_peace_default_dress() {
        let g = FirstSessionGuidance::default();
        assert!(!g.house_live);
        assert!(g.wears_peace_default_dress());
        assert!(g.house_dress_token().is_none());
        // First minutes still People+Want (L1); no race lobby / dress token yet.
        assert!(g.speaks_people_want());
        assert!(g.objective.prompt().contains("WASD"));
    }

    /// CARD L2-REPLAY — House live lands one dress token / people tint.
    /// Card stays Ledger (not a Title create / race lobby).
    #[test]
    fn house_live_lands_one_dress_token() {
        let mut g = FirstSessionGuidance::default();
        g.house_live = true;
        g.resume_from_pack();
        assert_eq!(g.objective, GuidanceObjective::OpenLedger);
        assert_eq!(g.house_dress_token(), Some(HOUSE_PEOPLE_TINT));
        assert!(!g.wears_peace_default_dress());
        assert_eq!(g.objective.prompt(), "L opens the Ledger");
        assert!(!g.objective.prompt().to_lowercase().contains("race"));
        assert!(!g.objective.prompt().to_lowercase().contains("lobby"));
    }

    /// CARD L1 SANCTUARY-WANT — H hush still works after People + Want.
    #[test]
    fn h_hush_still_works_after_people_want() {
        let mut g = FirstSessionGuidance::default();
        assert!(g.speaks_people_want());
        g.dismiss();
        assert!(g.dismissed);
        assert!(!g.active);
        assert!(!g.speaks_people_want());
        g.advance_if_ready();
        assert!(g.dismissed);
        assert!(!g.speaks_people_want());
        // World still owns the Want line; the card just hushes.
        assert_eq!(
            first_minutes_people_want_line(),
            "Human · the yard needs tending or the well goes quiet"
        );
    }

    /// CARD L1 GARDEN-WANT — prove-line: Want on Garden / boot plane, no Sanctuary dirt.
    #[test]
    fn garden_boot_speaks_want_without_sanctuary_dirt() {
        let spoken = garden_boot_want_line(true, false).expect("Want on Garden boot");
        assert_eq!(spoken, first_minutes_people_want_line());
        assert_eq!(GARDEN_PEOPLE, SANCTUARY_PEOPLE);
        assert_eq!(GARDEN_WANT, SANCTUARY_WANT);
        assert!(spoken.contains(GARDEN_PEOPLE));
        assert!(spoken.contains(GARDEN_WANT));
        assert_eq!(spoken, "Human · the yard needs tending or the well goes quiet");
        assert!(spoken.contains("tend"));
        assert!(spoken.contains("the well goes quiet"));
        // Cite only — People stays Human; no fifth Place named on the boot line.
        assert!(!spoken.contains("Sanctuary"));
        assert!(!spoken.contains("Heartwood"));
        assert!(!spoken.contains("Market"));
        // Not on the boot plane → Garden line stays silent (0 new Places / no dirt required).
        assert!(garden_boot_want_line(false, false).is_none());
        // 0 new verbs — WASD / E stay the hands; this line is Want speech only.
        assert!(!spoken.contains("WASD"));
        assert!(!spoken.contains("Tab"));
        assert!(!spoken.contains("Q plant"));
    }

    /// CARD L1 GARDEN-WANT — H hush still works on the Garden / boot plane.
    #[test]
    fn garden_boot_h_hush_still_works() {
        let mut g = FirstSessionGuidance::default();
        assert!(g.speaks_people_want());
        assert!(garden_boot_want_line(true, !g.speaks_people_want()).is_some());
        g.dismiss();
        assert!(g.dismissed);
        assert!(!g.active);
        assert!(!g.speaks_people_want());
        assert!(garden_boot_want_line(true, !g.speaks_people_want()).is_none());
        g.advance_if_ready();
        assert!(g.dismissed);
        assert!(garden_boot_want_line(true, !g.speaks_people_want()).is_none());
        // Retargeted constants survive hush — the plane just stops speaking.
        assert_eq!(GARDEN_WANT, SANCTUARY_WANT);
        assert_eq!(
            first_minutes_people_want_line(),
            "Human · the yard needs tending or the well goes quiet"
        );
    }

    /// CARD L2 HOUSE-PEOPLE-GATES — House offers five Peoples; card stays Ledger.
    /// Not the #459 dress-token prove-line. Hour 2 walk unchanged.
    #[test]
    fn house_live_offers_five_peoples_card_stays_ledger() {
        let mut g = FirstSessionGuidance::default();
        assert!(g.stays_light_peace());
        assert!(g.house_people_offer().is_none());
        assert!(!g.offers_five_peoples());
        g.house_live = true;
        g.resume_from_pack();
        assert_eq!(g.objective, GuidanceObjective::OpenLedger);
        assert_eq!(g.objective.prompt(), "L opens the Ledger");
        assert!(g.offers_five_peoples());
        assert!(!g.stays_light_peace());
        let offer = g.house_people_offer().expect("five Peoples after House");
        assert_eq!(offer, HOUSE_PEOPLES);
        assert_eq!(offer.len(), 5);
        assert_eq!(HousePeople::Cydruid.people_line(), "Cydruid · human-in-frame");
        assert!(!HousePeople::Cydruid.people_line().contains("treant"));
        assert!(!g.objective.prompt().to_lowercase().contains("race"));
        assert!(!g.objective.prompt().to_lowercase().contains("lobby"));
        for people in offer {
            assert!(!people.people_line().contains("Sanctuary tint"));
            assert!(!people.people_line().contains("dress token"));
        }
        assert_eq!(L2_MESH_BUDGET, 0);
        assert_eq!(L2_ASSET_BUDGET_CITE, "docs/ASSET_BUDGET_COURT.md");
    }

    /// CARD L2 — skip House stays light / Peace; doors dark until one Tend;
    /// crossing is one-way this session. Human → Sanctuary yard.
    #[test]
    fn skip_house_stays_light_doors_ignite_after_tend() {
        let mut g = FirstSessionGuidance::default();
        assert!(g.stays_light_peace());
        assert!(!g.god_plane_doors_ignited());
        let mut crossed = None;
        let mut travel = HexTravelState {
            current: shared::hex_travel::PlaceId::Sanctuary,
        };
        let mut bind = l5_demo_bind();
        assert!(g
            .try_cross_people_door(
                &mut crossed,
                HousePeople::Human,
                &mut travel,
                &mut bind,
                None,
                None,
            )
            .is_none());
        credit_harvest(&mut g);
        assert!(g.god_plane_doors_ignited());
        assert!(
            g.try_cross_people_door(
                &mut crossed,
                HousePeople::Human,
                &mut travel,
                &mut bind,
                None,
                None,
            )
            .is_none(),
            "Tend alone does not cross — skip House stays light"
        );
        g.house_live = true;
        let land = g
            .try_cross_people_door(
                &mut crossed,
                HousePeople::Human,
                &mut travel,
                &mut bind,
                None,
                None,
            )
            .expect("House + Tend opens one door");
        assert_eq!(land, PeopleLanding::SanctuaryYard);
        assert_eq!(land.landing_line(), "Sanctuary yard");
        assert!(
            g.try_cross_people_door(
                &mut crossed,
                HousePeople::Draek,
                &mut travel,
                &mut bind,
                None,
                None,
            )
            .is_none(),
            "crossing is one-way this session"
        );
        assert_eq!(crossed, Some(HousePeople::Human));
        assert_eq!(
            HousePeople::Draek.landing().landing_line(),
            "Depths (teal way-home)"
        );
        assert_eq!(
            HousePeople::Ambrosian.landing().place_name(),
            "Sanctuary"
        );
        assert_eq!(L2_MESH_BUDGET, 0);
    }

    fn l5_demo_bind() -> LivedHourBind {
        LivedHourBind {
            hour: shared::climate_node::LivedHour::new_demo(),
            climate: Default::default(),
            standing: Default::default(),
            week: Default::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        }
    }

    fn l5_first_session_land(
        house: bool,
        tend: bool,
        people: HousePeople,
        start: shared::hex_travel::PlaceId,
        mut presence: Option<&mut SoftPresence>,
    ) -> (Option<PeopleLanding>, shared::hex_travel::PlaceId) {
        let mut g = FirstSessionGuidance::default();
        g.house_live = house;
        if tend {
            g.harvests_completed = 1;
        }
        let mut travel = HexTravelState { current: start };
        let mut bind = l5_demo_bind();
        let mut crossed = None;
        let land = g.try_cross_people_door(
            &mut crossed,
            people,
            &mut travel,
            &mut bind,
            None,
            presence.as_deref_mut(),
        );
        (land, travel.current)
    }

    /// CARD L5 — skip House first-session garden cross → none, PlaceId unchanged.
    #[test]
    fn l5_skip_house_garden_cross_none_place_id_unchanged() {
        use shared::hex_travel::PlaceId;
        let start = PlaceId::Sanctuary;
        let (land, now) = l5_first_session_land(false, true, HousePeople::Human, start, None);
        assert!(land.is_none());
        assert_eq!(now, start);
        let g = FirstSessionGuidance::default();
        assert!(g.stays_light_peace());
        let (land, now) = l5_first_session_land(true, false, HousePeople::Draek, start, None);
        assert!(land.is_none());
        assert_eq!(now, start);
    }

    /// CARD L5 — Human first-session garden cross → Sanctuary + Sanctuary Prime dress.
    #[test]
    fn l5_human_garden_cross_sanctuary_prime_dress() {
        use crate::climate_plane::dress_token_for_place;
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Human, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::SanctuaryYard));
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(dress_token_for_place(now), "Sanctuary Prime");
    }

    /// CARD L5 — Cydruid → Heartwood + Verdant Heartwood + people_line human-in-frame.
    #[test]
    fn l5_cydruid_garden_cross_heartwood_verdant_human_in_frame() {
        use crate::climate_plane::dress_token_for_place;
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Cydruid, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::Heartwood));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(dress_token_for_place(now), "Verdant Heartwood");
        assert_eq!(HousePeople::Cydruid.people_line(), "Cydruid · human-in-frame");
        assert!(!HousePeople::Cydruid.people_line().contains("treant"));
    }

    /// CARD L5 — Quellorian → Heartwood + threshold_use_in_reach.
    #[test]
    fn l5_quellorian_garden_cross_heartwood_threshold_use_in_reach() {
        use crate::climate_plane::dress_token_for_place;
        use crate::human_presence::people_landing_wake;
        use shared::hex_travel::PlaceId;
        use shared::threshold_shelf::threshold_use_in_reach;

        let mut presence = SoftPresence::default();
        let (land, now) = l5_first_session_land(
            true,
            true,
            HousePeople::Quellorian,
            PlaceId::Sanctuary,
            Some(&mut presence),
        );
        assert_eq!(land, Some(PeopleLanding::Threshold));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(dress_token_for_place(now), "Verdant Heartwood");
        let wake = people_landing_wake(PeopleLanding::Threshold);
        assert_eq!(presence.position, wake);
        assert!(threshold_use_in_reach(
            now,
            presence.position.x,
            presence.position.z
        ));
    }

    /// CARD L5 — Draek → Depths + Abyssal Depths.
    #[test]
    fn l5_draek_garden_cross_depths_abyssal() {
        use crate::climate_plane::dress_token_for_place;
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Draek, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::DepthsTealWayHome));
        assert_eq!(now, PlaceId::Depths);
        assert_eq!(dress_token_for_place(now), "Abyssal Depths");
    }

    /// CARD L5 — Ambrosian → Sanctuary PlaceId same as Human.
    #[test]
    fn l5_ambrosian_garden_cross_sanctuary_same_as_human() {
        use crate::climate_plane::dress_token_for_place;
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Ambrosian, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::SanctuaryWellFromAbove));
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(now, HousePeople::Human.landing().place_id());
        assert_eq!(dress_token_for_place(now), "Sanctuary Prime");
    }

    /// CARD L5 — Title chrome strings still Play / Continue / Settings.
    #[test]
    fn l5_title_chrome_play_continue_settings() {
        use crate::title_screen::{
            l2_title_chrome_holds, TITLE_CHROME_CONTINUE, TITLE_CHROME_PLAY, TITLE_CHROME_SETTINGS,
        };
        assert_eq!(TITLE_CHROME_PLAY, "Play — first Hands");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert_eq!(TITLE_CHROME_SETTINGS, "Settings");
        assert!(l2_title_chrome_holds());
    }

    /// CARD L5 — STEWARD_ONLINE_YES stays false.
    #[test]
    fn l5_steward_online_yes_stays_false() {
        use shared::persona::STEWARD_ONLINE_YES;
        use shared::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};
        assert!(!STEWARD_ONLINE_YES);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
    }

    /// Comfort Low is mesh LOD + larger text_scale — Garden Want stays words.
    #[test]
    fn garden_boot_comfort_low_keeps_want_readable() {
        use shared::local_settings::{GraphicsPreset, LocalSettings};
        let spoken = garden_boot_want_line(true, false).expect("Want on Garden boot");
        assert!(spoken.contains("Human"));
        assert!(spoken.contains("the yard needs tending or the well goes quiet"));
        for _preset in GraphicsPreset::ALL {
            assert_eq!(
                garden_boot_want_line(true, false).expect("Want at every Comfort"),
                spoken
            );
        }
        let mut low = LocalSettings::peace_defaults();
        low.set_graphics_preset(GraphicsPreset::Low);
        assert_eq!(low.graphics_preset, GraphicsPreset::Low);
        assert!(
            low.text_scale >= 1.10,
            "Comfort Low bumps text_scale so boot Want stays readable"
        );
        assert_eq!(garden_boot_want_line(true, false).expect("Want at Low"), spoken);
        assert_eq!(GraphicsPreset::ALL.len(), 3);
        assert!(GraphicsPreset::ALL.contains(&GraphicsPreset::Low));
    }

    /// CARD L6 — skip House → SANCTUARY_WANT / GARDEN_WANT unchanged.
    #[test]
    fn l6_skip_house_sanctuary_garden_want_unchanged() {
        use shared::hex_travel::PlaceId;

        assert_eq!(GARDEN_WANT, SANCTUARY_WANT);
        assert_eq!(want_after_people_landing(None), GARDEN_WANT);
        assert_eq!(want_after_people_landing(None), SANCTUARY_WANT);
        let spoken = garden_boot_want_line(true, false).expect("Want on Garden boot");
        assert!(spoken.contains(GARDEN_WANT));
        assert!(spoken.contains(SANCTUARY_WANT));
        assert_eq!(spoken, first_minutes_people_want_line());

        let (land, now) =
            l5_first_session_land(false, true, HousePeople::Human, PlaceId::Sanctuary, None);
        assert!(land.is_none());
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(want_after_people_landing(land), GARDEN_WANT);
        assert_eq!(want_for_place(now), SANCTUARY_WANT);
        let (land, now) =
            l5_first_session_land(true, false, HousePeople::Draek, PlaceId::Sanctuary, None);
        assert!(land.is_none());
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(want_after_people_landing(land), SANCTUARY_WANT);
    }

    /// CARD L6 — Human land → Sanctuary Want.
    #[test]
    fn l6_human_land_sanctuary_want() {
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Human, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::SanctuaryYard));
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(want_after_people_landing(land), SANCTUARY_WANT);
        assert_eq!(want_for_place(now), SANCTUARY_WANT);
        assert_eq!(
            first_minutes_people_want_line_for_place(now),
            first_minutes_people_want_line()
        );
        assert!(want_after_people_landing(land).contains("the yard needs tending"));
        assert!(want_after_people_landing(land).contains("the well goes quiet"));
    }

    /// CARD L6 — Ambrosian land → same Want as Human (same PlaceId).
    #[test]
    fn l6_ambrosian_land_same_want_as_human() {
        use shared::hex_travel::PlaceId;

        let (human, human_place) =
            l5_first_session_land(true, true, HousePeople::Human, PlaceId::Sanctuary, None);
        let (ambrosian, ambrosian_place) =
            l5_first_session_land(true, true, HousePeople::Ambrosian, PlaceId::Sanctuary, None);
        assert_eq!(human_place, PlaceId::Sanctuary);
        assert_eq!(ambrosian_place, PlaceId::Sanctuary);
        assert_eq!(ambrosian_place, human_place);
        assert_eq!(
            want_after_people_landing(ambrosian),
            want_after_people_landing(human)
        );
        assert_eq!(want_after_people_landing(ambrosian), SANCTUARY_WANT);
        assert_eq!(want_for_place(ambrosian_place), want_for_place(human_place));
    }

    /// CARD L6 — Cydruid land → Heartwood Want · people_line still human-in-frame (NOT treant).
    #[test]
    fn l6_cydruid_land_heartwood_want_human_in_frame() {
        use crate::climate_plane::dress_token_for_place;
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Cydruid, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::Heartwood));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(dress_token_for_place(now), "Verdant Heartwood");
        assert_eq!(want_after_people_landing(land), HEARTWOOD_WANT);
        assert_eq!(want_for_place(now), HEARTWOOD_WANT);
        assert!(want_after_people_landing(land).contains("Heartwood"));
        assert!(want_after_people_landing(land).contains("Tend"));
        assert!(!want_after_people_landing(land).contains("the well goes quiet"));
        assert_eq!(HousePeople::Cydruid.people_line(), "Cydruid · human-in-frame");
        assert!(!HousePeople::Cydruid.people_line().contains("treant"));
        let spoken = first_minutes_people_want_line_for_place(now);
        assert!(spoken.contains(HEARTWOOD_WANT));
        assert!(!spoken.contains(SANCTUARY_WANT));
    }

    /// CARD L6 — Quellorian land → Heartwood Want (not Sanctuary well).
    #[test]
    fn l6_quellorian_land_heartwood_want_not_sanctuary_well() {
        use crate::climate_plane::dress_token_for_place;
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Quellorian, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::Threshold));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(dress_token_for_place(now), "Verdant Heartwood");
        assert_eq!(want_after_people_landing(land), HEARTWOOD_WANT);
        assert_eq!(want_for_place(now), HEARTWOOD_WANT);
        assert_eq!(
            want_after_people_landing(land),
            want_for_place(HousePeople::Cydruid.landing().place_id())
        );
        assert!(!want_after_people_landing(land).contains("the well goes quiet"));
        assert!(!want_after_people_landing(land).contains(SANCTUARY_WANT));
        assert!(!first_minutes_people_want_line_for_place(now).contains("well"));
    }

    /// CARD L6 — Draek land → Depths Want (restore, not Take).
    #[test]
    fn l6_draek_land_depths_want_restore_not_take() {
        use crate::climate_plane::dress_token_for_place;
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Draek, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::DepthsTealWayHome));
        assert_eq!(now, PlaceId::Depths);
        assert_eq!(dress_token_for_place(now), "Abyssal Depths");
        assert_eq!(want_after_people_landing(land), DEPTHS_WANT);
        assert_eq!(want_for_place(now), DEPTHS_WANT);
        assert_eq!(DEPTHS_WANT, "Depths Peace · restored");
        assert!(want_after_people_landing(land).contains("restored"));
        assert!(!want_after_people_landing(land).contains("Take"));
        assert!(!want_after_people_landing(land).contains(SANCTUARY_WANT));
        let spoken = first_minutes_people_want_line_for_place(now);
        assert!(spoken.contains("restored"));
        assert!(!spoken.contains("Take"));
    }

    /// CARD L6 — TITLE_CHROME_PLAY / Continue / Settings unchanged.
    #[test]
    fn l6_title_chrome_play_continue_settings() {
        use crate::title_screen::{
            l2_title_chrome_holds, TITLE_CHROME_CONTINUE, TITLE_CHROME_PLAY, TITLE_CHROME_SETTINGS,
        };
        assert_eq!(TITLE_CHROME_PLAY, "Play — first Hands");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert_eq!(TITLE_CHROME_SETTINGS, "Settings");
        assert!(l2_title_chrome_holds());
    }

    /// CARD L6 — STEWARD_ONLINE_YES stays false. Online grey.
    #[test]
    fn l6_steward_online_yes_stays_false() {
        use shared::persona::STEWARD_ONLINE_YES;
        use shared::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};
        assert!(!STEWARD_ONLINE_YES);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
    }

    fn s1_all_aftermath_lines() -> [&'static str; 5] {
        [
            aftermath_evidence_for_landing(PeopleLanding::SanctuaryYard),
            aftermath_evidence_for_landing(PeopleLanding::SanctuaryWellFromAbove),
            aftermath_evidence_for_landing(PeopleLanding::Heartwood),
            aftermath_evidence_for_landing(PeopleLanding::Threshold),
            aftermath_evidence_for_landing(PeopleLanding::DepthsTealWayHome),
        ]
    }

    /// CARD S1 — skip House keeps garden / Sanctuary boot Want. No aftermath.
    #[test]
    fn s1_skip_house_keeps_garden_sanctuary_boot_want() {
        use shared::hex_travel::PlaceId;

        assert_eq!(aftermath_after_people_landing(None), GARDEN_WANT);
        assert_eq!(aftermath_after_people_landing(None), SANCTUARY_WANT);
        assert_eq!(first_minutes_aftermath_line(None), first_minutes_people_want_line());
        let spoken = garden_guidance_after_land(true, false, None).expect("Garden boot");
        assert_eq!(spoken, first_minutes_people_want_line());
        assert!(spoken.contains(GARDEN_WANT));
        assert!(garden_guidance_after_land(false, false, None).is_none());

        let g = FirstSessionGuidance::default();
        assert!(g.people_landing.is_none());
        assert_eq!(g.aftermath_evidence_line(), GARDEN_WANT);
        let (land, now) =
            l5_first_session_land(false, true, HousePeople::Human, PlaceId::Sanctuary, None);
        assert!(land.is_none());
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(want_after_people_landing(land), GARDEN_WANT);
        assert_eq!(aftermath_after_people_landing(land), GARDEN_WANT);
        assert_eq!(want_after_people_landing(land), aftermath_after_people_landing(land));
    }

    /// CARD S1 — Human Sanctuary: yard still teaching; war is rumor at the well.
    /// Want stays SANCTUARY_WANT.
    #[test]
    fn s1_human_sanctuary_aftermath_yard_teaching_war_rumor_at_well() {
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Human, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::SanctuaryYard));
        assert_eq!(now, PlaceId::Sanctuary);
        let mut g = FirstSessionGuidance::default();
        g.house_live = true;
        g.harvests_completed = 1;
        let mut travel = HexTravelState {
            current: PlaceId::Sanctuary,
        };
        let mut bind = l5_demo_bind();
        let mut crossed = None;
        let recorded = g.try_cross_people_door(
            &mut crossed,
            HousePeople::Human,
            &mut travel,
            &mut bind,
            None,
            None,
        );
        assert_eq!(recorded, land);
        assert_eq!(g.people_landing, land);
        assert_eq!(g.aftermath_evidence_line(), HUMAN_AFTERMATH);
        assert_eq!(want_after_people_landing(land), SANCTUARY_WANT);
        assert_eq!(aftermath_after_people_landing(land), HUMAN_AFTERMATH);
        assert!(HUMAN_AFTERMATH.contains("yard still teaching"));
        assert!(HUMAN_AFTERMATH.contains("war is rumor at the well"));
        assert_ne!(aftermath_after_people_landing(land), want_after_people_landing(land));
        let spoken = first_minutes_aftermath_line(land);
        assert!(spoken.contains(SANCTUARY_WANT));
        assert!(spoken.contains(HUMAN_AFTERMATH));
        assert!(garden_guidance_after_land(true, false, land)
            .expect("after land")
            .contains(HUMAN_AFTERMATH));
        let lines = s1_all_aftermath_lines();
        assert!(lines.iter().filter(|l| **l == HUMAN_AFTERMATH).count() == 1);
    }

    /// CARD S1 — Ambrosian Sanctuary lift (Fork A): same disk, thinner fog, no hull.
    /// Want stays Sanctuary (same as Human). Aftermath differs. No hull.
    #[test]
    fn s1_ambrosian_sanctuary_lift_thinner_fog_no_hull_want_stays_sanctuary() {
        use shared::hex_travel::PlaceId;

        let (human, human_place) =
            l5_first_session_land(true, true, HousePeople::Human, PlaceId::Sanctuary, None);
        let (ambrosian, ambrosian_place) =
            l5_first_session_land(true, true, HousePeople::Ambrosian, PlaceId::Sanctuary, None);
        assert_eq!(ambrosian, Some(PeopleLanding::SanctuaryWellFromAbove));
        assert_eq!(ambrosian_place, PlaceId::Sanctuary);
        assert_eq!(ambrosian_place, human_place);
        assert_eq!(want_after_people_landing(ambrosian), SANCTUARY_WANT);
        assert_eq!(
            want_after_people_landing(ambrosian),
            want_after_people_landing(human)
        );
        assert_eq!(aftermath_after_people_landing(ambrosian), AMBROSIAN_AFTERMATH);
        assert_ne!(
            aftermath_after_people_landing(ambrosian),
            aftermath_after_people_landing(human)
        );
        assert!(AMBROSIAN_AFTERMATH.contains("same disk"));
        assert!(AMBROSIAN_AFTERMATH.contains("thinner fog"));
        assert!(AMBROSIAN_AFTERMATH.contains("no hull"));
        assert!(!AMBROSIAN_AFTERMATH.contains("Brood Spire"));
        let spoken = first_minutes_aftermath_line(ambrosian);
        assert!(spoken.contains(SANCTUARY_WANT));
        assert!(spoken.contains(AMBROSIAN_AFTERMATH));
        assert!(!spoken.contains(HUMAN_AFTERMATH));
    }

    /// CARD S1 — Cydruid Heartwood: human-in-frame · nature is practice (C0, NOT treant).
    /// Want stays HEARTWOOD_WANT.
    #[test]
    fn s1_cydruid_heartwood_aftermath_human_in_frame_nature_is_practice() {
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Cydruid, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::Heartwood));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(want_after_people_landing(land), HEARTWOOD_WANT);
        assert_eq!(aftermath_after_people_landing(land), CYDRUID_AFTERMATH);
        assert!(CYDRUID_AFTERMATH.contains("human-in-frame"));
        assert!(CYDRUID_AFTERMATH.contains("nature is practice"));
        assert!(!CYDRUID_AFTERMATH.contains("treant"));
        assert!(!CYDRUID_AFTERMATH.contains("bark"));
        assert_eq!(HousePeople::Cydruid.people_line(), "Cydruid · human-in-frame");
        assert!(!HousePeople::Cydruid.people_line().contains("treant"));
        let spoken = first_minutes_aftermath_line(land);
        assert!(spoken.contains(HEARTWOOD_WANT));
        assert!(spoken.contains(CYDRUID_AFTERMATH));
        assert!(!spoken.contains("treant"));
        assert!(!spoken.contains(SANCTUARY_WANT));
        assert_ne!(aftermath_after_people_landing(land), HUMAN_AFTERMATH);
    }

    /// CARD S1 — Quellorian Threshold: seam remembers the leaving.
    /// Want stays HEARTWOOD_WANT (PlaceId). Aftermath differs from Cydruid.
    #[test]
    fn s1_quellorian_threshold_aftermath_seam_remembers_the_leaving() {
        use shared::hex_travel::PlaceId;

        let (cydruid, _) =
            l5_first_session_land(true, true, HousePeople::Cydruid, PlaceId::Sanctuary, None);
        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Quellorian, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::Threshold));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(want_after_people_landing(land), HEARTWOOD_WANT);
        assert_eq!(
            want_after_people_landing(land),
            want_after_people_landing(cydruid)
        );
        assert_eq!(aftermath_after_people_landing(land), QUELLORIAN_AFTERMATH);
        assert_ne!(
            aftermath_after_people_landing(land),
            aftermath_after_people_landing(cydruid)
        );
        assert!(QUELLORIAN_AFTERMATH.contains("seam remembers the leaving"));
        assert!(!QUELLORIAN_AFTERMATH.contains("treant"));
        let spoken = first_minutes_aftermath_line(land);
        assert!(spoken.contains(HEARTWOOD_WANT));
        assert!(spoken.contains(QUELLORIAN_AFTERMATH));
        assert!(!spoken.contains(CYDRUID_AFTERMATH));
        assert!(!spoken.contains(SANCTUARY_WANT));
    }

    /// CARD S1 — Draek Depths: consume-scar + teal way-home.
    /// Want stays DEPTHS_WANT (restore, not Take).
    #[test]
    fn s1_draek_depths_aftermath_consume_scar_teal_way_home() {
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Draek, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::DepthsTealWayHome));
        assert_eq!(now, PlaceId::Depths);
        assert_eq!(want_after_people_landing(land), DEPTHS_WANT);
        assert_eq!(aftermath_after_people_landing(land), DRAEK_AFTERMATH);
        assert!(DRAEK_AFTERMATH.contains("consume-scar"));
        assert!(DRAEK_AFTERMATH.contains("teal way-home"));
        assert!(!DRAEK_AFTERMATH.contains("Take"));
        let spoken = first_minutes_aftermath_line(land);
        assert!(spoken.contains(DEPTHS_WANT));
        assert!(spoken.contains("restored"));
        assert!(spoken.contains(DRAEK_AFTERMATH));
        assert!(!spoken.contains("Take"));
        assert!(!spoken.contains(SANCTUARY_WANT));
        let lines = s1_all_aftermath_lines();
        for i in 0..lines.len() {
            for j in (i + 1)..lines.len() {
                assert_ne!(lines[i], lines[j], "five reads must differ");
            }
        }
    }

    /// CARD S1 — H hush still drops garden aftermath. Title chrome unchanged.
    #[test]
    fn s1_title_chrome_play_continue_settings_hush_and_online_grey() {
        use crate::title_screen::{
            l2_title_chrome_holds, TITLE_CHROME_CONTINUE, TITLE_CHROME_PLAY, TITLE_CHROME_SETTINGS,
        };
        use shared::persona::STEWARD_ONLINE_YES;
        use shared::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

        assert_eq!(TITLE_CHROME_PLAY, "Play — first Hands");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert_eq!(TITLE_CHROME_SETTINGS, "Settings");
        assert!(l2_title_chrome_holds());
        assert!(!STEWARD_ONLINE_YES);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        let land = Some(PeopleLanding::SanctuaryYard);
        assert!(garden_guidance_after_land(true, false, land).is_some());
        assert!(garden_guidance_after_land(true, true, land).is_none());
        let mut g = FirstSessionGuidance::default();
        g.people_landing = land;
        assert!(g.speaks_people_want());
        assert_eq!(g.aftermath_evidence_line(), HUMAN_AFTERMATH);
        g.dismiss();
        assert!(garden_guidance_after_land(true, !g.speaks_people_want(), land).is_none());
    }

    /// CARD S2 — decline / wrong door → garden light · not sealed.
    #[test]
    fn s2_decline_wrong_door_garden_light_not_sealed() {
        use shared::hex_travel::PlaceId;

        let start = PlaceId::Sanctuary;
        let (land, now) =
            l5_first_session_land(true, true, HousePeople::Cydruid, start, None);
        assert_eq!(land, Some(PeopleLanding::Heartwood));
        assert_eq!(now, PlaceId::Heartwood);

        let mut g = FirstSessionGuidance::default();
        g.house_live = true;
        g.harvests_completed = 1;
        let mut travel = HexTravelState { current: start };
        let mut bind = l5_demo_bind();
        let mut crossed = None;
        let land = g
            .try_cross_people_door(
                &mut crossed,
                HousePeople::Cydruid,
                &mut travel,
                &mut bind,
                None,
                None,
            )
            .expect("land");
        assert_eq!(land, PeopleLanding::Heartwood);
        assert_eq!(travel.current, PlaceId::Heartwood);
        assert!(g.is_light());
        assert!(g.doors_unsealed());
        assert!(!g.gate_sealed);

        assert!(g.decline_or_wrong_door(&mut crossed, &mut travel, &mut bind, None, start));
        assert!(crossed.is_none());
        assert!(g.people_landing.is_none());
        assert_eq!(travel.current, start);
        assert!(g.is_light());
        assert!(g.doors_unsealed());
        assert!(!g.gate_sealed);
        assert_eq!(g.aftermath_evidence_line(), GARDEN_WANT);
        assert_eq!(HUMAN_AFTERMATH, "yard still teaching · war is rumor at the well");

        let land = g
            .try_cross_people_door(
                &mut crossed,
                HousePeople::Human,
                &mut travel,
                &mut bind,
                None,
                None,
            )
            .expect("unsealed soul may try again");
        assert_eq!(land, PeopleLanding::SanctuaryYard);
        let sealed = g
            .confirm_seal_at_landing(PeaceKey::E, crossed)
            .expect("E seals");
        assert_eq!(sealed, (HousePeople::Human, PeopleLanding::SanctuaryYard));
        assert!(!g.is_light());
        assert!(!g.doors_unsealed());
        assert_eq!(g.peace_recall(), "vision home");
        assert!(!g.decline_or_wrong_door(&mut crossed, &mut travel, &mut bind, None, start));
        assert_eq!(g.sealed_people, Some(HousePeople::Human));
        assert_eq!(travel.current, PlaceId::Sanctuary);
    }

    /// CARD S2 — Skip House → no PlaceId change · not sealed.
    #[test]
    fn s2_skip_house_no_place_id_change_not_sealed() {
        use shared::hex_travel::PlaceId;

        let start = PlaceId::Sanctuary;
        let (land, now) = l5_first_session_land(false, true, HousePeople::Human, start, None);
        assert!(land.is_none());
        assert_eq!(now, start);
        let mut g = FirstSessionGuidance::default();
        assert!(g.stays_light_peace());
        assert!(g.is_light());
        assert!(g.doors_unsealed());
        assert!(!g.gate_sealed);
        assert!(g.confirm_seal_at_landing(PeaceKey::E, None).is_none());
        assert_eq!(g.peace_recall(), "vision home");
        assert_eq!(want_after_people_landing(land), GARDEN_WANT);
        assert_eq!(now, PlaceId::Sanctuary);
    }

    /// CARD S2 — PlaceId stays 3 · STEWARD_ONLINE_YES false · Title chrome unchanged.
    #[test]
    fn s2_place_id_stays_three_steward_online_grey_title_chrome() {
        use crate::title_screen::{
            l2_title_chrome_holds, TITLE_CHROME_CONTINUE, TITLE_CHROME_PLAY, TITLE_CHROME_SETTINGS,
        };
        use shared::hex_travel::{PlaceId, LOCAL_HEXES};
        use shared::persona::STEWARD_ONLINE_YES;
        use shared::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
        assert_eq!(PeopleLanding::Threshold.place_id(), PlaceId::Heartwood);
        assert_eq!(
            PeopleLanding::SanctuaryWellFromAbove.place_id(),
            PlaceId::Sanctuary
        );
        assert_eq!(TITLE_CHROME_PLAY, "Play — first Hands");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert_eq!(TITLE_CHROME_SETTINGS, "Settings");
        assert!(l2_title_chrome_holds());
        assert!(!STEWARD_ONLINE_YES);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert_eq!(HUMAN_AFTERMATH, "yard still teaching · war is rumor at the well");
        assert_eq!(AMBROSIAN_AFTERMATH, "same disk · thinner fog · no hull");
        assert_eq!(CYDRUID_AFTERMATH, "human-in-frame · nature is practice");
        assert_eq!(QUELLORIAN_AFTERMATH, "seam remembers the leaving");
        assert_eq!(DRAEK_AFTERMATH, "consume-scar · teal way-home");
    }

    fn f7_all_aftermath_variants() -> [&'static str; 5] {
        [
            f7_aftermath_variant_for_landing(PeopleLanding::SanctuaryYard),
            f7_aftermath_variant_for_landing(PeopleLanding::SanctuaryWellFromAbove),
            f7_aftermath_variant_for_landing(PeopleLanding::Heartwood),
            f7_aftermath_variant_for_landing(PeopleLanding::Threshold),
            f7_aftermath_variant_for_landing(PeopleLanding::DepthsTealWayHome),
        ]
    }

    fn f7_looks_like_trailer_or_plot_dump(line: &str) -> bool {
        let lower = line.to_lowercase();
        lower.contains("trailer")
            || lower.contains("cutscene")
            || lower.contains("imagine")
            || lower.contains("mothership")
            || lower.contains("crownstone")
            || lower.contains("wwiii")
            || lower.contains("drenadore")
            || lower.contains("brood spire")
            || lower.contains("plot dump")
            || lower.contains("you will be consumed")
            || line.contains(".glb")
            || line.contains(".gltf")
    }

    /// CARD F7 — each People gets exactly one extra aftermath lore line
    /// on the existing S1 plate. S1 evidence strings stay unread.
    #[test]
    fn f7_each_people_exactly_one_extra_aftermath_lore_line() {
        use shared::hex_travel::PlaceId;

        assert_eq!(HOUSE_PEOPLES.len(), 5);
        assert_eq!(f7_aftermath_variant_after_people_landing(None), None);
        assert_eq!(
            f7_first_minutes_aftermath_line(None),
            first_minutes_aftermath_line(None)
        );
        assert_eq!(f7_first_minutes_aftermath_line(None), first_minutes_people_want_line());

        let pairs = [
            (
                HousePeople::Human,
                PeopleLanding::SanctuaryYard,
                HUMAN_AFTERMATH,
                HUMAN_AFTERMATH_VARIANT,
                SANCTUARY_WANT,
            ),
            (
                HousePeople::Ambrosian,
                PeopleLanding::SanctuaryWellFromAbove,
                AMBROSIAN_AFTERMATH,
                AMBROSIAN_AFTERMATH_VARIANT,
                SANCTUARY_WANT,
            ),
            (
                HousePeople::Cydruid,
                PeopleLanding::Heartwood,
                CYDRUID_AFTERMATH,
                CYDRUID_AFTERMATH_VARIANT,
                HEARTWOOD_WANT,
            ),
            (
                HousePeople::Quellorian,
                PeopleLanding::Threshold,
                QUELLORIAN_AFTERMATH,
                QUELLORIAN_AFTERMATH_VARIANT,
                HEARTWOOD_WANT,
            ),
            (
                HousePeople::Draek,
                PeopleLanding::DepthsTealWayHome,
                DRAEK_AFTERMATH,
                DRAEK_AFTERMATH_VARIANT,
                DEPTHS_WANT,
            ),
        ];
        let variants = f7_all_aftermath_variants();
        assert_eq!(variants.len(), 5);
        for i in 0..variants.len() {
            assert_eq!(
                variants.iter().filter(|v| **v == variants[i]).count(),
                1,
                "exactly one extra line per People"
            );
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "five extra reads must differ");
            }
        }

        for (people, landing, s1, variant, want) in pairs {
            let (land, _now) =
                l5_first_session_land(true, true, people, PlaceId::Sanctuary, None);
            assert_eq!(land, Some(landing));
            assert_eq!(aftermath_after_people_landing(land), s1);
            assert_eq!(f7_aftermath_variant_after_people_landing(land), Some(variant));
            assert_eq!(f7_aftermath_variant_for_landing(landing), variant);
            assert_ne!(variant, s1, "extra line is not a rewrite of S1");
            let s1_plate = first_minutes_aftermath_line(land);
            let f7_plate = f7_first_minutes_aftermath_line(land);
            assert!(s1_plate.contains(want));
            assert!(s1_plate.contains(s1));
            assert!(!s1_plate.contains(variant), "S1 plate WRITE stays unread");
            assert_eq!(f7_plate, format!("{s1_plate} · {variant}"));
            assert_eq!(
                f7_plate.strip_suffix(&format!(" · {variant}")),
                Some(s1_plate.as_str())
            );
            assert_eq!(f7_plate.matches(variant).count(), 1);
            assert!(f7_garden_guidance_after_land(true, false, land)
                .expect("after land")
                .contains(variant));
            let mut g = FirstSessionGuidance::default();
            g.people_landing = land;
            assert_eq!(g.aftermath_evidence_line(), s1);
            assert_eq!(g.f7_aftermath_plate_line(), f7_plate);
        }

        assert_eq!(HUMAN_AFTERMATH, "yard still teaching · war is rumor at the well");
        assert_eq!(AMBROSIAN_AFTERMATH, "same disk · thinner fog · no hull");
        assert_eq!(CYDRUID_AFTERMATH, "human-in-frame · nature is practice");
        assert_eq!(QUELLORIAN_AFTERMATH, "seam remembers the leaving");
        assert_eq!(DRAEK_AFTERMATH, "consume-scar · teal way-home");
    }

    /// CARD F7 — extra lines are local evidence (well / lamp / seam / Peace plate).
    /// Not a trailer, cutscene, or plot dump.
    #[test]
    fn f7_aftermath_variants_are_local_evidence_not_trailer() {
        assert!(HUMAN_AFTERMATH_VARIANT.contains("well"));
        assert!(HUMAN_AFTERMATH_VARIANT.contains("glow"));
        assert!(AMBROSIAN_AFTERMATH_VARIANT.contains("well-from-above"));
        assert!(AMBROSIAN_AFTERMATH_VARIANT.contains("prism"));
        assert!(CYDRUID_AFTERMATH_VARIANT.contains("amber lamp"));
        assert!(CYDRUID_AFTERMATH_VARIANT.contains("person not the tree"));
        assert!(!CYDRUID_AFTERMATH_VARIANT.contains("treant"));
        assert!(!CYDRUID_AFTERMATH_VARIANT.contains("bark"));
        assert!(QUELLORIAN_AFTERMATH_VARIANT.contains("iron"));
        assert!(QUELLORIAN_AFTERMATH_VARIANT.contains("tend seam"));
        assert!(DRAEK_AFTERMATH_VARIANT.contains("teal Peace"));
        assert!(!DRAEK_AFTERMATH_VARIANT.contains("Take"));

        for variant in f7_all_aftermath_variants() {
            assert!(variant.len() < 48, "{variant} is a manifesto");
            assert!(!f7_looks_like_trailer_or_plot_dump(variant));
            assert!(!variant.contains("Market"));
            assert!(!variant.contains("Garden"));
            assert!(!variant.contains("WASD"));
        }

        let spoken = f7_first_minutes_aftermath_line(Some(PeopleLanding::DepthsTealWayHome));
        assert!(spoken.contains(DEPTHS_WANT));
        assert!(spoken.contains("restored"));
        assert!(!spoken.contains("Take"));
        assert!(!f7_looks_like_trailer_or_plot_dump(&spoken));
        assert!(f7_garden_guidance_after_land(true, true, Some(PeopleLanding::SanctuaryYard)).is_none());
        let boot = first_minutes_people_want_line();
        assert_eq!(
            f7_garden_guidance_after_land(true, false, None).as_deref(),
            Some(boot.as_str())
        );
    }

    /// CARD F7 — PlaceId / LOCAL_HEXES len == 3. No fifth Place.
    #[test]
    fn f7_place_id_local_hexes_len_three() {
        use shared::hex_travel::{PlaceId, LOCAL_HEXES};

        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
        assert_eq!(PeopleLanding::Threshold.place_id(), PlaceId::Heartwood);
        assert_eq!(
            PeopleLanding::SanctuaryWellFromAbove.place_id(),
            PlaceId::Sanctuary
        );
        for place in LOCAL_HEXES {
            assert_ne!(place.display_name(), "Garden");
            assert_ne!(place.as_str(), "market");
        }
        assert_eq!(HOUSE_PEOPLES.len(), 5);
    }

    /// CARD F7 — STEWARD_ONLINE_YES false. Title chrome unchanged. Online grey.
    #[test]
    fn f7_steward_online_yes_false() {
        use crate::title_screen::{
            l2_title_chrome_holds, TITLE_CHROME_CONTINUE, TITLE_CHROME_PLAY, TITLE_CHROME_SETTINGS,
        };
        use shared::persona::STEWARD_ONLINE_YES;
        use shared::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

        assert_eq!(TITLE_CHROME_PLAY, "Play — first Hands");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert_eq!(TITLE_CHROME_SETTINGS, "Settings");
        assert!(l2_title_chrome_holds());
        assert!(!STEWARD_ONLINE_YES);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
    }

    /// CARD F7 — 0 meshes · no .glb adds. Hands stay words on the S1 plate.
    #[test]
    fn f7_no_mesh_glb_adds() {
        assert_eq!(L2_MESH_BUDGET, 0);
        assert_eq!(L2_ASSET_BUDGET_CITE, "docs/ASSET_BUDGET_COURT.md");
        for variant in f7_all_aftermath_variants() {
            assert!(!variant.contains(".glb"));
            assert!(!variant.contains(".gltf"));
            assert!(!variant.to_lowercase().contains("mesh"));
        }
        let plate = f7_first_minutes_aftermath_line(Some(PeopleLanding::Heartwood));
        assert!(!plate.contains(".glb"));
        assert!(!plate.to_lowercase().contains("mesh"));
    }

    /// CARD FLESH-GUIDANCE-PLACE — week-bill sentence may name the Place stood in.
    /// Tend, week-bill, and Continue stay the same words. No new card.
    #[test]
    fn flesh_guidance_place_names_stood_place_week_bill_stays() {
        use crate::title_screen::TITLE_CHROME_CONTINUE;
        use shared::hex_travel::PlaceId;

        let week = GuidanceObjective::HourTwoHeld.prompt();
        assert_eq!(week, "climate on slab · week tons+restored");
        assert!(week.contains("week"));
        assert!(week.contains("tons"));

        assert_eq!(stood_place_name(PlaceId::Sanctuary, false), "Sanctuary");
        assert_eq!(stood_place_name(PlaceId::Heartwood, false), "Heartwood");
        assert_eq!(stood_place_name(PlaceId::Heartwood, true), "Threshold-near");
        assert_eq!(stood_place_name(PlaceId::Depths, false), "Depths");
        assert_eq!(stood_place_name(PlaceId::Depths, true), "Depths");

        let sanctuary = spoken_guidance(&GuidanceObjective::HourTwoHeld, Some("Sanctuary"));
        assert!(sanctuary.contains("Sanctuary"));
        assert!(sanctuary.contains(week));
        assert!(sanctuary.contains("week"));

        let near = spoken_guidance(
            &GuidanceObjective::HourTwoHeld,
            Some(stood_place_name(PlaceId::Heartwood, true)),
        );
        assert!(near.contains("Threshold-near"));
        assert!(near.contains("week"));
        assert!(near.contains("tons"));

        assert_eq!(spoken_guidance(&GuidanceObjective::HourTwoHeld, None), week);
        assert_eq!(
            spoken_guidance(&GuidanceObjective::HarvestWithInteract, Some("Sanctuary")),
            "E tend the glow"
        );
        assert!(GuidanceObjective::HarvestWithInteract
            .prompt()
            .contains("tend"));
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");

        let travel = HexTravelState {
            current: PlaceId::Heartwood,
        };
        let mut body = SoftPresence::default();
        assert_eq!(
            stood_place_label(Some(&travel), Some(&body)),
            Some("Heartwood")
        );
        let shelf = shared::threshold_shelf::THRESHOLD_SHELF_CENTER;
        body.position.x = shelf[0];
        body.position.z = shelf[2];
        assert_eq!(
            stood_place_label(Some(&travel), Some(&body)),
            Some("Threshold-near")
        );
        assert!(stood_place_label(None, Some(&body)).is_none());
    }
}
