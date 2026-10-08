//! CARD HUD-EDIT-DATA-1 — step 4a of the UI layout epic.
//!
//! Q16: the saved HUD layout is its own file, [`HUD_LAYOUT_PATH`], through
//! [`crate::user_persist::read_named`] and [`crate::user_persist::write_named`].
//! `powrush_settings.json` keeps only `hud_preset`.
//!
//! Design §5. A failure never edits the file. [`load_hud_layout`] only reads.
//! user_persist alone owns the last-good `.bak` / `.tmp` rule. This module does
//! no file I/O of its own, and its tests drive the pure classify and serde
//! functions on raw strings. Nothing calls load or save yet. Step 4b wires
//! edit mode.

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
    classify_hud_layout_read(read_named(HUD_LAYOUT_PATH), current_rev, known_anchor_ids)
}

/// Classify the result of a read. `Err` (missing, or not JSON per
/// `read_named`) is F1; text goes to [`classify_hud_layout`]. Pure.
pub fn classify_hud_layout_read(
    read: std::io::Result<String>,
    current_rev: u32,
    known_anchor_ids: &[&str],
) -> HudLayoutLoad {
    match read {
        Ok(raw) => classify_hud_layout(&raw, current_rev, known_anchor_ids),
        Err(_) => HudLayoutLoad::Missing,
    }
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

/// [`HudLayoutFile::to_json`] then [`write_named`]. Step 4b will call this
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
    use std::io::{Error, ErrorKind};

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
        let missing = Err(Error::new(ErrorKind::NotFound, "no save"));
        assert_eq!(
            classify_hud_layout_read(missing, 1, &["PLACE_NAME"]),
            HudLayoutLoad::Missing
        );
        let unreadable = Err(Error::new(
            ErrorKind::InvalidData,
            "persist file is not json",
        ));
        assert_eq!(
            classify_hud_layout_read(unreadable, 1, &["PLACE_NAME"]),
            HudLayoutLoad::Missing
        );
        let json = sample().to_json().unwrap();
        assert_eq!(
            classify_hud_layout_read(Ok(json), 1, &["PLACE_NAME"]),
            HudLayoutLoad::Loaded(sample())
        );
    }

    #[test]
    fn load_f2_wrong_shape_is_parse_and_keeps_bytes() {
        let raw = r#"{"schema":"powrush_hud_layout_v1","registry_rev":1,"base":"classic","anchors":[{"id":"VOICE","corner":"top-center"}]}"#;
        assert_eq!(
            classify_hud_layout(raw, 1, &["VOICE"]),
            HudLayoutLoad::Parse
        );
        assert_eq!(
            classify_hud_layout("not-json", 1, &["VOICE"]),
            HudLayoutLoad::Parse
        );
    }

    #[test]
    fn load_f3_unknown_schema_keeps_file() {
        let raw =
            r#"{"schema":"powrush_hud_layout_v2","registry_rev":1,"base":"classic","anchors":[]}"#;
        assert_eq!(classify_hud_layout(raw, 1, &[]), HudLayoutLoad::Schema);
    }

    #[test]
    fn load_f4_stale_rev_drops_overrides() {
        let raw = r#"{"schema":"powrush_hud_layout_v1","registry_rev":99,"base":"minimal","anchors":[{"id":"NOPE","corner":"top-left","x":1,"y":1,"width":900,"hidden":true}]}"#;
        assert_eq!(
            classify_hud_layout(raw, 1, &["CORNER"]),
            HudLayoutLoad::Stale {
                base: "minimal".into(),
            }
        );
    }

    #[test]
    fn load_f5_refuses_unknown_base_id_width_and_non_finite() {
        let known = ["PLACE_NAME", "VOICE"];
        let unknown_base =
            r#"{"schema":"powrush_hud_layout_v1","registry_rev":1,"base":"compact","anchors":[]}"#;
        assert_eq!(
            classify_hud_layout(unknown_base, 1, &known),
            HudLayoutLoad::Refused { base: None }
        );

        let unknown_id = r#"{"schema":"powrush_hud_layout_v1","registry_rev":1,"base":"classic","anchors":[{"id":"NOPE","corner":"top-left","x":16,"y":16,"width":280,"hidden":false}]}"#;
        assert_eq!(
            classify_hud_layout(unknown_id, 1, &known),
            HudLayoutLoad::Refused {
                base: Some("classic".into()),
            }
        );

        let wide = r#"{"schema":"powrush_hud_layout_v1","registry_rev":1,"base":"classic","anchors":[{"id":"VOICE","corner":"bottom-right","x":16,"y":221,"width":641,"hidden":false}]}"#;
        assert_eq!(
            classify_hud_layout(wide, 1, &known),
            HudLayoutLoad::Refused {
                base: Some("classic".into()),
            }
        );

        let zero = r#"{"schema":"powrush_hud_layout_v1","registry_rev":1,"base":"classic","anchors":[{"id":"VOICE","corner":"bottom-right","x":16,"y":221,"width":0,"hidden":false}]}"#;
        assert_eq!(
            classify_hud_layout(zero, 1, &known),
            HudLayoutLoad::Refused {
                base: Some("classic".into()),
            }
        );

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
        assert_eq!(
            classify_hud_layout(&json, 1, &["PLACE_NAME"]),
            HudLayoutLoad::Loaded(file)
        );
    }
}
