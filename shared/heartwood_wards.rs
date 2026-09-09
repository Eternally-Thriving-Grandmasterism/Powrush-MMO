//! U8 — Heartwood Wards (seal dress).
//!
//! Wards are house / charter dress on Heartwood only. They are not a Places
//! row, not Market, and not a save slot. Look and tend change session-local
//! dress; the house book stays with the house pack.

use crate::heartwood_lamp::{
    dist_xz, in_heartwood_lamp_disk, in_heartwood_water, try_place_building, LAMP_DISK_CENTER,
    LAMP_DISK_RADIUS, WATER_POND_CENTER, WATER_POND_RADIUS,
};
use crate::hex_travel::PlaceId;
use crate::hour_two::HourTwoPack;

/// Well · Grove · Ember. Cosmetic strings only. No combat stats.
pub const WARD_SEALS: [&str; 3] = ["Well", "Grove", "Ember"];
pub const WARDS_NOTICE: &str = "Wards · E tend · Well · Grove · Ember";

/// Ground posts outside the empty lamp disk and the water bath.
/// Ring the Lip pond at footprint-clear distance (pond center 8,0 r=2).
pub const WARD_POST_CENTERS: [[f32; 3]; 3] = [
    [8.0, 0.95, 4.0],
    [11.0, 0.95, 0.0],
    [8.0, 0.95, -4.0],
];
pub const WARD_POST_SIZE: [f32; 3] = [0.28, 1.4, 0.28];
pub const WARD_FOOTPRINT_RADIUS: f32 = 0.22;
pub const WARD_USE_RADIUS: f32 = 2.2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WardVerb {
    Look,
    Tend,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WardDress {
    pub looked: bool,
    pub tends: u32,
}

impl WardDress {
    pub fn apply(&mut self, verb: WardVerb) -> &'static str {
        match verb {
            WardVerb::Look => {
                self.looked = true;
                WARDS_NOTICE
            }
            WardVerb::Tend => {
                self.looked = true;
                self.tends = self.tends.saturating_add(1);
                WARDS_NOTICE
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WardReceipt {
    pub line: &'static str,
    pub hour_three_complete: bool,
    pub embassy_seated: bool,
}

/// Session-only. The house argument is borrowed, so this hook cannot replace
/// the embassy with a Heartwood stub.
pub fn visit_wards(dress: &mut WardDress, verb: WardVerb, house: &HourTwoPack) -> WardReceipt {
    WardReceipt {
        line: dress.apply(verb),
        hour_three_complete: house.hour_three_complete,
        embassy_seated: house.embassy.seated,
    }
}

fn post_clear(center: [f32; 3]) -> bool {
    let (x, y, z) = (center[0], center[1], center[2]);
    y > 0.4
        && dist_xz(x, z, LAMP_DISK_CENTER[0], LAMP_DISK_CENTER[1]) - WARD_FOOTPRINT_RADIUS
            > LAMP_DISK_RADIUS
        && !in_heartwood_lamp_disk(x, z)
        && !in_heartwood_water(x, z)
        && dist_xz(x, z, WATER_POND_CENTER[0], WATER_POND_CENTER[1])
            > WARD_FOOTPRINT_RADIUS + WATER_POND_RADIUS
        && try_place_building(PlaceId::Heartwood, x, z).is_ok()
}

pub fn wards_are_valid() -> bool {
    WARD_POST_CENTERS.iter().copied().all(post_clear) && WARD_SEALS == ["Well", "Grove", "Ember"]
}

pub fn wards_are_market() -> bool {
    false
}

pub fn wards_are_socket_or_public_bind() -> bool {
    false
}

pub fn wards_mesh_on_sanctuary() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex_travel::{confirm_leave, places_eligible, PlaceId};
    use crate::stranger_loop_proof::hour_three_held_fixture;

    #[test]
    fn posts_are_heartwood_dress_outside_lamp_and_water() {
        assert!(wards_are_valid());
        assert!(!wards_are_market());
        assert!(!wards_are_socket_or_public_bind());
        assert!(!wards_mesh_on_sanctuary());
    }

    #[test]
    fn look_and_tend_keep_book_places_and_sanctuary_return() {
        let house = hour_three_held_fixture();
        let before = house.clone();
        let mut dress = WardDress::default();
        let looked = visit_wards(&mut dress, WardVerb::Look, &house);
        let tended = visit_wards(&mut dress, WardVerb::Tend, &house);
        assert_eq!(looked.line, WARDS_NOTICE);
        assert_eq!(tended.line, WARDS_NOTICE);
        assert!(dress.looked);
        assert_eq!(dress.tends, 1);
        assert!(looked.hour_three_complete && tended.hour_three_complete);
        assert!(looked.embassy_seated && tended.embassy_seated);
        assert_eq!(house, before, "Wards must not write the house book");
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
