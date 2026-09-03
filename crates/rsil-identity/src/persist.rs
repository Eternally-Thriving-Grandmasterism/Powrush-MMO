//! Identity persist — Slice 1 (v23.2.5)
//!
//! charter_id, WarrantWeight, CharterKind, lethal_count, repair_ratio.
//! Band is computed. Peace still reports live W = 0.
//! No tradecraft, no method replay. Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};
use shared::space_law::{CharterKind, HexFlag, SpaceSession, WarrantBand, WarrantWeight};

/// Play-state bound to a DID. Not the signing key.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityPersist {
    pub did: String,
    pub charter_id: Option<String>,
    pub kind: CharterKind,
    pub warrant: WarrantWeight,
    pub lethal_count: u32,
    pub repair_ratio: f32,
}

impl IdentityPersist {
    pub fn new(did: impl Into<String>) -> Self {
        Self {
            did: did.into(),
            charter_id: None,
            kind: CharterKind::PatchworkFirm,
            warrant: WarrantWeight::default(),
            lethal_count: 0,
            repair_ratio: 0.0,
        }
    }

    pub fn band(&self, hex: HexFlag) -> WarrantBand {
        self.warrant.band(hex)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    /// Apply into the hour session. Hex stays with the world, not the DID.
    pub fn apply_to_session(&self, session: &mut SpaceSession) {
        session.charter_id = self.charter_id.clone();
        session.kind = self.kind;
        session.warrant = self.warrant;
    }

    /// Bridge export: listed fields only. No hunt/sabotage methods.
    pub fn bridge_export(&self, hex: HexFlag) -> serde_json::Value {
        serde_json::json!({
            "did": self.did,
            "charter_id": self.charter_id,
            "kind": self.kind,
            "warrant_live": self.warrant.live(hex),
            "band": self.band(hex).label(),
            "lethal_count": self.lethal_count,
            "repair_ratio": self.repair_ratio,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_keeps_charter_and_kind() {
        let mut p = IdentityPersist::new("did:powrush:test");
        p.charter_id = Some("iec-1".into());
        p.kind = CharterKind::CoOp;
        p.warrant.i = 4.0;
        let json = p.to_json().unwrap();
        let q = IdentityPersist::from_json(&json).unwrap();
        assert_eq!(q.charter_id.as_deref(), Some("iec-1"));
        assert_eq!(q.kind, CharterKind::CoOp);
        assert!((q.warrant.i - 4.0).abs() < 0.01);
        assert!(!json.to_lowercase().contains("sabotage"));
        assert!(!json.to_lowercase().contains("tradecraft"));
    }

    #[test]
    fn peace_live_w_stays_zero_after_load() {
        let mut p = IdentityPersist::new("did:powrush:test");
        p.warrant.h = 80.0;
        let mut session = SpaceSession::default();
        p.apply_to_session(&mut session);
        assert_eq!(session.hex, HexFlag::Peace);
        assert_eq!(session.warrant_live(), 0.0);
        assert_eq!(p.band(HexFlag::Peace), WarrantBand::Clean);
        let export = p.bridge_export(HexFlag::Peace);
        assert_eq!(export["warrant_live"], 0.0);
        assert_eq!(export["band"], "Clean");
    }

    #[test]
    fn frontier_band_is_readable() {
        let mut p = IdentityPersist::new("did:powrush:test");
        p.warrant.i = 50.0;
        assert_eq!(p.band(HexFlag::Frontier), WarrantBand::Cited);
        assert!(p.bridge_export(HexFlag::Frontier)["warrant_live"].as_f64().unwrap() > 15.0);
    }
}
