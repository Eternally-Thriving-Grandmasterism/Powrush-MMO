/*!
 * Climate Plane — v22.7.0 + Depths dress (H-2026-09-11-F4) + EARTH-CLIMATE beds
 * (H-2026-09-12-EARTH-CLIMATE) + PLACE-LOD (H-2026-09-16-PLACE-LOD)
 *
 * PLACE_DRESS_SPEC: Depths one way down / one way home — one deepen/wet-stone
 * material family + teal Peace accent so Place reads before any slab. F1
 * Sanctuary warm-yard, F2 Heartwood living-wood, and F3 Threshold pipe/edge
 * stay intact (do not freestyle-reopen). Four Places stay four. No mesh · no
 * fifth Place. Online grey. Tag 11c577e. Depths stay night in living_day.
 * Z travel moves the place. Climate 3 = Abyssal Depths (night, close fog).
 *
 * EARTH-CLIMATE: procedural weather beds from existing fog / tint / breath —
 * Place moods (Sanctuary sky/yard · Heartwood canopy · Threshold pipe air ·
 * Depths wet stone). FlowWeather band couples via WeatherBandCoupling.
 * Comfort GraphicsPreset → WeatherFidelity gates intensity (Low gentler ·
 * Medium default · High richer). No second HUD. No live Earth API / sockets.
 *
 * H-2026-09-16-PLACE-LOD: Comfort L/M/H [`MeshLodPlan`] / LocalMeshLodFeel
 * drives [`procedural_detail_scale`] on Place dress path stones. Low stays
 * readable Place identity (palette + four Places hold). Buildings / Astra
 * Medium stay REFS — no `.glb` dump. No fifth Place. No Comfort Ultra.
 * Cite [`docs/PLACE_DRESS_SPEC.md`] · [`docs/MESH_PERSONA_COURT.md`]
 * · [`docs/ART_BIBLE.md`] · [`docs/ASSET_BUDGET_COURT.md`] @ `5eff19c`.
 *
 * CARD FLESH-SANCTUARY-DRESS — Sanctuary yard earth is graphite-warm.
 * The warm-gold well stays the one readable glow on that earth. Comfort
 * L/M/H held (no Ultra). Cite PEAK_MEMORY_LAW: walked to a well · tended
 * it · week was the bill · yard remembered. No new Place. No `.glb`.
 *
 * CARD FLESH-HEARTWOOD-DRESS — Heartwood fog is living-wood haze.
 * The amber lamp stays the one accent. Lamp hush bed stays the banked
 * BED_GAIN_HEARTWOOD (client peace_audio); this file does not retune the
 * mixer. Comfort L/M/H held (no Ultra). Same peak memory. No new Place.
 * No `.glb`.
 *
 * CARD FLESH-THRESHOLD-DRESS — Threshold fog is pipe-air iron.
 * The tend seam stays the one accent. Ground and edge stones stay the
 * walked pipe tokens. Shelf stays on Heartwood via threshold_near; no
 * fifth PlaceId. Comfort L/M/H held (no Ultra). Same peak memory.
 * No `.glb`.
 *
 * CARD FLESH-DEPTHS-DRESS — Depths fog is wet-stone night haze.
 * Teal Peace stays the one accent. Quieter bed stays the banked
 * BED_GAIN_DEPTHS (client peace_audio); this file does not retune the
 * mixer. Comfort L/M/H held (no Ultra). Same peak memory. No new Place.
 * No `.glb`.
 *
 * CARD OPT-COMFORT-LOW — Comfort Low weather cap after FLESH dress.
 * Place fog / sky tokens stay. Low opens the close FLESH ramps (gentler
 * fog), slows breath (reduced motion), and caps amplitude so phones /
 * weak GPU do not carry the High breath cost. Medium and High keep the
 * dressed distances. No Ultra. Cite ASSET_BUDGET_COURT @ `5eff19c` ·
 * MESH_QUALITY_BUDGET.
 *
 * CARD OPT-WEATHER-FIDELITY-LOW — Comfort Low fog / mist bed density cap.
 * Start/end floors already open the close FLESH ramps. Low still paid full
 * extinction (fog alpha 1) on that opened bed. Bevy linear fog uses alpha as
 * the max mix, so Low now thins the same haze. No particle emitter on this
 * path — the weather bed is the mist bed. Medium and High keep alpha 1 and
 * the dressed distances. No Ultra.
 *
 * CARD L7 ARRIVAL-BEAT — People-door land applies one FogSettings beat from
 * existing look_for tokens. Human → SanctuarySkyYard / warm-gold well.
 * Ambrosian → brighter / thinner high fog on the same Sanctuary disk
 * (mothership-over-Earth PRESENTATION; no hull mesh). Cydruid amber canopy.
 * Draek wet-stone / teal Peace. Quellorian keeps Heartwood fog (shelf is the
 * culture). Garden title light yields — Sanctuary is the lived level. 0 meshes.
 *
 * CARD F5 WRONG-DOOR-BOUNCE — decline / wrong door restores garden PlaceId
 * dress via the existing look_for table. Does not recook L7 fog WRITE.
 * Garden bounce is Sanctuary boot disk, not a fifth Place. Online grey.
 *
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::pbr::{FogFalloff, FogSettings};
use bevy::prelude::*;

use shared::local_settings::{GraphicsPreset, WeatherFidelity};

use shared::hex_travel::PlaceId;

use crate::hour_sacred::PeopleLanding;
use crate::living_practice_loop::SoftPlayerRealm;
use crate::local_settings::{LocalMeshLodFeel, LocalSettingsState};
use crate::mercy_harvest_nodes::MercyHarvestNode;

/// CARD H-2026-09-16-PLACE-LOD — Comfort MeshLodPlan (PATHS-only compile).
#[path = "gltf_integration.rs"]
mod mesh_lod;

const NODE_ANCHORS: [Vec3; 3] = [
    Vec3::new(3.6, 0.55, 0.0),
    Vec3::new(-2.4, 0.55, 3.1),
    Vec3::new(1.2, 0.55, -3.4),
];

/// ART_BIBLE HANDS Sanctuary accent — warm gold well (not currency gold).
/// Rhymes with `human_presence::SANCTUARY_GOLD`; climate owns its copy so
/// this file stays the only F1 edit path.
const SANCTUARY_WELL_GOLD: Color = Color::srgb(0.86, 0.66, 0.29);

/// CARD FLESH-SANCTUARY-DRESS — graphite-warm yard earth (one family).
/// Warm grey-gold pulled toward graphite so the well stays the one glow.
/// Cite PLACE_DRESS Sanctuary yard · ART_BIBLE Human warm grey-gold ·
/// DRIVE_PLACE_CITE warm gold well. Not Draek graphite-crimson.
const SANCTUARY_GRAPHITE_EARTH: Color = Color::srgb(0.20, 0.19, 0.17);
const SANCTUARY_GRAPHITE_STONE: Color = Color::srgb(0.27, 0.26, 0.24);
const SANCTUARY_GRAPHITE_SKY: Color = Color::srgb(0.58, 0.56, 0.52);
const SANCTUARY_GRAPHITE_FOG: Color = Color::srgba(0.44, 0.42, 0.39, 1.0);
const SANCTUARY_GRAPHITE_AMBIENT: Color = Color::srgb(0.70, 0.67, 0.62);

/// Readable warm-gold well on graphite earth. Other Places keep the shared
/// greybox paint scale. Comfort L/M/H do not retint this. No Ultra.
pub const SANCTUARY_WELL_GLOW_SCALE: f32 = 3.2;
/// Shared well paint scale (Heartwood / Depths). Sanctuary lifts above this.
pub const PLACE_WELL_GLOW_SCALE: f32 = 2.4;

/// Shared greybox roughness for Sanctuary yard ground + path stones
/// (one material family — PLACE_DRESS_SPEC).
const SANCTUARY_YARD_ROUGHNESS: f32 = 0.90;

/// ART_BIBLE HANDS Heartwood accent — amber lamp (not Sanctuary warm-gold,
/// not currency gold). Climate owns its copy so this file stays the only F2
/// edit path.
const HEARTWOOD_AMBER_LAMP: Color = Color::srgb(0.90, 0.52, 0.14);

/// Shared living-wood roughness for Heartwood ground + ring path stones
/// (one material family — PLACE_DRESS_SPEC).
const HEARTWOOD_WOOD_ROUGHNESS: f32 = 0.82;

/// CARD FLESH-HEARTWOOD-DRESS — living-wood haze (one family).
/// Bark-warm canopy and fog so the amber lamp is the hush, not a lawn
/// second biome. Cite PLACE_DRESS Heartwood live/seal · ART_BIBLE amber
/// lamp · DRIVE_PLACE_CITE Heartwood amber lamp · PEAK_MEMORY_LAW.
/// Not Sanctuary graphite yard. Not Depths night.
const HEARTWOOD_WOOD_SKY: Color = Color::srgb(0.44, 0.32, 0.16);
const HEARTWOOD_WOOD_FOG: Color = Color::srgba(0.36, 0.25, 0.12, 1.0);
/// Lamp disk stays clear. Wood haze closes sooner than the open yard.
const HEARTWOOD_FOG_START: f32 = 10.0;
const HEARTWOOD_FOG_END: f32 = 36.0;

/// ART_BIBLE HANDS Threshold accent — iron + tend seam (not Sanctuary
/// warm-gold, not Heartwood amber, not currency gold). Climate owns its
/// copy so this file stays the only F3 edit path.
const THRESHOLD_TEND_SEAM: Color = Color::srgb(0.68, 0.36, 0.22);

/// Shared pipe/edge iron roughness for Threshold ground + edge path stones
/// (one material family — PLACE_DRESS_SPEC).
const THRESHOLD_PIPE_ROUGHNESS: f32 = 0.70;

/// CARD FLESH-THRESHOLD-DRESS — pipe-air iron haze (one family).
/// Steel sky and fog keep ground's cool iron bias (R≈G, B only a touch
/// above) so the tend seam is the door accent, not a generic blue sky.
/// Cite PLACE_DRESS Threshold look/tend/door · ART_BIBLE iron + tend seam
/// · DRIVE_PLACE_CITE · PEAK_MEMORY_LAW. Not Sanctuary graphite yard.
/// Not Heartwood living-wood. Not Depths wet-stone night.
const THRESHOLD_PIPE_SKY: Color = Color::srgb(0.24, 0.24, 0.28);
const THRESHOLD_PIPE_FOG: Color = Color::srgba(0.16, 0.16, 0.19, 1.0);
/// Tend disk stays clear. Pipe-air closes sooner than the open yard.
const THRESHOLD_FOG_START: f32 = 11.0;
const THRESHOLD_FOG_END: f32 = 34.0;

/// ART_BIBLE HANDS Depths accent — teal Peace (not Sanctuary warm-gold, not
/// Heartwood amber, not Threshold tend-seam, not currency gold). Climate owns
/// its copy so this file stays the only F4 edit path.
const DEPTHS_TEAL_PEACE: Color = Color::srgb(0.22, 0.92, 0.68);

/// Shared deepen/wet-stone roughness for Depths ground + path stones
/// (one material family — PLACE_DRESS_SPEC). Slightly slicker than dry yard.
const DEPTHS_WET_STONE_ROUGHNESS: f32 = 0.86;

/// CARD FLESH-DEPTHS-DRESS — wet-stone night haze (one family).
/// Cool teal-dark sky and fog so teal Peace is the one accent, not a
/// second biome. Cite PLACE_DRESS Depths one way down/home · ART_BIBLE
/// teal Peace · DRIVE_PLACE_CITE Depths teal Peace · PEAK_MEMORY_LAW.
/// Not Sanctuary graphite. Not Heartwood living-wood. Not Threshold
/// pipe-air iron.
const DEPTHS_WET_SKY: Color = Color::srgb(0.05, 0.10, 0.12);
const DEPTHS_WET_FOG: Color = Color::srgba(0.04, 0.09, 0.11, 1.0);
/// Night close fog (living_day Depths night — do not open into yard distances).
const DEPTHS_FOG_START: f32 = 3.5;
const DEPTHS_FOG_END: f32 = 16.0;

#[derive(Clone, Copy)]
struct ClimateLook {
    name: &'static str,
    ground: Color,
    sky: Color,
    fog: Color,
    ambient: Color,
    node: Color,
    stone: Color,
    fog_start: f32,
    fog_end: f32,
    ambient_bright: f32,
    /// One roughness for ground + stone in this Place's material family.
    roughness: f32,
}

fn look_for(realm: Option<u8>) -> ClimateLook {
    match realm {
        // Verdant Heartwood — live / seal room (PLACE_DRESS_SPEC).
        // One material family: living-wood bark ground + sapwood ring paths;
        // sky and fog are the same bark haze (FLESH-HEARTWOOD-DRESS). Well
        // node is the single amber-lamp accent (ART_BIBLE). Not Sanctuary
        // warm-gold carpet, not lawn green second biome, not Market chrome.
        Some(2) => ClimateLook {
            name: "Verdant Heartwood",
            ground: Color::srgb(0.20, 0.15, 0.07),
            sky: HEARTWOOD_WOOD_SKY,
            fog: HEARTWOOD_WOOD_FOG,
            ambient: Color::srgb(0.62, 0.58, 0.42),
            node: HEARTWOOD_AMBER_LAMP,
            stone: Color::srgb(0.30, 0.22, 0.11),
            fog_start: HEARTWOOD_FOG_START,
            fog_end: HEARTWOOD_FOG_END,
            ambient_bright: 260.0,
            roughness: HEARTWOOD_WOOD_ROUGHNESS,
        },
        // Threshold (Crystal Spires / Voidfarer Horizon) — look / tend / door
        // (PLACE_DRESS_SPEC). One material family: cool pipe/edge iron ground +
        // edge path stones; sky and fog are the same pipe-air iron
        // (FLESH-THRESHOLD-DRESS). Well node is the single tend-seam accent
        // (ART_BIBLE iron + tend seam). Not Sanctuary warm-yard, not Heartwood
        // living-wood, not Depths wet-stone, not instance-portal chrome,
        // not Market / fifth Place.
        Some(4) | Some(1) => ClimateLook {
            name: if realm == Some(1) {
                "Crystal Spires"
            } else {
                "Voidfarer Horizon"
            },
            ground: Color::srgb(0.12, 0.12, 0.15),
            sky: THRESHOLD_PIPE_SKY,
            fog: THRESHOLD_PIPE_FOG,
            ambient: Color::srgb(0.45, 0.48, 0.58),
            node: THRESHOLD_TEND_SEAM,
            stone: Color::srgb(0.20, 0.20, 0.25),
            fog_start: THRESHOLD_FOG_START,
            fog_end: THRESHOLD_FOG_END,
            ambient_bright: 200.0,
            roughness: THRESHOLD_PIPE_ROUGHNESS,
        },
        // Abyssal Depths — one way down / one way home (PLACE_DRESS_SPEC).
        // One material family: deepen wet-stone ground + path stones; sky and
        // fog are the same wet-stone night haze (FLESH-DEPTHS-DRESS). Well
        // node is the single teal Peace accent (ART_BIBLE). Night close fog
        // stays (living_day Depths night — do not fight). Not Sanctuary
        // warm-yard, not Heartwood living-wood, not Threshold pipe/edge, not
        // gold sink / Market / fifth Place.
        Some(3) => ClimateLook {
            name: "Abyssal Depths",
            ground: Color::srgb(0.04, 0.08, 0.09),
            sky: DEPTHS_WET_SKY,
            fog: DEPTHS_WET_FOG,
            ambient: Color::srgb(0.18, 0.42, 0.38),
            node: DEPTHS_TEAL_PEACE,
            stone: Color::srgb(0.08, 0.15, 0.17),
            fog_start: DEPTHS_FOG_START,
            fog_end: DEPTHS_FOG_END,
            ambient_bright: 90.0,
            roughness: DEPTHS_WET_STONE_ROUGHNESS,
        },
        // Sanctuary Prime — graphite-warm yard / teaching Peace
        // (PLACE_DRESS_SPEC · FLESH-SANCTUARY-DRESS). One material family:
        // graphite-warm earth ground + path stones; well node is the single
        // warm-gold accent, kept readable against that earth. Not Heartwood
        // green, not Brood Spire, not Market chrome. Not Draek crimson.
        _ => ClimateLook {
            name: "Sanctuary Prime",
            ground: SANCTUARY_GRAPHITE_EARTH,
            sky: SANCTUARY_GRAPHITE_SKY,
            fog: SANCTUARY_GRAPHITE_FOG,
            ambient: SANCTUARY_GRAPHITE_AMBIENT,
            node: SANCTUARY_WELL_GOLD,
            stone: SANCTUARY_GRAPHITE_STONE,
            fog_start: 12.0,
            fog_end: 40.0,
            ambient_bright: 250.0,
            roughness: SANCTUARY_YARD_ROUGHNESS,
        },
    }
}

/// Place weather mood — expressed via existing fog / sky tint / ambient breath.
/// Four Places stay four (PLACE_DRESS_SPEC). Not a fifth Place / HUD.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PlaceMood {
    /// Sanctuary sky / warm yard air.
    #[default]
    SanctuarySkyYard,
    /// Heartwood canopy shade / leaf-breath.
    HeartwoodCanopy,
    /// Threshold pipe air / edge draft.
    ThresholdPipeAir,
    /// Depths wet stone / close damp.
    DepthsWetStone,
}

impl PlaceMood {
    /// Short mood token for tests / logs — never a second HUD label.
    pub const fn mood_label(self) -> &'static str {
        match self {
            Self::SanctuarySkyYard => "sky/yard",
            Self::HeartwoodCanopy => "canopy",
            Self::ThresholdPipeAir => "pipe air",
            Self::DepthsWetStone => "wet stone",
        }
    }
}

/// Map SoftPlayerRealm id → Place mood (same realm table as [`look_for`]).
pub fn place_mood_for(realm: Option<u8>) -> PlaceMood {
    match realm {
        Some(2) => PlaceMood::HeartwoodCanopy,
        Some(4) | Some(1) => PlaceMood::ThresholdPipeAir,
        Some(3) => PlaceMood::DepthsWetStone,
        _ => PlaceMood::SanctuarySkyYard,
    }
}

/// CARD L4 — PlaceId → existing [`look_for`] realm. Same dresser Esc→Places
/// uses once `HexTravelState.current` changes. Threshold is not a PlaceId;
/// Quellorian rides Heartwood disk (realm 2) + shelf reach. 0 meshes.
pub fn dress_realm_for_place(place: PlaceId) -> Option<u8> {
    match place {
        PlaceId::Sanctuary => Some(0),
        PlaceId::Heartwood => Some(2),
        PlaceId::Depths => Some(3),
    }
}

/// Lived Place dress token (look name). Sanctuary Prime / Verdant Heartwood /
/// Abyssal Depths — not a second dresser, not a fourth PlaceId.
pub fn dress_token_for_place(place: PlaceId) -> &'static str {
    look_for(dress_realm_for_place(place)).name
}

/// Place mood already authored for this disk PlaceId.
pub fn dress_mood_for_place(place: PlaceId) -> PlaceMood {
    place_mood_for(dress_realm_for_place(place))
}

/// People-door dress token. Same PlaceId dresser as Esc→Places.
/// Ambrosian shares Sanctuary with Human. Quellorian = Heartwood token;
/// Threshold shelf reach lives in `shared/threshold_shelf.rs`.
pub fn dress_token_for_landing(landing: PeopleLanding) -> &'static str {
    dress_token_for_place(landing.place_id())
}

/// CARD L7 — existing FogSettings tokens for one arrival beat. No new mesh.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArrivalFog {
    pub color: Color,
    pub start: f32,
    pub end: f32,
}

fn look_fog(look: ClimateLook) -> ArrivalFog {
    ArrivalFog {
        color: look.fog,
        start: look.fog_start,
        end: look.fog_end,
    }
}

/// CARD L7 — fog keyed by PeopleLanding from existing [`look_for`] only.
/// Ambrosian keeps SanctuarySkyYard family, brighter / thinner (high).
/// mothership-over-Earth PRESENTATION; no hull mesh.
pub fn arrival_fog_for_landing(landing: PeopleLanding) -> ArrivalFog {
    let look = look_for(dress_realm_for_place(landing.place_id()));
    match landing {
        PeopleLanding::SanctuaryWellFromAbove => {
            let s = look.fog.to_srgba();
            ArrivalFog {
                color: Color::srgba(
                    (s.red * 1.22).min(1.0),
                    (s.green * 1.22).min(1.0),
                    (s.blue * 1.16).min(1.0),
                    s.alpha,
                ),
                start: look.fog_start + 6.0,
                end: look.fog_end + 14.0,
            }
        }
        PeopleLanding::SanctuaryYard
        | PeopleLanding::Heartwood
        | PeopleLanding::Threshold
        | PeopleLanding::DepthsTealWayHome => look_fog(look),
    }
}

/// CARD F5 — garden bounce dress. Existing PlaceId dresser (Sanctuary boot disk).
/// Not a fifth Place. Does not invent a Garden look_for row.
pub fn garden_bounce_dress_token(garden: PlaceId) -> &'static str {
    dress_token_for_place(garden)
}

/// CARD F5 — bounce does not recook L7 arrival fog WRITE.
pub fn wrong_door_bounce_recooks_l7_fog() -> bool {
    false
}

/// Garden title light yields on People-door land — lived Place dress is the level.
pub fn arrival_garden_title_light_yields(landing: PeopleLanding) -> bool {
    dress_token_for_landing(landing) != "Garden"
        && matches!(
            landing.place_id(),
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths
        )
}

/// Ambrosian high fog is thinner (starts later) and brighter than Human yard fog.
pub fn arrival_fog_is_thinner_brighter(high: ArrivalFog, yard: ArrivalFog) -> bool {
    high.start > yard.start
        && high.end > yard.end
        && srgb3(high.color).0 + srgb3(high.color).1 + srgb3(high.color).2
            > srgb3(yard.color).0 + srgb3(yard.color).1 + srgb3(yard.color).2
}

/// Comfort MeshLodPlan → Place dress procedural scale (path stones).
pub fn place_dress_lod_scale(preset: GraphicsPreset) -> f32 {
    mesh_lod::lived_place_dress_detail_scale(&mesh_lod::plan_for_preset(preset))
}

/// Buildings / Astra Medium stay refs — Place dress never dumps a `.glb`.
pub fn place_dress_dumps_glb(preset: GraphicsPreset) -> bool {
    mesh_lod::place_dress_uses_authored_glb(&mesh_lod::plan_for_preset(preset))
}

/// Low Comfort Place dress stays nameable — palettes hold, scale stays on-plane.
pub fn place_dress_low_stays_readable() -> bool {
    let plan = mesh_lod::plan_for_preset(GraphicsPreset::Low);
    let scale = mesh_lod::lived_place_dress_detail_scale(&plan);
    plan.primitives_only
        && !plan.persona_commit_dress
        && !mesh_lod::place_dress_uses_authored_glb(&plan)
        && place_dress_still_readable(scale)
}

/// Path-stone scale still leaves Place identity readable (palette + four Places).
pub fn place_dress_still_readable(scale: f32) -> bool {
    let s = look_for(Some(0));
    let h = look_for(Some(2));
    let t = look_for(Some(1));
    let d = look_for(Some(3));
    scale > 0.5
        && scale <= 1.15 + f32::EPSILON
        && is_graphite_warm_earth(s.ground)
        && is_graphite_warm_earth(s.stone)
        && is_warm_gold_well(s.node)
        && is_living_wood_earth(h.ground)
        && is_pipe_edge_iron(t.ground)
        && is_wet_stone_earth(d.ground)
        && s.name == "Sanctuary Prime"
        && h.name == "Verdant Heartwood"
        && t.name == "Crystal Spires"
        && d.name == "Abyssal Depths"
        && look_for(Some(4)).name == "Voidfarer Horizon"
        && climate_dress_copy_is_honest(s.name)
        && climate_dress_copy_is_honest(h.name)
        && climate_dress_copy_is_honest(t.name)
        && climate_dress_copy_is_honest(d.name)
}

/// FlowWeather band token mirrored here so climate beds couple without a
/// circular `flow_weather` import. Written by FlowWeather each tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum WeatherBandKind {
    #[default]
    Rise,
    Flow,
    Boredom,
    Anxiety,
}

/// Shared coupling slot: FlowWeather writes band; climate beds read it.
#[derive(Resource, Debug, Clone, Copy)]
pub struct WeatherBandCoupling {
    pub band: WeatherBandKind,
}

impl Default for WeatherBandCoupling {
    fn default() -> Self {
        Self {
            band: WeatherBandKind::Rise,
        }
    }
}

/// Procedural weather bed — fog / tint / breath from ClimateLook + fidelity.
/// No live Earth API. No binary weather pack. No second HUD.
#[derive(Clone, Copy, Debug)]
pub struct WeatherBed {
    pub mood: PlaceMood,
    pub fog: Color,
    pub sky: Color,
    pub ambient: Color,
    pub fog_start: f32,
    pub fog_end: f32,
    pub ambient_bright: f32,
    /// Soft breath rate (Hz-ish).
    pub breath_hz: f32,
    /// Breath amplitude already scaled by [`WeatherFidelity::intensity`].
    pub breath_amp: f32,
    /// Tint / emissive strength scale (fidelity).
    pub tint_strength: f32,
    /// Mist-bed density. 1.0 = dressed Medium/High extinction. Low is capped.
    pub bed_density: f32,
}

/// Base breath profile per Place mood (before fidelity / band coupling).
fn mood_breath_profile(mood: PlaceMood) -> (f32, f32) {
    match mood {
        // Sanctuary: slow warm sky/yard air.
        PlaceMood::SanctuarySkyYard => (0.11, 0.040),
        // Heartwood: slightly quicker canopy leaf-breath.
        PlaceMood::HeartwoodCanopy => (0.15, 0.055),
        // Threshold: thin pipe-air draft, lower amp.
        PlaceMood::ThresholdPipeAir => (0.09, 0.032),
        // Depths: slower wet-stone damp, closer fog breath.
        PlaceMood::DepthsWetStone => (0.07, 0.070),
    }
}

/// Comfort Low clear-distance floor (meters). FLESH dress starts close:
/// Sanctuary 12, Heartwood 10, Threshold 11, Depths 3.5. Low never inherits
/// that near ramp — phones / weak GPU stay readable (ASSET_BUDGET_COURT).
const LOW_FOG_START_FLOOR: f32 = 16.0;
/// Comfort Low full-fog horizon floor. Open yard dress ends at 40; Depths
/// night ends at 16. Low stays past both.
const LOW_FOG_END_FLOOR: f32 = 48.0;
/// Reduced-motion breath rate cap (Hz). Slowest dressed mood is Depths 0.07.
/// MESH_QUALITY_BUDGET Low = reduced motion.
const LOW_BREATH_HZ_CAP: f32 = 0.05;
/// Soft-breath amplitude cap. Quietest Medium mood is Threshold 0.032.
/// Depths dressed amp is 0.070; 0.55 intensity still leaves a richer pulse
/// than that quiet yard. Low must not carry that cost.
const LOW_BREATH_AMP_CAP: f32 = 0.018;
/// CARD OPT-WEATHER-FIDELITY-LOW — Comfort Low ceiling on fog / mist bed density.
/// Dressed beds pay full extinction (alpha 1). Bevy linear fog multiplies that
/// alpha into the mix, so 0.55 (Comfort Low intensity) thins the particle bed
/// without a second emitter. Medium and High stay at 1.
const LOW_BED_DENSITY_CAP: f32 = 0.55;

/// Low-only fog / mist bed density. Medium and High keep full extinction.
fn comfort_low_bed_density(fidelity: WeatherFidelity) -> f32 {
    match fidelity {
        WeatherFidelity::Low => LOW_BED_DENSITY_CAP,
        WeatherFidelity::Medium | WeatherFidelity::High => 1.0,
    }
}

/// Scale fog alpha by bed density. RGB stays the Place token.
fn fog_at_bed_density(color: Color, density: f32) -> Color {
    let s = color.to_srgba();
    Color::srgba(s.red, s.green, s.blue, s.alpha * density)
}

/// Low-only fog / breath cap. Medium and High return the dressed bed unchanged.
fn comfort_low_weather(
    fidelity: WeatherFidelity,
    fog_start: f32,
    fog_end: f32,
    hz: f32,
    amp0: f32,
    intensity: f32,
) -> (f32, f32, f32, f32) {
    match fidelity {
        WeatherFidelity::Low => {
            let start = fog_start.max(LOW_FOG_START_FLOOR);
            let end = fog_end.max(LOW_FOG_END_FLOOR).max(start + 8.0);
            let breath_hz = hz.min(LOW_BREATH_HZ_CAP);
            let breath_amp = (amp0 * intensity).min(LOW_BREATH_AMP_CAP);
            (start, end, breath_hz, breath_amp)
        }
        WeatherFidelity::Medium | WeatherFidelity::High => {
            (fog_start, fog_end, hz, amp0 * intensity)
        }
    }
}

/// Build a procedural weather bed for a realm + Comfort weather fidelity.
pub fn weather_bed_for(realm: Option<u8>, fidelity: WeatherFidelity) -> WeatherBed {
    let look = look_for(realm);
    let mood = place_mood_for(realm);
    let (hz, amp0) = mood_breath_profile(mood);
    let intensity = fidelity.intensity();
    let (fog_start, fog_end, breath_hz, breath_amp) =
        comfort_low_weather(fidelity, look.fog_start, look.fog_end, hz, amp0, intensity);
    let bed_density = comfort_low_bed_density(fidelity);
    // Medium/High keep the dressed Color, including alpha 1. Low thins alpha only.
    let fog = match fidelity {
        WeatherFidelity::Low => fog_at_bed_density(look.fog, bed_density),
        WeatherFidelity::Medium | WeatherFidelity::High => look.fog,
    };
    WeatherBed {
        mood,
        fog,
        sky: look.sky,
        ambient: look.ambient,
        fog_start,
        fog_end,
        ambient_bright: look.ambient_bright,
        breath_hz,
        breath_amp,
        tint_strength: intensity,
        bed_density,
    }
}

/// Couple FlowWeather band → Place mood breath / glow multiplier.
/// Flow lifts; Anxiety tightens; Boredom mutes; Rise is neutral.
pub fn place_band_mul(mood: PlaceMood, band: WeatherBandKind) -> f32 {
    let band_mul = match band {
        WeatherBandKind::Rise => 1.0,
        WeatherBandKind::Flow => 1.22,
        WeatherBandKind::Boredom => 0.72,
        WeatherBandKind::Anxiety => 0.85,
    };
    // Place-flavored nudge — canopy loves Flow; wet stone stays close under Anxiety.
    let place_mul = match (mood, band) {
        (PlaceMood::HeartwoodCanopy, WeatherBandKind::Flow) => 1.08,
        (PlaceMood::DepthsWetStone, WeatherBandKind::Anxiety) => 1.10,
        (PlaceMood::SanctuarySkyYard, WeatherBandKind::Rise) => 1.04,
        (PlaceMood::ThresholdPipeAir, WeatherBandKind::Boredom) => 0.92,
        _ => 1.0,
    };
    band_mul * place_mul
}

/// Effective breath amplitude after fidelity bed + band coupling.
pub fn coupled_breath_amp(bed: &WeatherBed, band: WeatherBandKind) -> f32 {
    bed.breath_amp * place_band_mul(bed.mood, band)
}

#[derive(Resource, Debug)]
pub struct ClimatePlane {
    pub applied: Option<u8>,
    pub mood: PlaceMood,
}

impl Default for ClimatePlane {
    fn default() -> Self {
        Self {
            applied: None,
            mood: PlaceMood::SanctuarySkyYard,
        }
    }
}

#[derive(Component)]
struct ClimateGround;
#[derive(Component)]
struct ClimateStone;
/// Path-stone mesh whose local scale follows Comfort [`mesh_lod::procedural_detail_scale`].
/// Ground plane stays unscaled so the yard still reads; Place identity holds on Low.
#[derive(Component)]
struct PlaceLodMesh;
#[derive(Component)]
struct ClimateNameRoot;
#[derive(Component)]
struct ClimateNameText;

pub struct ClimatePlanePlugin;

impl Plugin for ClimatePlanePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClimatePlane>()
            .init_resource::<WeatherBandCoupling>()
            .insert_resource(ClearColor(look_for(Some(0)).sky))
            .insert_resource(AmbientLight {
                color: look_for(Some(0)).ambient,
                brightness: look_for(Some(0)).ambient_bright,
            })
            .add_systems(Startup, (ensure_sanctuary, spawn_climate_place, spawn_climate_chip))
            .add_systems(
                Update,
                (
                    attach_fog_when_world_camera_arrives,
                    sync_place_dress_from_travel.before(apply_climate_look),
                    apply_climate_look,
                    apply_place_dress_mesh_lod,
                    breathe_weather_bed,
                    apply_arrival_beat_fog.after(breathe_weather_bed),
                    update_climate_chip,
                ),
            );
    }
}

fn ensure_sanctuary(mut realm: ResMut<SoftPlayerRealm>) {
    if realm.current.is_none() {
        realm.current = Some(0);
    }
}

fn spawn_climate_place(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cameras: Query<Entity, With<Camera3d>>,
    lights: Query<Entity, With<DirectionalLight>>,
    feel: Option<Res<LocalMeshLodFeel>>,
) {
    let look = look_for(Some(0));
    let preset = feel.map(|f| f.preset).unwrap_or(GraphicsPreset::Medium);
    let plan = mesh_lod::plan_for_preset(preset);
    let scale = mesh_lod::lived_place_dress_detail_scale(&plan);
    let _no_dump = mesh_lod::place_dress_uses_authored_glb(&plan);
    let ground = meshes.add(Plane3d::default().mesh().size(56.0, 56.0));
    commands.spawn((
        PbrBundle {
            mesh: ground,
            material: materials.add(StandardMaterial {
                base_color: look.ground,
                perceptual_roughness: SANCTUARY_YARD_ROUGHNESS,
                metallic: 0.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        ClimateGround,
        Name::new("ClimateGround"),
    ));

    let stone_mesh = meshes.add(Cylinder::new(0.18, 0.08));
    // Same roughness family as ground — greybox yard stones, not a second biome.
    let stone_mat = materials.add(StandardMaterial {
        base_color: look.stone,
        perceptual_roughness: SANCTUARY_YARD_ROUGHNESS,
        metallic: 0.0,
        ..default()
    });
    for target in NODE_ANCHORS {
        let dir = Vec3::new(target.x, 0.0, target.z);
        let steps = 4;
        for i in 1..=steps {
            let t = i as f32 / (steps as f32 + 0.35);
            let p = dir * t;
            commands.spawn((
                PbrBundle {
                    mesh: stone_mesh.clone(),
                    material: stone_mat.clone(),
                    transform: lod_stone_tf(p.x, 0.04, p.z, scale),
                    ..default()
                },
                ClimateStone,
                PlaceLodMesh,
            ));
        }
    }

    // Never spawn a second Camera3d here — main owns the yard camera.
    // A duplicate at order 0 races the lived UI camera on soft GPU (lavapipe)
    // and can bury Title / pause / Ledger under the world pass.
    if cameras.iter().next().is_none() {
        warn!(
            target: "powrush::climate",
            "no Camera3d yet — fog waits for world camera (ui-above-world)"
        );
    } else {
        for entity in &cameras {
            commands.entity(entity).insert(FogSettings {
                color: look.fog,
                falloff: FogFalloff::Linear {
                    start: look.fog_start,
                    end: look.fog_end,
                },
                ..default()
            });
        }
    }

    if lights.iter().next().is_none() {
        commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 8_500.0,
                shadows_enabled: false,
                color: Color::srgb(1.0, 0.96, 0.88),
                ..default()
            },
            transform: Transform::from_xyz(8.0, 18.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    }

    info!(target: "powrush::climate", "climate plane seeded — Sanctuary Prime warm yard");
}

fn spawn_climate_chip(mut commands: Commands) {
    // Existing place-name chip only — not a second HUD. Warm Sanctuary chrome
    // so the boot yard reads warm-gold before any climate slab.
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(18.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(280.0),
                    margin: UiRect::left(Val::Px(-140.0)),
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.08, 0.07, 0.05, 0.72).into(),
                border_color: Color::srgba(0.86, 0.66, 0.29, 0.40).into(),
                ..default()
            },
            ClimateNameRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "Sanctuary Prime",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.94, 0.90, 0.78),
                        ..default()
                    },
                ),
                ClimateNameText,
            ));
        });
}

/// CARD L4 — People-door / Esc→Places `PlaceId` turns on the existing
/// [`apply_climate_look`] dresser (fog / tint / path stones). No second table.
fn sync_place_dress_from_travel(
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut realm: ResMut<SoftPlayerRealm>,
) {
    let Some(travel) = travel else {
        return;
    };
    if !travel.is_changed() {
        return;
    }
    let id = dress_realm_for_place(travel.current);
    if realm.current != id {
        realm.current = id;
    }
}

fn attach_fog_when_world_camera_arrives(
    mut commands: Commands,
    cameras: Query<Entity, (With<Camera3d>, Without<FogSettings>)>,
    realm: Res<SoftPlayerRealm>,
) {
    if cameras.is_empty() {
        return;
    }
    let look = look_for(realm.current.or(Some(0)));
    for entity in &cameras {
        commands.entity(entity).insert(FogSettings {
            color: look.fog,
            falloff: FogFalloff::Linear {
                start: look.fog_start,
                end: look.fog_end,
            },
            ..default()
        });
    }
}

fn apply_climate_look(
    realm: Res<SoftPlayerRealm>,
    mut plane: ResMut<ClimatePlane>,
    mut clear: ResMut<ClearColor>,
    mut ambient: ResMut<AmbientLight>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grounds: Query<&Handle<StandardMaterial>, With<ClimateGround>>,
    stones: Query<&Handle<StandardMaterial>, With<ClimateStone>>,
    nodes: Query<&Handle<StandardMaterial>, With<MercyHarvestNode>>,
    mut fogs: Query<&mut FogSettings>,
) {
    let id = realm.current.unwrap_or(0);
    if plane.applied == Some(id) && !realm.is_changed() {
        return;
    }
    plane.applied = Some(id);
    plane.mood = place_mood_for(Some(id));
    let look = look_for(Some(id));
    // Sanctuary well glow lifts on graphite earth. Other realms keep 2.2;
    // lived paint in climate_visible owns the per-tick scale after this.
    let node_glow = if look.name == "Sanctuary Prime" {
        SANCTUARY_WELL_GLOW_SCALE
    } else {
        2.2
    };
    clear.0 = look.sky;
    ambient.color = look.ambient;
    ambient.brightness = look.ambient_bright;

    for handle in &grounds {
        if let Some(mat) = materials.get_mut(handle) {
            mat.base_color = look.ground;
            mat.perceptual_roughness = look.roughness;
            mat.metallic = 0.0;
        }
    }
    for handle in &stones {
        if let Some(mat) = materials.get_mut(handle) {
            mat.base_color = look.stone;
            mat.perceptual_roughness = look.roughness;
            mat.metallic = 0.0;
        }
    }
    for handle in &nodes {
        if let Some(mat) = materials.get_mut(handle) {
            mat.base_color = look.node;
            mat.emissive = LinearRgba::from(look.node).with_alpha(1.0) * node_glow;
        }
    }
    for mut fog in &mut fogs {
        fog.color = look.fog;
        fog.falloff = FogFalloff::Linear {
            start: look.fog_start,
            end: look.fog_end,
        };
    }
    info!(target: "powrush::climate", climate = look.name, id, "place shifted");
}

/// CARD L7 — Ambrosian lift plays brighter / thinner high fog from existing
/// FogSettings. Same Sanctuary PlaceId as Human. No new mesh / Camera3d.
fn apply_arrival_beat_fog(
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    presence: Option<Res<crate::human_presence::SoftPresence>>,
    settings: Option<Res<LocalSettingsState>>,
    mut fogs: Query<&mut FogSettings>,
) {
    let Some(presence) = presence else {
        return;
    };
    let Some(travel) = travel else {
        return;
    };
    if travel.current != PlaceId::Sanctuary {
        return;
    }
    if !crate::human_presence::presence_reads_ambrosian_lift(presence.position) {
        return;
    }
    let fog = arrival_fog_for_landing(PeopleLanding::SanctuaryWellFromAbove);
    let fidelity = settings
        .as_ref()
        .map(|s| s.inner.weather_fidelity())
        .unwrap_or(WeatherFidelity::Medium);
    // Distances stay the L7 beat. Low only thins alpha; Medium/High color is exact.
    let color = match fidelity {
        WeatherFidelity::Low => fog_at_bed_density(fog.color, comfort_low_bed_density(fidelity)),
        WeatherFidelity::Medium | WeatherFidelity::High => fog.color,
    };
    for mut fog_settings in &mut fogs {
        fog_settings.color = color;
        fog_settings.falloff = FogFalloff::Linear {
            start: fog.start,
            end: fog.end,
        };
    }
}

/// Apply Comfort MeshLodPlan scale when Esc Graphics changes.
/// Ground stays put; path stones thin/fill; palettes still name the Place.
fn apply_place_dress_mesh_lod(
    feel: Option<Res<LocalMeshLodFeel>>,
    mut meshes: Query<&mut Transform, With<PlaceLodMesh>>,
) {
    let Some(feel) = feel else {
        return;
    };
    if !feel.is_changed() {
        return;
    }
    let plan = mesh_lod::plan_for_preset(feel.preset);
    let scale = mesh_lod::lived_place_dress_detail_scale(&plan);
    let _no_dump = mesh_lod::place_dress_uses_authored_glb(&plan);
    for mut tf in &mut meshes {
        tf.scale = Vec3::splat(scale);
    }
}

fn lod_stone_tf(x: f32, y: f32, z: f32, scale: f32) -> Transform {
    Transform::from_xyz(x, y, z).with_scale(Vec3::splat(scale))
}

/// Soft fog / ambient breath from Place weather bed + FlowWeather band coupling.
/// Uses existing FogSettings / AmbientLight only — no second HUD, no sockets.
/// Comfort Low beds are already capped in [`weather_bed_for`] (gentler fog,
/// slower breath, thinner mist density). Medium and High keep the dressed ramp.
fn breathe_weather_bed(
    realm: Res<SoftPlayerRealm>,
    settings: Option<Res<LocalSettingsState>>,
    coupling: Res<WeatherBandCoupling>,
    time: Res<Time>,
    mut ambient: ResMut<AmbientLight>,
    mut fogs: Query<&mut FogSettings>,
) {
    let fidelity = settings
        .as_ref()
        .map(|s| s.inner.weather_fidelity())
        .unwrap_or(WeatherFidelity::Medium);
    let bed = weather_bed_for(realm.current.or(Some(0)), fidelity);
    let amp = coupled_breath_amp(&bed, coupling.band);
    let pulse = (time.elapsed_seconds() * bed.breath_hz * std::f32::consts::TAU).sin() * amp;
    // Fog distance breathes gently around the Place bed.
    for mut fog in &mut fogs {
        fog.color = bed.fog;
        let start = (bed.fog_start * (1.0 - pulse * 0.35)).max(0.5);
        let end = (bed.fog_end * (1.0 + pulse)).max(start + 1.0);
        fog.falloff = FogFalloff::Linear { start, end };
    }
    // Ambient tint strength follows fidelity; color stays Place dress.
    ambient.color = bed.ambient;
    ambient.brightness = (bed.ambient_bright * (1.0 + pulse * 0.45 * bed.tint_strength))
        .clamp(40.0, 520.0);
}

fn update_climate_chip(
    realm: Res<SoftPlayerRealm>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut text_q: Query<&mut Text, With<ClimateNameText>>,
) {
    // CARD L4 — chip follows HexTravelState. SoftPlayerRealm is synced from
    // the same PlaceId so apply_climate_look (not a second dresser) turns on.
    let name = travel
        .as_ref()
        .map(|t| t.chip_name())
        .unwrap_or_else(|| look_for(realm.current).name);
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != name {
                s.value = name.to_string();
            }
        }
    }
}

/// Presentation-copy honesty for Place dress (refuse Brood / Market / Online /
/// currency-gold / NFT). Warm-gold *color* is ART_BIBLE accent, not this copy.
fn climate_dress_copy_is_honest(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    !(lower.contains("brood")
        || lower.contains("market")
        || lower.contains("online")
        || lower.contains("nft")
        || lower.contains("auction")
        || lower.contains("gold sink")
        || lower.contains("currency")
        || lower.contains("portal")
        || lower.contains("instance"))
}

fn srgb3(c: Color) -> (f32, f32, f32) {
    let s = c.to_srgba();
    (s.red, s.green, s.blue)
}

/// Graphite-warm earth: warm grey-gold with tight chroma (FLESH-SANCTUARY-DRESS).
/// Inside [`is_warm_yard_earth`], greyer than tan dirt, still warm (R≈G > B).
fn is_graphite_warm_earth(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    is_warm_yard_earth(c) && (r - g) <= 0.025 && (r - b) <= 0.05 && (0.16..=0.32).contains(&r)
}

/// Emissive paint scale for a lived Place well. Sanctuary lifts so the
/// warm-gold accent stays readable on graphite-warm earth (ART_BIBLE: one glow).
/// Heartwood and Depths keep the shared scale. Comfort L/M/H do not retint.
pub fn well_glow_scale_for_place(place: PlaceId) -> f32 {
    match place {
        PlaceId::Sanctuary => SANCTUARY_WELL_GLOW_SCALE,
        PlaceId::Heartwood | PlaceId::Depths => PLACE_WELL_GLOW_SCALE,
    }
}

/// Warm-yard earth: desaturated warm grey-gold (R≈G > B), not bark and not lawn.
fn is_warm_yard_earth(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    r + 0.02 >= g
        && g > b
        && r > b
        && (r - g) <= 0.045
        && (g - b) < 0.12
        && (r - b) < 0.10
}

/// Warm-gold well accent: R > G > B with gold chroma (ART_BIBLE Sanctuary).
/// Amber lamp is warmer/orange ((r-g) larger, G lower) and must not match.
fn is_warm_gold_well(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    r > g && g > b && r > 0.7 && (r - b) > 0.35 && (r - g) <= 0.28 && g > 0.55
}

/// Living-wood / ring earth: warm bark–sapwood (R > G > B), not Sanctuary
/// grey-gold yard and not lawn where G strongly dominates.
fn is_living_wood_earth(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    r > g + 0.015
        && g > b
        && (r - b) > 0.08
        && r >= 0.14
        && !is_warm_yard_earth(c)
}

/// Amber lamp accent: warmer/orange than Sanctuary well gold (ART_BIBLE).
fn is_amber_lamp(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    r > g && g > b && r > 0.75 && (r - g) > 0.28 && (r - b) > 0.50
}

/// Pipe / edge iron: cool near-neutral steel (B ≥ G), not warm yard and not
/// living-wood bark. Says leave-enter door, not Sanctuary carpet.
fn is_pipe_edge_iron(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    let chroma = r.max(g).max(b) - r.min(g).min(b);
    chroma <= 0.08
        && b + 0.005 >= g
        && r >= 0.08
        && r <= 0.32
        && !is_warm_yard_earth(c)
        && !is_living_wood_earth(c)
}

/// Tend-seam accent: dry bronze / iron-tend (ART_BIBLE). Not Sanctuary
/// warm-gold (g lower), not Heartwood amber (r lower / less orange flare).
fn is_tend_seam(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    r > g
        && g > b
        && r > 0.55
        && r < 0.82
        && (r - g) > 0.18
        && (r - g) < 0.45
        && (r - b) > 0.30
        && g < 0.50
        && !is_amber_lamp(c)
        && !is_warm_gold_well(c)
}

/// Deepen / wet-stone earth: cool teal-dark stone (G and B above R, low lum).
/// Not warm yard, not living-wood bark, not cool-neutral pipe/edge iron.
fn is_wet_stone_earth(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    let chroma = r.max(g).max(b) - r.min(g).min(b);
    g > r
        && b > r
        && r < 0.16
        && g < 0.26
        && b < 0.28
        && (g - r) >= 0.02
        && chroma >= 0.035
        && !is_warm_yard_earth(c)
        && !is_living_wood_earth(c)
        && !is_pipe_edge_iron(c)
}

/// Teal Peace accent: G > B > R biolum teal (ART_BIBLE Depths). Not gold /
/// amber / tend-seam warm accents.
fn is_teal_peace(c: Color) -> bool {
    let (r, g, b) = srgb3(c);
    g > b
        && b > r
        && g > 0.70
        && (g - r) > 0.40
        && b > 0.50
        && !is_warm_gold_well(c)
        && !is_amber_lamp(c)
        && !is_tend_seam(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn climates_disagree() {
        assert_ne!(look_for(Some(0)).name, look_for(Some(2)).name);
        assert_ne!(look_for(Some(2)).name, look_for(Some(3)).name);
        assert_eq!(look_for(Some(3)).name, "Abyssal Depths");
        assert_eq!(look_for(Some(0)).name, "Sanctuary Prime");
    }

    #[test]
    fn sanctuary_boot_does_not_use_heartwood_look() {
        assert_eq!(look_for(Some(0)).name, "Sanctuary Prime");
        assert_eq!(look_for(None).name, "Sanctuary Prime");
        assert_ne!(look_for(Some(0)).name, look_for(Some(2)).name);
    }

    #[test]
    fn sanctuary_greybox_is_one_warm_yard_family() {
        let s = look_for(Some(0));
        assert!(
            is_warm_yard_earth(s.ground),
            "Sanctuary ground must read warm yard earth, got {:?}",
            srgb3(s.ground)
        );
        assert!(
            is_warm_yard_earth(s.stone),
            "Sanctuary stones must share yard earth family, got {:?}",
            srgb3(s.stone)
        );
        assert!(
            is_warm_gold_well(s.node),
            "Sanctuary well must be warm-gold accent, got {:?}",
            srgb3(s.node)
        );
        // Ground + stone stay one family: same warm bias; path stones catch
        // a touch more light so they read as yard dressing, not a second biome.
        let (gr, gg, gb) = srgb3(s.ground);
        let (sr, sg, sb) = srgb3(s.stone);
        let g_lum = (gr + gg + gb) / 3.0;
        let s_lum = (sr + sg + sb) / 3.0;
        assert!(
            s_lum > g_lum,
            "path stones should sit slightly above ground luminance"
        );
        assert!(
            (sr - gr).abs() < 0.12 && (sg - gg).abs() < 0.12 && (sb - gb).abs() < 0.12,
            "stone drifted out of the yard earth family"
        );
        // F1 Sanctuary stays warm-yard; F2 Heartwood is living-wood — not
        // the same family, and Sanctuary must not wear Heartwood dress.
        let h = look_for(Some(2));
        assert!(
            is_living_wood_earth(h.ground),
            "Heartwood must stay living-wood, got {:?}",
            srgb3(h.ground)
        );
        assert!(!is_living_wood_earth(s.ground));
        assert_ne!(srgb3(s.ground), srgb3(h.ground));
        assert_ne!(srgb3(s.node), srgb3(h.node));
        assert!(!is_amber_lamp(s.node));
    }

    /// CARD FLESH-SANCTUARY-DRESS — graphite-warm earth, readable warm-gold well.
    /// Comfort stays Low / Medium / High. No Ultra. Other Places keep their dress.
    #[test]
    fn flesh_sanctuary_dress_is_graphite_warm_with_readable_well() {
        let s = look_for(Some(0));
        assert!(is_graphite_warm_earth(s.ground), "{:?}", srgb3(s.ground));
        assert!(is_graphite_warm_earth(s.stone), "{:?}", srgb3(s.stone));
        assert_eq!(srgb3(s.ground), srgb3(SANCTUARY_GRAPHITE_EARTH));
        assert_eq!(srgb3(s.stone), srgb3(SANCTUARY_GRAPHITE_STONE));
        assert_eq!(srgb3(s.sky), srgb3(SANCTUARY_GRAPHITE_SKY));
        assert_eq!(srgb3(s.fog), srgb3(SANCTUARY_GRAPHITE_FOG));
        assert_eq!(srgb3(s.ambient), srgb3(SANCTUARY_GRAPHITE_AMBIENT));
        // Weather stays in the yard family so fog is not a second biome.
        assert!(is_warm_yard_earth(s.fog));
        assert!(is_warm_yard_earth(s.sky));
        assert!(is_warm_yard_earth(s.ambient));
        assert!(is_warm_gold_well(s.node));
        let (gr, gg, gb) = srgb3(s.ground);
        let (nr, ng, nb) = srgb3(s.node);
        let earth = (gr + gg + gb) / 3.0;
        let well = (nr + ng + nb) / 3.0;
        assert!(
            well > earth + 0.35,
            "warm-gold well must read above graphite earth ({well} vs {earth})"
        );
        assert!(SANCTUARY_WELL_GLOW_SCALE > PLACE_WELL_GLOW_SCALE);
        assert_eq!(
            well_glow_scale_for_place(PlaceId::Sanctuary),
            SANCTUARY_WELL_GLOW_SCALE
        );
        assert_eq!(
            well_glow_scale_for_place(PlaceId::Heartwood),
            PLACE_WELL_GLOW_SCALE
        );
        assert_eq!(
            well_glow_scale_for_place(PlaceId::Depths),
            PLACE_WELL_GLOW_SCALE
        );
        // Wells sit inside the clear band; horizon stays a yard, not Depths night.
        assert!(s.fog_start >= 12.0);
        assert!(s.fog_end > s.fog_start);
        assert!(s.fog_end > 20.0);
        assert_eq!(GraphicsPreset::ALL.len(), 3);
        for preset in GraphicsPreset::ALL {
            assert!(!preset.label().contains("Ultra"));
            assert!(place_dress_still_readable(place_dress_lod_scale(preset)));
        }
        assert!(!is_graphite_warm_earth(look_for(Some(2)).ground));
        assert!(!is_graphite_warm_earth(look_for(Some(1)).ground));
        assert!(!is_graphite_warm_earth(look_for(Some(3)).ground));
        assert!(!is_graphite_warm_earth(look_for(Some(4)).ground));
    }

    #[test]
    fn four_places_stay_four_material_moods() {
        let sanctuary = look_for(Some(0));
        let heartwood = look_for(Some(2));
        let threshold = look_for(Some(1));
        let depths = look_for(Some(3));
        assert_eq!(sanctuary.name, "Sanctuary Prime");
        assert_eq!(heartwood.name, "Verdant Heartwood");
        assert_eq!(threshold.name, "Crystal Spires");
        assert_eq!(depths.name, "Abyssal Depths");
        // Threshold realm 4 shares Threshold dress (not a fifth Place).
        assert_eq!(look_for(Some(4)).name, "Voidfarer Horizon");
        assert_ne!(srgb3(sanctuary.ground), srgb3(heartwood.ground));
        assert_ne!(srgb3(sanctuary.ground), srgb3(threshold.ground));
        assert_ne!(srgb3(sanctuary.ground), srgb3(depths.ground));
        assert_ne!(srgb3(heartwood.ground), srgb3(depths.ground));
        assert_ne!(srgb3(threshold.ground), srgb3(depths.ground));
    }

    #[test]
    fn sanctuary_well_accent_matches_art_bible_warm_gold() {
        let node = look_for(Some(0)).node;
        assert_eq!(srgb3(node), srgb3(SANCTUARY_WELL_GOLD));
        assert!(is_warm_gold_well(node));
        // Currency-gold refuse is about copy / Market — accent color is law.
        assert!(climate_dress_copy_is_honest(look_for(Some(0)).name));
    }

    #[test]
    fn climate_place_names_refuse_brood_market_online_gold() {
        for id in [0_u8, 1, 2, 3, 4] {
            let name = look_for(Some(id)).name;
            assert!(
                climate_dress_copy_is_honest(name),
                "climate name not dress-honest: {name}"
            );
            let lower = name.to_ascii_lowercase();
            assert!(!lower.contains("brood"));
            assert!(!lower.contains("market"));
            assert!(!lower.contains("online"));
            assert!(!lower.contains("gold"));
        }
    }

    #[test]
    fn place_readable_from_sanctuary_dress_before_slab() {
        // Nameable from presentation alone: warm yard + warm-gold well + name.
        let s = look_for(Some(0));
        assert_eq!(s.name, "Sanctuary Prime");
        assert!(is_warm_yard_earth(s.ground));
        assert!(is_warm_gold_well(s.node));
        assert!(s.fog_end > s.fog_start);
        assert!(climate_dress_copy_is_honest(s.name));
    }

    #[test]
    fn heartwood_greybox_is_one_living_wood_family() {
        let h = look_for(Some(2));
        assert_eq!(h.name, "Verdant Heartwood");
        assert!(
            is_living_wood_earth(h.ground),
            "Heartwood ground must read living-wood, got {:?}",
            srgb3(h.ground)
        );
        assert!(
            is_living_wood_earth(h.stone),
            "Heartwood ring paths must share living-wood family, got {:?}",
            srgb3(h.stone)
        );
        assert!(
            is_amber_lamp(h.node),
            "Heartwood well must be amber-lamp accent, got {:?}",
            srgb3(h.node)
        );
        assert_eq!(srgb3(h.node), srgb3(HEARTWOOD_AMBER_LAMP));
        assert_eq!(h.roughness, HEARTWOOD_WOOD_ROUGHNESS);
        // Ground + stone stay one family: same wood bias; ring paths catch
        // a touch more light so they read as seal-room dressing, not a second biome.
        let (gr, gg, gb) = srgb3(h.ground);
        let (sr, sg, sb) = srgb3(h.stone);
        let g_lum = (gr + gg + gb) / 3.0;
        let s_lum = (sr + sg + sb) / 3.0;
        assert!(
            s_lum > g_lum,
            "ring paths should sit slightly above ground luminance"
        );
        assert!(
            (sr - gr).abs() < 0.14 && (sg - gg).abs() < 0.14 && (sb - gb).abs() < 0.14,
            "stone drifted out of the living-wood family"
        );
        // Not Sanctuary warm-yard carpet pasted over.
        let s = look_for(Some(0));
        assert!(!is_warm_yard_earth(h.ground));
        assert!(!is_warm_gold_well(h.node));
        assert_ne!(srgb3(h.ground), srgb3(s.ground));
        assert_ne!(srgb3(h.node), srgb3(s.node));
        assert_ne!(h.roughness, s.roughness);
    }

    #[test]
    fn place_readable_from_heartwood_dress_before_slab() {
        // Nameable from presentation alone: living-wood + amber lamp + name.
        let h = look_for(Some(2));
        assert_eq!(h.name, "Verdant Heartwood");
        assert!(is_living_wood_earth(h.ground));
        assert!(is_living_wood_earth(h.stone));
        assert!(is_amber_lamp(h.node));
        assert!(h.fog_end > h.fog_start);
        assert!(climate_dress_copy_is_honest(h.name));
        let lower = h.name.to_ascii_lowercase();
        assert!(!lower.contains("brood"));
        assert!(!lower.contains("market"));
        assert!(!lower.contains("online"));
        assert!(!lower.contains("gold"));
        assert!(!lower.contains("sanctuary"));
    }

    /// CARD FLESH-HEARTWOOD-DRESS — living-wood fog, amber lamp still the hush.
    /// Ground and ring paths stay the walked F2 family. Comfort L/M/H. No Ultra.
    /// Lamp-hush bed gain stays the banked constant (peace_audio). No mixer rewrite.
    #[test]
    fn flesh_heartwood_dress_is_living_wood_fog_with_lamp_hush() {
        let h = look_for(Some(2));
        let s = look_for(Some(0));
        assert_eq!(h.name, "Verdant Heartwood");
        assert_eq!(srgb3(h.sky), srgb3(HEARTWOOD_WOOD_SKY));
        assert_eq!(srgb3(h.fog), srgb3(HEARTWOOD_WOOD_FOG));
        assert!(
            is_living_wood_earth(h.fog),
            "Heartwood fog must read living-wood, got {:?}",
            srgb3(h.fog)
        );
        assert!(
            is_living_wood_earth(h.sky),
            "Heartwood sky must stay in the living-wood family, got {:?}",
            srgb3(h.sky)
        );
        assert!(is_living_wood_earth(h.ground));
        assert!(is_living_wood_earth(h.stone));
        assert!(is_living_wood_earth(h.ambient));
        assert!(is_amber_lamp(h.node));
        assert_eq!(srgb3(h.node), srgb3(HEARTWOOD_AMBER_LAMP));
        assert_eq!(srgb3(h.ground), (0.20, 0.15, 0.07));
        assert_eq!(srgb3(h.stone), (0.30, 0.22, 0.11));
        assert_eq!(h.roughness, HEARTWOOD_WOOD_ROUGHNESS);
        // Lawn green and Sanctuary graphite are other families.
        assert!(!is_warm_yard_earth(h.fog));
        assert!(!is_graphite_warm_earth(h.fog));
        assert!(!is_warm_yard_earth(h.sky));
        assert!(!is_wet_stone_earth(h.fog));
        assert!(!is_pipe_edge_iron(h.fog));
        assert_ne!(srgb3(h.fog), srgb3(s.fog));
        assert_ne!(srgb3(h.sky), srgb3(s.sky));
        assert!(!is_warm_gold_well(h.node));
        let (fr, fg, fb) = srgb3(h.fog);
        let (nr, ng, nb) = srgb3(h.node);
        let fog_lum = (fr + fg + fb) / 3.0;
        let lamp_lum = (nr + ng + nb) / 3.0;
        assert!(
            lamp_lum > fog_lum + 0.20,
            "amber lamp must read above living-wood fog ({lamp_lum} vs {fog_lum})"
        );
        // Lamp disk stays clear. Haze closes sooner than the open yard, not Depths night.
        assert!((h.fog_start - HEARTWOOD_FOG_START).abs() < f32::EPSILON);
        assert!((h.fog_end - HEARTWOOD_FOG_END).abs() < f32::EPSILON);
        assert!(h.fog_start >= 10.0);
        assert!(h.fog_end > 20.0);
        assert!(h.fog_end < s.fog_end);
        let bed = weather_bed_for(Some(2), WeatherFidelity::Medium);
        assert_eq!(bed.mood, PlaceMood::HeartwoodCanopy);
        assert_eq!(srgb3(bed.fog), srgb3(h.fog));
        assert_eq!(bed.fog_start, h.fog_start);
        assert_eq!(bed.fog_end, h.fog_end);
        assert_eq!(
            arrival_fog_for_landing(PeopleLanding::Heartwood).color,
            h.fog
        );
        // Banked lamp hush. Fog dress does not retune the mixer.
        assert!(shared::peace_audio::BED_GAIN_HEARTWOOD > 0.0);
        assert!(shared::peace_audio::BED_GAIN_HEARTWOOD < shared::peace_audio::BED_GAIN_OPEN);
        assert!(shared::peace_audio::BED_GAIN_HEARTWOOD < shared::peace_audio::BED_GAIN_DEPTHS);
        assert_eq!(
            well_glow_scale_for_place(PlaceId::Heartwood),
            PLACE_WELL_GLOW_SCALE
        );
        assert_eq!(GraphicsPreset::ALL.len(), 3);
        for preset in GraphicsPreset::ALL {
            assert!(!preset.label().contains("Ultra"));
            assert!(place_dress_still_readable(place_dress_lod_scale(preset)));
        }
        assert!(!is_living_wood_earth(s.fog));
        assert!(!is_living_wood_earth(look_for(Some(1)).fog));
        assert!(!is_living_wood_earth(look_for(Some(3)).fog));
    }

    #[test]
    fn heartwood_amber_refuses_sanctuary_gold_carpet() {
        let h = look_for(Some(2));
        let s = look_for(Some(0));
        assert_eq!(srgb3(h.node), srgb3(HEARTWOOD_AMBER_LAMP));
        assert_eq!(srgb3(s.node), srgb3(SANCTUARY_WELL_GOLD));
        assert_ne!(srgb3(HEARTWOOD_AMBER_LAMP), srgb3(SANCTUARY_WELL_GOLD));
        assert!(is_amber_lamp(h.node));
        assert!(is_warm_gold_well(s.node));
        assert!(!is_amber_lamp(s.node));
        assert!(!is_warm_gold_well(h.node));
        assert!(climate_dress_copy_is_honest(h.name));
        assert!(climate_dress_copy_is_honest(s.name));
    }

    #[test]
    fn threshold_greybox_is_one_pipe_edge_family() {
        let t = look_for(Some(1));
        assert_eq!(t.name, "Crystal Spires");
        assert!(
            is_pipe_edge_iron(t.ground),
            "Threshold ground must read pipe/edge iron, got {:?}",
            srgb3(t.ground)
        );
        assert!(
            is_pipe_edge_iron(t.stone),
            "Threshold edge paths must share pipe/edge iron family, got {:?}",
            srgb3(t.stone)
        );
        assert!(
            is_tend_seam(t.node),
            "Threshold well must be tend-seam accent, got {:?}",
            srgb3(t.node)
        );
        assert_eq!(srgb3(t.node), srgb3(THRESHOLD_TEND_SEAM));
        assert_eq!(t.roughness, THRESHOLD_PIPE_ROUGHNESS);
        // Ground + stone stay one family: same cool iron bias; edge paths catch
        // a touch more light so they read as door dressing, not a second biome.
        let (gr, gg, gb) = srgb3(t.ground);
        let (sr, sg, sb) = srgb3(t.stone);
        let g_lum = (gr + gg + gb) / 3.0;
        let s_lum = (sr + sg + sb) / 3.0;
        assert!(
            s_lum > g_lum,
            "edge paths should sit slightly above ground luminance"
        );
        assert!(
            (sr - gr).abs() < 0.14 && (sg - gg).abs() < 0.14 && (sb - gb).abs() < 0.14,
            "stone drifted out of the pipe/edge iron family"
        );
        // Realm 4 shares the same Threshold dress (not a fifth Place).
        let v = look_for(Some(4));
        assert_eq!(v.name, "Voidfarer Horizon");
        assert_eq!(srgb3(v.ground), srgb3(t.ground));
        assert_eq!(srgb3(v.stone), srgb3(t.stone));
        assert_eq!(srgb3(v.node), srgb3(t.node));
        assert_eq!(v.roughness, t.roughness);
        // Not Sanctuary warm-yard or Heartwood living-wood carpet pasted over.
        let s = look_for(Some(0));
        let h = look_for(Some(2));
        assert!(!is_warm_yard_earth(t.ground));
        assert!(!is_living_wood_earth(t.ground));
        assert!(!is_warm_gold_well(t.node));
        assert!(!is_amber_lamp(t.node));
        assert_ne!(srgb3(t.ground), srgb3(s.ground));
        assert_ne!(srgb3(t.ground), srgb3(h.ground));
        assert_ne!(srgb3(t.node), srgb3(s.node));
        assert_ne!(srgb3(t.node), srgb3(h.node));
        assert_ne!(t.roughness, s.roughness);
        assert_ne!(t.roughness, h.roughness);
    }

    #[test]
    fn place_readable_from_threshold_dress_before_slab() {
        // Nameable from presentation alone: pipe/edge iron + tend seam + name.
        let t = look_for(Some(1));
        assert_eq!(t.name, "Crystal Spires");
        assert!(is_pipe_edge_iron(t.ground));
        assert!(is_pipe_edge_iron(t.stone));
        assert!(is_tend_seam(t.node));
        assert!(t.fog_end > t.fog_start);
        assert!(climate_dress_copy_is_honest(t.name));
        let lower = t.name.to_ascii_lowercase();
        assert!(!lower.contains("brood"));
        assert!(!lower.contains("market"));
        assert!(!lower.contains("online"));
        assert!(!lower.contains("gold"));
        assert!(!lower.contains("portal"));
        assert!(!lower.contains("instance"));
        assert!(!lower.contains("sanctuary"));
        assert!(!lower.contains("heartwood"));
        let v = look_for(Some(4));
        assert!(climate_dress_copy_is_honest(v.name));
        let vlower = v.name.to_ascii_lowercase();
        assert!(!vlower.contains("portal"));
        assert!(!vlower.contains("instance"));
        assert!(!vlower.contains("market"));
    }

    /// CARD FLESH-THRESHOLD-DRESS — pipe-air iron fog, tend seam still the accent.
    /// Ground and edge stones stay the walked F3 family. Comfort L/M/H. No Ultra.
    /// Shelf arrival stays Heartwood fog (Quellorian culture). No fifth PlaceId.
    #[test]
    fn flesh_threshold_dress_is_pipe_air_iron_fog_with_tend_seam() {
        let t = look_for(Some(1));
        let v = look_for(Some(4));
        let s = look_for(Some(0));
        let h = look_for(Some(2));
        let d = look_for(Some(3));
        assert_eq!(t.name, "Crystal Spires");
        assert_eq!(v.name, "Voidfarer Horizon");
        assert_eq!(srgb3(t.sky), srgb3(THRESHOLD_PIPE_SKY));
        assert_eq!(srgb3(t.fog), srgb3(THRESHOLD_PIPE_FOG));
        assert_eq!(srgb3(v.sky), srgb3(t.sky));
        assert_eq!(srgb3(v.fog), srgb3(t.fog));
        assert_eq!(v.fog_start, t.fog_start);
        assert_eq!(v.fog_end, t.fog_end);
        assert!(
            is_pipe_edge_iron(t.fog),
            "Threshold fog must read pipe-air iron, got {:?}",
            srgb3(t.fog)
        );
        assert!(
            is_pipe_edge_iron(t.sky),
            "Threshold sky must stay in the pipe-air iron family, got {:?}",
            srgb3(t.sky)
        );
        assert!(is_pipe_edge_iron(t.ground));
        assert!(is_pipe_edge_iron(t.stone));
        assert!(is_tend_seam(t.node));
        assert_eq!(srgb3(t.node), srgb3(THRESHOLD_TEND_SEAM));
        assert_eq!(srgb3(v.node), srgb3(t.node));
        // Walked F3 tokens. Sky/fog join them; ground and stone do not move.
        assert_eq!(srgb3(t.ground), (0.12, 0.12, 0.15));
        assert_eq!(srgb3(t.stone), (0.20, 0.20, 0.25));
        assert_eq!(srgb3(v.ground), srgb3(t.ground));
        assert_eq!(srgb3(v.stone), srgb3(t.stone));
        assert_eq!(t.roughness, THRESHOLD_PIPE_ROUGHNESS);
        assert_eq!(v.roughness, t.roughness);
        assert_eq!(srgb3(t.ambient), (0.45, 0.48, 0.58));
        // Iron bias: R≈G, blue only a touch above — not a generic cool-blue sky.
        for (label, color) in [
            ("ground", t.ground),
            ("fog", t.fog),
            ("sky", t.sky),
            ("stone", t.stone),
        ] {
            let (r, g, b) = srgb3(color);
            assert!(
                (r - g).abs() <= 0.01,
                "{label} left the iron R≈G family: {r} {g} {b}"
            );
            assert!(
                b + f32::EPSILON >= g && (b - r) <= 0.06,
                "{label} blue cast left the iron family: {r} {g} {b}"
            );
        }
        assert!(!is_graphite_warm_earth(t.fog));
        assert!(!is_graphite_warm_earth(t.sky));
        assert!(!is_warm_yard_earth(t.fog));
        assert!(!is_warm_yard_earth(t.sky));
        assert!(!is_living_wood_earth(t.fog));
        assert!(!is_living_wood_earth(t.sky));
        assert!(!is_wet_stone_earth(t.fog));
        assert!(!is_wet_stone_earth(t.sky));
        assert!(!is_pipe_edge_iron(s.fog));
        assert!(!is_pipe_edge_iron(h.fog));
        assert!(!is_pipe_edge_iron(d.fog));
        assert_ne!(srgb3(t.fog), srgb3(s.fog));
        assert_ne!(srgb3(t.fog), srgb3(h.fog));
        assert_ne!(srgb3(t.fog), srgb3(d.fog));
        assert_ne!(srgb3(t.sky), srgb3(s.sky));
        assert_ne!(srgb3(t.sky), srgb3(h.sky));
        assert_ne!(srgb3(t.sky), srgb3(d.sky));
        assert!(!is_warm_gold_well(t.node));
        assert!(!is_amber_lamp(t.node));
        assert!(!is_teal_peace(t.node));
        let (fr, fg, fb) = srgb3(t.fog);
        let (nr, ng, nb) = srgb3(t.node);
        let fog_lum = (fr + fg + fb) / 3.0;
        let seam_lum = (nr + ng + nb) / 3.0;
        assert!(
            seam_lum > fog_lum + 0.20,
            "tend seam must read above pipe-air fog ({seam_lum} vs {fog_lum})"
        );
        // Tend disk stays clear. Pipe-air closes sooner than the open yard, not Depths night.
        assert!((t.fog_start - THRESHOLD_FOG_START).abs() < f32::EPSILON);
        assert!((t.fog_end - THRESHOLD_FOG_END).abs() < f32::EPSILON);
        assert!(t.fog_start >= 10.0);
        assert!(t.fog_end > 20.0);
        assert!(t.fog_end < s.fog_end);
        assert!(t.fog_end > d.fog_end);
        let bed = weather_bed_for(Some(1), WeatherFidelity::Medium);
        assert_eq!(bed.mood, PlaceMood::ThresholdPipeAir);
        assert_eq!(srgb3(bed.fog), srgb3(t.fog));
        assert_eq!(srgb3(bed.sky), srgb3(t.sky));
        assert_eq!(bed.fog_start, t.fog_start);
        assert_eq!(bed.fog_end, t.fog_end);
        let horizon = weather_bed_for(Some(4), WeatherFidelity::Medium);
        assert_eq!(horizon.mood, PlaceMood::ThresholdPipeAir);
        assert_eq!(srgb3(horizon.fog), srgb3(t.fog));
        // Quellorian shelf still rides Heartwood fog. Realm 1/4 is the pipe look.
        assert_eq!(
            arrival_fog_for_landing(PeopleLanding::Threshold).color,
            h.fog
        );
        assert_ne!(
            srgb3(arrival_fog_for_landing(PeopleLanding::Threshold).color),
            srgb3(t.fog)
        );
        assert_eq!(dress_realm_for_place(PlaceId::Heartwood), Some(2));
        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
        assert_eq!(
            well_glow_scale_for_place(PlaceId::Heartwood),
            PLACE_WELL_GLOW_SCALE
        );
        assert_eq!(GraphicsPreset::ALL.len(), 3);
        for preset in GraphicsPreset::ALL {
            assert!(!preset.label().contains("Ultra"));
            assert!(place_dress_still_readable(place_dress_lod_scale(preset)));
        }
    }

    #[test]
    fn threshold_tend_seam_refuses_sanctuary_and_heartwood_accents() {
        let t = look_for(Some(1));
        let s = look_for(Some(0));
        let h = look_for(Some(2));
        assert_eq!(srgb3(t.node), srgb3(THRESHOLD_TEND_SEAM));
        assert_eq!(srgb3(s.node), srgb3(SANCTUARY_WELL_GOLD));
        assert_eq!(srgb3(h.node), srgb3(HEARTWOOD_AMBER_LAMP));
        assert_ne!(srgb3(THRESHOLD_TEND_SEAM), srgb3(SANCTUARY_WELL_GOLD));
        assert_ne!(srgb3(THRESHOLD_TEND_SEAM), srgb3(HEARTWOOD_AMBER_LAMP));
        assert!(is_tend_seam(t.node));
        assert!(is_warm_gold_well(s.node));
        assert!(is_amber_lamp(h.node));
        assert!(!is_tend_seam(s.node));
        assert!(!is_tend_seam(h.node));
        assert!(!is_warm_gold_well(t.node));
        assert!(!is_amber_lamp(t.node));
        assert!(climate_dress_copy_is_honest(t.name));
        // F1 / F2 dress stays intact under F3.
        assert!(is_warm_yard_earth(s.ground));
        assert!(is_living_wood_earth(h.ground));
        assert_eq!(s.roughness, SANCTUARY_YARD_ROUGHNESS);
        assert_eq!(h.roughness, HEARTWOOD_WOOD_ROUGHNESS);
    }

    #[test]
    fn depths_greybox_is_one_deepen_wet_stone_family() {
        let d = look_for(Some(3));
        assert_eq!(d.name, "Abyssal Depths");
        assert!(
            is_wet_stone_earth(d.ground),
            "Depths ground must read deepen wet-stone, got {:?}",
            srgb3(d.ground)
        );
        assert!(
            is_wet_stone_earth(d.stone),
            "Depths path stones must share wet-stone family, got {:?}",
            srgb3(d.stone)
        );
        assert!(
            is_teal_peace(d.node),
            "Depths well must be teal Peace accent, got {:?}",
            srgb3(d.node)
        );
        assert_eq!(srgb3(d.node), srgb3(DEPTHS_TEAL_PEACE));
        assert_eq!(d.roughness, DEPTHS_WET_STONE_ROUGHNESS);
        // Ground + stone stay one family: same cool teal-dark bias; path stones
        // catch a touch more light so they read as landing dress, not a second biome.
        let (gr, gg, gb) = srgb3(d.ground);
        let (sr, sg, sb) = srgb3(d.stone);
        let g_lum = (gr + gg + gb) / 3.0;
        let s_lum = (sr + sg + sb) / 3.0;
        assert!(
            s_lum > g_lum,
            "wet-stone paths should sit slightly above ground luminance"
        );
        assert!(
            (sr - gr).abs() < 0.14 && (sg - gg).abs() < 0.14 && (sb - gb).abs() < 0.14,
            "stone drifted out of the wet-stone family"
        );
        // Night / close fog intact (living_day Depths stay night — do not fight).
        assert!(d.fog_end > d.fog_start);
        assert!(d.fog_end <= 20.0);
        assert!(d.ambient_bright < 150.0);
        // Not Sanctuary / Heartwood / Threshold carpet pasted over.
        let s = look_for(Some(0));
        let h = look_for(Some(2));
        let t = look_for(Some(1));
        assert!(!is_warm_yard_earth(d.ground));
        assert!(!is_living_wood_earth(d.ground));
        assert!(!is_pipe_edge_iron(d.ground));
        assert!(!is_warm_gold_well(d.node));
        assert!(!is_amber_lamp(d.node));
        assert!(!is_tend_seam(d.node));
        assert_ne!(srgb3(d.ground), srgb3(s.ground));
        assert_ne!(srgb3(d.ground), srgb3(h.ground));
        assert_ne!(srgb3(d.ground), srgb3(t.ground));
        assert_ne!(srgb3(d.node), srgb3(s.node));
        assert_ne!(srgb3(d.node), srgb3(h.node));
        assert_ne!(srgb3(d.node), srgb3(t.node));
        assert_ne!(d.roughness, s.roughness);
        assert_ne!(d.roughness, h.roughness);
        assert_ne!(d.roughness, t.roughness);
    }

    #[test]
    fn place_readable_from_depths_dress_before_slab() {
        // Nameable from presentation alone: wet-stone + teal Peace + name.
        let d = look_for(Some(3));
        assert_eq!(d.name, "Abyssal Depths");
        assert!(is_wet_stone_earth(d.ground));
        assert!(is_wet_stone_earth(d.stone));
        assert!(is_teal_peace(d.node));
        assert!(d.fog_end > d.fog_start);
        assert!(climate_dress_copy_is_honest(d.name));
        let lower = d.name.to_ascii_lowercase();
        assert!(!lower.contains("brood"));
        assert!(!lower.contains("market"));
        assert!(!lower.contains("online"));
        assert!(!lower.contains("gold"));
        assert!(!lower.contains("portal"));
        assert!(!lower.contains("instance"));
        assert!(!lower.contains("sanctuary"));
        assert!(!lower.contains("heartwood"));
        assert!(!lower.contains("auction"));
    }

    /// CARD FLESH-DEPTHS-DRESS — wet-stone night fog, teal Peace still the accent.
    /// Ground and path stones stay the walked F4 family. Comfort L/M/H. No Ultra.
    /// Quieter bed stays the banked constant (peace_audio). No mixer rewrite.
    #[test]
    fn flesh_depths_dress_is_wet_stone_fog_with_teal_peace() {
        let d = look_for(Some(3));
        let s = look_for(Some(0));
        let h = look_for(Some(2));
        let t = look_for(Some(1));
        assert_eq!(d.name, "Abyssal Depths");
        assert_eq!(srgb3(d.sky), srgb3(DEPTHS_WET_SKY));
        assert_eq!(srgb3(d.fog), srgb3(DEPTHS_WET_FOG));
        assert!(
            is_wet_stone_earth(d.fog),
            "Depths fog must read wet-stone, got {:?}",
            srgb3(d.fog)
        );
        assert!(
            is_wet_stone_earth(d.sky),
            "Depths sky must stay in the wet-stone family, got {:?}",
            srgb3(d.sky)
        );
        assert!(is_wet_stone_earth(d.ground));
        assert!(is_wet_stone_earth(d.stone));
        assert!(is_teal_peace(d.node));
        assert_eq!(srgb3(d.node), srgb3(DEPTHS_TEAL_PEACE));
        assert_eq!(srgb3(d.ground), (0.04, 0.08, 0.09));
        assert_eq!(srgb3(d.stone), (0.08, 0.15, 0.17));
        assert_eq!(srgb3(d.ambient), (0.18, 0.42, 0.38));
        assert_eq!(d.roughness, DEPTHS_WET_STONE_ROUGHNESS);
        // Graphite yard, living-wood, and pipe-air iron are other families.
        assert!(!is_graphite_warm_earth(d.fog));
        assert!(!is_graphite_warm_earth(d.sky));
        assert!(!is_warm_yard_earth(d.fog));
        assert!(!is_warm_yard_earth(d.sky));
        assert!(!is_living_wood_earth(d.fog));
        assert!(!is_living_wood_earth(d.sky));
        assert!(!is_pipe_edge_iron(d.fog));
        assert!(!is_pipe_edge_iron(d.sky));
        assert_ne!(srgb3(d.fog), srgb3(s.fog));
        assert_ne!(srgb3(d.fog), srgb3(h.fog));
        assert_ne!(srgb3(d.fog), srgb3(t.fog));
        assert_ne!(srgb3(d.sky), srgb3(s.sky));
        assert_ne!(srgb3(d.sky), srgb3(h.sky));
        assert_ne!(srgb3(d.sky), srgb3(t.sky));
        assert!(!is_warm_gold_well(d.node));
        assert!(!is_amber_lamp(d.node));
        assert!(!is_tend_seam(d.node));
        let (fr, fg, fb) = srgb3(d.fog);
        let (nr, ng, nb) = srgb3(d.node);
        let fog_lum = (fr + fg + fb) / 3.0;
        let teal_lum = (nr + ng + nb) / 3.0;
        assert!(
            teal_lum > fog_lum + 0.20,
            "teal Peace must read above wet-stone fog ({teal_lum} vs {fog_lum})"
        );
        // Night close fog stays named. Closer than the open yards — do not fight living_day.
        assert!((d.fog_start - DEPTHS_FOG_START).abs() < f32::EPSILON);
        assert!((d.fog_end - DEPTHS_FOG_END).abs() < f32::EPSILON);
        assert!(d.fog_end <= 20.0);
        assert!(d.fog_end < s.fog_end);
        assert!(d.fog_end < h.fog_end);
        assert!(d.fog_end < t.fog_end);
        assert!(d.fog_start < s.fog_start);
        assert!(d.fog_start < h.fog_start);
        assert!(d.fog_start < t.fog_start);
        let bed = weather_bed_for(Some(3), WeatherFidelity::Medium);
        assert_eq!(bed.mood, PlaceMood::DepthsWetStone);
        assert_eq!(srgb3(bed.fog), srgb3(d.fog));
        assert_eq!(srgb3(bed.sky), srgb3(d.sky));
        assert_eq!(bed.fog_start, d.fog_start);
        assert_eq!(bed.fog_end, d.fog_end);
        assert_eq!(
            arrival_fog_for_landing(PeopleLanding::DepthsTealWayHome).color,
            d.fog
        );
        // Banked quieter bed. Fog dress does not retune the mixer.
        assert!(shared::peace_audio::BED_GAIN_DEPTHS > 0.0);
        assert!(shared::peace_audio::BED_GAIN_DEPTHS < shared::peace_audio::BED_GAIN_OPEN);
        assert_eq!(
            well_glow_scale_for_place(PlaceId::Depths),
            PLACE_WELL_GLOW_SCALE
        );
        assert_eq!(GraphicsPreset::ALL.len(), 3);
        for preset in GraphicsPreset::ALL {
            assert!(!preset.label().contains("Ultra"));
            assert!(place_dress_still_readable(place_dress_lod_scale(preset)));
        }
        // F1 / F2 / F3 dress stays intact under F4 fog.
        // Sanctuary graphite is the earth (ground + stone). Sky and fog stay
        // the named warm-yard graphite family, brighter than that earth band.
        assert!(is_graphite_warm_earth(s.ground));
        assert!(is_graphite_warm_earth(s.stone));
        assert_eq!(srgb3(s.sky), srgb3(SANCTUARY_GRAPHITE_SKY));
        assert_eq!(srgb3(s.fog), srgb3(SANCTUARY_GRAPHITE_FOG));
        assert!(is_warm_yard_earth(s.fog));
        assert!(is_warm_yard_earth(s.sky));
        assert!(is_living_wood_earth(h.ground));
        assert!(is_living_wood_earth(h.fog));
        assert!(is_living_wood_earth(h.sky));
        assert!(is_pipe_edge_iron(t.ground));
        assert!(is_pipe_edge_iron(t.fog));
        assert!(is_pipe_edge_iron(t.sky));
        assert!(!is_wet_stone_earth(s.fog));
        assert!(!is_wet_stone_earth(h.fog));
        assert!(!is_wet_stone_earth(t.fog));
    }

    #[test]
    fn depths_teal_peace_refuses_prior_place_accents() {
        let d = look_for(Some(3));
        let s = look_for(Some(0));
        let h = look_for(Some(2));
        let t = look_for(Some(1));
        assert_eq!(srgb3(d.node), srgb3(DEPTHS_TEAL_PEACE));
        assert_eq!(srgb3(s.node), srgb3(SANCTUARY_WELL_GOLD));
        assert_eq!(srgb3(h.node), srgb3(HEARTWOOD_AMBER_LAMP));
        assert_eq!(srgb3(t.node), srgb3(THRESHOLD_TEND_SEAM));
        assert_ne!(srgb3(DEPTHS_TEAL_PEACE), srgb3(SANCTUARY_WELL_GOLD));
        assert_ne!(srgb3(DEPTHS_TEAL_PEACE), srgb3(HEARTWOOD_AMBER_LAMP));
        assert_ne!(srgb3(DEPTHS_TEAL_PEACE), srgb3(THRESHOLD_TEND_SEAM));
        assert!(is_teal_peace(d.node));
        assert!(is_warm_gold_well(s.node));
        assert!(is_amber_lamp(h.node));
        assert!(is_tend_seam(t.node));
        assert!(!is_teal_peace(s.node));
        assert!(!is_teal_peace(h.node));
        assert!(!is_teal_peace(t.node));
        assert!(!is_warm_gold_well(d.node));
        assert!(!is_amber_lamp(d.node));
        assert!(!is_tend_seam(d.node));
        assert!(climate_dress_copy_is_honest(d.name));
        // F1 / F2 / F3 dress stays intact under F4.
        assert!(is_warm_yard_earth(s.ground));
        assert!(is_living_wood_earth(h.ground));
        assert!(is_pipe_edge_iron(t.ground));
        assert_eq!(s.roughness, SANCTUARY_YARD_ROUGHNESS);
        assert_eq!(h.roughness, HEARTWOOD_WOOD_ROUGHNESS);
        assert_eq!(t.roughness, THRESHOLD_PIPE_ROUGHNESS);
    }

    #[test]
    fn place_mood_maps_four_places() {
        assert_eq!(place_mood_for(Some(0)), PlaceMood::SanctuarySkyYard);
        assert_eq!(place_mood_for(None), PlaceMood::SanctuarySkyYard);
        assert_eq!(place_mood_for(Some(2)), PlaceMood::HeartwoodCanopy);
        assert_eq!(place_mood_for(Some(1)), PlaceMood::ThresholdPipeAir);
        assert_eq!(place_mood_for(Some(4)), PlaceMood::ThresholdPipeAir);
        assert_eq!(place_mood_for(Some(3)), PlaceMood::DepthsWetStone);
        assert_eq!(PlaceMood::SanctuarySkyYard.mood_label(), "sky/yard");
        assert_eq!(PlaceMood::HeartwoodCanopy.mood_label(), "canopy");
        assert_eq!(PlaceMood::ThresholdPipeAir.mood_label(), "pipe air");
        assert_eq!(PlaceMood::DepthsWetStone.mood_label(), "wet stone");
        for mood in [
            PlaceMood::SanctuarySkyYard,
            PlaceMood::HeartwoodCanopy,
            PlaceMood::ThresholdPipeAir,
            PlaceMood::DepthsWetStone,
        ] {
            let label = mood.mood_label();
            assert!(climate_dress_copy_is_honest(label));
            assert!(!label.to_ascii_lowercase().contains("online"));
            assert!(!label.to_ascii_lowercase().contains("market"));
            assert!(!label.contains("http"));
        }
    }

    #[test]
    fn weather_bed_follows_place_and_fidelity() {
        use shared::local_settings::GraphicsPreset;

        let mid = weather_bed_for(Some(0), WeatherFidelity::Medium);
        assert_eq!(mid.mood, PlaceMood::SanctuarySkyYard);
        assert_eq!(srgb3(mid.fog), srgb3(look_for(Some(0)).fog));
        assert_eq!(srgb3(mid.sky), srgb3(look_for(Some(0)).sky));

        let canopy = weather_bed_for(Some(2), WeatherFidelity::Medium);
        assert_eq!(canopy.mood, PlaceMood::HeartwoodCanopy);
        assert_ne!(srgb3(mid.fog), srgb3(canopy.fog));

        let pipe = weather_bed_for(Some(1), WeatherFidelity::Medium);
        assert_eq!(pipe.mood, PlaceMood::ThresholdPipeAir);

        let wet = weather_bed_for(Some(3), WeatherFidelity::Medium);
        assert_eq!(wet.mood, PlaceMood::DepthsWetStone);
        assert!(wet.fog_end <= 20.0);

        let low = weather_bed_for(Some(0), WeatherFidelity::Low);
        let high = weather_bed_for(Some(0), WeatherFidelity::High);
        assert!(low.breath_amp < mid.breath_amp);
        assert!(mid.breath_amp < high.breath_amp);
        assert!(low.tint_strength < mid.tint_strength);
        assert!(mid.tint_strength < high.tint_strength);
        assert!(WeatherFidelity::Low.gentler());
        assert!(WeatherFidelity::High.richer());

        // GraphicsPreset gates fidelity 1:1 (Comfort).
        assert_eq!(
            GraphicsPreset::Low.weather_fidelity(),
            WeatherFidelity::Low
        );
        assert_eq!(
            GraphicsPreset::Medium.weather_fidelity(),
            WeatherFidelity::Medium
        );
        assert_eq!(
            GraphicsPreset::High.weather_fidelity(),
            WeatherFidelity::High
        );
    }

    /// Same-Place Low bed is strictly gentler: later fog, farther horizon,
    /// slower breath, smaller amplitude. Dress color token still matches.
    fn low_bed_gentler_than_medium(low: &WeatherBed, mid: &WeatherBed) -> bool {
        low.mood == mid.mood
            && srgb3(low.fog) == srgb3(mid.fog)
            && srgb3(low.sky) == srgb3(mid.sky)
            && low.fog_start > mid.fog_start
            && low.fog_end > mid.fog_end
            && low.fog_end > low.fog_start
            && low.breath_hz < mid.breath_hz
            && low.breath_amp < mid.breath_amp
            && low.tint_strength < mid.tint_strength
            && low.tint_strength > 0.0
    }

    /// Medium and High keep the FLESH dress distances and fog token.
    /// High breath stays richer than Medium. No fourth preset.
    fn medium_high_hold_dressed_fog(realm: Option<u8>) -> bool {
        let look = look_for(realm);
        let mid = weather_bed_for(realm, WeatherFidelity::Medium);
        let high = weather_bed_for(realm, WeatherFidelity::High);
        mid.fog_start == look.fog_start
            && mid.fog_end == look.fog_end
            && high.fog_start == look.fog_start
            && high.fog_end == look.fog_end
            && mid.breath_hz == high.breath_hz
            && mid.breath_amp < high.breath_amp
            && mid.tint_strength < high.tint_strength
            && mid.fog == look.fog
            && high.fog == look.fog
            && mid.bed_density == 1.0
            && high.bed_density == 1.0
            && srgb3(high.sky) == srgb3(look.sky)
    }

    /// CARD OPT-COMFORT-LOW — GraphicsPreset::Low / WeatherFidelity::Low
    /// fog and breath stay strictly gentler than Medium on dressed beds.
    /// Depths is the closest FLESH night fog; Sanctuary is the open yard.
    /// Medium / High identity holds. No Ultra.
    #[test]
    fn comfort_low_fog_breath_caps_gentler_than_medium_dress() {
        let low = weather_bed_for(Some(3), GraphicsPreset::Low.weather_fidelity());
        let mid = weather_bed_for(Some(3), WeatherFidelity::Medium);
        let high = weather_bed_for(Some(3), WeatherFidelity::High);
        assert_eq!(GraphicsPreset::Low.weather_fidelity(), WeatherFidelity::Low);
        assert!(WeatherFidelity::Low.gentler());
        assert!(low_bed_gentler_than_medium(&low, &mid));
        assert!(low.breath_amp < high.breath_amp);
        assert!(low.breath_hz < high.breath_hz);
        assert!(low.fog_start > high.fog_start);
        assert!(low.fog_end > high.fog_end);
        assert!(is_wet_stone_earth(low.fog));
        assert!(is_teal_peace(look_for(Some(3)).node));
        // Dressed night stays close on Medium. Low opens past that ramp.
        assert!(mid.fog_end <= 20.0);
        assert!(low.fog_end > 20.0);
        assert!(low.fog_start >= LOW_FOG_START_FLOOR);
        assert!(low.fog_end >= LOW_FOG_END_FLOOR);
        assert!(low.breath_hz <= LOW_BREATH_HZ_CAP);
        assert!(low.breath_amp <= LOW_BREATH_AMP_CAP);
        assert!(medium_high_hold_dressed_fog(Some(3)));

        let yard_low = weather_bed_for(Some(0), GraphicsPreset::Low.weather_fidelity());
        let yard_mid = weather_bed_for(Some(0), WeatherFidelity::Medium);
        assert!(low_bed_gentler_than_medium(&yard_low, &yard_mid));
        assert!(is_warm_yard_earth(yard_low.fog));
        assert!(is_warm_gold_well(look_for(Some(0)).node));
        assert!(medium_high_hold_dressed_fog(Some(0)));
        assert!(medium_high_hold_dressed_fog(Some(2)));
        assert!(medium_high_hold_dressed_fog(Some(1)));

        assert_eq!(GraphicsPreset::ALL.len(), 3);
        assert_eq!(WeatherFidelity::ALL.len(), 3);
        for preset in GraphicsPreset::ALL {
            assert!(!preset.label().contains("Ultra"));
            assert!(!preset.weather_fidelity().feel_label().contains("Ultra"));
        }
        assert!(!WeatherFidelity::Low.feel_label().contains("Ultra"));
    }

    /// CARD OPT-WEATHER-FIDELITY-LOW — Low mist-bed density is capped.
    /// Medium and High keep dressed fog color (alpha included) and density 1.
    /// Place RGB tokens stay. No particle emitter. No Ultra.
    #[test]
    fn comfort_low_bed_density_thins_mist_medium_high_stay_full() {
        for realm in [Some(0), Some(1), Some(2), Some(3), Some(4)] {
            let look = look_for(realm);
            let low = weather_bed_for(realm, WeatherFidelity::Low);
            let mid = weather_bed_for(realm, WeatherFidelity::Medium);
            let high = weather_bed_for(realm, WeatherFidelity::High);
            assert!((low.bed_density - LOW_BED_DENSITY_CAP).abs() < f32::EPSILON);
            assert!((mid.bed_density - 1.0).abs() < f32::EPSILON);
            assert!((high.bed_density - 1.0).abs() < f32::EPSILON);
            assert!(low.bed_density < mid.bed_density);
            assert_eq!(mid.bed_density, high.bed_density);
            assert_eq!(mid.fog, look.fog);
            assert_eq!(high.fog, look.fog);
            assert_eq!(srgb3(low.fog), srgb3(look.fog));
            let dressed_alpha = look.fog.to_srgba().alpha;
            let low_alpha = low.fog.to_srgba().alpha;
            assert!((low_alpha - dressed_alpha * LOW_BED_DENSITY_CAP).abs() < 1e-5);
            assert!(low_alpha < mid.fog.to_srgba().alpha);
            assert_eq!(mid.fog_start, look.fog_start);
            assert_eq!(mid.fog_end, look.fog_end);
            assert_eq!(high.fog_start, look.fog_start);
            assert_eq!(high.fog_end, look.fog_end);
        }
        assert_eq!(WeatherFidelity::ALL.len(), 3);
        assert_eq!(GraphicsPreset::ALL.len(), 3);
        assert!(!WeatherFidelity::Low.feel_label().contains("Ultra"));
    }

    #[test]
    fn flow_band_couples_to_place_mood() {
        let bed = weather_bed_for(Some(2), WeatherFidelity::Medium);
        let rise = coupled_breath_amp(&bed, WeatherBandKind::Rise);
        let flow = coupled_breath_amp(&bed, WeatherBandKind::Flow);
        let bore = coupled_breath_amp(&bed, WeatherBandKind::Boredom);
        assert!(flow > rise);
        assert!(bore < rise);
        // Depths + Anxiety tightens (higher mul on wet stone).
        let depths = weather_bed_for(Some(3), WeatherFidelity::Medium);
        let d_rise = place_band_mul(depths.mood, WeatherBandKind::Rise);
        let d_anx = place_band_mul(depths.mood, WeatherBandKind::Anxiety);
        assert!(d_anx > d_rise * 0.8);
        assert!(place_band_mul(PlaceMood::HeartwoodCanopy, WeatherBandKind::Flow) > 1.2);
    }

    #[test]
    fn weather_beds_refuse_live_earth_and_second_hud() {
        // Honesty: mood / fidelity labels never imply Online, sockets, or live Earth API.
        for fidelity in WeatherFidelity::ALL {
            let feel = fidelity.feel_label();
            let lower = feel.to_ascii_lowercase();
            assert!(!lower.contains("online"));
            assert!(!lower.contains("socket"));
            assert!(!lower.contains("earth api"));
            assert!(!feel.contains("http"));
            assert!(!feel.contains("ws://"));
        }
        for realm in [None, Some(0), Some(1), Some(2), Some(3), Some(4)] {
            let bed = weather_bed_for(realm, WeatherFidelity::Medium);
            assert!(climate_dress_copy_is_honest(bed.mood.mood_label()));
        }
        // Default plane boots Sanctuary mood — Peace boot, no network door.
        let plane = ClimatePlane::default();
        assert_eq!(plane.mood, PlaceMood::SanctuarySkyYard);
        assert!(plane.applied.is_none());
        let coupling = WeatherBandCoupling::default();
        assert_eq!(coupling.band, WeatherBandKind::Rise);
    }

    #[test]
    fn comfort_mesh_lod_plan_scales_place_dress_low_stays_readable() {
        assert!((place_dress_lod_scale(GraphicsPreset::Low) - 0.65).abs() < f32::EPSILON);
        assert!((place_dress_lod_scale(GraphicsPreset::Medium) - 1.0).abs() < f32::EPSILON);
        assert!((place_dress_lod_scale(GraphicsPreset::High) - 1.15).abs() < f32::EPSILON);
        assert!(place_dress_low_stays_readable());
        assert!(place_dress_still_readable(0.65));
        assert!(place_dress_still_readable(1.0));
        assert!(place_dress_still_readable(1.15));
        assert!(!place_dress_still_readable(0.4));

        let low = mesh_lod::plan_for_preset(GraphicsPreset::Low);
        assert!(low.primitives_only);
        assert!(!mesh_lod::place_dress_uses_authored_glb(&low));
        assert!(!place_dress_dumps_glb(GraphicsPreset::Low));
        assert!(!place_dress_dumps_glb(GraphicsPreset::Medium));
        assert!(!place_dress_dumps_glb(GraphicsPreset::High));
        assert_eq!(
            place_dress_lod_scale(GraphicsPreset::Low),
            mesh_lod::lived_presence_detail_scale(&low)
        );
        // Palettes (well glow) unchanged at every Comfort tier — E still finds the well.
        for preset in GraphicsPreset::ALL {
            assert!(place_dress_still_readable(place_dress_lod_scale(preset)));
            assert_eq!(srgb3(look_for(Some(0)).node), srgb3(SANCTUARY_WELL_GOLD));
            assert_eq!(NODE_ANCHORS.len(), 3);
        }
    }

    #[test]
    fn comfort_place_lod_does_not_open_fifth_place_race_lobby_or_ultra() {
        assert_eq!(look_for(Some(0)).name, "Sanctuary Prime");
        assert_eq!(look_for(Some(2)).name, "Verdant Heartwood");
        assert_eq!(look_for(Some(1)).name, "Crystal Spires");
        assert_eq!(look_for(Some(3)).name, "Abyssal Depths");
        assert_eq!(look_for(Some(4)).name, "Voidfarer Horizon");
        assert_ne!(look_for(Some(4)).name, "Market");
        assert!(mesh_lod::face_is_not_class());
        assert!(mesh_lod::practices_after_house());
        assert!(mesh_lod::race_lobby_closed());
        assert_eq!(GraphicsPreset::ALL.len(), 3);
        for preset in GraphicsPreset::ALL {
            let label = mesh_lod::lod_feel_label(mesh_lod::mesh_lod_for_preset(preset));
            assert!(!label.contains(".glb"), "{label}");
            assert!(!label.contains("Ultra"), "{label}");
            assert!(!label.contains("race lobby"), "{label}");
            assert!(!preset.label().contains("Ultra"));
            assert!(climate_dress_copy_is_honest(look_for(Some(0)).name));
            let feel = crate::local_settings::LocalMeshLodFeel { preset };
            assert_eq!(feel.place_dress_lod(), mesh_lod::mesh_lod_for_preset(preset));
        }
    }

    /// CARD L4 — Human land → Sanctuary dress token / PlaceId::Sanctuary.
    #[test]
    fn human_land_sanctuary_dress_token() {
        let land = PeopleLanding::SanctuaryYard;
        assert_eq!(land.place_id(), PlaceId::Sanctuary);
        assert_eq!(dress_token_for_landing(land), "Sanctuary Prime");
        assert_eq!(dress_token_for_place(PlaceId::Sanctuary), "Sanctuary Prime");
        assert_eq!(dress_mood_for_place(PlaceId::Sanctuary), PlaceMood::SanctuarySkyYard);
        assert_eq!(dress_realm_for_place(PlaceId::Sanctuary), Some(0));
        assert!(is_warm_gold_well(look_for(Some(0)).node));
        assert_eq!(srgb3(look_for(Some(0)).node), srgb3(SANCTUARY_WELL_GOLD));
    }

    /// CARD L4 — Cydruid land → Heartwood dress (amber lamp). C0 person stays human-in-frame.
    #[test]
    fn cydruid_land_heartwood_dress_token() {
        let land = PeopleLanding::Heartwood;
        assert_eq!(land.place_id(), PlaceId::Heartwood);
        assert_eq!(dress_token_for_landing(land), "Verdant Heartwood");
        assert_eq!(dress_token_for_place(PlaceId::Heartwood), "Verdant Heartwood");
        assert_eq!(dress_mood_for_place(PlaceId::Heartwood), PlaceMood::HeartwoodCanopy);
        assert!(is_amber_lamp(look_for(Some(2)).node));
        assert_eq!(
            crate::hour_sacred::HousePeople::Cydruid.people_line(),
            "Cydruid · human-in-frame"
        );
    }

    /// CARD L4 — Quellorian land → Heartwood PlaceId + Threshold shelf dress/reach.
    #[test]
    fn quellorian_land_heartwood_place_and_threshold_shelf_reach() {
        let land = PeopleLanding::Threshold;
        assert_eq!(land.place_id(), PlaceId::Heartwood);
        assert_eq!(
            dress_token_for_landing(land),
            dress_token_for_place(PlaceId::Heartwood)
        );
        let [x, _, z] = shared::threshold_shelf::THRESHOLD_SHELF_CENTER;
        assert!(shared::threshold_shelf::threshold_use_in_reach(
            PlaceId::Heartwood,
            x,
            z
        ));
        assert!(!shared::threshold_shelf::threshold_use_in_reach(
            PlaceId::Sanctuary,
            x,
            z
        ));
        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
    }

    /// CARD L4 — Draek land → Depths dress. DepthsPeaceTend restore-not-Take stays on Depths hex.
    #[test]
    fn draek_land_depths_dress_token() {
        let land = PeopleLanding::DepthsTealWayHome;
        assert_eq!(land.place_id(), PlaceId::Depths);
        assert_eq!(dress_token_for_landing(land), "Abyssal Depths");
        assert_eq!(dress_token_for_place(PlaceId::Depths), "Abyssal Depths");
        assert_eq!(dress_mood_for_place(PlaceId::Depths), PlaceMood::DepthsWetStone);
        assert!(is_teal_peace(look_for(Some(3)).node));
        assert_eq!(srgb3(look_for(Some(3)).node), srgb3(DEPTHS_TEAL_PEACE));
    }

    /// CARD L4 — Ambrosian land → Sanctuary PlaceId (same as Human). No new hex.
    #[test]
    fn ambrosian_land_sanctuary_dress_token_same_as_human() {
        let amb = PeopleLanding::SanctuaryWellFromAbove;
        let human = PeopleLanding::SanctuaryYard;
        assert_eq!(amb.place_id(), PlaceId::Sanctuary);
        assert_eq!(amb.place_id(), human.place_id());
        assert_eq!(dress_token_for_landing(amb), dress_token_for_landing(human));
        assert_eq!(dress_token_for_landing(amb), "Sanctuary Prime");
        assert_ne!(amb, human);
    }

    /// CARD L7 — Human beat uses SanctuarySkyYard / warm-gold well already on tip.
    #[test]
    fn l7_human_arrival_fog_is_sanctuary_sky_yard() {
        let fog = arrival_fog_for_landing(PeopleLanding::SanctuaryYard);
        let look = look_for(Some(0));
        assert_eq!(fog.color, look.fog);
        assert_eq!(fog.start, look.fog_start);
        assert_eq!(fog.end, look.fog_end);
        assert_eq!(dress_mood_for_place(PlaceId::Sanctuary), PlaceMood::SanctuarySkyYard);
        assert!(is_warm_gold_well(look.node));
        assert!(arrival_garden_title_light_yields(PeopleLanding::SanctuaryYard));
        assert_eq!(dress_token_for_landing(PeopleLanding::SanctuaryYard), "Sanctuary Prime");
    }

    /// CARD L7 — Ambrosian high fog is thinner / brighter on the same Sanctuary disk.
    #[test]
    fn l7_ambrosian_arrival_fog_thinner_brighter_same_place() {
        let human = arrival_fog_for_landing(PeopleLanding::SanctuaryYard);
        let amb = arrival_fog_for_landing(PeopleLanding::SanctuaryWellFromAbove);
        assert_eq!(
            PeopleLanding::SanctuaryWellFromAbove.place_id(),
            PeopleLanding::SanctuaryYard.place_id()
        );
        assert!(arrival_fog_is_thinner_brighter(amb, human));
        assert!(arrival_garden_title_light_yields(
            PeopleLanding::SanctuaryWellFromAbove
        ));
        assert_eq!(
            dress_token_for_landing(PeopleLanding::SanctuaryWellFromAbove),
            dress_token_for_landing(PeopleLanding::SanctuaryYard)
        );
    }

    /// CARD L7 — Cydruid / Draek / Quellorian reuse existing Place fog tokens.
    #[test]
    fn l7_cydruid_draek_quellorian_existing_place_fog() {
        let canopy = arrival_fog_for_landing(PeopleLanding::Heartwood);
        let heart = look_for(Some(2));
        assert_eq!(canopy.color, heart.fog);
        assert!(is_amber_lamp(heart.node));
        let depths = arrival_fog_for_landing(PeopleLanding::DepthsTealWayHome);
        let wet = look_for(Some(3));
        assert_eq!(depths.color, wet.fog);
        assert!(is_teal_peace(wet.node));
        assert_eq!(
            arrival_fog_for_landing(PeopleLanding::Threshold).color,
            heart.fog
        );
        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
    }

    /// CARD L4 — HexTravelState PlaceId turns on the existing look_for realm.
    #[test]
    fn travel_place_id_turns_on_existing_place_dress_realm() {
        use crate::hex_travel::HexTravelState;

        let mut app = App::new();
        app.init_resource::<SoftPlayerRealm>();
        app.insert_resource(HexTravelState {
            current: PlaceId::Sanctuary,
        });
        app.add_systems(Update, sync_place_dress_from_travel);
        app.update();
        assert_eq!(
            app.world().resource::<SoftPlayerRealm>().current,
            Some(0),
            "Sanctuary dress realm"
        );

        app.world_mut()
            .resource_mut::<HexTravelState>()
            .current = PlaceId::Heartwood;
        app.update();
        assert_eq!(
            app.world().resource::<SoftPlayerRealm>().current,
            Some(2),
            "Heartwood dress realm"
        );

        app.world_mut()
            .resource_mut::<HexTravelState>()
            .current = PlaceId::Depths;
        app.update();
        assert_eq!(
            app.world().resource::<SoftPlayerRealm>().current,
            Some(3),
            "Depths dress realm"
        );
    }

    /// CARD F5 — unsealed light may cross door → existing L7 fog cited · still unsealed until E/Q.
    #[test]
    fn f5_unsealed_light_may_cross_door_l7_beat_armed_still_unsealed_until_eq() {
        use crate::hour_sacred::{
            confirm_gate_seal, still_unsealed_until_eq, soul_is_light, HousePeople,
        };
        use crate::human_presence::{arrival_beat_after_land, run_arrival_beat};
        use shared::local_settings::PeaceKey;

        let beat = run_arrival_beat(PeopleLanding::SanctuaryYard);
        assert!(beat.armed);
        assert!(arrival_beat_after_land(Some(PeopleLanding::SanctuaryYard)).armed);
        let fog = arrival_fog_for_landing(PeopleLanding::SanctuaryYard);
        let look = look_for(Some(0));
        assert_eq!(fog.color, look.fog);
        assert!(!wrong_door_bounce_recooks_l7_fog());
        assert!(still_unsealed_until_eq(None));
        assert!(soul_is_light(None));
        let pending = Some((HousePeople::Human, PeopleLanding::SanctuaryYard));
        assert!(confirm_gate_seal(PeaceKey::Digit1, pending).is_none());
        let sealed = confirm_gate_seal(PeaceKey::E, pending).expect("E");
        assert!(!still_unsealed_until_eq(Some(sealed)));
    }

    /// CARD F5 — decline / wrong door → garden dress · not a fifth Place · L7 fog unread.
    #[test]
    fn f5_decline_wrong_door_garden_light_not_sealed_place_id_garden() {
        assert_eq!(
            garden_bounce_dress_token(PlaceId::Sanctuary),
            "Sanctuary Prime"
        );
        assert_eq!(
            garden_bounce_dress_token(PlaceId::Sanctuary),
            dress_token_for_place(PlaceId::Sanctuary)
        );
        assert_eq!(
            dress_mood_for_place(PlaceId::Sanctuary),
            PlaceMood::SanctuarySkyYard
        );
        assert!(!wrong_door_bounce_recooks_l7_fog());
        assert_ne!(garden_bounce_dress_token(PlaceId::Sanctuary), "Garden");
        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
    }

    /// CARD F5 — Peace recall does not clear seal / vision home for light.
    #[test]
    fn f5_peace_recall_does_not_clear_seal_vision_home_for_light() {
        use crate::hour_sacred::{peace_recall, HousePeople, PeopleLanding, PEACE_RECALL_VISION_HOME};

        let sealed = Some((HousePeople::Cydruid, PeopleLanding::Heartwood));
        let (after, line) = peace_recall(sealed);
        assert_eq!(after, sealed);
        assert_eq!(line, PEACE_RECALL_VISION_HOME);
        let (light_after, light_line) = peace_recall(None);
        assert!(light_after.is_none());
        assert_eq!(light_line, "vision home");
    }

    /// CARD F5 — PlaceId / LOCAL_HEXES len == 3.
    #[test]
    fn f5_place_id_local_hexes_len_three() {
        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
        assert_eq!(
            crate::hour_sacred::PeopleLanding::Threshold.place_id(),
            PlaceId::Heartwood
        );
    }

    /// CARD F5 — STEWARD_ONLINE_YES false.
    #[test]
    fn f5_steward_online_yes_false() {
        use shared::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};

        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(!shared::hex_protocol::default_client_listens());
    }

    /// CARD F5 — no fifth Place · no Title race lobby.
    #[test]
    fn f5_no_fifth_place_no_title_race_lobby() {
        use crate::hour_sacred::{f5_title_is_race_lobby, garden_roster_is_race_portrait_lobby};

        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
        assert_eq!(look_for(Some(0)).name, "Sanctuary Prime");
        assert_eq!(look_for(Some(2)).name, "Verdant Heartwood");
        assert_eq!(look_for(Some(1)).name, "Crystal Spires");
        assert_eq!(look_for(Some(3)).name, "Abyssal Depths");
        assert_ne!(look_for(Some(4)).name, "Market");
        assert!(!f5_title_is_race_lobby());
        assert!(!garden_roster_is_race_portrait_lobby());
        assert!(mesh_lod::race_lobby_closed());
        assert!(!wrong_door_bounce_recooks_l7_fog());
    }
}
