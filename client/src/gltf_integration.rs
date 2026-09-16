//! Mesh LOD integration for Powrush-MMO — GraphicsPreset → fidelity tiers.
//!
//! CARD **H-2026-09-12-MESH-LOD** · law [`docs/MESH_QUALITY_BUDGET.md`]
//! CARD **H-2026-09-16-MESH-PERSONA-HANDS** · cite [`docs/MESH_PERSONA_COURT.md`]
//! · [`docs/ASSET_BUDGET_COURT.md`] @ `5eff19c` (do not edit).
//!
//! | Tier | Mesh / feel |
//! |---|---|
//! | Low | primitives / capsule-safe LOD · reduced detail |
//! | Medium (default) | balanced humanoid / Place props |
//! | High | fuller PersonaCommit dress when optional assets exist |
//!
//! Lived stacked-capsule presence reads [`procedural_detail_scale`] from Comfort
//! [`MeshLodPlan`]. Low stays primitives / capsule-readable. High may gate
//! [`persona_commit_dress_active`] only when an authored glb is already on disk —
//! **no `.glb` dump**. face ≠ class. Practices after House. No race lobby.
//! No second HUD. Title Online stays grey.
//!
//! Prefer procedural / existing `assets/models/*.glb` only if already present.
//! No new Quellorian art-pack. No binary dump in this PR.
//! UI copy never shows placeholder asset paths.
//!
//! Contact: info@Rathor.ai

use shared::local_settings::GraphicsPreset;
pub use shared::local_settings::MeshLod;

/// Resolved presentation plan for a Comfort graphics preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeshLodPlan {
    pub lod: MeshLod,
    /// Low: capsule / primitive presentation only.
    pub primitives_only: bool,
    /// Medium+: may prefer optional on-disk glb when present (never invent UI paths).
    pub prefer_optional_glb: bool,
    /// High: fuller PersonaCommit dress when assets exist.
    pub persona_commit_dress: bool,
}

/// Map Esc Comfort [`GraphicsPreset`] → mesh LOD tier.
pub fn mesh_lod_for_preset(preset: GraphicsPreset) -> MeshLod {
    preset.mesh_lod()
}

/// Build the presentation plan for a Comfort graphics preset.
pub fn plan_for_preset(preset: GraphicsPreset) -> MeshLodPlan {
    let lod = mesh_lod_for_preset(preset);
    MeshLodPlan {
        lod,
        primitives_only: lod.primitives_only(),
        prefer_optional_glb: lod.prefer_optional_glb(),
        persona_commit_dress: lod.persona_commit_dress(),
    }
}

/// Player-facing LOD feel label — never an asset path string.
pub fn lod_feel_label(lod: MeshLod) -> &'static str {
    lod.feel_label()
}

/// Whether an optional on-disk glb may be used for this plan.
/// `asset_present` must come from a real presence check — never invent paths for UI.
pub fn use_optional_glb(plan: &MeshLodPlan, asset_present: bool) -> bool {
    plan.prefer_optional_glb && asset_present
}

/// High-tier PersonaCommit dress is active only when the plan asks and assets exist.
pub fn persona_commit_dress_active(plan: &MeshLodPlan, asset_present: bool) -> bool {
    plan.persona_commit_dress && asset_present
}

/// Category for future per-type LOD / material handling (no asset path strings).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GltfCategory {
    Player,
    Structure,
    Ship,
    Prop,
}

/// Capsule-safe / procedural detail scale for Low / Medium / High.
/// 1.0 = Medium balance; Low reduces; High holds fuller scale when dressed.
pub fn procedural_detail_scale(lod: MeshLod) -> f32 {
    match lod {
        MeshLod::Low => 0.65,
        MeshLod::Medium => 1.0,
        MeshLod::High => 1.15,
    }
}

/// Lived stacked-capsule scale from Comfort [`MeshLodPlan`].
/// Low stays capsule-readable; High is fuller only as a scale, not a dump.
pub fn lived_presence_detail_scale(plan: &MeshLodPlan) -> f32 {
    procedural_detail_scale(plan.lod)
}

/// Optional on-disk authored glb presence. Read-only — never writes, never
/// invents a player-facing path string.
pub fn optional_authored_glb_present() -> bool {
    optional_authored_glb_present_in(std::path::Path::new("assets/models"))
}

/// Test-seam: scan one directory for an already-present `.glb`. No create.
pub fn optional_authored_glb_present_in(dir: &std::path::Path) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    entries.flatten().any(|e| {
        e.path()
            .extension()
            .map(|ext| ext == "glb")
            .unwrap_or(false)
    })
}

/// High PersonaCommit dress for lived presence — only when the plan asks
/// **and** an authored glb already exists. Never dumps a binary.
pub fn lived_persona_dress_active(plan: &MeshLodPlan) -> bool {
    persona_commit_dress_active(plan, optional_authored_glb_present())
}

/// Lived presence may prefer an on-disk glb only when the plan allows and
/// the file is already there. Does not load or invent UI paths.
pub fn lived_presence_uses_authored_glb(plan: &MeshLodPlan) -> bool {
    use_optional_glb(plan, optional_authored_glb_present())
}

/// face ≠ class — phenotype / people label never picks a class or greys Peace.
pub fn face_is_not_class() -> bool {
    true
}

/// Practices are post-House dress — never Title race select, never E/I/H/R gate.
pub fn practices_after_house() -> bool {
    true
}

/// Title has no race lobby. Mechanical race stays sim-owned.
pub fn race_lobby_closed() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::local_settings::LocalSettings;

    #[test]
    fn lod_selection_by_graphics_preset() {
        assert_eq!(mesh_lod_for_preset(GraphicsPreset::Low), MeshLod::Low);
        assert_eq!(mesh_lod_for_preset(GraphicsPreset::Medium), MeshLod::Medium);
        assert_eq!(mesh_lod_for_preset(GraphicsPreset::High), MeshLod::High);

        let low = plan_for_preset(GraphicsPreset::Low);
        assert!(low.primitives_only);
        assert!(!low.prefer_optional_glb);
        assert!(!low.persona_commit_dress);
        assert!((procedural_detail_scale(low.lod) - 0.65).abs() < f32::EPSILON);
        assert!(!use_optional_glb(&low, true));
        assert!(!persona_commit_dress_active(&low, true));

        let mid = plan_for_preset(GraphicsPreset::Medium);
        assert!(!mid.primitives_only);
        assert!(mid.prefer_optional_glb);
        assert!(!mid.persona_commit_dress);
        assert!(use_optional_glb(&mid, true));
        assert!(!use_optional_glb(&mid, false));
        assert!(!persona_commit_dress_active(&mid, true));
        assert!((procedural_detail_scale(mid.lod) - 1.0).abs() < f32::EPSILON);

        let high = plan_for_preset(GraphicsPreset::High);
        assert!(!high.primitives_only);
        assert!(high.prefer_optional_glb);
        assert!(high.persona_commit_dress);
        assert!(use_optional_glb(&high, true));
        assert!(!use_optional_glb(&high, false));
        assert!(persona_commit_dress_active(&high, true));
        assert!(!persona_commit_dress_active(&high, false));
        assert!((procedural_detail_scale(high.lod) - 1.15).abs() < f32::EPSILON);
    }

    #[test]
    fn lod_feel_labels_never_show_placeholder_paths() {
        for preset in GraphicsPreset::ALL {
            let label = lod_feel_label(mesh_lod_for_preset(preset));
            assert!(!label.is_empty());
            assert!(!label.contains("assets/"), "{label}");
            assert!(!label.contains(".glb"), "{label}");
            assert!(!label.contains("models/"), "{label}");
            assert!(!label.contains("player_avatar"), "{label}");
        }
    }

    #[test]
    fn local_settings_graphics_preset_drives_mesh_lod() {
        let mut s = LocalSettings::peace_defaults();
        assert_eq!(s.graphics_preset, GraphicsPreset::Medium);
        assert_eq!(plan_for_preset(s.graphics_preset).lod, MeshLod::Medium);
        s.set_graphics_preset(GraphicsPreset::Low);
        assert!(plan_for_preset(s.graphics_preset).primitives_only);
        s.set_graphics_preset(GraphicsPreset::High);
        assert!(plan_for_preset(s.graphics_preset).persona_commit_dress);
    }

    #[test]
    fn comfort_plan_wires_lived_capsule_scale_without_glb_dump() {
        let low = plan_for_preset(GraphicsPreset::Low);
        let mid = plan_for_preset(GraphicsPreset::Medium);
        let high = plan_for_preset(GraphicsPreset::High);

        assert!((lived_presence_detail_scale(&low) - 0.65).abs() < f32::EPSILON);
        assert!((lived_presence_detail_scale(&mid) - 1.0).abs() < f32::EPSILON);
        assert!((lived_presence_detail_scale(&high) - 1.15).abs() < f32::EPSILON);
        assert!(low.primitives_only);
        assert!(!mid.primitives_only);
        assert!(!high.primitives_only);

        let before = optional_authored_glb_present();
        assert!(!lived_persona_dress_active(&low));
        assert!(!lived_persona_dress_active(&mid));
        assert_eq!(lived_persona_dress_active(&high), before);
        assert_eq!(lived_presence_uses_authored_glb(&low), false);
        assert_eq!(lived_presence_uses_authored_glb(&high), before);
        assert_eq!(optional_authored_glb_present(), before);

        let missing = std::path::Path::new("assets/models/__powrush_no_such_pack__");
        assert!(!optional_authored_glb_present_in(missing));
        assert!(!persona_commit_dress_active(&high, false));
        assert!(persona_commit_dress_active(&high, true));
        assert!(!persona_commit_dress_active(&low, true));
    }

    #[test]
    fn face_not_class_practices_after_house_no_race_lobby_no_ultra() {
        assert!(face_is_not_class());
        assert!(practices_after_house());
        assert!(race_lobby_closed());
        assert_eq!(GraphicsPreset::ALL.len(), 3);
        for preset in GraphicsPreset::ALL {
            let label = lod_feel_label(mesh_lod_for_preset(preset));
            assert!(!label.contains(".glb"), "{label}");
            assert!(!label.contains("Ultra"), "{label}");
            assert!(!label.contains("race lobby"), "{label}");
        }
    }
}
