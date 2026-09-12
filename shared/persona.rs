//! MERCY_PERSONA P1 — shared types + unit tests only (CARD H-2026-09-11-P1)
//!
//! Spec: `docs/MERCY_PERSONA_CREATION.md` (design tick 23.2.P).
//! Flat module (same pattern as `temper.rs` T1): data model for later P2–P5
//! rungs without shipping client UI, Title race lobby, or LLM calls.
//! Title Online stays grey. Persistence stores a *record about* the persona —
//! never the soul (`docs/PERSISTENCE_ORIGINAL_OWNERSHIP.md`).
//!
//! Scope note: Capable · Bounded · Corrigible here means research-style framing
//! for the module (later rungs possible; P1 bounded to types+tests; corrigible
//! via Commit later) — not a product warranty, AGSi certification, or lobby SKU.
//! Contact: info@Rathor.ai · Independent of xAI.

use serde::{Deserialize, Serialize};

/// Feature gate for later slices (P2+ creator UI). Hour can finish without persons.
pub const PERSONA_CREATOR_ENABLED: bool = false;

/// Soft caps validated more strictly at PersonaCommit (P4). P1 stores only.
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
        assert!(!PERSONA_CREATOR_ENABLED);
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
    fn persona_creator_flag_defaults_off() {
        assert!(!PERSONA_CREATOR_ENABLED);
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
}
