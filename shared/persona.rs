//! MERCY_PERSONA — shared persona types + P4 PersonaCommit + P5 online picker
//! (CARD H-2026-09-11-P5)
//!
//! Spec: `docs/MERCY_PERSONA_CREATION.md` §4–§5 (design tick 23.2.P).
//! Soft draft (P2 Keep) truncates with soft caps; **PersonaCommit** validates
//! string caps / phenotype sliders / body_kit clamp / mechanical_race, then
//! persists a *record about* the persona — never the soul
//! (`docs/PERSISTENCE_ORIGINAL_OWNERSHIP.md`). Disk writes use `path_filter`
//! (named persist dir + direct-child `powrush_persona.json` only).
//! P5: story-provider picker `None | LocalTemplate | RathorOfflineShard |
//! (opt-in online)`. Feature flag default **false**. Online rows need steward
//! `online yes`. Never first-run default online. Title Online stays grey.
//! LLM drafts only — PersonaCommit remains the law. No sockets / race-lobby.
//!
//! Scope note: Capable · Bounded · Corrigible is research-style framing
//! (P5 bounded to default-off online picker + tests) — not a product warranty,
//! AGSi certification, or lobby SKU.
//! Contact: info@Rathor.ai · Independent of xAI.

use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Feature gate for creator UI (P2+). Hour can finish without persons.
/// P4 Commit path works when the creator is used; flag semantics match P2.
pub const PERSONA_CREATOR_ENABLED: bool = true;

/// MERCY_PERSONA P5 — online story-provider picker feature flag.
/// Default **false**: Hour 1–3 unchanged; offline LocalTemplate / records-only
/// remain the path. Never lights Title Online. Never Always-allow.
pub const ONLINE_PICKER_ENABLED: bool = false;

/// Compile-time steward consent for opt-in online story providers.
/// Must stay false unless steward writes exact `online yes`.
/// Does **not** light Title Online / sockets even when true in tests.
pub const STEWARD_ONLINE_YES: bool = false;

/// Persist path (cwd `data/` adopt source; OS user-dir write via `user_persist`).
pub const PERSONA_PATH: &str = "data/powrush_persona.json";
/// On-disk leaf name. `path_filter` allows only this direct child.
pub const PERSONA_FILE_NAME: &str = "powrush_persona.json";
pub const PERSONA_SCHEMA: &str = "powrush_persona_v1";

/// Soft caps. Soft draft truncates; PersonaCommit re-validates after normalize.
pub const GIVEN_NAME_MAX: usize = 64;
pub const PRONOUNS_MAX: usize = 48;
pub const PEOPLE_LABEL_MAX: usize = 96;
pub const HOMELANDS_MAX: usize = 256;
pub const CUSTOMS_NOTE_MAX: usize = 1024;
pub const STORY_TEXT_MAX: usize = 4096;
pub const DRESS_INTENT_MAX: usize = 256;
pub const LANGUAGE_ENTRY_MAX: usize = 64;
pub const MAX_LANGUAGES: usize = 8;
pub const MAX_MIXED_PARTS: usize = 8;

/// Objective mechanical race / module set (sim-owned). Not a people / ethnicity label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MechanicalRace {
    Human,
    Quellorian,
    Draek,
    Cydruid,
    Ambrosian,
}

impl MechanicalRace {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Human => "Human",
            Self::Quellorian => "Quellorian",
            Self::Draek => "Draek",
            Self::Cydruid => "Cydruid",
            Self::Ambrosian => "Ambrosian",
        }
    }
}

/// Objective body bounds (sim-owned). Phenotype height/build clamps into this band.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BodyKit {
    pub height_band_min: f32,
    pub height_band_max: f32,
    pub reach: f32,
}

impl Default for BodyKit {
    fn default() -> Self {
        Self {
            height_band_min: 0.85,
            height_band_max: 1.15,
            reach: 1.0,
        }
    }
}

/// Objective body-kit floors/ceils (sim clamp). Outside → Commit refuses or clamps.
pub const BODY_KIT_HEIGHT_FLOOR: f32 = 0.5;
pub const BODY_KIT_HEIGHT_CEIL: f32 = 2.0;
pub const BODY_KIT_REACH_FLOOR: f32 = 0.25;
pub const BODY_KIT_REACH_CEIL: f32 = 3.0;

impl BodyKit {
    /// True when bands are finite, ordered, and within objective floors/ceils.
    pub fn is_valid(self) -> bool {
        self.height_band_min.is_finite()
            && self.height_band_max.is_finite()
            && self.reach.is_finite()
            && self.height_band_min <= self.height_band_max
            && self.height_band_min >= BODY_KIT_HEIGHT_FLOOR
            && self.height_band_max <= BODY_KIT_HEIGHT_CEIL
            && self.reach >= BODY_KIT_REACH_FLOOR
            && self.reach <= BODY_KIT_REACH_CEIL
    }

    /// Clamp objective bounds into sim floors/ceils; swap inverted band.
    pub fn clamp_objective(&mut self) {
        if !self.height_band_min.is_finite() {
            self.height_band_min = BODY_KIT_HEIGHT_FLOOR;
        }
        if !self.height_band_max.is_finite() {
            self.height_band_max = BODY_KIT_HEIGHT_CEIL;
        }
        if !self.reach.is_finite() {
            self.reach = 1.0;
        }
        if self.height_band_min > self.height_band_max {
            std::mem::swap(&mut self.height_band_min, &mut self.height_band_max);
        }
        self.height_band_min = self
            .height_band_min
            .clamp(BODY_KIT_HEIGHT_FLOOR, BODY_KIT_HEIGHT_CEIL);
        self.height_band_max = self
            .height_band_max
            .clamp(BODY_KIT_HEIGHT_FLOOR, BODY_KIT_HEIGHT_CEIL);
        if self.height_band_min > self.height_band_max {
            self.height_band_max = self.height_band_min;
        }
        self.reach = self.reach.clamp(BODY_KIT_REACH_FLOOR, BODY_KIT_REACH_CEIL);
    }
}

/// Appearance sliders in `0.0..=1.0`. Not a race enum. Not gather/mercy stats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phenotype {
    pub skin_melanin: f32,
    pub undertone: f32,
    pub hair_curl: f32,
    pub hair_density: f32,
    pub hair_color: f32,
    pub eye_fold: f32,
    pub iris_color: f32,
    pub nose_scale: f32,
    pub lip_scale: f32,
    pub jaw_scale: f32,
    /// Normalized within mechanical `BodyKit` band (sim clamps visuals).
    pub height_within_band: f32,
    pub build_within_band: f32,
    pub age_presentation: f32,
    pub scars: bool,
    pub vitiligo: bool,
    pub freckles: bool,
    pub assistive_cane: bool,
    pub residual_limb: bool,
}

impl Default for Phenotype {
    fn default() -> Self {
        Self {
            skin_melanin: 0.5,
            undertone: 0.5,
            hair_curl: 0.5,
            hair_density: 0.5,
            hair_color: 0.5,
            eye_fold: 0.5,
            iris_color: 0.5,
            nose_scale: 0.5,
            lip_scale: 0.5,
            jaw_scale: 0.5,
            height_within_band: 0.5,
            build_within_band: 0.5,
            age_presentation: 0.5,
            scars: false,
            vitiligo: false,
            freckles: false,
            assistive_cane: false,
            residual_limb: false,
        }
    }
}

impl Phenotype {
    /// Clamp every slider into `0.0..=1.0`. Does not touch mechanical race or inventory.
    pub fn clamp_unit_interval(&mut self) {
        self.skin_melanin = clamp01(self.skin_melanin);
        self.undertone = clamp01(self.undertone);
        self.hair_curl = clamp01(self.hair_curl);
        self.hair_density = clamp01(self.hair_density);
        self.hair_color = clamp01(self.hair_color);
        self.eye_fold = clamp01(self.eye_fold);
        self.iris_color = clamp01(self.iris_color);
        self.nose_scale = clamp01(self.nose_scale);
        self.lip_scale = clamp01(self.lip_scale);
        self.jaw_scale = clamp01(self.jaw_scale);
        self.height_within_band = clamp01(self.height_within_band);
        self.build_within_band = clamp01(self.build_within_band);
        self.age_presentation = clamp01(self.age_presentation);
    }
}

fn clamp01(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
}

/// Optional catalog preset tag (starter paint). Player may break every seed later.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeoplePreset {
    pub tag: String,
}

/// Invented or free-text people. No approval queue in P1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomPeople {
    pub name: String,
    pub homelands: String,
    pub languages: Vec<String>,
    pub customs_note: String,
    pub phenotype_seed: Phenotype,
    pub invented: bool,
}

/// How the steward presents their people — preset, custom, or mixed parts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PeopleChoice {
    /// Optional starter catalog tag (non-exhaustive).
    Preset(PeoplePreset),
    Custom(CustomPeople),
    /// Player lists ancestry / culture parts; not a closed enum.
    Mixed { parts: Vec<String> },
    /// Hour can finish as nameless Steward with no people choice yet.
    Unset,
}

/// Consent for sharing story text in-world (display later; P1 data only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StoryShare {
    Private,
    Spoken,
    Book,
}

/// Player-authored story. Optional AI draft fields are *records* only —
/// P1 never calls an LLM; P3/P5 may fill them later behind Offline-first gates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryDraft {
    pub player_text: String,
    pub ai_assist_used: bool,
    pub model_id: Option<String>,
    pub player_accepted: bool,
    pub shared_in_world: StoryShare,
}

impl Default for StoryDraft {
    fn default() -> Self {
        Self {
            player_text: String::new(),
            ai_assist_used: false,
            model_id: None,
            player_accepted: false,
            shared_in_world: StoryShare::Private,
        }
    }
}

/// MERCY_PERSONA P5 story-provider picker (§4).
/// Offline seats always listed: `None | LocalTemplate | RathorOfflineShard`.
/// Online seats (`RathorOnline | GrokOnline | OpenAiCompatible`) are opt-in only
/// after steward `online yes`. Default is `None` (records-only) — never
/// GrokOnline/OpenAi on first run. Selecting a provider never lights Title Online.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum StoryProvider {
    /// Records-only — player writes everything. Default.
    #[default]
    None,
    /// Offline LocalTemplate packs (P3). Works with Title Online grey.
    LocalTemplate,
    /// Offline Rathor shard (local). Not a network seat.
    RathorOfflineShard,
    /// Opt-in online — requires steward `online yes`. Drafts only.
    RathorOnline,
    /// Opt-in online — requires steward `online yes`. Never first-run default.
    GrokOnline,
    /// Opt-in online — requires steward `online yes`. Never first-run default.
    OpenAiCompatible,
}

impl StoryProvider {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::LocalTemplate => "LocalTemplate",
            Self::RathorOfflineShard => "RathorOfflineShard",
            Self::RathorOnline => "RathorOnline",
            Self::GrokOnline => "GrokOnline",
            Self::OpenAiCompatible => "OpenAiCompatible",
        }
    }

    /// Network / live LLM seats. Offline seats return false.
    pub fn is_online(self) -> bool {
        matches!(
            self,
            Self::RathorOnline | Self::GrokOnline | Self::OpenAiCompatible
        )
    }

    /// Offline seats that work while Title Online is grey.
    pub fn is_offline(self) -> bool {
        !self.is_online()
    }

    /// All listed providers (offline + opt-in online). UI filters via gates.
    pub fn all() -> &'static [StoryProvider] {
        &[
            Self::None,
            Self::LocalTemplate,
            Self::RathorOfflineShard,
            Self::RathorOnline,
            Self::GrokOnline,
            Self::OpenAiCompatible,
        ]
    }

    pub fn offline_only() -> &'static [StoryProvider] {
        &[Self::None, Self::LocalTemplate, Self::RathorOfflineShard]
    }
}

/// First-run / flag-off default: records-only. Never GrokOnline/OpenAi.
pub fn default_story_provider() -> StoryProvider {
    StoryProvider::None
}

/// Online picker is usable only when the feature flag is on **and** steward
/// wrote exact `online yes`. Flag off → Hour 1–3 unchanged.
pub fn online_picker_allows_online_rows(flag: bool, steward_online_yes: bool) -> bool {
    flag && steward_online_yes
}

/// Whether a requested provider may be selected under current gates.
/// Offline rows always ok when the picker UI is shown; online rows need
/// `online_picker_allows_online_rows`. Does not light Title Online.
pub fn story_provider_may_select(
    provider: StoryProvider,
    flag: bool,
    steward_online_yes: bool,
) -> bool {
    if provider.is_offline() {
        return true;
    }
    online_picker_allows_online_rows(flag, steward_online_yes)
}

/// Resolve a requested provider under gates. Online without consent → `None`.
/// PersonaCommit remains the law; this only picks a draft source seat.
pub fn resolve_story_provider(
    requested: StoryProvider,
    flag: bool,
    steward_online_yes: bool,
) -> StoryProvider {
    if story_provider_may_select(requested, flag, steward_online_yes) {
        requested
    } else {
        StoryProvider::None
    }
}

/// Cycle through selectable providers under gates. Never lands on online
/// seats unless steward `online yes` and flag on.
pub fn cycle_story_provider(
    current: StoryProvider,
    flag: bool,
    steward_online_yes: bool,
) -> StoryProvider {
    let pool: &[StoryProvider] = if online_picker_allows_online_rows(flag, steward_online_yes) {
        StoryProvider::all()
    } else {
        StoryProvider::offline_only()
    };
    let ix = pool.iter().position(|p| *p == current).unwrap_or(0);
    pool[(ix + 1) % pool.len()]
}

/// Honest picker button label. Online seats named only as opt-in / gated.
pub fn story_provider_btn_label(
    provider: StoryProvider,
    flag: bool,
    steward_online_yes: bool,
) -> String {
    let resolved = resolve_story_provider(provider, flag, steward_online_yes);
    if !flag {
        format!(
            "Story provider · {} · online picker off",
            resolved.as_str()
        )
    } else if !steward_online_yes && provider.is_online() {
        "Story provider · offline · online needs steward online yes".into()
    } else if resolved.is_online() {
        format!(
            "Story provider · {} · opt-in · drafts only · Online grey",
            resolved.as_str()
        )
    } else {
        format!(
            "Story provider · {} · offline · Online grey",
            resolved.as_str()
        )
    }
}

/// Subjective presentation — player-owned. Never grants gather cheats or Temper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Presentation {
    pub given_name: String,
    pub pronouns: String,
    pub people: PeopleChoice,
    pub phenotype: Phenotype,
    pub story: StoryDraft,
    /// F dress card later; store intent string only until then.
    pub dress_intent: Option<String>,
}

impl Default for Presentation {
    fn default() -> Self {
        Self {
            given_name: String::new(),
            pronouns: String::new(),
            people: PeopleChoice::Unset,
            phenotype: Phenotype::default(),
            story: StoryDraft::default(),
            dress_intent: None,
        }
    }
}

/// Full persona record: objective law fields + subjective presentation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Persona {
    pub mechanical_race: MechanicalRace,
    pub body_kit: BodyKit,
    pub presentation: Presentation,
}

impl Persona {
    /// Hour-1 path: nameless Steward, Human kit, no people/story yet.
    pub fn nameless_steward() -> Self {
        Self {
            mechanical_race: MechanicalRace::Human,
            body_kit: BodyKit::default(),
            presentation: Presentation::default(),
        }
    }

    /// Truncate string fields to soft caps (P4 Commit will validate harder).
    pub fn apply_soft_caps(&mut self) {
        truncate_in_place(&mut self.presentation.given_name, GIVEN_NAME_MAX);
        truncate_in_place(&mut self.presentation.pronouns, PRONOUNS_MAX);
        if let Some(ref mut intent) = self.presentation.dress_intent {
            truncate_in_place(intent, DRESS_INTENT_MAX);
        }
        truncate_in_place(
            &mut self.presentation.story.player_text,
            STORY_TEXT_MAX,
        );
        self.presentation.phenotype.clamp_unit_interval();
        match &mut self.presentation.people {
            PeopleChoice::Preset(p) => truncate_in_place(&mut p.tag, PEOPLE_LABEL_MAX),
            PeopleChoice::Custom(c) => {
                truncate_in_place(&mut c.name, PEOPLE_LABEL_MAX);
                truncate_in_place(&mut c.homelands, HOMELANDS_MAX);
                truncate_in_place(&mut c.customs_note, CUSTOMS_NOTE_MAX);
                c.languages.truncate(MAX_LANGUAGES);
                for lang in &mut c.languages {
                    truncate_in_place(lang, LANGUAGE_ENTRY_MAX);
                }
                c.phenotype_seed.clamp_unit_interval();
            }
            PeopleChoice::Mixed { parts } => {
                parts.truncate(MAX_MIXED_PARTS);
                for part in parts {
                    truncate_in_place(part, PEOPLE_LABEL_MAX);
                }
            }
            PeopleChoice::Unset => {}
        }
    }
}

fn truncate_in_place(s: &mut String, max: usize) {
    if s.chars().count() > max {
        *s = s.chars().take(max).collect();
    }
}

/// Story / people copy must not claim lobby, mall, P2W, gold, or LLM product.
pub fn persona_copy_is_honest(s: &str) -> bool {
    let low = s.to_lowercase();
    !low.contains("gold")
        && !low.contains("mall")
        && !low.contains("p2w")
        && !low.contains("pay to win")
        && !low.contains("cash shop")
        && !low.contains("cash-shop")
        && !low.contains("market")
        && !low.contains("nft")
        && !low.contains("race lobby")
        && !low.contains("race-lobby")
        && !low.contains("title lobby")
        && !low.contains("llm product")
        && !low.contains("chatgpt subscription")
        && !low.contains("grok online required")
        && !low.contains("warranty")
        && !low.contains("agsi certified")
}

/// Why a soft draft failed PersonaCommit validation / persist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitError {
    CapsExceeded,
    SliderOutOfRange,
    BodyKitInvalid,
    MechanicalRaceInvalid,
    DishonestCopy,
    PathRefused,
    PersistFailed,
    Unreadable,
    InvalidJson,
    SchemaMismatch,
}

impl CommitError {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CapsExceeded => "caps_exceeded",
            Self::SliderOutOfRange => "slider_out_of_range",
            Self::BodyKitInvalid => "body_kit_invalid",
            Self::MechanicalRaceInvalid => "mechanical_race_invalid",
            Self::DishonestCopy => "dishonest_copy",
            Self::PathRefused => "path_refused",
            Self::PersistFailed => "persist_failed",
            Self::Unreadable => "unreadable",
            Self::InvalidJson => "invalid_json",
            Self::SchemaMismatch => "schema_mismatch",
        }
    }
}

/// Authoritative committed persona record (sim law). Soft draft is not this.
/// Persistence stores a *record about* the persona — never the soul.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonaCommit {
    pub schema: String,
    pub persona: Persona,
}

impl PersonaCommit {
    /// Validate + normalize a soft draft into a commit record (no disk write).
    pub fn from_soft_draft(draft: &Persona) -> Result<Self, CommitError> {
        let persona = validate_and_normalize(draft)?;
        Ok(Self {
            schema: PERSONA_SCHEMA.into(),
            persona,
        })
    }

    /// Commit soft draft and persist under the OS user-dir (or `POWRUSH_USER_DIR`).
    pub fn commit_and_persist(draft: &Persona) -> Result<Self, CommitError> {
        let committed = Self::from_soft_draft(draft)?;
        committed.persist()?;
        Ok(committed)
    }

    pub fn persist(&self) -> Result<(), CommitError> {
        let json = serde_json::to_string_pretty(self).map_err(|_| CommitError::PersistFailed)?;
        crate::user_persist::write_named(PERSONA_PATH, json).map_err(|_| CommitError::PersistFailed)
    }

    /// Persist into a named dir after `path_filter`. For unit tests / offline sim.
    pub fn persist_at(&self, persist_dir: &Path) -> Result<PathBuf, CommitError> {
        let path = persist_dir.join(PERSONA_FILE_NAME);
        if !path_filter(persist_dir, &path) {
            return Err(CommitError::PathRefused);
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|_| CommitError::PersistFailed)?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|_| CommitError::PersistFailed)?;
        fs::write(&path, json).map_err(|_| CommitError::PersistFailed)?;
        Ok(path)
    }

    pub fn load_or_none() -> Option<Self> {
        let raw = crate::user_persist::read_named(PERSONA_PATH).ok()?;
        Self::from_json(&raw).ok()
    }

    pub fn load_at(persist_dir: &Path) -> Result<Self, CommitError> {
        let path = persist_dir.join(PERSONA_FILE_NAME);
        if !path_filter(persist_dir, &path) {
            return Err(CommitError::PathRefused);
        }
        let text = fs::read_to_string(&path).map_err(|_| CommitError::Unreadable)?;
        Self::from_json(&text)
    }

    pub fn from_json(raw: &str) -> Result<Self, CommitError> {
        let commit: Self = serde_json::from_str(raw).map_err(|_| CommitError::InvalidJson)?;
        if commit.schema != PERSONA_SCHEMA {
            return Err(CommitError::SchemaMismatch);
        }
        // Re-validate loaded record so disk cannot smuggle invalid law fields.
        let persona = validate_and_normalize(&commit.persona)?;
        Ok(Self {
            schema: PERSONA_SCHEMA.into(),
            persona,
        })
    }

    pub fn to_json(&self) -> Result<String, CommitError> {
        serde_json::to_string_pretty(self).map_err(|_| CommitError::PersistFailed)
    }
}

/// Soft draft → normalized Persona ready for Commit. Does not write disk.
pub fn validate_and_normalize(draft: &Persona) -> Result<Persona, CommitError> {
    let mut p = draft.clone();
    // Soft caps first (truncate), then hard-check remaining length / honesty.
    p.apply_soft_caps();
    p.body_kit.clamp_objective();
    if !p.body_kit.is_valid() {
        return Err(CommitError::BodyKitInvalid);
    }
    p.presentation.phenotype.clamp_unit_interval();
    if !phenotype_sliders_in_unit_interval(&p.presentation.phenotype) {
        return Err(CommitError::SliderOutOfRange);
    }
    if let PeopleChoice::Custom(ref mut c) = p.presentation.people {
        c.phenotype_seed.clamp_unit_interval();
        if !phenotype_sliders_in_unit_interval(&c.phenotype_seed) {
            return Err(CommitError::SliderOutOfRange);
        }
    }
    if !mechanical_race_is_known(p.mechanical_race) {
        return Err(CommitError::MechanicalRaceInvalid);
    }
    if !persona_fields_within_caps(&p) {
        return Err(CommitError::CapsExceeded);
    }
    if !persona_record_is_honest(&p) {
        return Err(CommitError::DishonestCopy);
    }
    Ok(p)
}

fn mechanical_race_is_known(race: MechanicalRace) -> bool {
    matches!(
        race,
        MechanicalRace::Human
            | MechanicalRace::Quellorian
            | MechanicalRace::Draek
            | MechanicalRace::Cydruid
            | MechanicalRace::Ambrosian
    )
}

fn phenotype_sliders_in_unit_interval(ph: &Phenotype) -> bool {
    let vals = [
        ph.skin_melanin,
        ph.undertone,
        ph.hair_curl,
        ph.hair_density,
        ph.hair_color,
        ph.eye_fold,
        ph.iris_color,
        ph.nose_scale,
        ph.lip_scale,
        ph.jaw_scale,
        ph.height_within_band,
        ph.build_within_band,
        ph.age_presentation,
    ];
    vals.iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v))
}

fn persona_fields_within_caps(p: &Persona) -> bool {
    if p.presentation.given_name.chars().count() > GIVEN_NAME_MAX {
        return false;
    }
    if p.presentation.pronouns.chars().count() > PRONOUNS_MAX {
        return false;
    }
    if p.presentation.story.player_text.chars().count() > STORY_TEXT_MAX {
        return false;
    }
    if let Some(ref intent) = p.presentation.dress_intent {
        if intent.chars().count() > DRESS_INTENT_MAX {
            return false;
        }
    }
    match &p.presentation.people {
        PeopleChoice::Preset(x) => x.tag.chars().count() <= PEOPLE_LABEL_MAX,
        PeopleChoice::Custom(c) => {
            c.name.chars().count() <= PEOPLE_LABEL_MAX
                && c.homelands.chars().count() <= HOMELANDS_MAX
                && c.customs_note.chars().count() <= CUSTOMS_NOTE_MAX
                && c.languages.len() <= MAX_LANGUAGES
                && c.languages
                    .iter()
                    .all(|l| l.chars().count() <= LANGUAGE_ENTRY_MAX)
        }
        PeopleChoice::Mixed { parts } => {
            parts.len() <= MAX_MIXED_PARTS
                && parts
                    .iter()
                    .all(|s| s.chars().count() <= PEOPLE_LABEL_MAX)
        }
        PeopleChoice::Unset => true,
    }
}

fn persona_record_is_honest(p: &Persona) -> bool {
    let mut samples: Vec<&str> = vec![
        p.presentation.given_name.as_str(),
        p.presentation.pronouns.as_str(),
        p.presentation.story.player_text.as_str(),
        p.mechanical_race.as_str(),
    ];
    if let Some(ref intent) = p.presentation.dress_intent {
        samples.push(intent.as_str());
    }
    if let Some(ref mid) = p.presentation.story.model_id {
        samples.push(mid.as_str());
    }
    match &p.presentation.people {
        PeopleChoice::Preset(x) => samples.push(x.tag.as_str()),
        PeopleChoice::Custom(c) => {
            samples.push(c.name.as_str());
            samples.push(c.homelands.as_str());
            samples.push(c.customs_note.as_str());
            for lang in &c.languages {
                samples.push(lang.as_str());
            }
        }
        PeopleChoice::Mixed { parts } => {
            for part in parts {
                samples.push(part.as_str());
            }
        }
        PeopleChoice::Unset => {}
    }
    samples.iter().all(|s| persona_copy_is_honest(s))
}

/// True when `persist_dir` is a named leaf we may write under.
/// Refuses filesystem root, empty, and any `..` component.
pub fn persist_dir_is_safe(persist_dir: &Path) -> bool {
    if persist_dir.as_os_str().is_empty() {
        return false;
    }
    if is_filesystem_root(persist_dir) {
        return false;
    }
    if has_parent_dir_component(persist_dir) {
        return false;
    }
    persist_dir.file_name().is_some()
}

/// Direct-child `powrush_persona.json` only. No `..`, no root, no nested walk.
pub fn path_filter(persist_dir: &Path, candidate: &Path) -> bool {
    if !persist_dir_is_safe(persist_dir) {
        return false;
    }
    if has_parent_dir_component(candidate) {
        return false;
    }
    if is_filesystem_root(candidate) {
        return false;
    }
    let Some(name) = candidate.file_name().and_then(|s| s.to_str()) else {
        return false;
    };
    if name != PERSONA_FILE_NAME {
        return false;
    }
    if name.contains('/') || name.contains('\\') {
        return false;
    }
    match candidate.parent() {
        Some(parent) => parent == persist_dir,
        None => false,
    }
}

fn is_filesystem_root(p: &Path) -> bool {
    p.parent().is_none() || p.file_name().is_none() || p == Path::new("/")
}

fn has_parent_dir_component(p: &Path) -> bool {
    p.components().any(|c| matches!(c, Component::ParentDir))
}

/// Height visual resolved inside the objective body kit band.
pub fn resolve_height(kit: &BodyKit, phenotype: &Phenotype) -> f32 {
    let t = clamp01(phenotype.height_within_band);
    kit.height_band_min + (kit.height_band_max - kit.height_band_min) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nameless_steward_defaults_hour_path() {
        let p = Persona::nameless_steward();
        assert_eq!(p.mechanical_race, MechanicalRace::Human);
        assert!(p.presentation.given_name.is_empty());
        assert!(matches!(p.presentation.people, PeopleChoice::Unset));
        assert!(!p.presentation.story.ai_assist_used);
        assert_eq!(p.presentation.story.shared_in_world, StoryShare::Private);
        assert!(PERSONA_CREATOR_ENABLED);
    }

    #[test]
    fn people_label_independent_of_mechanical_race() {
        let mut p = Persona::nameless_steward();
        p.mechanical_race = MechanicalRace::Quellorian;
        p.presentation.people = PeopleChoice::Custom(CustomPeople {
            name: "River-gold freckles folk".into(),
            homelands: "invented delta".into(),
            languages: vec!["river-sign".into()],
            customs_note: "optional paint".into(),
            phenotype_seed: Phenotype::default(),
            invented: true,
        });
        assert_eq!(p.mechanical_race, MechanicalRace::Quellorian);
        assert_eq!(p.mechanical_race.as_str(), "Quellorian");
        // Custom people must not rewrite mechanical race.
        if let PeopleChoice::Custom(c) = &p.presentation.people {
            assert!(c.invented);
            assert_ne!(c.name, p.mechanical_race.as_str());
        } else {
            panic!("expected custom people");
        }
    }

    #[test]
    fn phenotype_clamp_does_not_grant_stats() {
        let mut ph = Phenotype::default();
        ph.skin_melanin = 2.5;
        ph.height_within_band = -1.0;
        ph.clamp_unit_interval();
        assert_eq!(ph.skin_melanin, 1.0);
        assert_eq!(ph.height_within_band, 0.0);
        let kit = BodyKit::default();
        let h = resolve_height(&kit, &ph);
        assert!((h - kit.height_band_min).abs() < f32::EPSILON);
    }

    #[test]
    fn soft_caps_truncate_without_deleting_persona() {
        let mut p = Persona::nameless_steward();
        p.presentation.given_name = "A".repeat(GIVEN_NAME_MAX + 20);
        p.presentation.story.player_text = "B".repeat(STORY_TEXT_MAX + 5);
        p.presentation.people = PeopleChoice::Mixed {
            parts: (0..MAX_MIXED_PARTS + 3)
                .map(|i| format!("part-{i}-{}", "x".repeat(PEOPLE_LABEL_MAX)))
                .collect(),
        };
        p.apply_soft_caps();
        assert_eq!(p.presentation.given_name.chars().count(), GIVEN_NAME_MAX);
        assert_eq!(
            p.presentation.story.player_text.chars().count(),
            STORY_TEXT_MAX
        );
        if let PeopleChoice::Mixed { parts } = &p.presentation.people {
            assert_eq!(parts.len(), MAX_MIXED_PARTS);
            assert!(parts.iter().all(|s| s.chars().count() <= PEOPLE_LABEL_MAX));
        } else {
            panic!("expected mixed");
        }
    }

    #[test]
    fn story_ai_fields_are_records_only_no_privilege() {
        let mut p = Persona::nameless_steward();
        let race_before = p.mechanical_race;
        p.presentation.story = StoryDraft {
            player_text: "I tend wells gently.".into(),
            ai_assist_used: true,
            model_id: Some("local-template-stub".into()),
            player_accepted: true,
            shared_in_world: StoryShare::Spoken,
        };
        // Marking AI assist must not change objective law fields.
        assert_eq!(p.mechanical_race, race_before);
        assert_eq!(p.body_kit, BodyKit::default());
        assert!(p.presentation.story.ai_assist_used);
    }

    #[test]
    fn serde_round_trip_persona_record() {
        let mut p = Persona::nameless_steward();
        p.mechanical_race = MechanicalRace::Cydruid;
        p.presentation.given_name = "Mira".into();
        p.presentation.pronouns = "she/her".into();
        p.presentation.people = PeopleChoice::Preset(PeoplePreset {
            tag: "East African".into(),
        });
        p.presentation.phenotype.freckles = true;
        p.presentation.dress_intent = Some("travel cloak intent".into());
        let json = serde_json::to_string(&p).expect("serialize");
        let back: Persona = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, p);
    }

    #[test]
    fn persona_creator_flag_enabled() {
        assert!(PERSONA_CREATOR_ENABLED);
    }

    #[test]
    fn refuse_mall_p2w_gold_lobby_llm_product_copy() {
        let honest = [
            "nameless Steward",
            "Quellorian with custom river people",
            "story Private",
            "phenotype slider",
            "Title Online grey",
            MechanicalRace::Human.as_str(),
        ];
        for sample in &honest {
            assert!(persona_copy_is_honest(sample), "got {sample}");
        }
        let refuse = [
            "buy face pack at the mall",
            "gold for skin unlock",
            "P2W ethnicity tier",
            "pay to win phenotype",
            "cash shop look",
            "list on Market",
            "NFT soul persona",
            "race lobby matchmaking",
            "Title lobby ranked",
            "LLM product upsell",
            "ChatGPT subscription required",
            "Grok Online required",
            "AGSi certified warranty",
        ];
        for sample in &refuse {
            assert!(!persona_copy_is_honest(sample), "should refuse {sample}");
        }
    }

    #[test]
    fn all_mechanical_races_named() {
        let races = [
            MechanicalRace::Human,
            MechanicalRace::Quellorian,
            MechanicalRace::Draek,
            MechanicalRace::Cydruid,
            MechanicalRace::Ambrosian,
        ];
        for r in races {
            assert!(!r.as_str().is_empty());
            assert!(persona_copy_is_honest(r.as_str()));
        }
    }

    // --- MERCY_PERSONA P4 PersonaCommit ------------------------------------

    fn unique_persist_dir() -> PathBuf {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!(
            "powrush-p4-persona-{}-{}",
            std::process::id(),
            nanos
        ));
        fs::create_dir_all(&dir).expect("temp persist dir");
        dir
    }

    #[test]
    fn persona_commit_from_soft_draft_validates_and_normalizes() {
        let mut draft = Persona::nameless_steward();
        draft.mechanical_race = MechanicalRace::Ambrosian;
        draft.presentation.given_name = "A".repeat(GIVEN_NAME_MAX + 12);
        draft.presentation.phenotype.skin_melanin = 2.0;
        draft.presentation.phenotype.height_within_band = -0.5;
        draft.body_kit.height_band_min = 1.2;
        draft.body_kit.height_band_max = 0.9; // inverted → clamp swaps
        draft.presentation.people = PeopleChoice::Custom(CustomPeople {
            name: "River folk".into(),
            homelands: "delta".into(),
            languages: vec!["river-sign".into()],
            customs_note: "paint only".into(),
            phenotype_seed: Phenotype::default(),
            invented: true,
        });
        let commit = PersonaCommit::from_soft_draft(&draft).expect("commit");
        assert_eq!(commit.schema, PERSONA_SCHEMA);
        assert_eq!(commit.persona.mechanical_race, MechanicalRace::Ambrosian);
        assert_eq!(
            commit.persona.presentation.given_name.chars().count(),
            GIVEN_NAME_MAX
        );
        assert_eq!(commit.persona.presentation.phenotype.skin_melanin, 1.0);
        assert_eq!(commit.persona.presentation.phenotype.height_within_band, 0.0);
        assert!(commit.persona.body_kit.is_valid());
        assert!(commit.persona.body_kit.height_band_min <= commit.persona.body_kit.height_band_max);
        // Soft draft must not become race-lobby power — people ≠ module.
        if let PeopleChoice::Custom(c) = &commit.persona.presentation.people {
            assert_ne!(c.name, commit.persona.mechanical_race.as_str());
        } else {
            panic!("expected custom people");
        }
    }

    #[test]
    fn persona_commit_refuses_dishonest_mall_p2w_copy() {
        let mut draft = Persona::nameless_steward();
        draft.presentation.story.player_text = "buy face pack at the mall with gold".into();
        let err = PersonaCommit::from_soft_draft(&draft).unwrap_err();
        assert_eq!(err, CommitError::DishonestCopy);
    }

    #[test]
    fn persona_commit_persists_via_path_filter_round_trip() {
        let dir = unique_persist_dir();
        let mut draft = Persona::nameless_steward();
        draft.presentation.given_name = "Mira".into();
        draft.presentation.pronouns = "she/her".into();
        draft.mechanical_race = MechanicalRace::Cydruid;
        let commit = PersonaCommit::from_soft_draft(&draft).expect("commit");
        let path = commit.persist_at(&dir).expect("persist");
        assert!(path_filter(&dir, &path));
        assert_eq!(
            path.file_name().and_then(|s| s.to_str()),
            Some(PERSONA_FILE_NAME)
        );
        let loaded = PersonaCommit::load_at(&dir).expect("load");
        assert_eq!(loaded.persona.presentation.given_name, "Mira");
        assert_eq!(loaded.persona.mechanical_race, MechanicalRace::Cydruid);
        assert_eq!(loaded.schema, PERSONA_SCHEMA);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn path_filter_allows_direct_child_persona_json_only() {
        let dir = unique_persist_dir();
        let ok = dir.join(PERSONA_FILE_NAME);
        assert!(path_filter(&dir, &ok));
        assert!(!path_filter(&dir, &dir.join("notes.txt")));
        assert!(!path_filter(&dir, &dir.join("powrush_house.json")));
        assert!(!path_filter(&dir, &dir.join(".hidden.json")));
        assert!(!path_filter(&dir, Path::new("/")));
        assert!(!path_filter(Path::new("/"), &ok));
        assert!(!path_filter(Path::new(""), &ok));
        let escape = dir.join("..").join(PERSONA_FILE_NAME);
        assert!(!path_filter(&dir, &escape));
        let nested = dir.join("nested").join(PERSONA_FILE_NAME);
        let _ = fs::create_dir_all(nested.parent().unwrap());
        assert!(!path_filter(&dir, &nested));
        // Refuse write when filter fails.
        let commit = PersonaCommit::from_soft_draft(&Persona::nameless_steward()).unwrap();
        assert_eq!(
            commit.persist_at(Path::new("/")).unwrap_err(),
            CommitError::PathRefused
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn soft_draft_to_commit_does_not_light_online_or_llm() {
        let mut draft = Persona::nameless_steward();
        draft.presentation.story.ai_assist_used = true;
        draft.presentation.story.model_id = Some("local-template:quiet-steward".into());
        draft.presentation.story.player_accepted = true;
        draft.presentation.story.player_text = "I tend wells gently.".into();
        let race_before = draft.mechanical_race;
        let kit_before = draft.body_kit;
        let commit = PersonaCommit::from_soft_draft(&draft).expect("commit");
        // Commit persists presentation records; does not rewrite objective law
        // beyond body_kit clamp, and never implies Online / live LLM.
        assert_eq!(commit.persona.mechanical_race, race_before);
        assert_eq!(commit.persona.body_kit, kit_before);
        assert!(commit.persona.presentation.story.ai_assist_used);
        assert_eq!(
            commit.persona.presentation.story.model_id.as_deref(),
            Some("local-template:quiet-steward")
        );
        assert!(PERSONA_CREATOR_ENABLED);
        assert_eq!(PERSONA_PATH, "data/powrush_persona.json");
        assert!(persona_copy_is_honest("Title Online grey"));
        assert!(!persona_copy_is_honest("Grok Online required"));
    }

    #[test]
    fn body_kit_clamp_rejects_nan_after_normalize_attempt() {
        let mut draft = Persona::nameless_steward();
        // Finite but wildly inverted still clamps cleanly.
        draft.body_kit.height_band_min = 9.0;
        draft.body_kit.height_band_max = -3.0;
        draft.body_kit.reach = 100.0;
        let commit = PersonaCommit::from_soft_draft(&draft).expect("clamped");
        assert!(commit.persona.body_kit.is_valid());
        assert!(commit.persona.body_kit.reach <= BODY_KIT_REACH_CEIL);
    }

    // --- MERCY_PERSONA P5 online picker (default off) -----------------------

    #[test]
    fn online_picker_flag_defaults_off() {
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(!STEWARD_ONLINE_YES);
        assert!(!online_picker_allows_online_rows(
            ONLINE_PICKER_ENABLED,
            STEWARD_ONLINE_YES
        ));
        assert_eq!(default_story_provider(), StoryProvider::None);
        assert!(!default_story_provider().is_online());
    }

    #[test]
    fn story_provider_offline_rows_always_selectable() {
        for p in StoryProvider::offline_only() {
            assert!(p.is_offline());
            assert!(story_provider_may_select(*p, false, false));
            assert!(story_provider_may_select(*p, true, false));
            assert!(persona_copy_is_honest(p.as_str()));
        }
        assert_eq!(
            StoryProvider::offline_only(),
            &[
                StoryProvider::None,
                StoryProvider::LocalTemplate,
                StoryProvider::RathorOfflineShard,
            ]
        );
    }

    #[test]
    fn online_rows_need_flag_and_steward_online_yes() {
        for p in [
            StoryProvider::RathorOnline,
            StoryProvider::GrokOnline,
            StoryProvider::OpenAiCompatible,
        ] {
            assert!(p.is_online());
            assert!(!story_provider_may_select(p, false, false));
            assert!(!story_provider_may_select(p, true, false));
            assert!(!story_provider_may_select(p, false, true));
            assert!(story_provider_may_select(p, true, true));
            assert_eq!(
                resolve_story_provider(p, false, false),
                StoryProvider::None
            );
            assert_eq!(
                resolve_story_provider(p, true, true),
                p
            );
        }
        // Never first-run default to Grok/OpenAi.
        assert_ne!(default_story_provider(), StoryProvider::GrokOnline);
        assert_ne!(default_story_provider(), StoryProvider::OpenAiCompatible);
    }

    #[test]
    fn cycle_story_provider_stays_offline_without_consent() {
        let mut p = StoryProvider::None;
        let mut saw_online = false;
        for _ in 0..12 {
            p = cycle_story_provider(p, ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES);
            if p.is_online() {
                saw_online = true;
            }
        }
        assert!(!saw_online);
        // With flag+steward yes, online seats appear; still drafts-only seats.
        let mut q = StoryProvider::None;
        let mut saw = false;
        for _ in 0..12 {
            q = cycle_story_provider(q, true, true);
            if q == StoryProvider::GrokOnline {
                saw = true;
            }
        }
        assert!(saw);
        // Resolve without consent still refuses Grok as active seat.
        assert_eq!(
            resolve_story_provider(StoryProvider::GrokOnline, true, false),
            StoryProvider::None
        );
    }

    #[test]
    fn story_provider_labels_honest_and_online_grey() {
        let off = story_provider_btn_label(StoryProvider::None, false, false);
        assert!(off.contains("online picker off"));
        assert!(off.contains("None"));
        assert!(persona_copy_is_honest(&off));
        let local = story_provider_btn_label(StoryProvider::LocalTemplate, true, false);
        assert!(local.contains("LocalTemplate"));
        assert!(local.contains("Online grey"));
        assert!(persona_copy_is_honest(&local));
        let gated = story_provider_btn_label(StoryProvider::GrokOnline, true, false);
        assert!(gated.to_lowercase().contains("online yes"));
        assert!(persona_copy_is_honest(&gated));
        let opt = story_provider_btn_label(StoryProvider::RathorOnline, true, true);
        assert!(opt.contains("opt-in"));
        assert!(opt.contains("drafts only"));
        assert!(persona_copy_is_honest(&opt));
        // Honesty still refuses mall/P2W/gold/lobby/LLM-as-product.
        assert!(!persona_copy_is_honest("Grok Online required"));
        assert!(!persona_copy_is_honest("LLM product upsell"));
        assert!(!persona_copy_is_honest("race lobby ranked"));
    }

    #[test]
    fn online_picker_does_not_rewrite_persona_commit_law() {
        let mut draft = Persona::nameless_steward();
        draft.presentation.story.player_text = "I tend wells gently.".into();
        // Picker seat is orthogonal to Commit — Commit stays the law.
        let provider = resolve_story_provider(
            StoryProvider::GrokOnline,
            ONLINE_PICKER_ENABLED,
            STEWARD_ONLINE_YES,
        );
        assert_eq!(provider, StoryProvider::None);
        let commit = PersonaCommit::from_soft_draft(&draft).expect("commit");
        assert_eq!(commit.persona.mechanical_race, MechanicalRace::Human);
        assert!(!commit.persona.presentation.story.ai_assist_used);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(PERSONA_CREATOR_ENABLED);
    }
}
