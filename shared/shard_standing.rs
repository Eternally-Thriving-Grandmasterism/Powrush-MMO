//! Phase R — local hex faction standing (v23.2.37)
//!
//! Standing beside climate. Same verbs write both.
//! declared_lethal flips only via L1 hex sign / Ledger 3 after Settled + Hour three (opt-in).
//! No race select. No second HUD. Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// Local hex law. Peace / Bind default. Fighting = Ledger 3 only (parked).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShardStanding {
    pub hex_id: String,
    /// 0..1 civic calm
    pub peace: f32,
    /// 0..1 shared thriving (mirrors climate feel, not a second bar)
    pub harmony: f32,
    /// 0..1 take / lane pressure
    pub consumption: f32,
    /// 0..1 mend / reserve / care stewardship
    pub steward: f32,
    /// Reserved for hybrid heat; starts at 0
    pub human_hybrid_heat: f32,
    /// Opt-in hex sign after Settled + book. Never default E. Missing/unknown → off.
    #[serde(default)]
    pub declared_lethal: bool,
    /// Blood tariff units paid this week (not refunded on clear).
    #[serde(default)]
    pub tariff_paid: u32,
    pub updated_at: u64,
}

impl Default for ShardStanding {
    fn default() -> Self {
        Self {
            hex_id: "local-hex".into(),
            peace: 0.72,
            harmony: 0.55,
            consumption: 0.12,
            steward: 0.40,
            human_hybrid_heat: 0.0,
            declared_lethal: false,
            tariff_paid: 0,
            updated_at: 0,
        }
    }
}

impl ShardStanding {
    pub fn clamp_fields(&mut self) {
        self.peace = self.peace.clamp(0.0, 1.0);
        self.harmony = self.harmony.clamp(0.0, 1.0);
        self.consumption = self.consumption.clamp(0.0, 1.0);
        self.steward = self.steward.clamp(0.0, 1.0);
        self.human_hybrid_heat = self.human_hybrid_heat.clamp(0.0, 1.0);
        // declared_lethal is set only by declare_lethal / clear_lethal — clamp does not wipe it.
    }

    fn touch(&mut self) {
        self.updated_at = self.updated_at.saturating_add(1);
        self.clamp_fields();
    }

    pub fn on_care_tend(&mut self) {
        self.peace = (self.peace + 0.05).clamp(0.0, 1.0);
        self.harmony = (self.harmony + 0.05).clamp(0.0, 1.0);
        self.steward = (self.steward + 0.04).clamp(0.0, 1.0);
        self.consumption = (self.consumption - 0.02).clamp(0.0, 1.0);
        self.touch();
    }

    pub fn on_glowing_take(&mut self) {
        self.consumption = (self.consumption + 0.06).clamp(0.0, 1.0);
        self.harmony = (self.harmony - 0.03).clamp(0.0, 1.0);
        self.peace = (self.peace - 0.02).clamp(0.0, 1.0);
        self.touch();
    }

    pub fn on_tired_refuse(&mut self) {
        self.consumption = (self.consumption + 0.04).clamp(0.0, 1.0);
        self.peace = (self.peace - 0.05).clamp(0.0, 1.0);
        self.harmony = (self.harmony - 0.03).clamp(0.0, 1.0);
        self.touch();
    }

    pub fn on_flow(&mut self) {
        self.peace = (self.peace + 0.06).clamp(0.0, 1.0);
        self.harmony = (self.harmony + 0.06).clamp(0.0, 1.0);
        self.consumption = (self.consumption - 0.04).clamp(0.0, 1.0);
        self.touch();
    }

    pub fn on_reserve(&mut self) {
        self.steward = (self.steward + 0.07).clamp(0.0, 1.0);
        self.peace = (self.peace + 0.02).clamp(0.0, 1.0);
        self.touch();
    }

    pub fn on_mend(&mut self) {
        self.steward = (self.steward + 0.08).clamp(0.0, 1.0);
        self.harmony = (self.harmony + 0.03).clamp(0.0, 1.0);
        self.touch();
    }

    pub fn on_lane(&mut self) {
        self.consumption = (self.consumption + 0.03).clamp(0.0, 1.0);
        self.steward = (self.steward + 0.04).clamp(0.0, 1.0);
        self.touch();
    }

    /// Ledger 3 after Hour three held. Returns false if book missing or already lethal.
    pub fn declare_lethal(&mut self, hour_three_held: bool) -> bool {
        if !hour_three_held || self.declared_lethal {
            return false;
        }
        self.declared_lethal = true;
        self.harmony = (self.harmony - 0.18).clamp(0.0, 1.0);
        self.peace = (self.peace - 0.12).clamp(0.0, 1.0);
        self.consumption = (self.consumption + 0.15).clamp(0.0, 1.0);
        self.tariff_paid = self.tariff_paid.saturating_add(1);
        self.touch();
        true
    }

    /// L1 sign confirm on THIS hex only. Reuses `declare_lethal` — no second flag.
    /// Settled + book required. Otherwise false (caller shows wait / not your charter).
    pub fn confirm_hex_sign(&mut self, settled: bool, book_held: bool) -> bool {
        if !settled || !book_held {
            return false;
        }
        self.declare_lethal(true)
    }

    /// Mercy / second L clears the flag. Tariff already paid stays.
    pub fn clear_lethal(&mut self) -> bool {
        if !self.declared_lethal {
            return false;
        }
        self.declared_lethal = false;
        self.peace = (self.peace + 0.06).clamp(0.0, 1.0);
        self.touch();
        true
    }

    /// Optional one clause — not a standing HUD.
    pub fn slab_line(&self) -> Option<&'static str> {
        if self.declared_lethal {
            // Unreachable in Phase R after clamp; kept for later opt-in speech.
            Some("lethal is declared — the hex will remember")
        } else if self.peace >= 0.70 && self.harmony >= 0.55 && self.consumption <= 0.35 {
            Some("the yard holds peace")
        } else if self.consumption >= 0.55 || self.peace <= 0.40 {
            Some("standing thins — tend the hex")
        } else if self.steward >= 0.65 {
            Some("stewardship is writing the hex")
        } else {
            None
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        let mut s: Self = serde_json::from_str(raw)?;
        s.clamp_fields();
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_lethal_is_false() {
        let s = ShardStanding::default();
        assert!(!s.declared_lethal);
        assert_eq!(s.human_hybrid_heat, 0.0);
    }

    #[test]
    fn care_tend_raises_peace_and_steward() {
        let mut s = ShardStanding::default();
        let p = s.peace;
        let st = s.steward;
        s.on_care_tend();
        assert!(s.peace > p);
        assert!(s.steward > st);
        assert!(!s.declared_lethal);
    }

    #[test]
    fn take_raises_consumption() {
        let mut s = ShardStanding::default();
        let c = s.consumption;
        s.on_glowing_take();
        assert!(s.consumption > c);
    }

    #[test]
    fn flow_raises_peace() {
        let mut s = ShardStanding {
            peace: 0.4,
            ..Default::default()
        };
        s.on_flow();
        assert!(s.peace > 0.4);
    }

    #[test]
    fn reserve_and_mend_raise_steward() {
        let mut s = ShardStanding::default();
        s.on_reserve();
        s.on_mend();
        assert!(s.steward > 0.40);
    }

    #[test]
    fn clamp_preserves_declared_lethal() {
        let mut s = ShardStanding::default();
        assert!(s.declare_lethal(true));
        s.clamp_fields();
        assert!(s.declared_lethal);
        assert_eq!(s.tariff_paid, 1);
    }

    #[test]
    fn declare_requires_hour_three() {
        let mut s = ShardStanding::default();
        assert!(!s.declare_lethal(false));
        assert!(!s.declared_lethal);
        assert!(s.declare_lethal(true));
        assert!(s.declared_lethal);
    }

    #[test]
    fn clear_keeps_tariff_paid() {
        let mut s = ShardStanding::default();
        assert!(s.declare_lethal(true));
        assert!(s.clear_lethal());
        assert!(!s.declared_lethal);
        assert_eq!(s.tariff_paid, 1);
    }

    #[test]
    fn json_roundtrip() {
        let mut s = ShardStanding::default();
        s.on_lane();
        let raw = s.to_json().unwrap();
        let loaded = ShardStanding::from_json(&raw).unwrap();
        assert_eq!(loaded.updated_at, s.updated_at);
        assert!(!loaded.declared_lethal);
    }

    #[test]
    fn missing_declared_lethal_is_off() {
        let raw = r#"{"hex_id":"local-hex","peace":0.72,"harmony":0.55,"consumption":0.12,"steward":0.40,"human_hybrid_heat":0.0,"updated_at":0}"#;
        let loaded = ShardStanding::from_json(raw).unwrap();
        assert!(!loaded.declared_lethal);
        assert_eq!(loaded.tariff_paid, 0);
    }

    #[test]
    fn confirm_hex_sign_requires_settled_and_book() {
        let mut s = ShardStanding::default();
        assert!(!s.confirm_hex_sign(false, false));
        assert!(!s.confirm_hex_sign(true, false));
        assert!(!s.confirm_hex_sign(false, true));
        assert!(!s.declared_lethal);
        assert!(s.confirm_hex_sign(true, true));
        assert!(s.declared_lethal);
        assert!(!s.confirm_hex_sign(true, true));
    }
}
