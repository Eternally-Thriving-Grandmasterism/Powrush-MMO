/*!
 * client/steam_partner_config.rs
 * Powrush-MMO — Load publishing/steam/steam_cloud_config.json
 *
 * AppID resolution order:
 *   1. STEAM_APP_ID env
 *   2. app_id.shipping (if set in config)
 *   3. steam_appid.txt beside CWD / executable
 *   4. app_id.development (default 480 Spacewar)
 *
 * Also exposes partner checklist readiness for logs / settings UI.
 *
 * v21.89.6 | AG-SML v1.0 | TOLC 8 | Permanent PATSAGi
 * Contact: info@Rathor.ai
 */

use bevy::prelude::*;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

pub const SPACEWAR_APP_ID: u32 = 480;
pub const CONFIG_RELATIVE: &str = "publishing/steam/steam_cloud_config.json";

#[derive(Debug, Clone, Deserialize)]
pub struct SteamPartnerFileConfig {
    pub schema: Option<String>,
    pub app_id: SteamAppIdConfig,
    pub steam_cloud: Option<SteamCloudSection>,
    pub auto_cloud: Option<AutoCloudSection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SteamAppIdConfig {
    pub shipping: Option<u32>,
    pub development: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SteamCloudSection {
    pub enabled_for_app: Option<bool>,
    pub byte_quota_recommended: Option<u64>,
    pub remote_storage_files: Option<Vec<RemoteFileDef>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemoteFileDef {
    pub name: String,
    pub purpose: Option<String>,
    pub max_expected_bytes: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AutoCloudSection {
    pub enabled: Option<bool>,
    pub rules: Option<Vec<AutoCloudRule>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AutoCloudRule {
    pub root: String,
    pub subdirectory: String,
    pub pattern: String,
    pub recursive: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppIdSource {
    Env,
    ConfigShipping,
    SteamAppIdTxt,
    ConfigDevelopment,
    FallbackSpacewar,
}

impl AppIdSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Env => "STEAM_APP_ID env",
            Self::ConfigShipping => "config app_id.shipping",
            Self::SteamAppIdTxt => "steam_appid.txt",
            Self::ConfigDevelopment => "config app_id.development",
            Self::FallbackSpacewar => "fallback Spacewar 480",
        }
    }
}

#[derive(Resource, Clone)]
pub struct SteamPartnerRuntimeConfig {
    pub app_id: u32,
    pub app_id_source: AppIdSource,
    pub shipping_app_id: Option<u32>,
    pub is_development_app_id: bool,
    pub auto_cloud_enabled: bool,
    pub auto_cloud_rule_count: u32,
    pub remote_catalog_name: String,
    pub config_loaded: bool,
    pub config_path: Option<PathBuf>,
}

impl Default for SteamPartnerRuntimeConfig {
    fn default() -> Self {
        Self {
            app_id: SPACEWAR_APP_ID,
            app_id_source: AppIdSource::FallbackSpacewar,
            shipping_app_id: None,
            is_development_app_id: true,
            auto_cloud_enabled: true,
            auto_cloud_rule_count: 0,
            remote_catalog_name: "catalog_cloud_v1.json".into(),
            config_loaded: false,
            config_path: None,
        }
    }
}

/// Partner checklist items that can only be fully verified after Steam init
#[derive(Resource, Clone, Default, Debug)]
pub struct SteamPartnerChecklistState {
    pub config_present: bool,
    pub shipping_app_id_set: bool,
    pub not_using_spacewar_for_ship_intent: bool,
    pub steam_init_ok: bool,
    pub cloud_enabled_account: bool,
    pub cloud_enabled_app: bool,
    pub remote_write_tested: bool,
    pub status_summary: String,
}

impl SteamPartnerChecklistState {
    pub fn recompute_summary(&mut self) {
        let mut parts = Vec::new();
        parts.push(if self.config_present {
            "config✓"
        } else {
            "config✗"
        });
        parts.push(if self.shipping_app_id_set {
            "shipping_appid✓"
        } else {
            "shipping_appid✗"
        });
        parts.push(if self.steam_init_ok {
            "steam_init✓"
        } else {
            "steam_init✗"
        });
        parts.push(if self.cloud_enabled_app {
            "app_cloud✓"
        } else {
            "app_cloud✗ (enable+publish in Partner)"
        });
        parts.push(if self.cloud_enabled_account {
            "account_cloud✓"
        } else {
            "account_cloud✗"
        });
        self.status_summary = parts.join(" · ");
    }
}

pub struct SteamPartnerConfigPlugin;

impl Plugin for SteamPartnerConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SteamPartnerRuntimeConfig>()
            .init_resource::<SteamPartnerChecklistState>()
            .add_systems(Startup, load_partner_config_system);
    }
}

fn load_partner_config_system(
    mut runtime: ResMut<SteamPartnerRuntimeConfig>,
    mut checklist: ResMut<SteamPartnerChecklistState>,
) {
    let (cfg, path) = load_config_from_disk();
    if let Some(ref p) = path {
        runtime.config_path = Some(p.clone());
        runtime.config_loaded = true;
        checklist.config_present = true;
    }

    let shipping = cfg.as_ref().and_then(|c| c.app_id.shipping);
    let development = cfg
        .as_ref()
        .and_then(|c| c.app_id.development)
        .unwrap_or(SPACEWAR_APP_ID);

    runtime.shipping_app_id = shipping;
    checklist.shipping_app_id_set = shipping.is_some();

    if let Some(ref c) = cfg {
        runtime.auto_cloud_enabled = c
            .auto_cloud
            .as_ref()
            .and_then(|a| a.enabled)
            .unwrap_or(true);
        runtime.auto_cloud_rule_count = c
            .auto_cloud
            .as_ref()
            .and_then(|a| a.rules.as_ref())
            .map(|r| r.len() as u32)
            .unwrap_or(0);
        if let Some(name) = c
            .steam_cloud
            .as_ref()
            .and_then(|s| s.remote_storage_files.as_ref())
            .and_then(|f| f.first())
            .map(|f| f.name.clone())
        {
            runtime.remote_catalog_name = name;
        }
    }

    let (app_id, source) = resolve_app_id(shipping, development);
    runtime.app_id = app_id;
    runtime.app_id_source = source;
    runtime.is_development_app_id = app_id == SPACEWAR_APP_ID;
    checklist.not_using_spacewar_for_ship_intent = shipping.is_some() && shipping != Some(SPACEWAR_APP_ID);

    checklist.recompute_summary();

    info!(
        target: "powrush::steam",
        app_id = runtime.app_id,
        source = runtime.app_id_source.as_str(),
        shipping = ?runtime.shipping_app_id,
        auto_cloud_rules = runtime.auto_cloud_rule_count,
        checklist = %checklist.status_summary,
        "Steam partner config loaded"
    );

    if runtime.is_development_app_id {
        warn!(
            target: "powrush::steam",
            "Using development AppID {}. Set app_id.shipping in {} for production.",
            SPACEWAR_APP_ID,
            CONFIG_RELATIVE
        );
    }
}

fn load_config_from_disk() -> (Option<SteamPartnerFileConfig>, Option<PathBuf>) {
    let candidates = [
        PathBuf::from(CONFIG_RELATIVE),
        PathBuf::from("../").join(CONFIG_RELATIVE),
        PathBuf::from("../../").join(CONFIG_RELATIVE),
    ];
    for path in candidates {
        if path.exists() {
            if let Ok(text) = fs::read_to_string(&path) {
                if let Ok(cfg) = serde_json::from_str::<SteamPartnerFileConfig>(&text) {
                    return (Some(cfg), Some(path));
                }
            }
        }
    }
    (None, None)
}

pub fn resolve_app_id(
    shipping: Option<u32>,
    development: u32,
) -> (u32, AppIdSource) {
    if let Ok(env) = std::env::var("STEAM_APP_ID") {
        if let Ok(id) = env.parse::<u32>() {
            return (id, AppIdSource::Env);
        }
    }
    if let Some(id) = shipping {
        return (id, AppIdSource::ConfigShipping);
    }
    if let Some(id) = read_steam_appid_txt() {
        return (id, AppIdSource::SteamAppIdTxt);
    }
    if development != 0 {
        return (development, AppIdSource::ConfigDevelopment);
    }
    (SPACEWAR_APP_ID, AppIdSource::FallbackSpacewar)
}

fn read_steam_appid_txt() -> Option<u32> {
    for path in ["steam_appid.txt", "client/steam_appid.txt"] {
        if let Ok(text) = fs::read_to_string(path) {
            if let Ok(id) = text.trim().parse::<u32>() {
                return Some(id);
            }
        }
    }
    None
}

/// OS-specific Auto-Cloud stage root for production builds
pub fn preferred_auto_cloud_stage_root() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            return Path::new(&local)
                .join("Powrush-MMO")
                .join("steam_cloud")
                .join("audio_moments");
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs_next_home() {
            return home
                .join("Library")
                .join("Application Support")
                .join("Powrush-MMO")
                .join("steam_cloud")
                .join("audio_moments");
        }
    }
    #[cfg(target_os = "linux")]
    {
        if let Some(home) = dirs_next_home() {
            return home
                .join(".local")
                .join("share")
                .join("Powrush-MMO")
                .join("steam_cloud")
                .join("audio_moments");
        }
    }
    PathBuf::from("steam_cloud/audio_moments")
}

fn dirs_next_home() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

// Thunder locked in. Yoi ⚡
