//! Lived-hour teaching climates — Slice 18 (v23.2.25)
//!
//! Witness mercy-restore vs extract-only on the existing well slab.
//! Does not rewrite harvest_feel.
//!
//! CARD FLESH-CLIMATE-SCRIPT — the existing mercy / extract sentences may
//! name the Place (`HexTravelState::chip_name`). Absent travel keeps each
//! bare line. No climate HUD. Fog / Low caps stay outside this file.
//! Peak memory, cited: walked · tended · week was the bill · yard remembered.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::climate_script::{extract_line, extract_only_holds, mercy_line, mercy_restore_holds};

use crate::lived_hour_bind::LivedHourBind;

#[derive(Resource, Debug, Default)]
pub struct TeachingClaim {
    pub extract_seen: bool,
    pub mercy_seen: bool,
    pub node_id: Option<u32>,
    pub line: Option<&'static str>,
}

impl TeachingClaim {
    pub fn sentence_for(&self, climate_id: u32) -> Option<&'static str> {
        if self.node_id == Some(climate_id) {
            self.line
        } else {
            None
        }
    }
}

/// CARD FLESH-CLIMATE-SCRIPT — `{place} · {bare}` for the three chip names.
/// Any other place, or no travel, returns `bare` byte for byte.
fn climate_script_line(bare: &'static str, place: Option<&str>) -> &'static str {
    match place {
        Some("Sanctuary Prime") if bare == mercy_line() => "Sanctuary Prime · flow restored the well",
        Some("Sanctuary Prime") if bare == extract_line() => "Sanctuary Prime · extract left it tired",
        Some("Heartwood") if bare == mercy_line() => "Heartwood · flow restored the well",
        Some("Heartwood") if bare == extract_line() => "Heartwood · extract left it tired",
        Some("Depths") if bare == mercy_line() => "Depths · flow restored the well",
        Some("Depths") if bare == extract_line() => "Depths · extract left it tired",
        _ => bare,
    }
}

pub struct ClimateScriptPlugin;

impl Plugin for ClimateScriptPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TeachingClaim>()
            .add_systems(Update, witness_teaching_climates);
    }
}

fn witness_teaching_climates(
    mut bind: ResMut<LivedHourBind>,
    mut claim: ResMut<TeachingClaim>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
) {
    let place = travel.as_ref().map(|state| state.chip_name());
    let id = bind.focus_id.unwrap_or(1);
    if extract_only_holds(&bind.hour, id) {
        claim.extract_seen = true;
        if !claim.mercy_seen || claim.node_id != Some(id) {
            claim.node_id = Some(id);
            let line = climate_script_line(extract_line(), place);
            claim.line = Some(line);
            if bind.last_line != line {
                bind.last_line = line.to_string();
            }
        }
    }
    if mercy_restore_holds(&bind.hour, id) {
        claim.mercy_seen = true;
        claim.node_id = Some(id);
        let line = climate_script_line(mercy_line(), place);
        claim.line = Some(line);
        if bind.last_line != line {
            bind.last_line = line.to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::climate_script::{run_extract_only, run_mercy_restore};

    #[test]
    fn extract_script_is_a_sentence() {
        let hour = run_extract_only();
        assert!(extract_only_holds(&hour, 1));
        assert_eq!(extract_line(), "extract left it tired");
    }

    #[test]
    fn mercy_script_is_a_sentence() {
        let hour = run_mercy_restore();
        assert!(mercy_restore_holds(&hour, 1));
        assert_eq!(mercy_line(), "flow restored the well");
    }

    #[test]
    fn sentence_stays_on_the_well() {
        let mut claim = TeachingClaim {
            extract_seen: true,
            mercy_seen: false,
            node_id: Some(1),
            line: Some(extract_line()),
        };
        assert_eq!(claim.sentence_for(1), Some(extract_line()));
        assert_eq!(claim.sentence_for(2), None);
    }

    /// CARD FLESH-CLIMATE-SCRIPT — Place prefixes the slab sentence; no travel keeps it exact.
    #[test]
    fn flesh_climate_script_line_names_chip_or_keeps_bare_line() {
        use shared::hex_travel::PlaceId;

        assert_eq!(climate_script_line(mercy_line(), None), mercy_line());
        assert_eq!(climate_script_line(extract_line(), None), extract_line());
        assert_eq!(climate_script_line(mercy_line(), None), "flow restored the well");
        assert_eq!(
            climate_script_line(extract_line(), None),
            "extract left it tired"
        );

        let cases = [
            (PlaceId::Sanctuary, "Sanctuary Prime"),
            (PlaceId::Heartwood, "Heartwood"),
            (PlaceId::Depths, "Depths"),
        ];
        for (id, name) in cases {
            assert_eq!(id.chip_name(), name);
            let travel = crate::hex_travel::HexTravelState { current: id };
            assert_eq!(travel.chip_name(), id.chip_name());
            let mercy = climate_script_line(mercy_line(), Some(travel.chip_name()));
            let extract = climate_script_line(extract_line(), Some(id.chip_name()));
            assert_eq!(mercy, format!("{name} · flow restored the well"));
            assert_eq!(extract, format!("{name} · extract left it tired"));
            assert!(mercy.ends_with(mercy_line()));
            assert!(extract.ends_with(extract_line()));
        }

        for sample in [
            climate_script_line(mercy_line(), Some(PlaceId::Heartwood.chip_name())),
            climate_script_line(extract_line(), None),
        ] {
            let low = sample.to_lowercase();
            assert!(!low.contains("gold"), "{sample}");
            assert!(!low.contains("market"), "{sample}");
            assert!(!low.contains("xp"), "{sample}");
            assert!(!low.contains("hud"), "{sample}");
            assert!(!low.contains("ultra"), "{sample}");
            assert!(!sample.contains("Threshold"), "{sample}");
        }
    }
}
