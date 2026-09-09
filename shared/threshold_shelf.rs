//! U7 — Threshold shelf (look + tend).
//!
//! The Threshold is a local Heartwood roof shelf, not a place row or save
//! slot. Looking and tending change only session-local shelf state; the house
//! book and embassy remain in the house pack.

use crate::heartwood_lamp::{
    dist_xz, in_heartwood_lamp_disk, in_heartwood_water, try_place_building, LAMP_DISK_CENTER,
    LAMP_DISK_RADIUS, WATER_POND_CENTER, WATER_POND_RADIUS,
};
use crate::hex_travel::PlaceId;
use crate::hour_two::HourTwoPack;

pub const THRESHOLD_SHELF_CENTER: [f32; 3] = [0.0, 2.72, 6.0];
pub const THRESHOLD_SHELF_SIZE: [f32; 3] = [4.4, 0.18, 0.72];
pub const THRESHOLD_SHELF_FOOTPRINT_RADIUS: f32 = 2.23;
pub const THRESHOLD_SHELF_USE_RADIUS: f32 = 2.4;
pub const THRESHOLD_NODE_CENTER: [f32; 3] = [0.0, 3.08, 6.0];
pub const THRESHOLD_NODE_RADIUS: f32 = 0.34;
pub const THRESHOLD_PEACE_VERBS: [&str; 4] = ["Tend", "Flow", "Reserve", "Mend"];
pub const THRESHOLD_STRESSED_SIGMA: f32 = 0.55;

#[derive(Debug, Clone, PartialEq)]
pub struct ThresholdPeaceNode {
    /// Session-local tiredness. It never aliases persisted hex climate.
    pub sigma: f32,
    pub tends: u32,
}

impl Default for ThresholdPeaceNode {
    fn default() -> Self {
        Self {
            sigma: 0.15,
            tends: 0,
        }
    }
}

impl ThresholdPeaceNode {
    pub fn face(&self) -> &'static str {
        if self.sigma >= THRESHOLD_STRESSED_SIGMA {
            "Stressed"
        } else {
            "Idle"
        }
    }

    pub fn speech(&self) -> String {
        format!(
            "Threshold node · {} · Tend / Flow / Reserve / Mend",
            self.face()
        )
    }

    pub fn tend(&mut self) -> String {
        self.tends = self.tends.saturating_add(1);
        self.sigma = (self.sigma + 0.25).clamp(0.0, 1.0);
        self.speech()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdVerb {
    Look,
    Tend,
}

pub const THRESHOLD_VERBS: [ThresholdVerb; 2] = [ThresholdVerb::Look, ThresholdVerb::Tend];

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ThresholdShelfState {
    pub looked: bool,
    pub tends: u32,
}

impl ThresholdShelfState {
    pub fn apply(&mut self, verb: ThresholdVerb) -> &'static str {
        match verb {
            ThresholdVerb::Look => {
                self.looked = true;
                "Threshold shelf · E tend"
            }
            ThresholdVerb::Tend => {
                self.looked = true;
                self.tends = self.tends.saturating_add(1);
                "Threshold tended · the roof holds"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThresholdReceipt {
    pub line: &'static str,
    pub hour_three_complete: bool,
    pub embassy_seated: bool,
}

/// Session-only action. The immutable house argument makes replacing the house
/// embassy with a Heartwood stub impossible at this hook.
pub fn visit_threshold(
    shelf: &mut ThresholdShelfState,
    verb: ThresholdVerb,
    house: &HourTwoPack,
) -> ThresholdReceipt {
    let line = shelf.apply(verb);
    ThresholdReceipt {
        line,
        hour_three_complete: house.hour_three_complete,
        embassy_seated: house.embassy.seated,
    }
}

pub fn threshold_shelf_is_valid() -> bool {
    let x = THRESHOLD_SHELF_CENTER[0];
    let y = THRESHOLD_SHELF_CENTER[1];
    let z = THRESHOLD_SHELF_CENTER[2];
    y > 2.0
        && dist_xz(x, z, LAMP_DISK_CENTER[0], LAMP_DISK_CENTER[1])
            - THRESHOLD_SHELF_FOOTPRINT_RADIUS
            > LAMP_DISK_RADIUS
        && !in_heartwood_lamp_disk(x, z)
        && !in_heartwood_water(x, z)
        && dist_xz(x, z, WATER_POND_CENTER[0], WATER_POND_CENTER[1])
            > THRESHOLD_SHELF_FOOTPRINT_RADIUS + WATER_POND_RADIUS
        && try_place_building(PlaceId::Heartwood, x, z).is_ok()
}

pub fn threshold_node_is_valid() -> bool {
    let x = THRESHOLD_NODE_CENTER[0];
    let y = THRESHOLD_NODE_CENTER[1];
    let z = THRESHOLD_NODE_CENTER[2];
    y > THRESHOLD_SHELF_CENTER[1]
        && dist_xz(x, z, LAMP_DISK_CENTER[0], LAMP_DISK_CENTER[1]) - THRESHOLD_NODE_RADIUS
            > LAMP_DISK_RADIUS
        && dist_xz(x, z, WATER_POND_CENTER[0], WATER_POND_CENTER[1]) - THRESHOLD_NODE_RADIUS
            > WATER_POND_RADIUS
        && !in_heartwood_lamp_disk(x, z)
        && !in_heartwood_water(x, z)
        && try_place_building(PlaceId::Heartwood, x, z).is_ok()
}

pub fn threshold_is_market() -> bool {
    false
}

pub fn threshold_is_socket_or_public_bind() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex_protocol::Op;
    use crate::hex_travel::{confirm_leave, places_eligible, PlaceId};
    use crate::stranger_loop_proof::hour_three_held_fixture;
    use crate::week_audit::WeekAudit;

    #[test]
    fn shelf_fills_roof_outside_lamp_and_water() {
        assert!(threshold_shelf_is_valid());
        assert!(threshold_node_is_valid());
        assert_eq!(THRESHOLD_VERBS, [ThresholdVerb::Look, ThresholdVerb::Tend]);
        assert!(!threshold_is_market());
        assert!(!threshold_is_socket_or_public_bind());
    }

    #[test]
    fn peace_node_speaks_four_existing_verbs_from_sigma() {
        let mut node = ThresholdPeaceNode::default();
        assert_eq!(node.face(), "Idle");
        let idle = node.speech();
        for verb in THRESHOLD_PEACE_VERBS {
            assert!(idle.contains(verb), "speech omitted {verb}: {idle}");
        }
        assert_eq!(idle.matches('/').count(), THRESHOLD_PEACE_VERBS.len() - 1);
        assert_eq!(THRESHOLD_PEACE_VERBS.len(), 4);
        assert_eq!(Op::Take.as_str(), "take", "global Take remains available");
        assert!(!THRESHOLD_PEACE_VERBS.contains(&"Take"));

        let _ = node.tend();
        let stressed = node.tend();
        assert_eq!(node.face(), "Stressed");
        assert!(stressed.contains("Stressed"));
        assert!(!node.speech().contains("Credit"));
        assert!(!node.speech().contains("Lethal"));

        let mut week = WeekAudit::default();
        week.sync_from_climate(3, 2);
        assert!(week.slab_line().contains("3 tons"));
    }

    #[test]
    fn look_and_tend_keep_book_places_and_sanctuary_return() {
        let house = hour_three_held_fixture();
        let before = house.clone();
        let mut shelf = ThresholdShelfState::default();
        let mut node = ThresholdPeaceNode::default();

        let looked = visit_threshold(&mut shelf, ThresholdVerb::Look, &house);
        let tended = visit_threshold(&mut shelf, ThresholdVerb::Tend, &house);
        let speech = node.tend();

        assert!(shelf.looked);
        assert_eq!(shelf.tends, 1);
        assert_eq!(node.tends, 1);
        assert!(speech.contains("Tend / Flow / Reserve / Mend"));
        assert!(looked.hour_three_complete && tended.hour_three_complete);
        assert!(looked.embassy_seated && tended.embassy_seated);
        assert_eq!(house, before, "Threshold must not write the house book");
        assert!(places_eligible(house.complete, house.hour_three_complete));
        assert_eq!(
            confirm_leave(
                house.complete,
                house.hour_three_complete,
                PlaceId::Heartwood,
                PlaceId::Sanctuary,
            ),
            Ok(PlaceId::Sanctuary)
        );
    }
}
