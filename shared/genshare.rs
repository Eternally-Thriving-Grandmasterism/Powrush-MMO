//! GenShare Method A — L0 disk recipe (offline, no socket)
//!
//! Persist grove seed story beside house/climate:
//! `data/powrush_genshare.jsonl` — append-only JSONL.
//! Recipe = seed story so a peer (later Method B) can rebuild the same yard.
//! Never stream glTF / mesh floods. Never bind / dial. Dress = seal+heritage
//! caption only — no combat stats. Prefer with Method D (optional climate
//! piggyback fields). Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::powrush_gen::{climate_epoch, grove_seed_from, id_from_str};

/// L0 offline recipe path (append-only JSONL).
pub const GENSHARE_PATH: &str = "data/powrush_genshare.jsonl";

/// Envelope version for Method A JSONL rows.
pub const GENSHARE_V: u16 = 1;

/// GenShare recipe envelope — seed story, not triangles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenShare {
    pub v: u16,
    /// House display id used in house⊕hex match (local offline writer).
    #[serde(default)]
    pub house: String,
    pub hex: String,
    pub epoch: u64,
    pub seed_u64: u64,
    /// Hex digest of climate truth used at write time (integrity / match).
    pub climate_digest: String,
    /// Seal + heritage caption only — no combat stats / +take / +STR.
    pub dress: String,
}

impl GenShare {
    /// Build envelope from lived house/hex/climate feel + dress caption.
    /// Seed is the same grove seed G0 uses (`grove_seed_from`).
    pub fn build(
        house: &str,
        hex: &str,
        epoch: u64,
        seed_u64: u64,
        harmony: f32,
        stress: f32,
        dress: &str,
    ) -> Self {
        Self {
            v: GENSHARE_V,
            house: house.to_string(),
            hex: hex.to_string(),
            epoch,
            seed_u64,
            climate_digest: climate_digest_hex(harmony, stress, hex),
            dress: dress.to_string(),
        }
    }

    /// Convenience: compute epoch + seed from harmony/stress, then build.
    pub fn from_lived(
        house: &str,
        hex: &str,
        harmony: f32,
        stress: f32,
        dress: &str,
    ) -> Self {
        let epoch = climate_epoch(harmony, stress);
        let seed_u64 = grove_seed_from(house, hex, harmony, stress);
        Self::build(house, hex, epoch, seed_u64, harmony, stress, dress)
    }

    pub fn to_json_line(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json_line(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw.trim())
    }
}

/// FNV-style hex digest of climate feel + hex id (integrity string, not padded theatre).
pub fn climate_digest_hex(harmony: f32, stress: f32, hex: &str) -> String {
    let mut h = id_from_str(hex);
    // Quantize floats so tiny noise does not churn the digest string.
    let hq = (harmony.clamp(0.0, 1.0) * 1000.0).round() as u64;
    let sq = (stress.clamp(0.0, 1.0) * 1000.0).round() as u64;
    h ^= hq.wrapping_mul(0x9e3779b97f4a7c15);
    h = h.rotate_left(13).wrapping_mul(0xbf58476d1ce4e5b9);
    h ^= sq.wrapping_mul(0x94d049bb133111eb);
    h = h.rotate_left(17).wrapping_mul(0x94d049bb133111eb);
    format!("{h:016x}")
}

/// Soft-fail append one JSON line to `path` (creates parent `data/` as needed).
pub fn append_genshare_at(path: &Path, row: &GenShare) -> bool {
    let Ok(line) = row.to_json_line() else {
        return false;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) else {
        return false;
    };
    writeln!(f, "{line}").is_ok()
}

/// Append one JSON line to [`GENSHARE_PATH`]. Soft-fail.
pub fn append_genshare(row: &GenShare) -> bool {
    append_genshare_at(Path::new(GENSHARE_PATH), row)
}

/// Parse all valid GenShare rows from JSONL text (skips blank / corrupt lines).
pub fn parse_genshare_jsonl(raw: &str) -> Vec<GenShare> {
    raw.lines()
        .filter_map(|line| {
            let t = line.trim();
            if t.is_empty() {
                return None;
            }
            GenShare::from_json_line(t).ok()
        })
        .collect()
}

fn house_matches(row_house: &str, want_house: Option<&str>) -> bool {
    match want_house {
        None => true,
        Some(w) => {
            let a = row_house.trim();
            let b = w.trim();
            a.eq_ignore_ascii_case(b)
        }
    }
}

/// Load best matching row from JSONL text.
///
/// - Prefer last row matching `house` ⊕ `hex` (when `house` is Some).
/// - Else latest row for `hex`.
/// - On same hex, newer `epoch` wins — never merge two seeds.
pub fn load_best_from_jsonl(raw: &str, hex: &str, house: Option<&str>) -> Option<GenShare> {
    let rows = parse_genshare_jsonl(raw);
    let hex_t = hex.trim();
    if hex_t.is_empty() {
        return None;
    }

    // First pass: house⊕hex matches (file order; later + higher epoch wins).
    let mut best_house: Option<GenShare> = None;
    for row in &rows {
        if row.hex.trim() != hex_t {
            continue;
        }
        if !house_matches(&row.house, house) {
            continue;
        }
        // When house filter is set, require house match; when None, all hex rows compete below.
        if house.is_some() {
            best_house = Some(match best_house {
                None => row.clone(),
                Some(prev) => {
                    if row.epoch >= prev.epoch {
                        row.clone()
                    } else {
                        prev
                    }
                }
            });
        }
    }
    if best_house.is_some() {
        return best_house;
    }

    // Fallback: latest for hex (newer epoch wins; later equal-epoch line wins).
    let mut best: Option<GenShare> = None;
    for row in rows {
        if row.hex.trim() != hex_t {
            continue;
        }
        best = Some(match best {
            None => row,
            Some(prev) => {
                if row.epoch >= prev.epoch {
                    row
                } else {
                    prev
                }
            }
        });
    }
    best
}

/// Soft-fail load from path. See [`load_best_from_jsonl`].
pub fn load_genshare_at(path: &Path, hex: &str, house: Option<&str>) -> Option<GenShare> {
    let raw = fs::read_to_string(path).ok()?;
    load_best_from_jsonl(&raw, hex, house)
}

/// Soft-fail load from [`GENSHARE_PATH`].
pub fn load_genshare(hex: &str, house: Option<&str>) -> Option<GenShare> {
    load_genshare_at(Path::new(GENSHARE_PATH), hex, house)
}

/// Resolve seed for scatter: prefer persisted GenShare seed for house⊕hex,
/// else compute local grove seed. Returns `(seed, from_disk)`.
pub fn resolve_grove_seed(
    house: &str,
    hex: &str,
    harmony: f32,
    stress: f32,
    path: Option<&Path>,
) -> (u64, bool) {
    let p = path.unwrap_or_else(|| Path::new(GENSHARE_PATH));
    if let Some(row) = load_genshare_at(p, hex, Some(house)) {
        return (row.seed_u64, true);
    }
    // Hex-only fallback (house absent on older rows).
    if let Some(row) = load_genshare_at(p, hex, None) {
        return (row.seed_u64, true);
    }
    (grove_seed_from(house, hex, harmony, stress), false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn tmp_path(tag: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("powrush_genshare_{tag}_{nanos}.jsonl"))
    }

    #[test]
    fn path_const_beside_house_climate() {
        assert_eq!(GENSHARE_PATH, "data/powrush_genshare.jsonl");
        assert!(GENSHARE_PATH.starts_with("data/powrush_"));
    }

    #[test]
    fn roundtrip_json_line() {
        let g = GenShare::from_lived(
            "Unnamed House",
            "local-hex",
            0.55,
            0.15,
            "Seal · Well · human",
        );
        let line = g.to_json_line().unwrap();
        let back = GenShare::from_json_line(&line).unwrap();
        assert_eq!(g, back);
        assert_eq!(back.v, GENSHARE_V);
        assert!(!back.climate_digest.is_empty());
        assert!(!back.dress.contains("+take"));
        assert!(!back.dress.contains("+STR"));
    }

    #[test]
    fn append_and_load_house_hex_match() {
        let path = tmp_path("append");
        let _ = fs::remove_file(&path);
        let a = GenShare::build(
            "Peace",
            "hex-a",
            3,
            0x1111,
            0.5,
            0.2,
            "Seal · Grove",
        );
        let b = GenShare::build(
            "Other",
            "hex-a",
            5,
            0x2222,
            0.6,
            0.1,
            "Seal · Ember",
        );
        assert!(append_genshare_at(&path, &a));
        assert!(append_genshare_at(&path, &b));
        let hit = load_genshare_at(&path, "hex-a", Some("Peace")).unwrap();
        assert_eq!(hit.seed_u64, 0x1111);
        assert_eq!(hit.house, "Peace");
        let latest = load_genshare_at(&path, "hex-a", None).unwrap();
        // Newer epoch wins on same hex when house filter absent.
        assert_eq!(latest.seed_u64, 0x2222);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn newer_epoch_wins_never_merge_seeds() {
        let path = tmp_path("epoch");
        let _ = fs::remove_file(&path);
        let old = GenShare::build("H", "hx", 1, 0xaaaa, 0.4, 0.3, "");
        let new = GenShare::build("H", "hx", 4, 0xbbbb, 0.7, 0.1, "Seal · Well");
        assert!(append_genshare_at(&path, &old));
        assert!(append_genshare_at(&path, &new));
        // Older epoch rewritten later must not beat newer epoch.
        let old_again = GenShare::build("H", "hx", 2, 0xcccc, 0.4, 0.3, "");
        assert!(append_genshare_at(&path, &old_again));
        let best = load_genshare_at(&path, "hx", Some("H")).unwrap();
        assert_eq!(best.seed_u64, 0xbbbb);
        assert_eq!(best.epoch, 4);
        // Never average / merge — one seed only.
        assert_ne!(best.seed_u64, (0xaaaa + 0xbbbb) / 2);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn resolve_prefers_disk_seed() {
        let path = tmp_path("resolve");
        let _ = fs::remove_file(&path);
        let row = GenShare::build(
            "Unnamed House",
            "local-hex",
            9,
            0xdeadbeef,
            0.55,
            0.15,
            "",
        );
        assert!(append_genshare_at(&path, &row));
        let (seed, from_disk) =
            resolve_grove_seed("Unnamed House", "local-hex", 0.55, 0.15, Some(&path));
        assert!(from_disk);
        assert_eq!(seed, 0xdeadbeef);
        let (local, from_disk2) =
            resolve_grove_seed("Fresh House", "brand-new-hex", 0.55, 0.15, Some(&path));
        assert!(!from_disk2);
        assert_eq!(
            local,
            grove_seed_from("Fresh House", "brand-new-hex", 0.55, 0.15)
        );
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn from_lived_seed_matches_grove_seed_from() {
        let g = GenShare::from_lived("Yard", "hx-1", 0.55, 0.15, "Seal · none");
        assert_eq!(g.seed_u64, grove_seed_from("Yard", "hx-1", 0.55, 0.15));
        assert_eq!(g.epoch, climate_epoch(0.55, 0.15));
    }
}
