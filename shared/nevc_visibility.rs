// shared/nevc_visibility.rs
// Phase 10 — Visibility Surfaces Helpers
//
// Formats NevcSummary for UI / Steam overlay / operator consoles.
// Never invents a third contribution class.
//
// CARD F3 HOSTILE-PRACTICE — NEVC labels display only (no wage invent).
// Hostile Take / refuse / embargo may show these labels. Never wages.
//
// CARD F9 NEVC-ON-STANCE — sealed Hostile Take writes a ledger row + NEVC
// display label (no wage). Sealed Open-trade Bind shows Reserve cue on the
// existing allocate path (no new verb). Garden light = neither.
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

/// CARD F3 — NEVC labels are display only. Hostile practice does not invent wages.
pub const NEVC_LABELS_ARE_WAGES: bool = false;
pub const NEVC_INVENTS_WAGES: bool = false;

/// CARD F3 — NEVC never pays. Hostile Take / refuse / embargo stay labels.
pub fn nevc_invents_wages() -> bool {
    NEVC_INVENTS_WAGES || NEVC_LABELS_ARE_WAGES
}

/// CARD F3 — Hostile practice may show NEVC labels. Display only — never wages.
pub fn hostile_practice_nevc_labels(summary: &NevcSummary) -> String {
    format!(
        "NEVC: {} | stewardship / harm gate={:.3} | display only | not wages",
        summary.label, summary.score
    )
}

/// CARD F9 — existing allocate Reserve cue. Not a new verb. Not wages.
pub const OPEN_TRADE_BIND_RESERVE_CUE: &str = "Reserve · repair-rights";

/// CARD F9 — sealed Hostile Take NEVC display label. Display only — never wages.
pub fn hostile_take_stance_nevc_label() -> String {
    let r = crate::nevc_adapter::score_instant(0.999999, 0.0);
    let s = summary_from_result(&r);
    hostile_practice_nevc_labels(&s)
}

/// CARD F9 — sealed Open-trade Bind Reserve cue on the existing allocate path.
/// Uses the banked allocate confirm when Reserve landed; else the existing path title.
pub fn open_trade_bind_reserve_cue(
    may_show: bool,
    allocation: &crate::climate_node::Allocation,
) -> Option<String> {
    if !may_show {
        return None;
    }
    Some(
        allocation
            .reserve_bank_line()
            .unwrap_or_else(|| OPEN_TRADE_BIND_RESERVE_CUE.to_string()),
    )
}

/// CARD F9 — garden light gets neither Hostile Take NEVC nor Open-trade Reserve cue.
pub fn garden_light_nevc_on_stance_label() -> Option<String> {
    None
}

/// CARD F9 — garden light gets no Reserve cue.
pub fn garden_light_reserve_cue() -> Option<String> {
    None
}

/// True when a line invents wage / gold / payout language. Denial copy is allowed.
pub fn nevc_line_invents_wages(line: &str) -> bool {
    let low = line.to_lowercase();
    if low.contains("not wages") || low.contains("display only") {
        return low.contains("payout")
            || low.contains("salary")
            || low.contains("gold")
            || low.contains("abundance score");
    }
    low.contains("wage")
        || low.contains("payout")
        || low.contains("salary")
        || low.contains("gold")
        || low.contains("abundance score")
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

    /// CARD F3 — NEVC labels display only (no wage invent).
    #[test]
    fn f3_nevc_labels_display_only_no_wage_invent() {
        assert!(!NEVC_LABELS_ARE_WAGES);
        assert!(!NEVC_INVENTS_WAGES);
        assert!(!nevc_invents_wages());
        assert!(!crate::persona::HOSTILE_PRACTICE_INVENTS_WAGES);

        let r = score_instant(0.999999, 0.0);
        let s = summary_from_result(&r);
        let line = hostile_practice_nevc_labels(&s);
        assert!(line.contains("NEVC:"));
        assert!(line.contains(s.label));
        assert!(line.contains("stewardship / harm gate"));
        assert!(line.contains("display only"));
        assert!(line.contains("not wages"));
        assert!(!nevc_line_invents_wages(&line));
        assert!(!line.contains("abundance score"));
        assert!(!status_line(&s).contains("wage"));
        assert_eq!(badge_text(s.class), "Contributor");
        assert_eq!(panel_fields(&s)[1].0, "stewardship / harm gate");
    }

    /// CARD F3 — sealed Hostile may Take / refuse / embargo without wage invent.
    #[test]
    fn f3_sealed_hostile_may_take_refuse_embargo() {
        use crate::persona::{
            sealed_hostile_may_embargo, sealed_hostile_may_refuse, sealed_hostile_may_take,
            HostilePractice, SoulStance,
        };

        let hostile = Some(SoulStance::Hostile);
        assert!(sealed_hostile_may_take(true, hostile));
        assert!(sealed_hostile_may_refuse(true, hostile));
        assert!(sealed_hostile_may_embargo(true, hostile));
        for verb in HostilePractice::ALL {
            let _ = verb;
            assert!(!nevc_invents_wages());
        }
    }

    /// CARD F3 — no lockout · soul stays playable under Hostile.
    #[test]
    fn f3_no_lockout_soul_stays_playable_under_hostile() {
        use crate::persona::{
            soul_stays_playable_under_hostile, SoulStance, HOSTILE_LOCKOUT,
        };

        assert!(!HOSTILE_LOCKOUT);
        assert!(soul_stays_playable_under_hostile(
            true,
            Some(SoulStance::Hostile)
        ));
        let r = score_instant(0.0, 2.0);
        let s = summary_from_result(&r);
        assert!(!nevc_line_invents_wages(&hostile_practice_nevc_labels(&s)));
    }

    /// CARD F3 — PlaceId / LOCAL_HEXES len == 3.
    #[test]
    fn f3_place_id_local_hexes_len_three() {
        use crate::hex_travel::{PlaceId, LOCAL_HEXES};
        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
    }

    /// CARD F3 — STEWARD_ONLINE_YES false / Online grey.
    #[test]
    fn f3_steward_online_yes_false_online_grey() {
        use crate::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};
        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
    }

    /// CARD F3 — 0 meshes · no auction_*.rs · no new persist file.
    #[test]
    fn f3_zero_meshes_no_auction_rs_no_new_persist_file() {
        use std::path::Path;

        assert_eq!(crate::persona::F3_MESH_BUDGET, 0);
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(!here.join("auction.rs").exists());
        assert!(!here.join("nevc_wages.rs").exists());
        assert_eq!(crate::persona::PERSONA_PATH, "data/powrush_persona.json");
        assert_ne!(crate::persona::PERSONA_FILE_NAME, "powrush_hostile.json");
    }

    /// CARD F9 — sealed Hostile Take writes a ledger row + NEVC display label (no wage).
    #[test]
    fn f9_sealed_hostile_take_writes_ledger_row_nevc_label_no_wage() {
        use crate::persona::{
            sealed_hostile_take_writes_ledger_nevc, SoulStance, HOSTILE_PRACTICE_INVENTS_WAGES,
        };

        assert!(!HOSTILE_PRACTICE_INVENTS_WAGES);
        assert!(!nevc_invents_wages());
        assert!(sealed_hostile_take_writes_ledger_nevc(
            true,
            Some(SoulStance::Hostile)
        ));
        let line = hostile_take_stance_nevc_label();
        assert!(line.contains("NEVC:"));
        assert!(line.contains("stewardship / harm gate"));
        assert!(line.contains("display only"));
        assert!(line.contains("not wages"));
        assert!(!nevc_line_invents_wages(&line));
        assert!(!line.contains("abundance score"));
        assert!(!line.contains("gold"));
    }

    /// CARD F9 — sealed Open-trade Bind shows Reserve cue on existing allocate path.
    #[test]
    fn f9_sealed_open_trade_bind_shows_reserve_cue_on_allocate_path() {
        use crate::climate_node::Allocation;
        use crate::persona::{
            f9_invents_new_verb, sealed_open_trade_bind_shows_reserve_cue, SoulStance,
        };

        assert!(!f9_invents_new_verb());
        assert!(sealed_open_trade_bind_shows_reserve_cue(
            true,
            Some(SoulStance::OpenTrade)
        ));
        let mut allocation = Allocation::default();
        allocation.reserve = 1;
        let cue = open_trade_bind_reserve_cue(true, &allocation).expect("reserve cue");
        assert_eq!(cue, allocation.reserve_bank_line().expect("banked"));
        assert!(cue.contains("Reserve"));
        assert!(cue.contains("repair-rights"));
        assert!(!nevc_line_invents_wages(&cue));
        assert_eq!(
            open_trade_bind_reserve_cue(true, &Allocation::default()).as_deref(),
            Some(OPEN_TRADE_BIND_RESERVE_CUE)
        );
        assert!(open_trade_bind_reserve_cue(false, &allocation).is_none());
    }

    /// CARD F9 — garden light gets neither.
    #[test]
    fn f9_garden_light_gets_neither() {
        use crate::climate_node::Allocation;
        use crate::persona::{
            garden_light_gets_nevc_on_stance, garden_light_stance,
            sealed_hostile_take_writes_ledger_nevc, sealed_open_trade_bind_shows_reserve_cue,
        };

        assert!(!garden_light_gets_nevc_on_stance());
        assert!(garden_light_nevc_on_stance_label().is_none());
        assert!(garden_light_reserve_cue().is_none());
        assert!(!sealed_hostile_take_writes_ledger_nevc(
            false,
            garden_light_stance()
        ));
        assert!(!sealed_open_trade_bind_shows_reserve_cue(
            false,
            garden_light_stance()
        ));
        let mut allocation = Allocation::default();
        allocation.reserve = 1;
        assert!(open_trade_bind_reserve_cue(false, &allocation).is_none());
    }

    /// CARD F9 — PlaceId / LOCAL_HEXES len == 3.
    #[test]
    fn f9_place_id_local_hexes_len_three() {
        use crate::hex_travel::{PlaceId, LOCAL_HEXES};
        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
    }

    /// CARD F9 — STEWARD_ONLINE_YES false / Online grey.
    #[test]
    fn f9_steward_online_yes_false_online_grey() {
        use crate::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};
        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
    }

    /// CARD F9 — no lockout · 0 meshes · no auction_*.rs · no new persist file.
    #[test]
    fn f9_no_lockout_zero_meshes_no_auction_rs_no_new_persist_file() {
        use std::path::Path;

        use crate::persona::{f9_invents_new_verb, SoulStance, F9_MESH_BUDGET, HOSTILE_LOCKOUT};

        assert!(!HOSTILE_LOCKOUT);
        assert!(!crate::persona::soul_is_locked_out(
            true,
            Some(SoulStance::Hostile)
        ));
        assert_eq!(F9_MESH_BUDGET, 0);
        assert!(!f9_invents_new_verb());
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(!here.join("auction.rs").exists());
        assert!(!here.join("nevc_wages.rs").exists());
        assert!(!here.join("nevc_stance.rs").exists());
        assert_eq!(crate::persona::PERSONA_PATH, "data/powrush_persona.json");
        assert_ne!(crate::persona::PERSONA_FILE_NAME, "powrush_nevc_stance.json");
    }
}
