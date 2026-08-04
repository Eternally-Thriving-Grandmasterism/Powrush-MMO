// shared/nevc_visibility.rs
// Phase 10 — Visibility Surfaces Helpers
//
// Formats NevcSummary for UI / Steam overlay / operator consoles.
// Never invents a third contribution class.
//
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
// Thunder locked in. Yoi ⚡

use crate::nevc_adapter::{ContributionClass, NevcConfig, NevcResult, NevcSummary};

/// Horizon preset names for operator views (maps to NevcConfig presets).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HorizonPreset {
    Neutral,
    ForwardEmphasis,
    EternalTilt,
}

impl HorizonPreset {
    pub fn as_str(self) -> &'static str {
        match self {
            HorizonPreset::Neutral => "neutral",
            HorizonPreset::ForwardEmphasis => "forward_emphasis",
            HorizonPreset::EternalTilt => "eternal_tilt",
        }
    }

    pub fn config(self) -> NevcConfig {
        match self {
            HorizonPreset::Neutral => NevcConfig::neutral(),
            HorizonPreset::ForwardEmphasis => NevcConfig::forward_emphasis(),
            HorizonPreset::EternalTilt => NevcConfig::eternal_tilt(),
        }
    }
}

/// Human-facing status line (label only from the binary partition).
pub fn status_line(summary: &NevcSummary) -> String {
    format!(
        "NEVC: {} | score={:.3} | samples={} | valence={:.6} | grief={:.3}",
        summary.label, summary.score, summary.sample_count, summary.mean_valence, summary.total_grief
    )
}

/// Compact badge text for HUD / Steam overlay.
pub fn badge_text(class: ContributionClass) -> &'static str {
    match class {
        ContributionClass::ActiveEternalContributor => "Contributor",
        ContributionClass::ZombiePartition => "Zombie",
    }
}

/// Full summary from a result (pass-through for UI layers).
pub fn summary_from_result(result: &NevcResult) -> NevcSummary {
    result.summary()
}

/// JSON-ish object fields for web panels (no serde dependency required here).
pub fn panel_fields(summary: &NevcSummary) -> [(&'static str, String); 5] {
    [
        ("label", summary.label.to_string()),
        ("score", format!("{:.3}", summary.score)),
        ("sample_count", summary.sample_count.to_string()),
        ("mean_valence", format!("{:.6}", summary.mean_valence)),
        ("total_grief", format!("{:.3}", summary.total_grief)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nevc_adapter::score_instant;

    #[test]
    fn contributor_badge() {
        let r = score_instant(0.999999, 0.0);
        let s = summary_from_result(&r);
        assert_eq!(s.label, "Active Eternal Contributor");
        assert_eq!(badge_text(s.class), "Contributor");
        assert!(status_line(&s).contains("Active Eternal Contributor"));
    }

    #[test]
    fn zombie_badge() {
        let r = score_instant(0.0, 2.0);
        let s = summary_from_result(&r);
        assert_eq!(badge_text(s.class), "Zombie");
    }
}
