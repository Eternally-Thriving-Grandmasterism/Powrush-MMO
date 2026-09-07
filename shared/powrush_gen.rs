//! G0 light gen — hex-seed grove + env door (v23.2.62+)
//!
//! `POWRUSH_GEN=light` enables tiny atlas scatter. **Default off** so lavapipe
//! Title / pause / Settings / L / Q clicks stay clean. G0.5 Settings Grove
//! (`off`|`light`) ORs with env — same gen path, not a second system. No
//! Avian/Rapier, no second Camera3d, no combat stats, no GenShare net.
//! Contact: info@Rathor.ai

/// Env values that enable light gen (case-insensitive).
pub const POWRUSH_GEN_LIGHT: &str = "light";

/// Parsed gen mode. Only `Light` draws scatter; everything else is off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PowrushGen {
    #[default]
    Off,
    Light,
}

impl PowrushGen {
    pub fn as_str(self) -> &'static str {
        match self {
            PowrushGen::Off => "off",
            PowrushGen::Light => "light",
        }
    }

    pub fn is_light(self) -> bool {
        matches!(self, PowrushGen::Light)
    }
}

/// Parse `POWRUSH_GEN` raw value. Default / unknown → Off.
pub fn parse_powrush_gen_from(raw: Option<&str>) -> PowrushGen {
    match raw {
        None => PowrushGen::Off,
        Some(v) => {
            let t = v.trim().to_ascii_lowercase();
            match t.as_str() {
                "light" | "on" | "1" | "true" | "yes" => PowrushGen::Light,
                "off" | "0" | "false" | "no" | "" => PowrushGen::Off,
                _ => PowrushGen::Off,
            }
        }
    }
}

/// Process env. Default **off** — door-safe for lavapipe.
pub fn parse_powrush_gen() -> PowrushGen {
    parse_powrush_gen_from(std::env::var("POWRUSH_GEN").ok().as_deref())
}

/// Convenience: true only when `POWRUSH_GEN=light` (or on/1/true/yes).
pub fn light_gen_enabled() -> bool {
    parse_powrush_gen().is_light()
}

/// Same as [`light_gen_enabled`] but from an explicit string (tests).
pub fn light_gen_enabled_from(raw: Option<&str>) -> bool {
    parse_powrush_gen_from(raw).is_light()
}

/// G0.5 door: settings grove is light **OR** env `POWRUSH_GEN=light` (OR).
/// `settings_grove` missing / unknown / off → settings half is off; env still wins.
/// Same gen path as env — not a second system.
pub fn light_gen_enabled_with(settings_grove: Option<&str>) -> bool {
    light_gen_enabled_from(settings_grove) || light_gen_enabled()
}

/// Pure OR helper for tests (no process env): settings light OR env_raw light.
pub fn light_gen_enabled_or(settings_grove: Option<&str>, env_raw: Option<&str>) -> bool {
    light_gen_enabled_from(settings_grove) || light_gen_enabled_from(env_raw)
}

/// FNV-1a 64 — stable string → u64 for house/hex ids.
pub fn id_from_str(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Coarse climate epoch from harmony/stress bands (0..15).
/// Small tends stay in-band so the grove does not jump every E.
pub fn climate_epoch(harmony: f32, stress: f32) -> u64 {
    let h = (harmony.clamp(0.0, 1.0) * 3.0).round() as u64;
    let s = (stress.clamp(0.0, 1.0) * 3.0).round() as u64;
    (h << 2) | s
}

/// Deterministic grove seed: hash(house_id ⊕ hex_id ⊕ climate_epoch).
/// Same three numbers ⇒ same grove.
pub fn grove_seed(house_id: u64, hex_id: u64, climate_epoch: u64) -> u64 {
    let mut h: u64 = 0x9e3779b97f4a7c15;
    h ^= house_id.wrapping_mul(0xbf58476d1ce4e5b9);
    h = h.rotate_left(27).wrapping_mul(0x94d049bb133111eb);
    h ^= hex_id.wrapping_mul(0xbf58476d1ce4e5b9);
    h = h.rotate_left(31).wrapping_mul(0x94d049bb133111eb);
    h ^= climate_epoch.wrapping_mul(0xbf58476d1ce4e5b9);
    h = h.rotate_left(17).wrapping_mul(0x94d049bb133111eb);
    h ^ (h >> 33)
}

/// Seed from lived strings + climate feel.
pub fn grove_seed_from(house_id: &str, hex_id: &str, harmony: f32, stress: f32) -> u64 {
    grove_seed(
        id_from_str(house_id),
        id_from_str(hex_id),
        climate_epoch(harmony, stress),
    )
}

/// Cull light-gen props when lived plates are open (Title / pause / Settings / L / Q).
/// Returns true when scatter should hide so soft-GPU clicks stay clean.
pub fn cull_gen_when_plate_open(
    title_open: bool,
    pause_or_settings_open: bool,
    ledger_open: bool,
    q_open: bool,
) -> bool {
    title_open || pause_or_settings_open || ledger_open || q_open
}

/// Atlas kind — capped to 4 mesh types (trunk / canopy / rock / bush).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScatterKind {
    Trunk,
    Canopy,
    Rock,
    Bush,
}

/// One placed instance (xz + kind). No physics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScatterSpot {
    pub x: f32,
    pub z: f32,
    pub y: f32,
    pub scale: f32,
    pub kind: ScatterKind,
}

/// Tiny atlas scatter from seed. Caps: ≤4 kinds, ≤12 instances (trees+stones).
pub fn scatter_from_seed(seed: u64) -> Vec<ScatterSpot> {
    let mut rng = seed ^ 0xa0761d6478bd642f;
    let next = |rng: &mut u64| -> u64 {
        *rng = rng
            .wrapping_mul(0x9e3779b97f4a7c15)
            .wrapping_add(0x6a09e667f3bcc909);
        *rng
    };
    let unit = |rng: &mut u64| -> f32 {
        let v = next(rng);
        (v as f32) / (u64::MAX as f32)
    };

    let tree_count = 3 + (next(&mut rng) % 3) as usize; // 3..5 trees
    let rock_count = 2 + (next(&mut rng) % 2) as usize; // 2..3 rocks
    let mut out = Vec::with_capacity(tree_count * 2 + rock_count);

    for _ in 0..tree_count {
        let ang = unit(&mut rng) * std::f32::consts::TAU;
        let rad = 4.5 + unit(&mut rng) * 5.5;
        let x = ang.cos() * rad;
        let z = ang.sin() * rad;
        let scale = 0.85 + unit(&mut rng) * 0.35;
        out.push(ScatterSpot {
            x,
            z,
            y: 1.1 * scale,
            scale,
            kind: ScatterKind::Trunk,
        });
        out.push(ScatterSpot {
            x,
            z,
            y: 1.1 * scale + 1.25 * scale,
            scale: scale * 0.95,
            kind: ScatterKind::Canopy,
        });
    }

    for _ in 0..rock_count {
        let ang = unit(&mut rng) * std::f32::consts::TAU;
        let rad = 3.2 + unit(&mut rng) * 6.0;
        let x = ang.cos() * rad;
        let z = ang.sin() * rad;
        let scale = 0.7 + unit(&mut rng) * 0.5;
        // Alternate rock / bush — still within 4 mesh types.
        let kind = if next(&mut rng) % 3 == 0 {
            ScatterKind::Bush
        } else {
            ScatterKind::Rock
        };
        let y = match kind {
            ScatterKind::Bush => 0.35 * scale,
            _ => 0.22 * scale,
        };
        out.push(ScatterSpot {
            x,
            z,
            y,
            scale,
            kind,
        });
    }

    // Hard cull: never more than 12 placed meshes.
    out.truncate(12);
    out
}

/// Fog linear start/end from stress (and harmony). World Camera3d only.
pub fn fog_falloff_from_climate(stress: f32, harmony: f32, regen: f32) -> (f32, f32) {
    let s = stress.clamp(0.0, 1.0);
    let h = harmony.clamp(0.0, 1.0);
    let r = regen.clamp(0.0, 1.0);
    let fog_start = 5.5 + (1.0 - s) * 10.0 + r * 2.0;
    let fog_end = 26.0 + (1.0 - s) * 24.0 + h * 8.0;
    (fog_start, fog_end.max(fog_start + 8.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_flag_off_by_default() {
        assert_eq!(parse_powrush_gen_from(None), PowrushGen::Off);
        assert!(!light_gen_enabled_from(None));
        assert!(!light_gen_enabled_from(Some("")));
        assert!(!light_gen_enabled_from(Some("off")));
        assert!(!light_gen_enabled_from(Some("garbage")));
        // Explicit refuse of accidental "gen=on" as product default spelling —
        // "on" still enables for steward F3, but unset stays off.
        assert_eq!(parse_powrush_gen_from(Some("light")), PowrushGen::Light);
        assert!(light_gen_enabled_from(Some("light")));
        assert!(light_gen_enabled_from(Some("ON")));
        assert!(light_gen_enabled_from(Some("1")));
    }

    #[test]
    fn light_gen_or_settings_with_env() {
        // Both off → off
        assert!(!light_gen_enabled_or(None, None));
        assert!(!light_gen_enabled_or(Some("off"), Some("off")));
        assert!(!light_gen_enabled_or(Some("garbage"), None));
        // Settings light alone → on
        assert!(light_gen_enabled_or(Some("light"), None));
        assert!(light_gen_enabled_or(Some("LIGHT"), Some("off")));
        // Env light alone → on
        assert!(light_gen_enabled_or(Some("off"), Some("light")));
        assert!(light_gen_enabled_or(None, Some("1")));
        // Both light → on
        assert!(light_gen_enabled_or(Some("light"), Some("light")));
    }

    #[test]
    fn seed_stable_same_triple() {
        let a = grove_seed(42, 7, 3);
        let b = grove_seed(42, 7, 3);
        assert_eq!(a, b);
        let c = grove_seed(42, 7, 4);
        assert_ne!(a, c);
        let d = grove_seed(43, 7, 3);
        assert_ne!(a, d);
        let e = grove_seed_from("Unnamed House", "local-hex", 0.55, 0.15);
        let f = grove_seed_from("Unnamed House", "local-hex", 0.55, 0.15);
        assert_eq!(e, f);
        // Same harmony/stress band → same epoch → same seed.
        let g = grove_seed_from("Unnamed House", "local-hex", 0.54, 0.16);
        assert_eq!(e, g);
    }

    #[test]
    fn cull_when_plate_open_helper() {
        assert!(!cull_gen_when_plate_open(false, false, false, false));
        assert!(cull_gen_when_plate_open(true, false, false, false));
        assert!(cull_gen_when_plate_open(false, true, false, false));
        assert!(cull_gen_when_plate_open(false, false, true, false));
        assert!(cull_gen_when_plate_open(false, false, false, true));
        assert!(cull_gen_when_plate_open(true, true, true, true));
    }

    #[test]
    fn scatter_capped_and_deterministic() {
        let seed = grove_seed(1, 2, 3);
        let a = scatter_from_seed(seed);
        let b = scatter_from_seed(seed);
        assert_eq!(a.len(), b.len());
        assert!(a.len() <= 12);
        assert!(!a.is_empty());
        for (x, y) in a.iter().zip(b.iter()) {
            assert_eq!(x.kind, y.kind);
            assert!((x.x - y.x).abs() < 1e-5);
            assert!((x.z - y.z).abs() < 1e-5);
        }
        let kinds: std::collections::HashSet<_> = a.iter().map(|s| s.kind).collect();
        assert!(kinds.len() <= 4);
        let other = scatter_from_seed(grove_seed(9, 9, 9));
        // Different seed should usually disagree on first spot (probabilistic but FNV-stable).
        assert!(
            a.first().map(|s| (s.x, s.z)) != other.first().map(|s| (s.x, s.z))
                || a.len() != other.len()
        );
    }

    #[test]
    fn fog_closes_when_stressed() {
        let (lo_s, lo_e) = fog_falloff_from_climate(0.1, 0.8, 0.1);
        let (hi_s, hi_e) = fog_falloff_from_climate(0.9, 0.2, 0.05);
        assert!(hi_s < lo_s);
        assert!(hi_e < lo_e);
        assert!(hi_e > hi_s);
    }
}
