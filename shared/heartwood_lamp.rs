//! U3 — Heartwood lamp spatial rules
//!
//! Embassy lamp (`Embassy.lamp_live`) is the house seat cue — display-only on
//! the Heartwood stub. The spatial rule lives on Heartwood hex generation /
//! the lamp disk: no buildings in water, no buildings in the lamp disk.
//! The disk stays empty of structures. Same Peace Use (E).
//!
//! Isolation gamma stays 0. Do not couple tons, seeds, or declared_lethal
//! across hexes. Do not hang mesh on Sanctuary. Do not boot Depths.
//! Contact: info@Rathor.ai. Independent of xAI.

use crate::hex_listen::PowrushNet;
use crate::hex_protocol::default_client_listens;
use crate::hex_travel::{PlaceId, ISOLATION_GAMMA};
use crate::space_law::HexFlag;

/// Quiet center of Heartwood (PLACES_BIBLE Lamp ring). xz world units.
pub const LAMP_DISK_CENTER: [f32; 2] = [0.0, 0.0];
pub const LAMP_DISK_RADIUS: f32 = 3.0;

/// Lip pond — water outside the lamp disk so refuse reasons stay distinct.
pub const WATER_POND_CENTER: [f32; 2] = [8.0, 0.0];
pub const WATER_POND_RADIUS: f32 = 2.0;

/// Dry Lip sample — buildable on Heartwood, outside water and the lamp.
pub const LIP_BUILD_POINT: [f32; 2] = [0.0, 6.0];

/// Return point after stepping into the Heartwood pond. It is dry Lip ground,
/// outside the lamp disk.
pub const HEARTWOOD_BATH_RETURN: [f32; 2] = LIP_BUILD_POINT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeartwoodLipKind {
    WalkwayCapsule,
    HangingRib,
}

/// Fixed Heartwood dress. `footprint_radius` encloses the whole mesh in xz,
/// so validating the footprint (not only its origin) keeps the lamp disk empty.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeartwoodLipInstance {
    pub kind: HeartwoodLipKind,
    pub center: [f32; 3],
    pub footprint_radius: f32,
}

/// Two dry walkway capsules and five overhead seed ribs on the Lip. These are
/// deterministic fixed dress and do not consume or alter the grove seed.
pub const HEARTWOOD_LIP_INSTANCES: [HeartwoodLipInstance; 7] = [
    HeartwoodLipInstance {
        kind: HeartwoodLipKind::WalkwayCapsule,
        center: [-1.35, 0.30, 5.20],
        footprint_radius: 1.40,
    },
    HeartwoodLipInstance {
        kind: HeartwoodLipKind::WalkwayCapsule,
        center: [1.35, 0.30, 5.20],
        footprint_radius: 1.40,
    },
    HeartwoodLipInstance {
        kind: HeartwoodLipKind::HangingRib,
        center: [-2.40, 2.80, 5.20],
        footprint_radius: 1.55,
    },
    HeartwoodLipInstance {
        kind: HeartwoodLipKind::HangingRib,
        center: [0.0, 2.95, 5.20],
        footprint_radius: 1.55,
    },
    HeartwoodLipInstance {
        kind: HeartwoodLipKind::HangingRib,
        center: [2.40, 2.80, 5.20],
        footprint_radius: 1.55,
    },
    HeartwoodLipInstance {
        kind: HeartwoodLipKind::HangingRib,
        center: [-4.65, 2.65, 5.35],
        footprint_radius: 1.55,
    },
    HeartwoodLipInstance {
        kind: HeartwoodLipKind::HangingRib,
        center: [4.65, 2.65, 5.35],
        footprint_radius: 1.55,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroundKind {
    Land,
    Water,
}

/// Why a building was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildRefuse {
    Water,
    LampDisk,
}

pub fn dist_xz(ax: f32, az: f32, bx: f32, bz: f32) -> f32 {
    let dx = ax - bx;
    let dz = az - bz;
    (dx * dx + dz * dz).sqrt()
}

pub fn in_disk(x: f32, z: f32, center: [f32; 2], radius: f32) -> bool {
    dist_xz(x, z, center[0], center[1]) <= radius
}

/// Heartwood lamp disk. Sanctuary must not grow this disk.
pub fn in_heartwood_lamp_disk(x: f32, z: f32) -> bool {
    in_disk(x, z, LAMP_DISK_CENTER, LAMP_DISK_RADIUS)
}

/// Heartwood Lip pond. Water is hex-generation terrain, not house embassy.
pub fn in_heartwood_water(x: f32, z: f32) -> bool {
    in_disk(x, z, WATER_POND_CENTER, WATER_POND_RADIUS)
}

/// Water tiles on this hex. Sanctuary yard has none in this slice (wells
/// are not build-water). Heartwood has the Lip pond.
pub fn in_water(place: PlaceId, x: f32, z: f32) -> bool {
    match place {
        PlaceId::Heartwood => in_heartwood_water(x, z),
        PlaceId::Sanctuary => false,
        PlaceId::Depths => false,
    }
}

pub fn ground_at(place: PlaceId, x: f32, z: f32) -> GroundKind {
    if in_water(place, x, z) {
        GroundKind::Water
    } else {
        GroundKind::Land
    }
}

/// Lamp disk is Heartwood-only. Sanctuary origin is not a lamp disk.
pub fn in_lamp_disk(place: PlaceId, x: f32, z: f32) -> bool {
    place == PlaceId::Heartwood && in_heartwood_lamp_disk(x, z)
}

/// Ground-kind gate (water is hex-agnostic). Lamp disk still needs Heartwood.
pub fn refuse_ground(place: PlaceId, ground: GroundKind, x: f32, z: f32) -> Result<(), BuildRefuse> {
    if ground == GroundKind::Water {
        return Err(BuildRefuse::Water);
    }
    if in_lamp_disk(place, x, z) {
        return Err(BuildRefuse::LampDisk);
    }
    Ok(())
}

/// Place a building on hex generation. Water and the Heartwood lamp disk refuse.
pub fn try_place_building(place: PlaceId, x: f32, z: f32) -> Result<(), BuildRefuse> {
    refuse_ground(place, ground_at(place, x, z), x, z)
}

pub fn lip_instance_outside_lamp(instance: &HeartwoodLipInstance) -> bool {
    dist_xz(
        instance.center[0],
        instance.center[2],
        LAMP_DISK_CENTER[0],
        LAMP_DISK_CENTER[1],
    ) - instance.footprint_radius
        > LAMP_DISK_RADIUS
}

pub fn lip_instance_outside_water(instance: &HeartwoodLipInstance) -> bool {
    dist_xz(
        instance.center[0],
        instance.center[2],
        WATER_POND_CENTER[0],
        WATER_POND_CENTER[1],
    ) - instance.footprint_radius
        > WATER_POND_RADIUS
}

pub fn heartwood_lip_is_valid() -> bool {
    HEARTWOOD_LIP_INSTANCES
        .iter()
        .filter(|instance| instance.kind == HeartwoodLipKind::WalkwayCapsule)
        .count()
        == 2
        && HEARTWOOD_LIP_INSTANCES
            .iter()
            .filter(|instance| instance.kind == HeartwoodLipKind::HangingRib)
            .count()
            == 5
        && HEARTWOOD_LIP_INSTANCES.iter().all(|instance| {
            lip_instance_outside_lamp(instance)
                && lip_instance_outside_water(instance)
                && try_place_building(
                    PlaceId::Heartwood,
                    instance.center[0],
                    instance.center[2],
                )
                .is_ok()
        })
}

/// A pond step is a local bath/return, never a house-persist operation.
pub fn heartwood_bath_return(place: PlaceId, x: f32, z: f32) -> Option<[f32; 2]> {
    if place == PlaceId::Heartwood && in_heartwood_water(x, z) {
        Some(HEARTWOOD_BATH_RETURN)
    } else {
        None
    }
}

/// Session yard for Heartwood structures. Not written into house persist.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HeartwoodYard {
    pub buildings: Vec<[f32; 2]>,
}

impl HeartwoodYard {
    pub fn try_place(&mut self, x: f32, z: f32) -> Result<(), BuildRefuse> {
        try_place_building(PlaceId::Heartwood, x, z)?;
        self.buildings.push([x, z]);
        Ok(())
    }

    pub fn lamp_disk_empty(&self) -> bool {
        !self
            .buildings
            .iter()
            .any(|p| in_heartwood_lamp_disk(p[0], p[1]))
    }
}

/// Embassy `lamp_live` is the house seat cue, not the spatial gate.
pub fn embassy_lamp_is_spatial_gate() -> bool {
    false
}

/// U6 ships only the Heartwood Lip ribs, place-scoped outside the lamp disk.
pub fn hanging_mesh_shipped() -> bool {
    true
}

pub fn heartwood_mesh_on_sanctuary() -> bool {
    false
}

pub fn depths_is_boot() -> bool {
    false
}

pub fn heartwood_peace_use() -> HexFlag {
    PlaceId::Heartwood.peace_hex()
}

/// Spatial law is disk-only. Title Online stays grey.
pub fn spatial_is_disk_only() -> bool {
    !default_client_listens()
        && !PowrushNet::Off.title_online_enabled()
        && ISOLATION_GAMMA == 0.0
        && !depths_is_boot()
        && !heartwood_mesh_on_sanctuary()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::f_book_fixture::load_f_book_disk;
    use crate::hex_travel::{
        apply_travel_at, heartwood_stub_embassy, house_embassy_on_place, house_pack_after_leave,
        may_enter, new_game_writes_heartwood, places_eligible, places_row_label, places_row_or_inert,
        read_hex_at, sanctuary_fresh_climate, sanctuary_fresh_standing, stub_hex_file,
    };
    use crate::pause_ledger_face::NOT_YOUR_CHARTER;
    use crate::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};
    use std::fs;
    use std::path::PathBuf;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "powrush-u3-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
            tag
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("scratch");
        dir
    }

    #[test]
    fn buildings_refused_in_water_and_heartwood_lamp_disk() {
        assert_eq!(
            try_place_building(
                PlaceId::Heartwood,
                WATER_POND_CENTER[0],
                WATER_POND_CENTER[1]
            ),
            Err(BuildRefuse::Water)
        );
        assert_eq!(
            try_place_building(
                PlaceId::Heartwood,
                LAMP_DISK_CENTER[0],
                LAMP_DISK_CENTER[1]
            ),
            Err(BuildRefuse::LampDisk)
        );
        // Water rule is hex-agnostic ground — not the house embassy lamp.
        assert_eq!(
            refuse_ground(PlaceId::Sanctuary, GroundKind::Water, 1.0, 1.0),
            Err(BuildRefuse::Water)
        );
        assert_eq!(
            try_place_building(PlaceId::Heartwood, LIP_BUILD_POINT[0], LIP_BUILD_POINT[1]),
            Ok(())
        );
        // Sanctuary origin is not a Heartwood lamp disk.
        assert_eq!(
            try_place_building(PlaceId::Sanctuary, 0.0, 0.0),
            Ok(())
        );
        assert!(!in_lamp_disk(PlaceId::Sanctuary, 0.0, 0.0));

        let mut yard = HeartwoodYard::default();
        assert_eq!(
            yard.try_place(WATER_POND_CENTER[0], WATER_POND_CENTER[1]),
            Err(BuildRefuse::Water)
        );
        assert_eq!(
            yard.try_place(LAMP_DISK_CENTER[0], LAMP_DISK_CENTER[1]),
            Err(BuildRefuse::LampDisk)
        );
        assert!(yard.try_place(LIP_BUILD_POINT[0], LIP_BUILD_POINT[1]).is_ok());
        assert!(yard.lamp_disk_empty());
        assert_eq!(yard.buildings.len(), 1);

        assert!(
            dist_xz(
                LAMP_DISK_CENTER[0],
                LAMP_DISK_CENTER[1],
                WATER_POND_CENTER[0],
                WATER_POND_CENTER[1]
            ) > LAMP_DISK_RADIUS + WATER_POND_RADIUS,
            "lamp disk and water pond stay disjoint so refuse reasons isolate"
        );
        assert_eq!(ground_at(PlaceId::Heartwood, 8.0, 0.0), GroundKind::Water);
        assert_eq!(ground_at(PlaceId::Heartwood, 0.0, 0.0), GroundKind::Land);
        assert!(!embassy_lamp_is_spatial_gate());
        assert!(!heartwood_stub_embassy().lamp_live);
    }

    #[test]
    fn lip_capsules_and_ribs_leave_lamp_and_water_empty() {
        assert!(heartwood_lip_is_valid());
        assert_eq!(
            HEARTWOOD_LIP_INSTANCES
                .iter()
                .filter(|instance| instance.kind == HeartwoodLipKind::WalkwayCapsule)
                .count(),
            2
        );
        for instance in HEARTWOOD_LIP_INSTANCES {
            assert!(lip_instance_outside_lamp(&instance));
            assert!(lip_instance_outside_water(&instance));
            assert_eq!(
                try_place_building(
                    PlaceId::Heartwood,
                    instance.center[0],
                    instance.center[2]
                ),
                Ok(())
            );
        }
        assert_eq!(
            heartwood_bath_return(
                PlaceId::Heartwood,
                WATER_POND_CENTER[0],
                WATER_POND_CENTER[1]
            ),
            Some(HEARTWOOD_BATH_RETURN)
        );
        assert_eq!(
            heartwood_bath_return(
                PlaceId::Sanctuary,
                WATER_POND_CENTER[0],
                WATER_POND_CENTER[1]
            ),
            None
        );
        assert!(!in_heartwood_lamp_disk(
            HEARTWOOD_BATH_RETURN[0],
            HEARTWOOD_BATH_RETURN[1]
        ));
        assert!(!in_heartwood_water(
            HEARTWOOD_BATH_RETURN[0],
            HEARTWOOD_BATH_RETURN[1]
        ));
    }

    #[test]
    fn more_seed_ribs_stay_clear_and_keep_wards_valid() {
        let ribs: Vec<_> = HEARTWOOD_LIP_INSTANCES
            .iter()
            .filter(|instance| instance.kind == HeartwoodLipKind::HangingRib)
            .collect();
        assert_eq!(ribs.len(), 5, "U9 adds two fixed seed ribs");
        assert!(ribs
            .iter()
            .all(|instance| lip_instance_outside_lamp(instance)));
        assert!(ribs
            .iter()
            .all(|instance| lip_instance_outside_water(instance)));
        assert!(crate::heartwood_wards::wards_are_valid());
    }

    #[test]
    fn house_book_and_hex_isolation_still_hold() {
        let dir = scratch("book-iso");
        let (house, climate, standing, _week, _name) = load_f_book_disk();
        assert!(house.complete && house.hour_three_complete && house.embassy.seated);
        let house_embassy = house.embassy.clone();
        assert!(!standing.declared_lethal);

        let loaded = apply_travel_at(
            &dir,
            house.complete,
            house.hour_three_complete,
            PlaceId::Sanctuary,
            PlaceId::Heartwood,
            &climate,
            &standing,
        )
        .expect("leave Heartwood");
        assert!(loaded.lamp_empty);
        assert!(!loaded.standing.declared_lethal);
        assert_eq!(loaded.climate.tons_moved, 0);
        assert!(loaded.climate.seed_u64.is_none());
        assert_eq!(loaded.climate.hex_id, PlaceId::Heartwood.as_str());

        let mut persist = house.clone();
        persist.embassy = heartwood_stub_embassy();
        persist.keep_house_book_over_hex_stub(&house);
        persist.mark_hour_three();
        assert!(persist.hour_three_complete);
        assert!(persist.embassy.seated);
        assert_eq!(persist.embassy, house_embassy);
        assert_eq!(
            house_embassy_on_place(&house_embassy, PlaceId::Heartwood),
            house_embassy
        );
        assert_eq!(
            house_pack_after_leave(&house, PlaceId::Heartwood).embassy,
            house_embassy
        );

        let mut yard = HeartwoodYard::default();
        assert_eq!(
            yard.try_place(LAMP_DISK_CENTER[0], LAMP_DISK_CENTER[1]),
            Err(BuildRefuse::LampDisk)
        );
        assert!(yard.try_place(LIP_BUILD_POINT[0], LIP_BUILD_POINT[1]).is_ok());
        assert!(yard.lamp_disk_empty());
        // Placement is hex generation, not house persist / hex climate copy.
        let hw = read_hex_at(&dir, PlaceId::Heartwood).expect("heartwood hex");
        assert!(hw.lamp_empty);
        assert!(!hw.standing.declared_lethal);
        assert_eq!(hw.climate.tons_moved, 0);
        let san = read_hex_at(&dir, PlaceId::Sanctuary).expect("sanctuary hex");
        assert_eq!(san.climate.tons_moved, climate.tons_moved);
        assert_eq!(san.standing.declared_lethal, standing.declared_lethal);
        assert_ne!(san.climate.hex_id, hw.climate.hex_id);

        assert!(places_eligible(persist.complete, persist.hour_three_complete));
        assert!(!may_enter(PlaceId::Heartwood, false));
        assert_eq!(places_row_label(false, false), None);
        assert_eq!(
            places_row_or_inert(false, false, false),
            NOT_YOUR_CHARTER
        );
        assert!(!new_game_writes_heartwood());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn peace_e_online_grey_no_mesh_on_sanctuary() {
        assert_eq!(heartwood_peace_use(), HexFlag::Peace);
        assert_eq!(PlaceId::Sanctuary.peace_hex(), HexFlag::Peace);
        assert_eq!(stub_hex_file(PlaceId::Heartwood).standing.declared_lethal, false);
        assert!(!sanctuary_fresh_standing().declared_lethal);
        assert_eq!(sanctuary_fresh_climate().hex_id, PlaceId::Sanctuary.as_str());
        assert!(hanging_mesh_shipped());
        assert!(!heartwood_mesh_on_sanctuary());
        assert!(!depths_is_boot());
        assert!(spatial_is_disk_only());
        assert!(!PowrushNet::Off.title_online_enabled());
        assert!(!PowrushNet::Localhost.title_online_enabled());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!default_client_listens());
        assert_eq!(ISOLATION_GAMMA, 0.0);
    }
}
