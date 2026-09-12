//! Mesh LOD integration for Powrush-MMO — GraphicsPreset → fidelity tiers.
//!
//! CARD **H-2026-09-12-MESH-LOD** · law [`docs/MESH_QUALITY_BUDGET.md`]
//!
//! | Tier | Mesh / feel |
//! |---|---|
//! | Low | primitives / capsule-safe LOD · reduced detail |
//! | Medium (default) | balanced humanoid / Place props |
//! | High | fuller PersonaCommit dress when optional assets exist |
//!
//! Prefer procedural / existing `assets/models/*.glb` only if already present.
//! No new Quellorian art-pack. No binary dump in this PR.
//! UI copy never shows placeholder asset paths.
//! Title Online stays grey.
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
}
