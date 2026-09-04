//! Lived-hour teaching climates — Slice 18 (v23.2.25)
//!
//! Witness mercy-restore vs extract-only on the existing well slab.
//! Does not rewrite harvest_feel. Contact: info@Rathor.ai

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

pub struct ClimateScriptPlugin;

impl Plugin for ClimateScriptPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TeachingClaim>()
            .add_systems(Update, witness_teaching_climates);
    }
}

fn witness_teaching_climates(mut bind: ResMut<LivedHourBind>, mut claim: ResMut<TeachingClaim>) {
    let id = bind.focus_id.unwrap_or(1);
    if extract_only_holds(&bind.hour, id) {
        claim.extract_seen = true;
        if !claim.mercy_seen || claim.node_id != Some(id) {
            claim.node_id = Some(id);
            claim.line = Some(extract_line());
            if bind.last_line != extract_line() {
                bind.last_line = extract_line().to_string();
            }
        }
    }
    if mercy_restore_holds(&bind.hour, id) {
        claim.mercy_seen = true;
        claim.node_id = Some(id);
        claim.line = Some(mercy_line());
        if bind.last_line != mercy_line() {
            bind.last_line = mercy_line().to_string();
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
}
