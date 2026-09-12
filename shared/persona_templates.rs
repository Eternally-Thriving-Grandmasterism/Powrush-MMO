//! MERCY_PERSONA P3 — LocalTemplate offline story packs (CARD H-2026-09-11-P3)
//!
//! Spec: `docs/MERCY_PERSONA_CREATION.md` §4. Offline-first provider
//! `None | LocalTemplate` always works with Title Online grey.
//! P3 ships LocalTemplate packs only — they fill `StoryDraft` records
//! without network, sockets, GrokOnline, OpenAI, or any live LLM.
//! On-disk discovery is `path_filter` + one-level `read_dir` of a named
//! pack directory. Never a recursive walk from filesystem root.
//! Title Online stays grey. Client bind / title_screen is not this card.
//! Persistence stores a *record about* the persona — never the soul
//! (`docs/PERSISTENCE_ORIGINAL_OWNERSHIP.md`).
//!
//! Scope note: Capable · Bounded · Corrigible is research-style framing
//! (later P4 Commit / P5 online picker possible; P3 bounded to offline
//! packs + unit tests) — not a product warranty, AGSi certification,
//! or lobby SKU. Contact: info@Rathor.ai · Independent of xAI.

use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::persona::{
    persona_copy_is_honest, CUSTOMS_NOTE_MAX, GIVEN_NAME_MAX, Persona, STORY_TEXT_MAX, StoryDraft,
    StoryShare,
};

/// Shared LocalTemplate catalog is live for unit tests / later client bind.
/// Does not enable Title Online, sockets, or Grok/OpenAI.
pub const LOCAL_TEMPLATE_OFFLINE: bool = true;

/// P3 does not light Title Online. LocalTemplate works while Online is grey.
pub const TITLE_ONLINE_GREY: bool = true;

/// Relative pack directory leaf. Never `/`. Never walked recursively.
pub const LOCAL_TEMPLATE_DIR_NAME: &str = "persona_templates";

/// Allowed on-disk suffix. ASSET BUDGET: none — P3 ships in-memory packs;
/// this suffix is for a later user-supplied leaf dir, not a cargo of files.
pub const LOCAL_TEMPLATE_SUFFIX: &str = ".json";

pub const TEMPLATE_ID_MAX: usize = 64;
pub const TEMPLATE_NAME_MAX: usize = GIVEN_NAME_MAX;
pub const TEMPLATE_QUESTION_MAX: usize = 160;
pub const MAX_TEMPLATE_NAMES: usize = 8;
pub const MAX_TEMPLATE_QUESTIONS: usize = 6;

/// Offline-first providers that work with Title Online grey.
/// P5 may add an online picker; P3 ships LocalTemplate only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OfflineStoryProvider {
    None,
    LocalTemplate,
}

impl OfflineStoryProvider {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::LocalTemplate => "LocalTemplate",
        }
    }

    /// Both variants are offline. Neither opens a socket.
    pub fn works_with_title_online_grey(self) -> bool {
        TITLE_ONLINE_GREY
    }

    pub fn is_network(self) -> bool {
        false
    }
}

/// Presentation tone for a pack (spec: three tones). Not a mechanical stat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StoryTone {
    Quiet,
    Plain,
    Lyrical,
}

impl StoryTone {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Quiet => "Quiet",
            Self::Plain => "Plain",
            Self::Lyrical => "Lyrical",
        }
    }
}

/// Why a disk pack was refused. Never a network error — P3 has no HTTP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateError {
    PathRefused,
    Unreadable,
    InvalidJson,
    DishonestCopy,
}

/// Offline story pack. Spec JSON shape: names, people_blurb, story_draft,
/// questions_for_player. Fills `StoryDraft` only — never mechanical_race,
/// inventory, Temper, well, or physics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalTemplate {
    pub id: String,
    pub tone: StoryTone,
    pub names: Vec<String>,
    pub people_blurb: String,
    pub story_draft: String,
    pub questions_for_player: Vec<String>,
}

impl LocalTemplate {
    pub fn model_id(&self) -> String {
        format!("local-template:{}", self.id)
    }

    pub fn is_honest(&self) -> bool {
        persona_copy_is_honest(&self.id)
            && persona_copy_is_honest(&self.people_blurb)
            && persona_copy_is_honest(&self.story_draft)
            && self.names.iter().all(|n| persona_copy_is_honest(n))
            && self
                .questions_for_player
                .iter()
                .all(|q| persona_copy_is_honest(q))
    }

    /// Soft-cap strings (P4 Commit validates harder). Does not delete the pack.
    pub fn apply_soft_caps(&mut self) {
        truncate_in_place(&mut self.id, TEMPLATE_ID_MAX);
        truncate_in_place(&mut self.people_blurb, CUSTOMS_NOTE_MAX);
        truncate_in_place(&mut self.story_draft, STORY_TEXT_MAX);
        self.names.truncate(MAX_TEMPLATE_NAMES);
        for name in &mut self.names {
            truncate_in_place(name, TEMPLATE_NAME_MAX);
        }
        self.questions_for_player.truncate(MAX_TEMPLATE_QUESTIONS);
        for q in &mut self.questions_for_player {
            truncate_in_place(q, TEMPLATE_QUESTION_MAX);
        }
    }

    pub fn soft_capped(mut self) -> Self {
        self.apply_soft_caps();
        self
    }

    /// Fill a StoryDraft from this pack. Player has not accepted yet.
    /// `ai_assist_used` records local-template assist — not an online LLM.
    pub fn into_story_draft(&self, share: StoryShare) -> StoryDraft {
        let mut pack = self.clone();
        pack.apply_soft_caps();
        let model_id = pack.model_id();
        StoryDraft {
            player_text: pack.story_draft,
            ai_assist_used: true,
            model_id: Some(model_id),
            player_accepted: false,
            shared_in_world: share,
        }
    }

    /// Accept copies story_draft into player_text (spec §4).
    pub fn accept_into(&self, draft: &mut StoryDraft, share: StoryShare) {
        *draft = self.into_story_draft(share);
        draft.player_accepted = true;
    }
}

fn truncate_in_place(s: &mut String, max: usize) {
    if s.chars().count() > max {
        *s = s.chars().take(max).collect();
    }
}

/// In-memory starter packs. ASSET BUDGET: none — no on-disk cargo this card.
pub fn builtin_packs() -> Vec<LocalTemplate> {
    vec![
        LocalTemplate {
            id: "quiet-steward".into(),
            tone: StoryTone::Quiet,
            names: vec!["Mira".into(), "Kojo".into()],
            people_blurb: "Optional paint. Tend first. Leave reserve for neighbours.".into(),
            story_draft: "I tend the well at first light and leave the reserve for whoever comes after.".into(),
            questions_for_player: vec![
                "What do you tend first?".into(),
                "Who taught you to leave reserve?".into(),
            ],
        },
        LocalTemplate {
            id: "river-people".into(),
            tone: StoryTone::Lyrical,
            names: vec!["Anil".into(), "Sora".into()],
            people_blurb: "Invented river-people. Custom allowed. Not a faction buff.".into(),
            story_draft: "Raised by invented river-people with amber freckles. I mend. I do not take.".into(),
            questions_for_player: vec!["Which river taught you your name?".into()],
        },
        LocalTemplate {
            id: "nameless-hour".into(),
            tone: StoryTone::Plain,
            names: vec![],
            people_blurb: "Hour can finish as nameless Steward. Story later.".into(),
            story_draft: String::new(),
            questions_for_player: vec!["Write your own tale, or keep Hour 1 nameless.".into()],
        },
    ]
}

pub fn pack_by_id(id: &str) -> Option<LocalTemplate> {
    builtin_packs().into_iter().find(|p| p.id == id)
}

/// Fill from an offline provider. `None` yields an empty draft. LocalTemplate
/// needs a pack. Never calls a network provider.
pub fn fill_from_provider(
    provider: OfflineStoryProvider,
    pack: Option<&LocalTemplate>,
    share: StoryShare,
) -> StoryDraft {
    match provider {
        OfflineStoryProvider::None => StoryDraft::default(),
        OfflineStoryProvider::LocalTemplate => pack
            .map(|p| p.into_story_draft(share))
            .unwrap_or_default(),
    }
}

/// Copy pack story into the persona record only. Objective law fields stay put.
pub fn apply_pack_to_persona(
    persona: &mut Persona,
    pack: &LocalTemplate,
    share: StoryShare,
    accept: bool,
) {
    let mut draft = pack.into_story_draft(share);
    draft.player_accepted = accept;
    persona.presentation.story = draft;
}

/// True when `pack_dir` is a named leaf we may one-level list.
/// Refuses filesystem root, empty, and any `..` component.
pub fn pack_dir_is_safe(pack_dir: &Path) -> bool {
    if pack_dir.as_os_str().is_empty() {
        return false;
    }
    if is_filesystem_root(pack_dir) {
        return false;
    }
    if has_parent_dir_component(pack_dir) {
        return false;
    }
    pack_dir.file_name().is_some()
}

/// Direct-child `.json` only. No `..`, no root, no nested walk.
pub fn path_filter(pack_dir: &Path, candidate: &Path) -> bool {
    if !pack_dir_is_safe(pack_dir) {
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
    if name.is_empty() || name.starts_with('.') {
        return false;
    }
    if !name.ends_with(LOCAL_TEMPLATE_SUFFIX) {
        return false;
    }
    if name.contains('/') || name.contains('\\') {
        return false;
    }
    match candidate.parent() {
        Some(parent) => parent == pack_dir,
        None => false,
    }
}

/// One-level `read_dir` of a named pack dir. Never walks from `/`.
pub fn list_pack_paths(pack_dir: &Path) -> Vec<PathBuf> {
    if !pack_dir_is_safe(pack_dir) {
        return Vec::new();
    }
    let entries = match fs::read_dir(pack_dir) {
        Ok(rd) => rd,
        Err(_) => return Vec::new(),
    };
    let mut out = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if is_regular_file(&path) && path_filter(pack_dir, &path) {
            out.push(path);
        }
    }
    out.sort();
    out
}

pub fn load_pack_file(pack_dir: &Path, candidate: &Path) -> Result<LocalTemplate, TemplateError> {
    if !path_filter(pack_dir, candidate) {
        return Err(TemplateError::PathRefused);
    }
    let text = fs::read_to_string(candidate).map_err(|_| TemplateError::Unreadable)?;
    let pack: LocalTemplate =
        serde_json::from_str(&text).map_err(|_| TemplateError::InvalidJson)?;
    if !pack.is_honest() {
        return Err(TemplateError::DishonestCopy);
    }
    Ok(pack.soft_capped())
}

fn is_filesystem_root(p: &Path) -> bool {
    p.parent().is_none() || p.file_name().is_none() || p == Path::new("/")
}

fn has_parent_dir_component(p: &Path) -> bool {
    p.components().any(|c| matches!(c, Component::ParentDir))
}

fn is_regular_file(p: &Path) -> bool {
    fs::metadata(p).map(|m| m.is_file()).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persona::{BodyKit, MechanicalRace};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_pack_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!(
            "powrush-p3-packs-{}-{}",
            std::process::id(),
            nanos
        ));
        fs::create_dir_all(&dir).expect("temp pack dir");
        dir
    }

    fn write_json(dir: &Path, name: &str, body: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, body).expect("write pack");
        path
    }

    #[test]
    fn builtin_packs_fill_story_draft_offline() {
        assert!(LOCAL_TEMPLATE_OFFLINE);
        assert!(TITLE_ONLINE_GREY);
        let packs = builtin_packs();
        assert_eq!(packs.len(), 3);
        for pack in &packs {
            assert!(pack.is_honest(), "pack {} must be honest", pack.id);
            assert!(persona_copy_is_honest(pack.tone.as_str()));
            let draft = pack.into_story_draft(StoryShare::Private);
            assert!(draft.ai_assist_used);
            assert_eq!(draft.model_id.as_deref(), Some(pack.model_id().as_str()));
            assert!(!draft.player_accepted);
            assert_eq!(draft.shared_in_world, StoryShare::Private);
            assert!(draft.player_text.chars().count() <= STORY_TEXT_MAX);
        }
        let quiet = pack_by_id("quiet-steward").expect("quiet");
        assert_eq!(quiet.tone, StoryTone::Quiet);
        assert!(quiet.into_story_draft(StoryShare::Spoken).player_text.contains("tend"));
    }

    #[test]
    fn fill_does_not_rewrite_objective_law() {
        let mut p = Persona::nameless_steward();
        p.mechanical_race = MechanicalRace::Quellorian;
        let kit_before = p.body_kit;
        let race_before = p.mechanical_race;
        let pack = pack_by_id("river-people").expect("river");
        apply_pack_to_persona(&mut p, &pack, StoryShare::Book, false);
        assert_eq!(p.mechanical_race, race_before);
        assert_eq!(p.body_kit, kit_before);
        assert_eq!(p.body_kit, BodyKit::default());
        assert!(p.presentation.story.ai_assist_used);
        assert_eq!(p.presentation.story.shared_in_world, StoryShare::Book);
        assert!(!p.presentation.story.player_accepted);
        assert_eq!(
            p.presentation.story.model_id.as_deref(),
            Some("local-template:river-people")
        );
    }

    #[test]
    fn accept_copies_into_player_text() {
        let pack = pack_by_id("quiet-steward").expect("quiet");
        let mut draft = StoryDraft::default();
        pack.accept_into(&mut draft, StoryShare::Spoken);
        assert_eq!(draft.player_text, pack.story_draft);
        assert!(draft.player_accepted);
        assert_eq!(draft.shared_in_world, StoryShare::Spoken);
        assert_eq!(draft.model_id.as_deref(), Some("local-template:quiet-steward"));
    }

    #[test]
    fn none_and_local_template_work_while_online_grey() {
        for provider in [OfflineStoryProvider::None, OfflineStoryProvider::LocalTemplate] {
            assert!(provider.works_with_title_online_grey());
            assert!(!provider.is_network());
            assert!(persona_copy_is_honest(provider.as_str()));
        }
        let empty = fill_from_provider(OfflineStoryProvider::None, None, StoryShare::Private);
        assert!(!empty.ai_assist_used);
        assert!(empty.player_text.is_empty());
        assert_eq!(empty.model_id, None);
        let pack = pack_by_id("nameless-hour").expect("nameless");
        let filled = fill_from_provider(
            OfflineStoryProvider::LocalTemplate,
            Some(&pack),
            StoryShare::Private,
        );
        assert!(filled.ai_assist_used);
        assert!(filled.player_text.is_empty());
    }

    #[test]
    fn path_filter_allows_direct_child_json_only() {
        let dir = unique_pack_dir();
        let ok = write_json(&dir, "quiet-steward.json", "{}");
        assert!(path_filter(&dir, &ok));
        assert!(!path_filter(&dir, &dir.join("notes.txt")));
        assert!(!path_filter(&dir, &dir.join(".hidden.json")));
        assert!(!path_filter(&dir, Path::new("/")));
        assert!(!path_filter(Path::new("/"), &ok));
        assert!(!path_filter(Path::new(""), &ok));
        let escape = dir.join("..").join("secret.json");
        assert!(!path_filter(&dir, &escape));
        let nested_dir = dir.join("nested");
        fs::create_dir_all(&nested_dir).expect("nested");
        let nested = write_json(&nested_dir, "deep.json", "{}");
        assert!(!path_filter(&dir, &nested));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn list_pack_paths_is_one_level_never_root_walk() {
        let dir = unique_pack_dir();
        let keep = write_json(
            &dir,
            "quiet-steward.json",
            r#"{"id":"quiet-steward","tone":"Quiet","names":["Mira"],"people_blurb":"tend","story_draft":"I tend.","questions_for_player":[]}"#,
        );
        let nested_dir = dir.join("sub");
        fs::create_dir_all(&nested_dir).expect("sub");
        write_json(
            &nested_dir,
            "hidden.json",
            r#"{"id":"hidden","tone":"Plain","names":[],"people_blurb":"no","story_draft":"no","questions_for_player":[]}"#,
        );
        write_json(&dir, "skip.txt", "not a pack");
        let listed = list_pack_paths(&dir);
        assert_eq!(listed, vec![keep.clone()]);
        assert!(list_pack_paths(Path::new("/")).is_empty());
        assert!(list_pack_paths(Path::new("")).is_empty());
        assert!(!pack_dir_is_safe(Path::new("/")));
        assert!(!pack_dir_is_safe(Path::new("..")));
        assert!(!pack_dir_is_safe(Path::new("/tmp/../")));
        let loaded = load_pack_file(&dir, &keep).expect("load");
        assert_eq!(loaded.id, "quiet-steward");
        assert_eq!(
            load_pack_file(&dir, &dir.join("sub").join("hidden.json")),
            Err(TemplateError::PathRefused)
        );
        assert_eq!(
            load_pack_file(&dir, &dir.join("..").join("secret.json")),
            Err(TemplateError::PathRefused)
        );
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn load_refuses_dishonest_and_invalid_copy() {
        let dir = unique_pack_dir();
        let mall = write_json(
            &dir,
            "mall.json",
            r#"{"id":"mall-skin","tone":"Plain","names":[],"people_blurb":"buy face pack at the mall","story_draft":"gold for skin unlock","questions_for_player":[]}"#,
        );
        assert_eq!(load_pack_file(&dir, &mall), Err(TemplateError::DishonestCopy));
        let bad = write_json(&dir, "bad.json", "not-json");
        assert_eq!(load_pack_file(&dir, &bad), Err(TemplateError::InvalidJson));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn serde_round_trip_local_template() {
        let pack = pack_by_id("river-people").expect("river");
        let json = serde_json::to_string(&pack).expect("serialize");
        let back: LocalTemplate = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, pack);
        assert!(!json.to_lowercase().contains("grokonline"));
        assert!(!json.to_lowercase().contains("openai"));
        assert!(!json.contains("http"));
    }

    #[test]
    fn soft_caps_truncate_without_deleting_pack() {
        let mut pack = LocalTemplate {
            id: "A".repeat(TEMPLATE_ID_MAX + 8),
            tone: StoryTone::Plain,
            names: (0..MAX_TEMPLATE_NAMES + 3)
                .map(|i| format!("n{i}-{}", "x".repeat(TEMPLATE_NAME_MAX)))
                .collect(),
            people_blurb: "B".repeat(CUSTOMS_NOTE_MAX + 4),
            story_draft: "C".repeat(STORY_TEXT_MAX + 9),
            questions_for_player: (0..MAX_TEMPLATE_QUESTIONS + 2)
                .map(|i| format!("q{i}-{}", "y".repeat(TEMPLATE_QUESTION_MAX)))
                .collect(),
        };
        pack.apply_soft_caps();
        assert_eq!(pack.id.chars().count(), TEMPLATE_ID_MAX);
        assert_eq!(pack.people_blurb.chars().count(), CUSTOMS_NOTE_MAX);
        assert_eq!(pack.story_draft.chars().count(), STORY_TEXT_MAX);
        assert_eq!(pack.names.len(), MAX_TEMPLATE_NAMES);
        assert!(pack.names.iter().all(|s| s.chars().count() <= TEMPLATE_NAME_MAX));
        assert_eq!(pack.questions_for_player.len(), MAX_TEMPLATE_QUESTIONS);
        assert!(pack
            .questions_for_player
            .iter()
            .all(|s| s.chars().count() <= TEMPLATE_QUESTION_MAX));
        let draft = pack.into_story_draft(StoryShare::Private);
        assert_eq!(draft.player_text.chars().count(), STORY_TEXT_MAX);
    }

    #[test]
    fn refuse_mall_p2w_gold_lobby_llm_product_copy() {
        let honest = [
            "quiet steward",
            "invented river-people",
            "Title Online grey",
            OfflineStoryProvider::LocalTemplate.as_str(),
            StoryTone::Quiet.as_str(),
            "local-template:quiet-steward",
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
    fn no_online_llm_provider_in_p3() {
        // Compile-time seat: only None + LocalTemplate exist this card.
        let listed = [
            OfflineStoryProvider::None.as_str(),
            OfflineStoryProvider::LocalTemplate.as_str(),
        ];
        assert_eq!(listed, ["None", "LocalTemplate"]);
        for name in listed {
            let low = name.to_lowercase();
            assert!(!low.contains("grok"));
            assert!(!low.contains("openai"));
            assert!(!low.contains("online"));
        }
    }
}
