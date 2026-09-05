//! Space law — Slice 0 hour-sacred stubs (v23.2.4) + hour-two door (v23.2.28)
//!
//! Peace hour: Warrant Weight is silent 0. Charter skin (Tab/G/L/Q)
//! is dead until `charter_id` AND a non-Peace hex exist.
//! After a first-hour allocate, Tab steps the ridge (Frontier visitor).
//! Q founds the local House. Formula is stored; UI never shows it.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// Hex flags. Industry and Ledger only where flagged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HexFlag {
    #[default]
    Peace,
    Frontier,
    War,
    ContestableCharter,
}

impl HexFlag {
    pub fn charter_jurisdiction(self) -> bool {
        !matches!(self, HexFlag::Peace)
    }

    pub fn industry_live(self) -> bool {
        self.charter_jurisdiction()
    }

    pub fn ledger_live(self) -> bool {
        self.charter_jurisdiction()
    }

    /// Contestable is an opt-in, never the Peace default.
    pub fn contestable(self) -> bool {
        matches!(self, HexFlag::ContestableCharter)
    }

    pub fn label(self) -> &'static str {
        match self {
            HexFlag::Peace => "Peace",
            HexFlag::Frontier => "Frontier",
            HexFlag::War => "War",
            HexFlag::ContestableCharter => "Contestable",
        }
    }
}

/// Company kinds. Independent Earth Coalition = Human default (Patchwork firm).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CharterKind {
    House,
    CoOp,
    Circle,
    GroveCharter,
    ResonanceLodge,
    #[default]
    PatchworkFirm,
    BroodCompany,
    ChoirAtelier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarrantBand {
    Clean,
    Cited,
    Wanted,
    OpenWarrant,
    Attainted,
}

impl WarrantBand {
    pub fn from_weight(w: f32) -> Self {
        if w < 15.0 {
            WarrantBand::Clean
        } else if w < 35.0 {
            WarrantBand::Cited
        } else if w < 65.0 {
            WarrantBand::Wanted
        } else if w < 90.0 {
            WarrantBand::OpenWarrant
        } else {
            WarrantBand::Attainted
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            WarrantBand::Clean => "Clean",
            WarrantBand::Cited => "Cited",
            WarrantBand::Wanted => "Wanted",
            WarrantBand::OpenWarrant => "Open Warrant",
            WarrantBand::Attainted => "Attainted",
        }
    }
}

/// Offense accumulators. Live W is 0 in Peace (silent stub).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WarrantWeight {
    pub h: f32,
    pub i: f32,
    pub c: f32,
    pub f: f32,
    pub x: f32,
    pub repair: f32,
    pub return_cargo: f32,
    pub council: f32,
    pub tend_spill: f32,
}

impl Default for WarrantWeight {
    fn default() -> Self {
        Self {
            h: 0.0,
            i: 0.0,
            c: 0.0,
            f: 0.0,
            x: 0.0,
            repair: 0.0,
            return_cargo: 0.0,
            council: 0.0,
            tend_spill: 0.0,
        }
    }
}

impl WarrantWeight {
    /// Sealed formula. UI shows band, never this.
    pub fn raw(&self) -> f32 {
        let v = 0.55 * self.h
            + 0.80 * self.i
            + 1.10 * self.c
            + 1.40 * self.f
            + 2.00 * self.x
            - 0.40 * self.repair
            - 0.35 * self.return_cargo
            - 0.25 * self.council
            - 0.20 * self.tend_spill;
        v.max(0.0)
    }

    /// Peace hour: always 0. Logout does not wipe components; Peace hides them.
    pub fn live(&self, hex: HexFlag) -> f32 {
        if hex == HexFlag::Peace {
            0.0
        } else {
            self.raw()
        }
    }

    pub fn band(&self, hex: HexFlag) -> WarrantBand {
        WarrantBand::from_weight(self.live(hex))
    }
}

/// Session the hour and later Charter skin both read.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpaceSession {
    pub charter_id: Option<String>,
    pub hex: HexFlag,
    pub kind: CharterKind,
    pub warrant: WarrantWeight,
}

impl Default for SpaceSession {
    fn default() -> Self {
        Self {
            charter_id: None,
            hex: HexFlag::Peace,
            kind: CharterKind::PatchworkFirm,
            warrant: WarrantWeight::default(),
        }
    }
}

impl SpaceSession {
    /// Tab / G / L / Q. Both charter_id and a non-Peace hex.
    pub fn charter_skin_live(&self) -> bool {
        self.charter_id.is_some() && self.hex.charter_jurisdiction()
    }

    pub fn warrant_live(&self) -> f32 {
        self.warrant.live(self.hex)
    }

    pub fn peace_visitor_on_frontier(&self) -> bool {
        self.charter_id.is_none() && self.hex.charter_jurisdiction()
    }

    /// First-hour allocate (flow or reserve) opens the Charter door. Not a tend.
    pub fn hour_two_door_ready(flow: u32, reserve: u32) -> bool {
        flow + reserve > 0
    }

    /// Tab after allocate. Steps the ridge as a Peace visitor. Q still founds.
    pub fn take_frontier_ridge(&mut self) -> bool {
        if self.hex != HexFlag::Peace {
            return false;
        }
        self.hex = HexFlag::Frontier;
        true
    }

    pub fn hour_two_line(&self, door_ready: bool) -> &'static str {
        if self.charter_skin_live() {
            "Frontier · L Ledger · Q House"
        } else if self.peace_visitor_on_frontier() {
            "Not your charter · Q plant a House stake"
        } else if door_ready {
            "Tab Charter — the ridge is open"
        } else {
            "Peace · tend then allocate"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peace_warrant_is_silent_zero() {
        let mut w = WarrantWeight::default();
        w.h = 40.0;
        w.i = 10.0;
        assert!(w.raw() > 0.0);
        assert_eq!(w.live(HexFlag::Peace), 0.0);
        assert_eq!(w.band(HexFlag::Peace), WarrantBand::Clean);
    }

    #[test]
    fn frontier_warrant_can_cite() {
        let mut w = WarrantWeight::default();
        w.i = 40.0;
        assert!(w.live(HexFlag::Frontier) > 15.0);
        assert_eq!(w.band(HexFlag::Frontier), WarrantBand::Cited);
    }

    #[test]
    fn charter_skin_needs_id_and_frontier() {
        let mut s = SpaceSession::default();
        assert!(!s.charter_skin_live());
        s.charter_id = Some("iec-1".into());
        assert!(!s.charter_skin_live());
        s.hex = HexFlag::Frontier;
        assert!(s.charter_skin_live());
        s.charter_id = None;
        assert!(s.peace_visitor_on_frontier());
        assert!(!s.charter_skin_live());
    }

    #[test]
    fn default_hour_is_peace_patchwork() {
        let s = SpaceSession::default();
        assert_eq!(s.hex, HexFlag::Peace);
        assert_eq!(s.kind, CharterKind::PatchworkFirm);
        assert_eq!(s.warrant_live(), 0.0);
    }

    #[test]
    fn contestable_is_opt_in_not_peace_default() {
        let s = SpaceSession::default();
        assert!(!s.hex.contestable());
        assert!(!s.hex.industry_live());
        assert!(!s.hex.ledger_live());
        assert_eq!(s.hex.label(), "Peace");
        let f = HexFlag::ContestableCharter;
        assert!(f.contestable());
        assert!(f.industry_live());
        assert_eq!(f.label(), "Contestable");
    }

    #[test]
    fn allocate_opens_door_tend_does_not() {
        assert!(!SpaceSession::hour_two_door_ready(0, 0));
        assert!(SpaceSession::hour_two_door_ready(1, 0));
        assert!(SpaceSession::hour_two_door_ready(0, 1));
    }

    #[test]
    fn tab_steps_ridge_q_still_founds() {
        let mut s = SpaceSession::default();
        assert_eq!(s.hour_two_line(true), "Tab Charter — the ridge is open");
        assert!(s.take_frontier_ridge());
        assert!(s.peace_visitor_on_frontier());
        assert!(!s.charter_skin_live());
        assert_eq!(s.hex, HexFlag::Frontier);
        assert_eq!(s.hour_two_line(true), "Not your charter · Q plant a House stake");
        assert!(!s.take_frontier_ridge());
        s.charter_id = Some("house-local".into());
        s.kind = CharterKind::House;
        assert!(s.charter_skin_live());
        assert_eq!(s.hour_two_line(true), "Frontier · L Ledger · Q House");
    }
}
