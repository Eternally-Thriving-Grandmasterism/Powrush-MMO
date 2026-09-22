//! Ledger + Bind/Escort — Slice 6, lethal clause Slice 10 (v23.2.14)
//!
//! Default win is Bind, not a corpse. Purse is flow + repair-rights, never pockets.
//! DeclaredLethal is an opt-in clause + hunter blood tariff. Not a combat key.
//!
//! CARD F1 STANCE-POLICY — Offline simulated Peoples honor sealed-soul stance
//! (Open-trade / Neutral / Closed / Hostile). Garden light = no stance · no trade.
//! Online humans are F10 only — not implemented. Stance rides the existing
//! hour-two board / seal disk. No new persist schema file.
//!
//! CARD F2 AH-WINDOW — AH is a ledger / inventory PANEL, not a Place.
//! Opens only if sealed stance == Open-trade. Garden light = no window.
//! Offline lots = ghost rows on this board (F8 folded). Bind/Settle list/take.
//! Window shuts if stance leaves Open-trade. 0 meshes · 0 new PlaceId.
//!
//! CARD F3 HOSTILE-PRACTICE — sealed Hostile may Take / refuse / embargo
//! on this board without Bind. NEVC labels display only (no wage invent).
//! No lockout. Open-trade Bind path unread beyond the F9 wire. F2 AH panel
//! unread beyond the stance gate already on tip. 0 meshes · 0 new persist file.
//!
//! CARD F9 NEVC-ON-STANCE — sealed Hostile Take writes a ledger row + NEVC
//! display label (no wage). Sealed Open-trade Bind shows Reserve cue on the
//! existing allocate path (no new verb). Garden light = neither. F3 practice
//! unread beyond that Take wire. F2 AH unread beyond the Open-trade gate.
//!
//! CARD F4 DOUBLE-AGENT — sealed dress stays (no dress swap). Sealed soul
//! may serve another well offline (simulated Peoples) without dress swap.
//! Online human double-serve waits F10 / Online-yes — not implemented.
//! F9 / F3 / F2 unread beyond existing wires. PlaceId stays 3 · Online grey
//! · no lockout. 0 meshes · no new persist file.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

use crate::persona::{HostilePractice, SoulStance};

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

    /// Second Digit3 / mercy path. Does not refund blood tariff.
    pub fn clear_lethal(&mut self) -> &'static str {
        if self.win != WinCondition::DeclaredLethal {
            return "idle";
        }
        self.win = WinCondition::BindEscort;
        self.last_line =
            "Peace declared on this listing · tariff already paid stays on the week".into();
        "cleared"
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

/// CARD F2 / F8 — Offline ghost lot. A ledger row, not a Place.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GhostLot {
    pub label: String,
}

impl GhostLot {
    pub fn offline_row(label: impl Into<String>) -> Self {
        Self { label: label.into() }
    }

    /// Ghost lots are rows on the ledger board. Never a PlaceId.
    pub const fn is_place() -> bool {
        false
    }

    pub fn line(&self) -> String {
        format!("Ghost lot · {} · ledger row · not a Place", self.label)
    }
}

/// CARD F2 — AH mesh / Place budget. Panel only.
pub const F2_MESH_BUDGET: u32 = 0;
pub const F2_AH_IS_PLACE: bool = false;

/// CARD F3 — Hostile practice mesh budget. Hands stay dark.
pub const F3_MESH_BUDGET: u32 = crate::persona::F3_MESH_BUDGET;

/// CARD F9 — NEVC-on-stance mesh budget. Hands stay dark.
pub const F9_MESH_BUDGET: u32 = crate::persona::F9_MESH_BUDGET;

/// CARD F4 — double-agent mesh budget. Hands stay dark.
pub const F4_MESH_BUDGET: u32 = crate::persona::F4_MESH_BUDGET;

/// CARD F9 — existing allocate Reserve cue. Not a new verb. Not wages.
pub const OPEN_TRADE_BIND_RESERVE_CUE: &str = crate::nevc_visibility::OPEN_TRADE_BIND_RESERVE_CUE;

/// CARD F9 — Hostile Take ledger row. Session display. Not a persist file. Not wages.
#[derive(Debug, Clone, PartialEq)]
pub struct HostileTakeLedgerRow {
    pub nevc_label: String,
}

impl HostileTakeLedgerRow {
    pub fn from_nevc_label(label: impl Into<String>) -> Self {
        Self {
            nevc_label: label.into(),
        }
    }

    pub fn line(&self) -> String {
        format!(
            "Hostile Take · {} · ledger row · not wages",
            self.nevc_label
        )
    }

    /// Hostile Take rows live on the ledger board. Never a PlaceId.
    pub const fn is_place() -> bool {
        false
    }

    pub fn invents_wages(&self) -> bool {
        crate::nevc_visibility::nevc_line_invents_wages(&self.nevc_label)
            || crate::nevc_visibility::nevc_line_invents_wages(&self.line())
    }
}

/// CARD F4 — Offline double-serve. Session honor, not a Place, not persist.
/// Sealed dress stays. Simulated Peoples only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineDoubleServe {
    pub sealed_dress: String,
    pub well_of: String,
}

impl OfflineDoubleServe {
    pub fn try_serve(sealed: bool, sealed_dress: &str, well_of: &str) -> Option<Self> {
        if !crate::persona::sealed_soul_may_serve_another_well_offline(
            sealed,
            sealed_dress,
            well_of,
        ) {
            return None;
        }
        Some(Self {
            sealed_dress: sealed_dress.to_string(),
            well_of: well_of.to_string(),
        })
    }

    pub fn dress_after(&self) -> &str {
        crate::persona::sealed_dress_after_offline_double_serve(&self.sealed_dress, &self.well_of)
    }

    /// Double-serve lives on the ledger board. Never a PlaceId.
    pub const fn is_place() -> bool {
        false
    }
}

/// CARD F3 — Hostile practice outcome. Not Bind. Not a persist file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HostilePracticeOutcome {
    Taken,
    Refused,
    Embargoed,
    #[default]
    Idle,
}

impl HostilePracticeOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Taken => "taken",
            Self::Refused => "refused",
            Self::Embargoed => "embargoed",
            Self::Idle => "idle",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LedgerBoard {
    pub contracts: Vec<LedgerContract>,
    /// CARD F1 — sealed soul stance. None = garden light / no stance.
    /// Rides the existing hour-two pack board. Not a new persist file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub soul_stance: Option<SoulStance>,
    /// CARD F2 — session panel. Not a Place. Not a persist file.
    #[serde(default, skip)]
    pub ah_window_open: bool,
    /// CARD F2 / F8 — Offline ghost lots as ledger rows. Not a Place.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ghost_lots: Vec<GhostLot>,
    /// CARD F3 — session embargo. Not a persist file.
    #[serde(default, skip)]
    pub hostile_embargo: bool,
    /// CARD F3 — last Hostile practice. Session only.
    #[serde(default, skip)]
    pub last_hostile_practice: Option<HostilePractice>,
    /// CARD F9 — Hostile Take ledger row. Session only. Not a persist file.
    #[serde(default, skip)]
    pub last_hostile_take_row: Option<HostileTakeLedgerRow>,
    /// CARD F9 — last NEVC display label. Session only. Not wages.
    #[serde(default, skip)]
    pub last_nevc_label: Option<String>,
    /// CARD F4 — last offline double-serve. Session only. Not a persist file.
    #[serde(default, skip)]
    pub last_offline_double_serve: Option<OfflineDoubleServe>,
}

/// CARD F1 — how Offline simulated Peoples honor a soul stance.
/// Online humans stay F10. This path never lights Title Online.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulatedPeopleHonor {
    /// Open-trade: they will trade.
    HonorTrade,
    /// Neutral: civil; they will not open trade.
    HonorNeutral,
    /// Closed: they will not trade.
    HonorClosed,
    /// Hostile: they treat the soul as hostile and will not trade.
    HonorHostile,
    /// Garden light / no stance: no trade.
    NoStanceNoTrade,
}

/// Offline simulated Peoples honor the sealed soul's stance.
pub fn offline_simulated_people_honor(stance: Option<SoulStance>) -> SimulatedPeopleHonor {
    match stance {
        None => SimulatedPeopleHonor::NoStanceNoTrade,
        Some(SoulStance::OpenTrade) => SimulatedPeopleHonor::HonorTrade,
        Some(SoulStance::Neutral) => SimulatedPeopleHonor::HonorNeutral,
        Some(SoulStance::Closed) => SimulatedPeopleHonor::HonorClosed,
        Some(SoulStance::Hostile) => SimulatedPeopleHonor::HonorHostile,
    }
}

/// Only Open-trade is honored as a trade. Garden light cannot trade.
pub fn offline_simulated_people_will_trade(stance: Option<SoulStance>) -> bool {
    matches!(
        offline_simulated_people_honor(stance),
        SimulatedPeopleHonor::HonorTrade
    )
}

/// CARD F2 — AH panel may open only when the soul is sealed Open-trade.
pub fn ah_panel_may_open(sealed: bool, stance: Option<SoulStance>) -> bool {
    sealed && offline_simulated_people_will_trade(stance)
}

impl LedgerBoard {
    /// CARD F1 — sealed soul may set stance on this board. Light cannot.
    /// CARD F2 — stance leaving Open-trade shuts the AH window.
    pub fn set_sealed_soul_stance(&mut self, sealed: bool, stance: SoulStance) -> Option<SoulStance> {
        let set = crate::persona::sealed_soul_may_set_stance(sealed, stance)?;
        self.soul_stance = Some(set);
        self.sync_ah_panel_to_stance(sealed);
        Some(set)
    }

    /// Garden light / unsealed: clear stance. No trade. AH window shuts.
    pub fn clear_garden_light_stance(&mut self) {
        self.soul_stance = None;
        self.sync_ah_panel_to_stance(false);
    }

    /// CARD F2 — sealed Open-trade only. Garden light / other stances refuse.
    pub fn ah_panel_may_open(&self, sealed: bool) -> bool {
        ah_panel_may_open(sealed, self.soul_stance)
    }

    /// CARD F2 — open the ledger/inventory AH panel. Not a Place.
    pub fn try_open_ah_panel(&mut self, sealed: bool) -> bool {
        if !self.ah_panel_may_open(sealed) {
            self.ah_window_open = false;
            return false;
        }
        self.ensure_offline_ghost_lots();
        self.ah_window_open = true;
        true
    }

    pub fn close_ah_panel(&mut self) {
        self.ah_window_open = false;
    }

    /// Window shuts if stance leaves Open-trade (or garden light).
    pub fn sync_ah_panel_to_stance(&mut self, sealed: bool) {
        if !self.ah_panel_may_open(sealed) {
            self.ah_window_open = false;
        }
    }

    /// CARD F2 / F8 — Offline lots live as ledger rows, not a Place.
    pub fn ensure_offline_ghost_lots(&mut self) {
        if self.ghost_lots.is_empty() {
            self.ghost_lots.push(GhostLot::offline_row("offline"));
        }
    }

    /// Panel rows: existing contracts plus offline ghost lots.
    pub fn ah_panel_rows(&self) -> Vec<String> {
        let mut rows: Vec<String> = self.contracts.iter().map(|c| c.line()).collect();
        rows.extend(self.ghost_lots.iter().map(|g| g.line()));
        rows
    }

    /// Bind/Settle already list/take. No new AH verb.
    pub fn ah_list_or_take(&mut self) -> &'static str {
        if !self.ah_window_open {
            return "idle";
        }
        self.act_local()
    }

    /// CARD F3 — sealed Hostile may Take / refuse / embargo. Other stances idle.
    pub fn hostile_practice_may(&self, sealed: bool, verb: HostilePractice) -> bool {
        crate::persona::sealed_hostile_may_practice(sealed, self.soul_stance, verb)
    }

    /// CARD F3 — run Hostile practice. Does not bind. Does not open AH.
    /// CARD F9 — Hostile Take writes a session ledger row + NEVC display label.
    pub fn try_hostile_practice(
        &mut self,
        sealed: bool,
        verb: HostilePractice,
    ) -> HostilePracticeOutcome {
        if !self.hostile_practice_may(sealed, verb) {
            return HostilePracticeOutcome::Idle;
        }
        self.last_hostile_practice = Some(verb);
        match verb {
            HostilePractice::Take => {
                self.write_hostile_take_ledger_row();
                HostilePracticeOutcome::Taken
            }
            HostilePractice::Refuse => HostilePracticeOutcome::Refused,
            HostilePractice::Embargo => {
                self.hostile_embargo = true;
                HostilePracticeOutcome::Embargoed
            }
        }
    }

    /// CARD F9 — write Hostile Take ledger row + NEVC display label. No wage.
    pub fn write_hostile_take_ledger_row(&mut self) {
        let label = crate::nevc_visibility::hostile_take_stance_nevc_label();
        self.last_nevc_label = Some(label.clone());
        self.last_hostile_take_row = Some(HostileTakeLedgerRow::from_nevc_label(label));
    }

    /// CARD F9 — Hostile Take ledger rows. Session display. Never a Place.
    pub fn hostile_take_ledger_rows(&self) -> Vec<String> {
        self.last_hostile_take_row
            .as_ref()
            .map(|row| vec![row.line()])
            .unwrap_or_default()
    }

    /// CARD F3 — Hostile practice never binds. Open-trade Bind is the existing verb (F9 cue).
    pub fn hostile_practice_uses_open_trade_bind() -> bool {
        false
    }

    /// CARD F9 — existing Bind verb already listed the contract. No new verb.
    pub fn is_bound(&self) -> bool {
        self.open()
            .map(|c| {
                matches!(
                    c.state,
                    ContractState::Taken | ContractState::Escorting | ContractState::Settled
                )
            })
            .unwrap_or(false)
    }

    /// CARD F9 — sealed Open-trade Bind shows Reserve cue on the existing allocate path.
    /// F2 AH unread beyond the Open-trade gate. No new verb.
    pub fn open_trade_bind_reserve_cue(
        &self,
        sealed: bool,
        allocation: &crate::climate_node::Allocation,
    ) -> Option<String> {
        let may_show = sealed
            && offline_simulated_people_will_trade(self.soul_stance)
            && self.is_bound()
            && !crate::persona::f9_invents_new_verb();
        crate::nevc_visibility::open_trade_bind_reserve_cue(may_show, allocation)
    }

    pub fn honored_offline(&self) -> SimulatedPeopleHonor {
        offline_simulated_people_honor(self.soul_stance)
    }

    /// CARD F4 — sealed soul may serve another well offline. Dress stays.
    /// Does not Bind. Does not open AH. Does not write Hostile Take / NEVC.
    pub fn try_offline_double_serve(
        &mut self,
        sealed: bool,
        sealed_dress: &str,
        well_of: &str,
    ) -> bool {
        let Some(serve) = OfflineDoubleServe::try_serve(sealed, sealed_dress, well_of) else {
            return false;
        };
        self.last_offline_double_serve = Some(serve);
        true
    }

    /// CARD F4 — Online human double-serve waits F10 / Online-yes.
    pub fn online_human_double_serve(&self) -> bool {
        crate::persona::online_human_double_serve_implemented()
    }

    /// CARD F4 — dress after last offline double-serve. None if none ran.
    pub fn dress_after_offline_double_serve(&self) -> Option<&str> {
        self.last_offline_double_serve
            .as_ref()
            .map(|serve| serve.dress_after())
    }

    pub fn offline_people_will_trade(&self) -> bool {
        offline_simulated_people_will_trade(self.soul_stance)
    }

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

    pub fn clear_lethal_local(&mut self) -> &'static str {
        self.open_mut().map(|c| c.clear_lethal()).unwrap_or("idle")
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
    fn clear_lethal_keeps_tariff() {
        let mut c = LedgerContract::from_i2("abc");
        assert_eq!(c.opt_lethal(), "lethal");
        assert_eq!(c.purse.blood_tariff, 2.0);
        assert_eq!(c.clear_lethal(), "cleared");
        assert_eq!(c.win, WinCondition::BindEscort);
        assert_eq!(c.purse.blood_tariff, 2.0);
    }

    #[test]
    fn default_listing_still_binds() {
        let mut b = LedgerBoard::default();
        b.ensure_i2("x");
        assert_eq!(b.open().unwrap().win, WinCondition::BindEscort);
        assert_eq!(b.act_local(), "bound");
    }

    #[test]
    fn f1_sealed_soul_can_set_open_trade_neutral_closed_hostile() {
        let mut b = LedgerBoard::default();
        assert!(b.soul_stance.is_none());
        for stance in SoulStance::ALL {
            assert_eq!(b.set_sealed_soul_stance(true, stance), Some(stance));
            assert_eq!(b.soul_stance, Some(stance));
        }
        assert!(b.set_sealed_soul_stance(false, SoulStance::OpenTrade).is_none());
    }

    #[test]
    fn f1_garden_light_has_no_stance_and_cannot_trade() {
        let mut b = LedgerBoard::default();
        assert!(b.soul_stance.is_none());
        assert_eq!(b.honored_offline(), SimulatedPeopleHonor::NoStanceNoTrade);
        assert!(!b.offline_people_will_trade());
        assert!(!offline_simulated_people_will_trade(None));
        assert!(b.set_sealed_soul_stance(false, SoulStance::OpenTrade).is_none());
        b.clear_garden_light_stance();
        assert!(b.soul_stance.is_none());
        assert!(!b.offline_people_will_trade());
    }

    #[test]
    fn f1_offline_simulated_peoples_honor_stance() {
        assert_eq!(
            offline_simulated_people_honor(Some(SoulStance::OpenTrade)),
            SimulatedPeopleHonor::HonorTrade
        );
        assert!(offline_simulated_people_will_trade(Some(SoulStance::OpenTrade)));
        assert_eq!(
            offline_simulated_people_honor(Some(SoulStance::Neutral)),
            SimulatedPeopleHonor::HonorNeutral
        );
        assert!(!offline_simulated_people_will_trade(Some(SoulStance::Neutral)));
        assert_eq!(
            offline_simulated_people_honor(Some(SoulStance::Closed)),
            SimulatedPeopleHonor::HonorClosed
        );
        assert!(!offline_simulated_people_will_trade(Some(SoulStance::Closed)));
        assert_eq!(
            offline_simulated_people_honor(Some(SoulStance::Hostile)),
            SimulatedPeopleHonor::HonorHostile
        );
        assert!(!offline_simulated_people_will_trade(Some(SoulStance::Hostile)));
        assert_eq!(
            offline_simulated_people_honor(None),
            SimulatedPeopleHonor::NoStanceNoTrade
        );

        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::OpenTrade);
        assert_eq!(b.honored_offline(), SimulatedPeopleHonor::HonorTrade);
        assert!(b.offline_people_will_trade());
        b.set_sealed_soul_stance(true, SoulStance::Closed);
        assert_eq!(b.honored_offline(), SimulatedPeopleHonor::HonorClosed);
        assert!(!b.offline_people_will_trade());
    }

    #[test]
    fn f1_steward_online_yes_false_online_grey() {
        use crate::hex_listen::PowrushNet;
        use crate::hex_protocol::default_client_listens;
        use crate::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};
        use crate::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(!PowrushNet::Off.title_online_enabled());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!default_client_listens());
    }

    #[test]
    fn f1_place_id_local_hexes_len_three() {
        use crate::hex_travel::{PlaceId, LOCAL_HEXES};
        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
    }

    #[test]
    fn f1_no_new_persist_schema_file() {
        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::Neutral);
        let json = serde_json::to_string(&b).expect("board json");
        let back: LedgerBoard = serde_json::from_str(&json).expect("load");
        assert_eq!(back.soul_stance, Some(SoulStance::Neutral));
        assert!(!json.contains("powrush_stance.json"));
        let empty = serde_json::to_string(&LedgerBoard::default()).expect("default");
        assert!(!empty.contains("soul_stance"), "garden light omits stance key");
        assert_eq!(crate::persona::PERSONA_PATH, "data/powrush_persona.json");
        assert_ne!(crate::persona::PERSONA_FILE_NAME, "powrush_stance.json");
    }

    /// CARD F2 — AH panel opens only when sealed stance is Open-trade.
    #[test]
    fn f2_ah_panel_opens_only_when_sealed_open_trade() {
        let mut b = LedgerBoard::default();
        assert!(!b.ah_window_open);
        assert!(!b.try_open_ah_panel(false));
        assert!(!b.ah_window_open);

        b.set_sealed_soul_stance(true, SoulStance::Neutral);
        assert!(!b.try_open_ah_panel(true));
        assert!(!b.ah_window_open);
        b.set_sealed_soul_stance(true, SoulStance::Closed);
        assert!(!b.try_open_ah_panel(true));
        b.set_sealed_soul_stance(true, SoulStance::Hostile);
        assert!(!b.try_open_ah_panel(true));

        b.set_sealed_soul_stance(true, SoulStance::OpenTrade);
        assert!(b.ah_panel_may_open(true));
        assert!(b.try_open_ah_panel(true));
        assert!(b.ah_window_open);
        assert!(!F2_AH_IS_PLACE);
    }

    /// CARD F2 — Garden light cannot open the AH window.
    #[test]
    fn f2_garden_light_cannot_open_ah_window() {
        let mut b = LedgerBoard::default();
        assert!(b.soul_stance.is_none());
        assert!(!b.try_open_ah_panel(false));
        assert!(!b.ah_window_open);
        assert!(!ah_panel_may_open(false, None));
        assert!(!ah_panel_may_open(false, Some(SoulStance::OpenTrade)));
        b.clear_garden_light_stance();
        assert!(!b.try_open_ah_panel(false));
        assert!(!b.ah_window_open);
    }

    /// CARD F2 — Window closes when stance leaves Open-trade.
    #[test]
    fn f2_window_closes_when_stance_leaves_open_trade() {
        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::OpenTrade);
        assert!(b.try_open_ah_panel(true));
        assert!(b.ah_window_open);

        b.set_sealed_soul_stance(true, SoulStance::Neutral);
        assert!(!b.ah_window_open);
        assert!(b.try_open_ah_panel(true) == false);

        b.set_sealed_soul_stance(true, SoulStance::OpenTrade);
        assert!(b.try_open_ah_panel(true));
        b.set_sealed_soul_stance(true, SoulStance::Closed);
        assert!(!b.ah_window_open);

        b.set_sealed_soul_stance(true, SoulStance::OpenTrade);
        assert!(b.try_open_ah_panel(true));
        b.set_sealed_soul_stance(true, SoulStance::Hostile);
        assert!(!b.ah_window_open);

        b.set_sealed_soul_stance(true, SoulStance::OpenTrade);
        assert!(b.try_open_ah_panel(true));
        b.clear_garden_light_stance();
        assert!(!b.ah_window_open);
    }

    /// CARD F2 / F8 — Offline ghost lots live as ledger rows, not a Place.
    #[test]
    fn f2_offline_ghost_lots_live_as_ledger_rows_not_a_place() {
        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::OpenTrade);
        assert!(b.try_open_ah_panel(true));
        assert!(!b.ghost_lots.is_empty());
        assert!(!GhostLot::is_place());
        assert!(!F2_AH_IS_PLACE);
        let rows = b.ah_panel_rows();
        assert!(rows.iter().any(|r| r.contains("Ghost lot") && r.contains("not a Place")));
        for lot in &b.ghost_lots {
            assert_ne!(lot.label, "Sanctuary");
            assert_ne!(lot.label, "Heartwood");
            assert_ne!(lot.label, "Depths");
            assert!(!lot.line().contains("PlaceId"));
        }
        b.ensure_i2("local-i2");
        assert_eq!(b.ah_list_or_take(), "bound");
        assert_eq!(b.ah_list_or_take(), "escorting");
        assert_eq!(crate::hex_travel::LOCAL_HEXES.len(), 3);
    }

    /// CARD F2 — PlaceId / LOCAL_HEXES len == 3. AH is not a Place.
    #[test]
    fn f2_place_id_local_hexes_len_three() {
        use crate::hex_travel::{PlaceId, LOCAL_HEXES};
        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
        for place in LOCAL_HEXES {
            assert_ne!(place.as_str(), "ah");
            assert_ne!(place.as_str(), "auction");
            assert_ne!(place.as_str(), "auction_house");
            assert_ne!(place.as_str(), "market");
        }
        assert!(!F2_AH_IS_PLACE);
    }

    /// CARD F2 — STEWARD_ONLINE_YES false / Online grey.
    #[test]
    fn f2_steward_online_yes_false_online_grey() {
        use crate::hex_listen::PowrushNet;
        use crate::hex_protocol::default_client_listens;
        use crate::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};
        use crate::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(!PowrushNet::Off.title_online_enabled());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!default_client_listens());
    }

    /// CARD F2 — 0 meshes · no auction_*.rs · no new persist crate.
    #[test]
    fn f2_zero_meshes_no_auction_rs_no_new_persist_crate() {
        use std::path::Path;

        assert_eq!(F2_MESH_BUDGET, 0);
        assert!(!F2_AH_IS_PLACE);
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(!here.join("auction.rs").exists());
        assert!(!here.join("auction_house.rs").exists());
        assert!(!here.join("auction_window.rs").exists());
        assert_eq!(crate::persona::PERSONA_PATH, "data/powrush_persona.json");
        assert_ne!(crate::persona::PERSONA_FILE_NAME, "powrush_auction.json");
        assert!(!crate::persona::PERSONA_PATH.contains("auction"));

        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::OpenTrade);
        b.try_open_ah_panel(true);
        let json = serde_json::to_string(&b).expect("board json");
        assert!(!json.contains("ah_window_open"), "panel is session, not a persist key");
        assert!(!json.contains("powrush_auction.json"));
    }

    /// CARD F3 — sealed Hostile may Take / refuse / embargo. Not Bind.
    #[test]
    fn f3_sealed_hostile_may_take_refuse_embargo() {
        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::Hostile);
        assert_eq!(b.honored_offline(), SimulatedPeopleHonor::HonorHostile);
        assert!(!b.offline_people_will_trade());
        assert!(!b.ah_panel_may_open(true), "F2 AH gate unread beyond stance");
        assert!(!b.try_open_ah_panel(true));
        assert!(!b.ah_window_open);

        b.ensure_i2("f3-hostile");
        assert_eq!(b.open().unwrap().state, ContractState::Posted);
        assert_eq!(
            b.try_hostile_practice(true, HostilePractice::Take),
            HostilePracticeOutcome::Taken
        );
        assert_eq!(
            b.try_hostile_practice(true, HostilePractice::Refuse),
            HostilePracticeOutcome::Refused
        );
        assert_eq!(
            b.try_hostile_practice(true, HostilePractice::Embargo),
            HostilePracticeOutcome::Embargoed
        );
        assert!(b.hostile_embargo);
        assert_eq!(b.last_hostile_practice, Some(HostilePractice::Embargo));
        assert_eq!(
            b.open().unwrap().state,
            ContractState::Posted,
            "Hostile practice does not Bind"
        );
        assert!(!LedgerBoard::hostile_practice_uses_open_trade_bind());
    }

    /// CARD F3 — NEVC labels display only (no wage invent).
    #[test]
    fn f3_nevc_labels_display_only_no_wage_invent() {
        assert!(!crate::persona::HOSTILE_PRACTICE_INVENTS_WAGES);
        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::Hostile);
        assert_eq!(
            b.try_hostile_practice(true, HostilePractice::Take),
            HostilePracticeOutcome::Taken
        );
        let purse = b.open().map(|c| c.purse.line()).unwrap_or_default();
        assert!(!purse.contains("wage"));
        assert!(!purse.contains("gold"));
    }

    /// CARD F3 — no lockout · soul stays playable under Hostile.
    #[test]
    fn f3_no_lockout_soul_stays_playable_under_hostile() {
        use crate::persona::{soul_is_locked_out, soul_stays_playable_under_hostile, HOSTILE_LOCKOUT};

        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::Hostile);
        assert!(!HOSTILE_LOCKOUT);
        assert!(!soul_is_locked_out(true, b.soul_stance));
        assert!(soul_stays_playable_under_hostile(true, b.soul_stance));
        assert_eq!(
            b.try_hostile_practice(true, HostilePractice::Take),
            HostilePracticeOutcome::Taken
        );
        for stance in [SoulStance::OpenTrade, SoulStance::Neutral, SoulStance::Closed] {
            b.set_sealed_soul_stance(true, stance);
            assert_eq!(
                b.try_hostile_practice(true, HostilePractice::Take),
                HostilePracticeOutcome::Idle
            );
        }
        b.clear_garden_light_stance();
        assert_eq!(
            b.try_hostile_practice(false, HostilePractice::Embargo),
            HostilePracticeOutcome::Idle
        );
    }

    /// CARD F3 — PlaceId / LOCAL_HEXES len == 3.
    #[test]
    fn f3_place_id_local_hexes_len_three() {
        use crate::hex_travel::{PlaceId, LOCAL_HEXES};
        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
        assert!(!F2_AH_IS_PLACE);
    }

    /// CARD F3 — STEWARD_ONLINE_YES false / Online grey.
    #[test]
    fn f3_steward_online_yes_false_online_grey() {
        use crate::hex_listen::PowrushNet;
        use crate::hex_protocol::default_client_listens;
        use crate::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};
        use crate::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(!PowrushNet::Off.title_online_enabled());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!default_client_listens());
    }

    /// CARD F3 — 0 meshes · no auction_*.rs · no new persist file.
    #[test]
    fn f3_zero_meshes_no_auction_rs_no_new_persist_file() {
        use std::path::Path;

        assert_eq!(F3_MESH_BUDGET, 0);
        assert_eq!(F2_MESH_BUDGET, 0);
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(!here.join("auction.rs").exists());
        assert!(!here.join("auction_house.rs").exists());
        assert!(!here.join("hostile_practice.rs").exists());
        assert_eq!(crate::persona::PERSONA_PATH, "data/powrush_persona.json");
        assert_ne!(crate::persona::PERSONA_FILE_NAME, "powrush_hostile.json");

        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::Hostile);
        b.try_hostile_practice(true, HostilePractice::Embargo);
        let json = serde_json::to_string(&b).expect("board json");
        assert!(!json.contains("hostile_embargo"), "embargo is session, not a persist key");
        assert!(!json.contains("last_hostile_practice"));
        assert!(!json.contains("powrush_hostile.json"));
        assert!(!json.contains("powrush_auction.json"));
    }

    /// CARD F9 — sealed Hostile Take writes a ledger row + NEVC display label (no wage).
    #[test]
    fn f9_sealed_hostile_take_writes_ledger_row_nevc_label_no_wage() {
        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::Hostile);
        assert!(!b.offline_people_will_trade());
        assert!(!b.ah_panel_may_open(true), "F2 AH unread beyond Open-trade gate");
        assert!(!b.try_open_ah_panel(true));
        b.ensure_i2("f9-hostile-take");
        assert_eq!(
            b.try_hostile_practice(true, HostilePractice::Take),
            HostilePracticeOutcome::Taken
        );
        let row = b.last_hostile_take_row.as_ref().expect("ledger row");
        assert!(row.line().contains("Hostile Take"));
        assert!(row.line().contains("ledger row"));
        assert!(row.line().contains("not wages"));
        assert!(!row.invents_wages());
        assert!(!HostileTakeLedgerRow::is_place());
        let label = b.last_nevc_label.as_deref().expect("NEVC label");
        assert!(label.contains("NEVC:"));
        assert!(label.contains("display only"));
        assert!(label.contains("not wages"));
        assert!(!crate::nevc_visibility::nevc_line_invents_wages(label));
        assert!(!crate::persona::HOSTILE_PRACTICE_INVENTS_WAGES);
        assert_eq!(
            b.open().unwrap().state,
            ContractState::Posted,
            "Hostile Take does not Bind"
        );
        assert!(!LedgerBoard::hostile_practice_uses_open_trade_bind());
        let rows = b.hostile_take_ledger_rows();
        assert_eq!(rows.len(), 1);
        assert!(rows[0].contains("Hostile Take"));
        let purse = b.open().unwrap().purse.line();
        assert!(!purse.contains("wage"));
        assert!(!purse.contains("gold"));
    }

    /// CARD F9 — sealed Open-trade Bind shows Reserve cue on existing allocate path.
    #[test]
    fn f9_sealed_open_trade_bind_shows_reserve_cue_on_allocate_path() {
        use crate::climate_node::Allocation;

        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::OpenTrade);
        b.ensure_i2("f9-open-trade-bind");
        assert_eq!(b.act_local(), "bound", "existing Bind verb — no new verb");
        assert!(b.is_bound());
        assert!(!crate::persona::f9_invents_new_verb());
        let mut allocation = Allocation::default();
        allocation.reserve = 1;
        let cue = b
            .open_trade_bind_reserve_cue(true, &allocation)
            .expect("Reserve cue");
        assert_eq!(cue, allocation.reserve_bank_line().expect("banked"));
        assert!(cue.contains("Reserve"));
        assert!(cue.contains("repair-rights"));
        assert!(!crate::nevc_visibility::nevc_line_invents_wages(&cue));
        assert_eq!(
            b.open_trade_bind_reserve_cue(true, &Allocation::default())
                .as_deref(),
            Some(OPEN_TRADE_BIND_RESERVE_CUE)
        );
        b.set_sealed_soul_stance(true, SoulStance::Hostile);
        assert!(b
            .open_trade_bind_reserve_cue(true, &allocation)
            .is_none());
        assert!(!b.ah_panel_may_open(true), "F2 AH unread beyond Open-trade gate");
    }

    /// CARD F9 — garden light gets neither.
    #[test]
    fn f9_garden_light_gets_neither() {
        use crate::climate_node::Allocation;

        let mut b = LedgerBoard::default();
        assert!(b.soul_stance.is_none());
        assert!(!crate::persona::garden_light_gets_nevc_on_stance());
        assert_eq!(
            b.try_hostile_practice(false, HostilePractice::Take),
            HostilePracticeOutcome::Idle
        );
        assert!(b.last_hostile_take_row.is_none());
        assert!(b.last_nevc_label.is_none());
        assert!(b.hostile_take_ledger_rows().is_empty());
        b.ensure_i2("f9-garden");
        assert_eq!(b.act_local(), "bound");
        let mut allocation = Allocation::default();
        allocation.reserve = 1;
        assert!(b.open_trade_bind_reserve_cue(false, &allocation).is_none());
        b.clear_garden_light_stance();
        assert!(b.soul_stance.is_none());
        assert!(b.open_trade_bind_reserve_cue(false, &allocation).is_none());
    }

    /// CARD F9 — PlaceId / LOCAL_HEXES len == 3.
    #[test]
    fn f9_place_id_local_hexes_len_three() {
        use crate::hex_travel::{PlaceId, LOCAL_HEXES};
        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
        assert!(!F2_AH_IS_PLACE);
        assert!(!HostileTakeLedgerRow::is_place());
    }

    /// CARD F9 — STEWARD_ONLINE_YES false / Online grey.
    #[test]
    fn f9_steward_online_yes_false_online_grey() {
        use crate::hex_listen::PowrushNet;
        use crate::hex_protocol::default_client_listens;
        use crate::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};
        use crate::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(!PowrushNet::Off.title_online_enabled());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!default_client_listens());
    }

    /// CARD F9 — no lockout · 0 meshes · no auction_*.rs · no new persist file.
    #[test]
    fn f9_no_lockout_zero_meshes_no_auction_rs_no_new_persist_file() {
        use std::path::Path;

        use crate::persona::{soul_is_locked_out, HOSTILE_LOCKOUT};

        assert!(!HOSTILE_LOCKOUT);
        assert_eq!(F9_MESH_BUDGET, 0);
        assert_eq!(F3_MESH_BUDGET, 0);
        assert_eq!(F2_MESH_BUDGET, 0);
        assert!(!crate::persona::f9_invents_new_verb());
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(!here.join("auction.rs").exists());
        assert!(!here.join("auction_house.rs").exists());
        assert!(!here.join("nevc_stance.rs").exists());
        assert_eq!(crate::persona::PERSONA_PATH, "data/powrush_persona.json");
        assert_ne!(crate::persona::PERSONA_FILE_NAME, "powrush_nevc_stance.json");

        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::Hostile);
        assert!(!soul_is_locked_out(true, b.soul_stance));
        b.try_hostile_practice(true, HostilePractice::Take);
        let json = serde_json::to_string(&b).expect("board json");
        assert!(!json.contains("last_hostile_take_row"), "Take row is session, not a persist key");
        assert!(!json.contains("last_nevc_label"));
        assert!(!json.contains("powrush_nevc_stance.json"));
        assert!(!json.contains("powrush_auction.json"));
    }

    /// CARD F4 — sealed dress unchanged when soul serves another well offline.
    #[test]
    fn f4_sealed_dress_unchanged_when_serving_another_well_offline() {
        let mut b = LedgerBoard::default();
        assert!(b.try_offline_double_serve(true, "Cydruid", "Draek"));
        assert_eq!(b.dress_after_offline_double_serve(), Some("Cydruid"));
        let serve = b.last_offline_double_serve.as_ref().expect("double-serve");
        assert_eq!(serve.sealed_dress, "Cydruid");
        assert_eq!(serve.well_of, "Draek");
        assert_eq!(serve.dress_after(), "Cydruid");
        assert!(!OfflineDoubleServe::is_place());
        assert!(!crate::persona::F4_DRESS_SWAP);
        assert!(!b.ah_window_open, "F2 AH unread beyond existing wire");
        assert!(b.last_hostile_take_row.is_none(), "F9 Take unread");
        assert!(b.last_nevc_label.is_none(), "F9 NEVC unread");
    }

    /// CARD F4 — Offline double-serve works without dress swap.
    #[test]
    fn f4_offline_double_serve_works_without_dress_swap() {
        let mut b = LedgerBoard::default();
        assert!(b.try_offline_double_serve(true, "Human", "Quellorian"));
        assert_eq!(b.dress_after_offline_double_serve(), Some("Human"));
        assert!(
            !b.try_offline_double_serve(true, "Human", "Human"),
            "home well is not a double-serve"
        );
        assert_eq!(
            b.dress_after_offline_double_serve(),
            Some("Human"),
            "failed home-well serve does not swap dress"
        );
        assert!(!b.try_offline_double_serve(false, "Human", "Draek"));
        assert!(!crate::persona::F4_DRESS_SWAP);
        assert!(!F2_AH_IS_PLACE);
    }

    /// CARD F4 — Online human double-serve NOT implemented / gated until Online-yes.
    #[test]
    fn f4_online_human_double_serve_not_implemented_gated_until_online_yes() {
        let b = LedgerBoard::default();
        assert!(!crate::persona::F4_ONLINE_HUMAN_DOUBLE_SERVE);
        assert!(!b.online_human_double_serve());
        assert!(crate::persona::online_human_double_serve_gated());
        assert!(!crate::persona::online_human_double_serve_implemented());
    }

    /// CARD F4 — PlaceId / LOCAL_HEXES len == 3.
    #[test]
    fn f4_place_id_local_hexes_len_three() {
        use crate::hex_travel::{PlaceId, LOCAL_HEXES};
        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
        assert!(!F2_AH_IS_PLACE);
        assert!(!OfflineDoubleServe::is_place());
        assert!(!HostileTakeLedgerRow::is_place());
    }

    /// CARD F4 — STEWARD_ONLINE_YES false / Online grey.
    #[test]
    fn f4_steward_online_yes_false_online_grey() {
        use crate::hex_listen::PowrushNet;
        use crate::hex_protocol::default_client_listens;
        use crate::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};
        use crate::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(!PowrushNet::Off.title_online_enabled());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!default_client_listens());
    }

    /// CARD F4 — no lockout · 0 meshes · no auction_*.rs · no new persist file.
    #[test]
    fn f4_no_lockout_zero_meshes_no_auction_rs_no_new_persist_file() {
        use std::path::Path;

        use crate::persona::{soul_is_locked_out, soul_stays_playable_under_double_serve, F4_LOCKOUT};

        assert!(!F4_LOCKOUT);
        assert!(soul_stays_playable_under_double_serve(true));
        assert_eq!(F4_MESH_BUDGET, 0);
        assert_eq!(F9_MESH_BUDGET, 0);
        assert_eq!(F3_MESH_BUDGET, 0);
        assert_eq!(F2_MESH_BUDGET, 0);
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(!here.join("auction.rs").exists());
        assert!(!here.join("auction_house.rs").exists());
        assert!(!here.join("double_agent.rs").exists());
        assert_eq!(crate::persona::PERSONA_PATH, "data/powrush_persona.json");
        assert_ne!(crate::persona::PERSONA_FILE_NAME, "powrush_double_agent.json");

        let mut b = LedgerBoard::default();
        b.set_sealed_soul_stance(true, SoulStance::OpenTrade);
        assert!(!soul_is_locked_out(true, b.soul_stance));
        assert!(b.try_offline_double_serve(true, "Draek", "Ambrosian"));
        let json = serde_json::to_string(&b).expect("board json");
        assert!(
            !json.contains("last_offline_double_serve"),
            "double-serve is session, not a persist key"
        );
        assert!(!json.contains("powrush_double_agent.json"));
        assert!(!json.contains("powrush_auction.json"));
    }
}
