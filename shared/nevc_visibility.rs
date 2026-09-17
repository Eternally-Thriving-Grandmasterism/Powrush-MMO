// shared/nevc_visibility.rs
// Phase 10 — Visibility Surfaces Helpers
//
// Formats NevcSummary for UI / Steam overlay / operator consoles.
// Never invents a third contribution class.
//
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
// Thunder locked in. Yoi ⚡

use crate::contribution_events::ContributionEvent;
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
/// The class-score field is a stewardship / harm gate, not an abundance score / wages.
pub fn status_line(summary: &NevcSummary) -> String {
    format!(
        "NEVC: {} | stewardship / harm gate={:.3} | samples={} | valence={:.6} | grief={:.3}",
        summary.label, summary.score, summary.sample_count, summary.mean_valence, summary.total_grief
    )
}

/// Raw alignment / craft from the event. Display only — never feeds class or recovery.
pub fn stewardship_quality(event: &ContributionEvent) -> f64 {
    event.stewardship_quality()
}

/// Two-channel HUD line: class score + raw event craft.
pub fn status_line_channels(summary: &NevcSummary, stewardship_quality: f64) -> String {
    format!(
        "NEVC: {} | stewardship / harm gate={:.3} | stewardship quality={:.3} | samples={} | valence={:.6} | grief={:.3}",
        summary.label,
        summary.score,
        stewardship_quality,
        summary.sample_count,
        summary.mean_valence,
        summary.total_grief
    )
}

/// Two-channel HUD line from the event that produced the score.
pub fn status_line_for_event(summary: &NevcSummary, event: &ContributionEvent) -> String {
    status_line_channels(summary, stewardship_quality(event))
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
        ("stewardship / harm gate", format!("{:.3}", summary.score)),
        ("sample_count", summary.sample_count.to_string()),
        ("mean_valence", format!("{:.6}", summary.mean_valence)),
        ("total_grief", format!("{:.3}", summary.total_grief)),
    ]
}

/// Two-channel panel fields: class score plus display-only raw craft.
pub fn panel_fields_channels(
    summary: &NevcSummary,
    stewardship_quality: f64,
) -> [(&'static str, String); 6] {
    [
        ("label", summary.label.to_string()),
        ("stewardship / harm gate", format!("{:.3}", summary.score)),
        ("stewardship quality", format!("{:.3}", stewardship_quality)),
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
        assert!(status_line(&s).contains("stewardship / harm gate"));
        assert!(!status_line(&s).contains("abundance score"));
        assert_eq!(panel_fields(&s)[1].0, "stewardship / harm gate");
    }

    #[test]
    fn zombie_badge() {
        let r = score_instant(0.0, 2.0);
        let s = summary_from_result(&r);
        assert_eq!(badge_text(s.class), "Zombie");
    }

    /// VALENCE-HUD-2: second channel is raw event craft, not the floor-lifted valence.
    #[test]
    fn second_channel_shows_raw_alignment_not_floor() {
        use crate::contribution_events::{apply_event, ContributionEvent};
        use crate::contribution_ledger::ContributionLedger;
        use crate::nevc_adapter::ContributionClass;
        use crate::nevc_game_loop::{harvest_to_event, HarvestNevcInput};

        let cfg = NevcConfig::default();
        assert_eq!(cfg.valence_floor, 0.999999);
        assert_eq!(cfg.positive_weight, 1.0);

        let fail = harvest_to_event(&HarvestNevcInput::from_harvest(1, false, true, true));
        let regen = harvest_to_event(&HarvestNevcInput::from_harvest(1, true, true, true));
        let sustain = harvest_to_event(&HarvestNevcInput::from_harvest(1, true, true, false));
        let waste = harvest_to_event(&HarvestNevcInput::from_harvest(1, true, false, false));
        match (fail, regen, sustain, waste) {
            (
                ContributionEvent::RbeAction {
                    abundance_alignment: 0.2,
                    waste_or_harm: 0.5,
                    ..
                },
                ContributionEvent::RbeAction {
                    abundance_alignment: 1.0,
                    waste_or_harm: 0.0,
                    ..
                },
                ContributionEvent::RbeAction {
                    abundance_alignment: 0.85,
                    waste_or_harm: 0.05,
                    ..
                },
                ContributionEvent::RbeAction {
                    abundance_alignment: 0.1,
                    waste_or_harm: 1.5,
                    ..
                },
            ) => {}
            _ => panic!("harvest table must stay NEVC-HONEST-1 values"),
        }

        let event = ContributionEvent::RbeAction {
            player_id: 21,
            abundance_alignment: 0.0,
            waste_or_harm: 0.0,
        };
        assert_eq!(stewardship_quality(&event), 0.0);

        let mut ledger = ContributionLedger::new();
        let r = apply_event(&mut ledger, event.clone());
        let s = summary_from_result(&r);
        assert_eq!(s.class, ContributionClass::ActiveEternalContributor);
        assert!(r.recovery_open());
        assert!(s.score > 0.0);
        assert_eq!(ledger.last_stewardship_quality(21), Some(0.0));

        let line = status_line_for_event(&s, &event);
        assert!(line.contains("stewardship / harm gate"));
        assert!(line.contains("stewardship quality=0.000"));
        assert!(!line.contains("abundance score"));
        let gate = format!("stewardship / harm gate={:.3}", s.score);
        assert!(line.contains(&gate));
        assert_ne!(s.score, 0.0, "class score is the gate, not raw a");

        let fields = panel_fields_channels(&s, stewardship_quality(&event));
        assert_eq!(fields[1].0, "stewardship / harm gate");
        assert_eq!(fields[1].1, format!("{:.3}", s.score));
        assert_eq!(fields[2].0, "stewardship quality");
        assert_eq!(fields[2].1, "0.000");

        let raw = ContributionEvent::RawSample {
            player_id: 22,
            valence: 0.42,
            grief_load: 0.0,
        };
        assert_eq!(stewardship_quality(&raw), 0.42);
        let raw_r = apply_event(&mut ledger, raw.clone());
        assert_eq!(raw_r.class, ContributionClass::ZombiePartition);
        assert!(raw_r.recovery_open());
        assert_eq!(ledger.last_stewardship_quality(22), Some(0.42));
        let raw_line = status_line_for_event(&summary_from_result(&raw_r), &raw);
        assert!(raw_line.contains("stewardship quality=0.420"));
    }
}
