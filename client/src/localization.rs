/*!
 * Powrush-MMO v18.9 — Dynamic Localization System
 *
 * Professional, data-driven, sovereign i18n foundation.
 * Supports runtime language switching, JSON locale loading, and graceful fallbacks.
 * Designed for global release: easy to extend to 11+ languages without code changes.
 * TOLC 8 Mercy Gates enforced. Mercy-gated (player always has choice).
 * Offline-capable and sovereign (no external services required).
 * PATSAGi Councils + Ra-Thor AGI + MIAL/MWPO sealed.
 * Mint-and-Print-Only-Perfection.
 */

use bevy::prelude::*;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Resource, Default, Debug)]
pub struct Localization {
    pub current_lang: String,
    /// lang_code -> (key -> translated_text)
    strings: HashMap<String, HashMap<String, String>>,
}

#[derive(Deserialize, Debug, Clone)]
struct LocaleFile {
    #[serde(flatten)]
    entries: HashMap<String, String>,
}

impl Localization {
    pub fn new(default_lang: &str) -> Self {
        let mut loc = Self {
            current_lang: default_lang.to_string(),
            strings: HashMap::new(),
        };
        // Seed with embedded professional defaults for onboarding + core whispers
        loc.register_locale("en", include_str!("../../content/locales/en.json"));
        loc.register_locale("es", include_str!("../../content/locales/es.json"));
        loc.register_locale("fr", include_str!("../../content/locales/fr.json"));
        loc.register_locale("de", include_str!("../../content/locales/de.json"));
        loc.register_locale("ar", include_str!("../../content/locales/ar.json"));
        loc
    }

    /// Register or override a full locale from JSON string.
    /// Production path: also support loading from Bevy AssetServer at runtime.
    pub fn register_locale(&mut self, lang: &str, json: &str) {
        if let Ok(parsed) = serde_json::from_str::<LocaleFile>(json) {
            self.strings.insert(lang.to_string(), parsed.entries);
        }
    }

    /// Set current language (player choice or settings).
    pub fn set_language(&mut self, lang: &str) {
        if self.strings.contains_key(lang) {
            self.current_lang = lang.to_string();
        }
    }

    /// Translate key with current language. Falls back gracefully.
    pub fn t(&self, key: &str) -> String {
        self.strings
            .get(&self.current_lang)
            .and_then(|m| m.get(key))
            .or_else(|| self.strings.get("en").and_then(|m| m.get(key)))
            .unwrap_or(key)
            .to_string()
    }

    /// Translate with explicit language (useful for UI language pickers).
    pub fn t_lang(&self, lang: &str, key: &str) -> String {
        self.strings
            .get(lang)
            .and_then(|m| m.get(key))
            .or_else(|| self.strings.get("en").and_then(|m| m.get(key)))
            .unwrap_or(key)
            .to_string()
    }
}

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Localization::new("en"));
    }
}
