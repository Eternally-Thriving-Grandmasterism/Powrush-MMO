//! Ledger + Bind/Escort — Slice 6, lethal clause Slice 10 (v23.2.14)
//!
//! Default win is Bind, not a corpse. Purse is flow + repair-rights, never pockets.
//! DeclaredLethal is an opt-in clause + hunter blood tariff. Not a combat key.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WinCondition {
    #[default]
    BindEscort,
    DeclaredLethal,
}

impl WinCondition {
    pub fn label(self) -> &'static str {
        match self {
            WinCondition::BindEscort => "Bind + Escort",
            WinCondition::DeclaredLethal => "DeclaredLethal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ContractState {
    #[default]
    Posted,
    Taken,
    Escorting,
    Tribunal,
    Settled,
    Failed,
    Expired,
}

impl ContractState {
    pub fn label(self) -> &'static str {
        match self {
            ContractState::Posted => "Posted",
            ContractState::Taken => "Taken",
            ContractState::Escorting => "Escorting",
            ContractState::Tribunal => "Tribunal",
            ContractState::Settled => "Settled",
            ContractState::Failed => "Failed",
            ContractState::Expired => "Expired",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Purse {
    pub flow: f32,
    pub repair_rights: f32,
    /// Hunter blood tariff. 0 unless DeclaredLethal is opted in.
    pub blood_tariff: f32,
}

impl Default for Purse {
    fn default() -> Self {
        Self {
            flow: 1.0,
            repair_rights: 1.0,
            blood_tariff: 0.0,
        }
    }
}

impl Purse {
    pub fn line(&self) -> String {
        if self.blood_tariff > 0.0 {
            format!(
                "flow {:.0} · repair-rights {:.0} · blood tariff {:.0}",
                self.flow, self.repair_rights, self.blood_tariff
            )
        } else {
            format!("flow {:.0} · repair-rights {:.0}", self.flow, self.repair_rights)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedgerContract {
    pub codes: Vec<String>,
    pub evidence_hash: String,
    pub purse: Purse,
    pub win: WinCondition,
    pub custody: bool,
    pub state: ContractState,
    pub stops_done: u8,
    pub lethal_count: u32,
    pub last_line: String,
}

impl LedgerContract {
    pub fn from_i2(hash: impl Into<String>) -> Self {
        let mut c = Self {
            codes: vec!["I2".into()],
            evidence_hash: hash.into(),
            purse: Purse::default(),
            win: WinCondition::BindEscort,
            custody: true,
            state: ContractState::Posted,
            stops_done: 0,
            lethal_count: 0,
            last_line: String::new(),
        };
        c.last_line = c.line();
        c
    }

    pub fn line(&self) -> String {
        format!(
            "Ledger · {} · {} · {} · {} · E Bind/Escort",
            self.state.label(),
            self.win.label(),
            self.codes.join(","),
            self.purse.line()
        )
    }

    /// Opt-in clause. Does not fire a kill. Hunter pays the tariff.
    pub fn opt_lethal(&mut self) -> &'static str {
        if self.state != ContractState::Posted {
            return "idle";
        }
        if self.win == WinCondition::DeclaredLethal {
            return "idle";
        }
        self.win = WinCondition::DeclaredLethal;
        self.purse.blood_tariff = 2.0;
        self.lethal_count = self.lethal_count.saturating_add(1);
        self.last_line =
            "DeclaredLethal · hunter blood tariff 2 · Bind remains the default on other listings"
                .into();
        "lethal"
    }

    pub fn bind(&mut self) -> &'static str {
        if self.state != ContractState::Posted {
            return "idle";
        }
        if self.win != WinCondition::BindEscort {
            self.last_line = "Lethal is not the default".into();
            return "idle";
        }
        self.state = ContractState::Taken;
        self.last_line = "Bound — escort to the post".into();
        "bound"
    }

    pub fn escort_step(&mut self) -> &'static str {
        match self.state {
            ContractState::Taken => {
                self.state = ContractState::Escorting;
                self.stops_done = 1;
                self.last_line = "Custody route 1 — dashed white to the post".into();
                "escorting"
            }
            ContractState::Escorting if self.stops_done < 2 => {
                self.stops_done += 1;
                self.last_line = "Custody route 2 — the post is in sight".into();
                "escorting"
            }
            ContractState::Escorting => {
                self.state = ContractState::Settled;
                self.last_line = "Settled — flow and the right to mend".into();
                "settled"
            }
            _ => "idle",
        }
    }

    pub fn act(&mut self) -> &'static str {
        match self.state {
            ContractState::Posted => self.bind(),
            ContractState::Taken | ContractState::Escorting => self.escort_step(),
            _ => "idle",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LedgerBoard {
    pub contracts: Vec<LedgerContract>,
}

impl LedgerBoard {
    pub fn ensure_i2(&mut self, hash: impl Into<String>) {
        if self.contracts.is_empty() {
            self.contracts.push(LedgerContract::from_i2(hash));
        }
    }

    pub fn open(&self) -> Option<&LedgerContract> {
        self.contracts.first()
    }

    pub fn open_mut(&mut self) -> Option<&mut LedgerContract> {
        self.contracts.get_mut(0)
    }

    pub fn act_local(&mut self) -> &'static str {
        self.open_mut().map(|c| c.act()).unwrap_or("idle")
    }

    pub fn opt_lethal_local(&mut self) -> &'static str {
        self.open_mut().map(|c| c.opt_lethal()).unwrap_or("idle")
    }

    pub fn sash_line(&self) -> String {
        self.open()
            .map(|c| {
                if c.last_line.is_empty() {
                    c.line()
                } else {
                    format!("{} · {}", c.line(), c.last_line)
                }
            })
            .unwrap_or_else(|| "L Ledger · no listing".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_win_is_bind_not_lethal() {
        let c = LedgerContract::from_i2("abc");
        assert_eq!(c.win, WinCondition::BindEscort);
        assert_ne!(c.win, WinCondition::DeclaredLethal);
        assert!(c.custody);
    }

    #[test]
    fn purse_is_flow_and_repair() {
        let p = Purse::default();
        assert!(p.flow > 0.0);
        assert!(p.repair_rights > 0.0);
        let blob = format!("{p:?}");
        assert!(!blob.contains("pocket"));
        assert!(!blob.contains("silver"));
        assert!(!blob.contains("Currency3"));
    }

    #[test]
    fn bind_then_escort_settles() {
        let mut b = LedgerBoard::default();
        b.ensure_i2("deadbeef");
        assert_eq!(b.act_local(), "bound");
        assert_eq!(b.act_local(), "escorting");
        assert_eq!(b.act_local(), "escorting");
        assert_eq!(b.act_local(), "settled");
        assert_eq!(b.open().unwrap().state, ContractState::Settled);
        assert!(b.open().unwrap().last_line.contains("right to mend"));
    }

    #[test]
    fn opt_lethal_sets_tariff_not_default() {
        let mut c = LedgerContract::from_i2("abc");
        assert_eq!(c.purse.blood_tariff, 0.0);
        assert_eq!(c.opt_lethal(), "lethal");
        assert_eq!(c.win, WinCondition::DeclaredLethal);
        assert_eq!(c.purse.blood_tariff, 2.0);
        assert_eq!(c.lethal_count, 1);
        assert_eq!(c.bind(), "idle");
        assert_eq!(c.state, ContractState::Posted);
    }

    #[test]
    fn default_listing_still_binds() {
        let mut b = LedgerBoard::default();
        b.ensure_i2("x");
        assert_eq!(b.open().unwrap().win, WinCondition::BindEscort);
        assert_eq!(b.act_local(), "bound");
    }
}
