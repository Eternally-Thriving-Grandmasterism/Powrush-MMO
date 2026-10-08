//! CARD HUD-EDIT-DATA-1 — step 4a of the UI layout epic.
//!
//! Q16: the saved HUD layout is its own file, [`HUD_LAYOUT_PATH`], through
//! [`crate::user_persist::read_named`] and [`crate::user_persist::write_named`].
//! `powrush_settings.json` keeps only `hud_preset`.
//!
//! Design §5. A failure never edits the file. [`load_hud_layout`] only reads.
//! Nothing outside this module's tests calls load or save. Step 4b wires edit mode.

use serde::{Deserialize, Serialize};

use crate::user_persist::{read_named, write_named};

/// Cwd-relative name. [`crate::user_persist::persist_file_name`] stores the file
/// as `powrush_hud_layout.json` in the resolved user dir.
pub const HUD_LAYOUT_PATH: &str = "data/powrush_hud_layout.json";

/// Breaking changes to the save shape take a new schema string.
pub const HUD_LAYOUT_SCHEMA: &str = "powrush_hud_layout_v1";

/// §6.2 absolute width cap. The per-anchor floor is the anchor's coded width,
/// applied by the client. This cap is `min(640, window width − 32)`'s ceiling.
pub const HUD_LAYOUT_WIDTH_MAX: f32 = 640.0;

/// Preset ids §5 allows in `base`.
pub const HUD_LAYOUT_PRESETS: [&str; 3] = ["classic", "minimal", "management"];

/// Design §2.1 corners. Edge midpoints use `centre` on the free axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HudLayoutCorner {
    TopLeft,
    TopCentre,
    TopRight,
    BottomLeft,
    BottomCentre,
    BottomRight,
}

/// One anchor override. Anchors absent from the list keep the preset.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HudLayoutAnchor {
    pub id: String,
    pub corner: HudLayoutCorner,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub hidden: bool,
}

/// Design §5 save shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HudLayoutFile {
    pub schema: String,
    pub registry_rev: u32,
    pub base: String,
    pub anchors: Vec<HudLayoutAnchor>,
}

impl HudLayoutFile {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }
}

/// Design §5 load outcomes F1–F5. The file is never rewritten.
#[derive(Clone, Debug, PartialEq)]
pub enum HudLayoutLoad {
    /// Parsed. The client still applies F4–F6 before anything is shown.
    Loaded(HudLayoutFile),
    /// F1. Missing or unreadable (`read_named` failed, including non-JSON).
    Missing,
    /// F2. JSON that is not this save shape.
    Parse,
    /// F3. Unknown or newer `schema`. The file is left as it was.
    Schema,
    /// F4. `registry_rev` is not the current rev. Overrides are dropped.
    /// `base` is a known preset.
    Stale { base: String },
    /// F5. Unknown anchor id or base, a non-finite number, or a width outside
    /// the §6.2 absolute cap. No partial apply. `base` is `Some` when the
    /// preset name is known; `None` means the caller uses classic.
    Refused { base: Option<String> },
}

/// Read [`HUD_LAYOUT_PATH`] and classify F1–F5. Does not write.
///
/// `current_rev` is the client's `HUD_REGISTRY_REV`. `known_anchor_ids` are
/// the anchor ids of `base` (step 4b passes the registry). This function does
/// not know coded widths; a width above [`HUD_LAYOUT_WIDTH_MAX`], below or
/// equal to zero, or non-finite is F5. The client refuses a width below the
/// anchor's coded width.
pub fn load_hud_layout(current_rev: u32, known_anchor_ids: &[&str]) -> HudLayoutLoad {
    let raw = match read_named(HUD_LAYOUT_PATH) {
        Ok(raw) => raw,
        Err(_) => return HudLayoutLoad::Missing,
    };
    classify_hud_layout(&raw, current_rev, known_anchor_ids)
}

/// Classify a save that already reads as text. Same F2–F5 rules as
/// [`load_hud_layout`]. Does not touch the disk.
pub fn classify_hud_layout(
    raw: &str,
    current_rev: u32,
    known_anchor_ids: &[&str],
) -> HudLayoutLoad {
    match HudLayoutFile::from_json(raw) {
        Ok(file) => classify_hud_layout_file(file, current_rev, known_anchor_ids),
        Err(_) => HudLayoutLoad::Parse,
    }
}

/// F3–F5 on a parsed file. JSON has no NaN, so a non-finite field is checked
/// here after parse. Does not touch the disk.
pub fn classify_hud_layout_file(
    file: HudLayoutFile,
    current_rev: u32,
    known_anchor_ids: &[&str],
) -> HudLayoutLoad {
    if file.schema != HUD_LAYOUT_SCHEMA {
        return HudLayoutLoad::Schema;
    }
    let base_known = known_preset(&file.base);
    if file.registry_rev != current_rev {
        if base_known {
            return HudLayoutLoad::Stale { base: file.base };
        }
        return HudLayoutLoad::Refused { base: None };
    }
    if !base_known {
        return HudLayoutLoad::Refused { base: None };
    }
    if anchors_refused(&file.anchors, known_anchor_ids) {
        return HudLayoutLoad::Refused {
            base: Some(file.base),
        };
    }
    HudLayoutLoad::Loaded(file)
}

/// Write `file` through [`write_named`]. Tests call this. Step 4b will call it
/// from Save. Load does not.
pub fn save_hud_layout(file: &HudLayoutFile) -> std::io::Result<()> {
    let json = file
        .to_json()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    write_named(HUD_LAYOUT_PATH, json)
}

fn known_preset(base: &str) -> bool {
    HUD_LAYOUT_PRESETS.contains(&base)
}

fn anchors_refused(anchors: &[HudLayoutAnchor], known_anchor_ids: &[&str]) -> bool {
    let mut seen: Vec<&str> = Vec::new();
    for anchor in anchors {
        if anchor.id.is_empty() || !known_anchor_ids.iter().any(|id| *id == anchor.id) {
            return true;
        }
        if seen.iter().any(|id| *id == anchor.id) {
            return true;
        }
        seen.push(anchor.id.as_str());
        if !anchor.x.is_finite() || !anchor.y.is_finite() || !anchor.width.is_finite() {
            return true;
        }
        if anchor.width <= 0.0 || anchor.width > HUD_LAYOUT_WIDTH_MAX {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_persist::USER_DIR_OVERRIDE_ENV;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "powrush-hud-layout-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
            tag
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("scratch");
        // A live powrush_* file so an empty dir does not adopt cwd `data/`.
        fs::write(dir.join("powrush_seal.json"), b"{}\n").unwrap();
        dir
    }

    fn with_user_dir<R>(dir: &Path, body: impl FnOnce() -> R + std::panic::UnwindSafe) -> R {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
        let prev = std::env::var(USER_DIR_OVERRIDE_ENV).ok();
        std::env::set_var(USER_DIR_OVERRIDE_ENV, dir);
        let result = std::panic::catch_unwind(body);
        match prev {
            Some(value) => std::env::set_var(USER_DIR_OVERRIDE_ENV, value),
            None => std::env::remove_var(USER_DIR_OVERRIDE_ENV),
        }
        match result {
            Ok(value) => value,
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }

    fn layout_path(dir: &Path) -> PathBuf {
        dir.join("powrush_hud_layout.json")
    }

    fn sample() -> HudLayoutFile {
        HudLayoutFile {
            schema: HUD_LAYOUT_SCHEMA.into(),
            registry_rev: 1,
            base: "classic".into(),
            anchors: vec![HudLayoutAnchor {
                id: "PLACE_NAME".into(),
                corner: HudLayoutCorner::TopCentre,
                x: 0.0,
                y: 16.0,
                width: 280.0,
                hidden: false,
            }],
        }
    }

    #[test]
    fn load_f1_missing_or_unreadable_writes_nothing() {
        let missing_dir = scratch("f1-missing");
        with_user_dir(&missing_dir, || {
            let path = layout_path(&missing_dir);
            assert!(!path.exists());
            assert_eq!(load_hud_layout(1, &["PLACE_NAME"]), HudLayoutLoad::Missing);
            assert!(!path.exists(), "F1 must not create the save");
        });

        let bad_dir = scratch("f1-unreadable");
        let path = layout_path(&bad_dir);
        let raw = "not-json";
        fs::write(&path, raw).unwrap();
        with_user_dir(&bad_dir, || {
            assert_eq!(load_hud_layout(1, &["PLACE_NAME"]), HudLayoutLoad::Missing);
            assert_eq!(fs::read_to_string(&path).unwrap(), raw);
            assert!(!bad_dir.join("powrush_hud_layout.json.bak").exists());
        });
        let _ = fs::remove_dir_all(&missing_dir);
        let _ = fs::remove_dir_all(&bad_dir);
    }

    #[test]
    fn load_f2_wrong_shape_is_parse_and_keeps_bytes() {
        let dir = scratch("f2");
        let path = layout_path(&dir);
        let raw = r#"{"schema":"powrush_hud_layout_v1","registry_rev":1,"base":"classic","anchors":[{"id":"VOICE","corner":"top-center"}]}"#;
        fs::write(&path, raw).unwrap();
        with_user_dir(&dir, || {
            assert_eq!(load_hud_layout(1, &["VOICE"]), HudLayoutLoad::Parse);
            assert_eq!(fs::read_to_string(&path).unwrap(), raw);
        });
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_f3_unknown_schema_keeps_file() {
        let dir = scratch("f3");
        let path = layout_path(&dir);
        let raw =
            r#"{"schema":"powrush_hud_layout_v2","registry_rev":1,"base":"classic","anchors":[]}"#;
        fs::write(&path, raw).unwrap();
        with_user_dir(&dir, || {
            assert_eq!(load_hud_layout(1, &[]), HudLayoutLoad::Schema);
            assert_eq!(fs::read_to_string(&path).unwrap(), raw);
            assert!(!dir.join("powrush_hud_layout.json.bak").exists());
        });
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_f4_stale_rev_drops_overrides() {
        let dir = scratch("f4");
        let path = layout_path(&dir);
        let raw = r#"{"schema":"powrush_hud_layout_v1","registry_rev":99,"base":"minimal","anchors":[{"id":"NOPE","corner":"top-left","x":1,"y":1,"width":900,"hidden":true}]}"#;
        fs::write(&path, raw).unwrap();
        with_user_dir(&dir, || {
            assert_eq!(
                load_hud_layout(1, &["CORNER"]),
                HudLayoutLoad::Stale {
                    base: "minimal".into(),
                }
            );
            assert_eq!(fs::read_to_string(&path).unwrap(), raw);
        });
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_f5_refuses_unknown_base_id_width_and_non_finite() {
        let known = ["PLACE_NAME", "VOICE"];
        let dir = scratch("f5");
        with_user_dir(&dir, || {
            let path = layout_path(&dir);
            let unknown_base = r#"{"schema":"powrush_hud_layout_v1","registry_rev":1,"base":"compact","anchors":[]}"#;
            fs::write(&path, unknown_base).unwrap();
            assert_eq!(
                load_hud_layout(1, &known),
                HudLayoutLoad::Refused { base: None }
            );
            assert_eq!(fs::read_to_string(&path).unwrap(), unknown_base);

            let unknown_id = r#"{"schema":"powrush_hud_layout_v1","registry_rev":1,"base":"classic","anchors":[{"id":"NOPE","corner":"top-left","x":16,"y":16,"width":280,"hidden":false}]}"#;
            fs::write(&path, unknown_id).unwrap();
            assert_eq!(
                load_hud_layout(1, &known),
                HudLayoutLoad::Refused {
                    base: Some("classic".into()),
                }
            );
            assert_eq!(fs::read_to_string(&path).unwrap(), unknown_id);

            let wide = r#"{"schema":"powrush_hud_layout_v1","registry_rev":1,"base":"classic","anchors":[{"id":"VOICE","corner":"bottom-right","x":16,"y":221,"width":641,"hidden":false}]}"#;
            fs::write(&path, wide).unwrap();
            assert_eq!(
                load_hud_layout(1, &known),
                HudLayoutLoad::Refused {
                    base: Some("classic".into()),
                }
            );

            let zero = r#"{"schema":"powrush_hud_layout_v1","registry_rev":1,"base":"classic","anchors":[{"id":"VOICE","corner":"bottom-right","x":16,"y":221,"width":0,"hidden":false}]}"#;
            fs::write(&path, zero).unwrap();
            assert_eq!(
                load_hud_layout(1, &known),
                HudLayoutLoad::Refused {
                    base: Some("classic".into()),
                }
            );
        });

        let mut non_finite = sample();
        non_finite.anchors[0].x = f32::NAN;
        assert_eq!(
            classify_hud_layout_file(non_finite, 1, &known),
            HudLayoutLoad::Refused {
                base: Some("classic".into()),
            }
        );
        let mut infinite = sample();
        infinite.anchors[0].width = f32::INFINITY;
        assert_eq!(
            classify_hud_layout_file(infinite, 1, &known),
            HudLayoutLoad::Refused {
                base: Some("classic".into()),
            }
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn save_round_trips_through_serde() {
        assert_eq!(HUD_LAYOUT_PATH, "data/powrush_hud_layout.json");
        assert_eq!(HUD_LAYOUT_SCHEMA, "powrush_hud_layout_v1");
        let file = sample();
        let json = file.to_json().unwrap();
        assert!(json.contains("\"schema\": \"powrush_hud_layout_v1\""));
        assert!(json.contains("\"corner\": \"top-centre\""));
        assert!(json.contains("\"registry_rev\": 1"));
        assert!(json.contains("\"base\": \"classic\""));
        let back = HudLayoutFile::from_json(&json).unwrap();
        assert_eq!(back, file);

        let dir = scratch("round-trip");
        with_user_dir(&dir, || {
            save_hud_layout(&file).unwrap();
            let path = layout_path(&dir);
            let before = fs::read(&path).unwrap();
            assert_eq!(
                load_hud_layout(1, &["PLACE_NAME"]),
                HudLayoutLoad::Loaded(file.clone())
            );
            assert_eq!(fs::read(&path).unwrap(), before, "load must not rewrite");
        });
        let _ = fs::remove_dir_all(&dir);
    }
}
