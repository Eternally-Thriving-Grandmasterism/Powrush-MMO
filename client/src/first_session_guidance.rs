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
 * Contact: info@Rathor.ai | Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::embassy::EmbassyYard;
use crate::fabricator::FabricatorYard;
use crate::hex_travel::{apply_people_landing, HexTravelState};
use crate::hour_sacred::{
    god_plane_doors_ignited, offer_house_peoples, skip_house_stays_light, try_cross_people_door,
    HousePeople, PeopleLanding, HourSacred, HOUSE_PEOPLES, L2_ASSET_BUDGET_CITE, L2_MESH_BUDGET,
};
use crate::human_presence::SoftPresence;
use crate::ledger_bind::LedgerYard;
use crate::lived_hour_bind::LivedHourBind;
use crate::mercy_harvest_nodes::NearbyMercyNode;
use crate::title_screen::LaunchDoor;
use shared::ledger_bind::ContractState;
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
pub fn garden_boot_want_line(on_garden_boot: bool, hush: bool) -> Option<String> {
    if !on_garden_boot || hush {
        None
    } else {
        Some(first_minutes_people_want_line())
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
    pub fn try_cross_people_door(
        &self,
        crossed: &mut Option<HousePeople>,
        people: HousePeople,
        travel: &mut HexTravelState,
        bind: &mut LivedHourBind,
        embassy: Option<&mut EmbassyYard>,
        presence: Option<&mut SoftPresence>,
    ) -> Option<PeopleLanding> {
        garden_cross_people_land(
            self.house_live,
            self.harvests_completed >= 1,
            crossed,
            people,
            travel,
            bind,
            embassy,
            presence,
        )
    }

    /// CARD L3 — Garden wrapper: L2 cross then apply_place / lived bind. Call only.
    pub fn garden_cross_people_land(
        &self,
        crossed: &mut Option<HousePeople>,
        people: HousePeople,
        travel: &mut HexTravelState,
        bind: &mut LivedHourBind,
        embassy: Option<&mut EmbassyYard>,
        presence: Option<&mut SoftPresence>,
    ) -> Option<PeopleLanding> {
        garden_cross_people_land(
            self.house_live,
            self.harvests_completed >= 1,
            crossed,
            people,
            travel,
            bind,
            embassy,
            presence,
        )
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

/// CARD L3 / L5 — Garden wrapper. Calls L2 try_cross then apply_people_landing.
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
}
