//! Infra offline + spill — Slice 5 (v23.2.9)
//!
//! Climate spill is the witness. I-code evidence pack is readable state,
//! not a method. Does not teach real infra attack.
//! Local graph. Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

use crate::space_law::HexFlag;
use crate::vertical_factory::FactoryNodeKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfraLife {
    Ghost,
    Live,
    Starved,
    Offline,
    StainedLive,
    Dismantle,
}

impl InfraLife {
    pub fn label(self) -> &'static str {
        match self {
            InfraLife::Ghost => "Ghost",
            InfraLife::Live => "Live",
            InfraLife::Starved => "Starved",
            InfraLife::Offline => "Offline",
            InfraLife::StainedLive => "Stained",
            InfraLife::Dismantle => "Dismantle",
        }
    }
}

/// I-codes only this slice. H/C/F/X stay on the Ledger (Slice 6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OffenseCode {
    I1,
    I2,
}

impl OffenseCode {
    pub fn label(self) -> &'static str {
        match self {
            OffenseCode::I1 => "I1",
            OffenseCode::I2 => "I2",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidencePack {
    pub code: OffenseCode,
    pub node: FactoryNodeKind,
    pub spill: bool,
    pub hash: String,
}

impl EvidencePack {
    pub fn offline_spill(node: FactoryNodeKind) -> Self {
        let code = OffenseCode::I2;
        Self {
            code,
            node,
            spill: true,
            hash: evidence_hash(code, node, true),
        }
    }

    /// Fields the bridge may export. No method, no payload, no tradecraft.
    pub fn bridge_fields(&self) -> Vec<(&'static str, String)> {
        vec![
            ("code", self.code.label().into()),
            ("node", self.node.label().into()),
            ("spill", if self.spill { "1" } else { "0" }.into()),
            ("hash", self.hash.clone()),
        ]
    }

    pub fn line(&self) -> String {
        format!(
            "{} {} · {} · spill {} · pack {}",
            self.code.label(),
            self.node.label(),
            "Offline",
            if self.spill { "visible" } else { "none" },
            &self.hash[..8.min(self.hash.len())]
        )
    }
}

fn evidence_hash(code: OffenseCode, node: FactoryNodeKind, spill: bool) -> String {
    let s = format!("{}|{}|{}", code.label(), node.label(), spill as u8);
    format!("{:016x}", fnv64(s.as_bytes()))
}

fn fnv64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InfraWitness {
    pub life: Option<InfraLife>,
    pub pack: Option<EvidencePack>,
    pub last_line: String,
    pub seen: bool,
}

impl InfraWitness {
    pub fn ensure_offline_extractor(&mut self) {
        if self.pack.is_some() {
            return;
        }
        self.life = Some(InfraLife::Offline);
        self.pack = Some(EvidencePack::offline_spill(FactoryNodeKind::Extractor));
        self.last_line = self.slab_line();
    }

    pub fn slab_line(&self) -> String {
        match &self.pack {
            Some(p) => format!(
                "Extractor {} · spill on the ground · {}",
                self.life.map(InfraLife::label).unwrap_or("Offline"),
                p.line()
            ),
            None => "No spill on this yard".into(),
        }
    }

    pub fn visible_on(&self, hex: HexFlag) -> bool {
        hex.industry_live() && self.pack.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offline_extractor_has_visible_spill() {
        let mut w = InfraWitness::default();
        w.ensure_offline_extractor();
        let p = w.pack.as_ref().unwrap();
        assert_eq!(p.code, OffenseCode::I2);
        assert!(p.spill);
        assert_eq!(w.life, Some(InfraLife::Offline));
        assert!(w.visible_on(HexFlag::Frontier));
        assert!(!w.visible_on(HexFlag::Peace));
    }

    #[test]
    fn evidence_hash_is_stable() {
        let a = EvidencePack::offline_spill(FactoryNodeKind::Extractor);
        let b = EvidencePack::offline_spill(FactoryNodeKind::Extractor);
        assert_eq!(a.hash, b.hash);
        assert_eq!(a.hash.len(), 16);
    }

    #[test]
    fn pack_has_no_method_fields() {
        let p = EvidencePack::offline_spill(FactoryNodeKind::Extractor);
        let keys: Vec<_> = p.bridge_fields().into_iter().map(|(k, _)| k).collect();
        assert_eq!(keys, ["code", "node", "spill", "hash"]);
        let blob = format!("{p:?}");
        for banned in ["exploit", "payload", "cve", "port scan", "how to"] {
            assert!(!blob.to_lowercase().contains(banned));
        }
    }
}
