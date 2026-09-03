//! Vertical factory — Slice 3 (v23.2.7)
//!
//! House found → extractor + depot + hauler + two stops → arrival.
//! Local graph. Two-client AOI waits on the parked server.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

use crate::space_law::CharterKind;

const PLACE_COST: f32 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactoryNodeKind {
    Extractor,
    Depot,
    Hauler,
}

impl FactoryNodeKind {
    pub fn label(self) -> &'static str {
        match self {
            FactoryNodeKind::Extractor => "extractor",
            FactoryNodeKind::Depot => "depot",
            FactoryNodeKind::Hauler => "hauler",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeLife {
    Ghost,
    Live,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactoryNode {
    pub kind: FactoryNodeKind,
    pub life: NodeLife,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerticalFactory {
    pub founded: bool,
    pub kind: CharterKind,
    pub reserve: f32,
    pub nodes: Vec<FactoryNode>,
    pub route_stops: u8,
    pub arrived: bool,
    pub last_line: String,
}

impl Default for VerticalFactory {
    fn default() -> Self {
        Self {
            founded: false,
            kind: CharterKind::House,
            reserve: 3.0,
            nodes: Vec::new(),
            route_stops: 0,
            arrived: false,
            last_line: String::new(),
        }
    }
}

impl VerticalFactory {
    pub fn found_house(&mut self) {
        self.founded = true;
        self.kind = CharterKind::House;
        self.last_line = "House founded — plant an extractor".into();
    }

    pub fn has(&self, kind: FactoryNodeKind) -> bool {
        self.nodes.iter().any(|n| n.kind == kind && n.life == NodeLife::Live)
    }

    /// Tutorial order: extractor → depot → hauler → stop → stop → arrival.
    pub fn advance(&mut self) -> &'static str {
        if !self.founded {
            self.last_line = "Plant a stake first".into();
            return "unfounded";
        }
        if !self.has(FactoryNodeKind::Extractor) {
            return self.commit(FactoryNodeKind::Extractor);
        }
        if !self.has(FactoryNodeKind::Depot) {
            return self.commit(FactoryNodeKind::Depot);
        }
        if !self.has(FactoryNodeKind::Hauler) {
            return self.commit(FactoryNodeKind::Hauler);
        }
        if self.route_stops < 2 {
            self.route_stops += 1;
            self.last_line = format!("Route stop {} — the hauler has a path", self.route_stops);
            return "stop";
        }
        if !self.arrived {
            self.arrived = true;
            self.last_line = "The machine exists — a crate arrived".into();
            return "arrived";
        }
        self.last_line = "The machine exists".into();
        "idle"
    }

    fn commit(&mut self, kind: FactoryNodeKind) -> &'static str {
        if self.reserve < PLACE_COST {
            self.last_line = "Reserve too thin to plant".into();
            return "starved";
        }
        self.reserve -= PLACE_COST;
        self.nodes.push(FactoryNode {
            kind,
            life: NodeLife::Live,
        });
        self.last_line = format!("{} live — the yard takes shape", kind.label());
        "placed"
    }

    pub fn tutorial_complete(&self) -> bool {
        self.arrived && self.has(FactoryNodeKind::Extractor) && self.has(FactoryNodeKind::Depot) && self.has(FactoryNodeKind::Hauler) && self.route_stops >= 2
    }

    pub fn slab_line(&self) -> String {
        if !self.founded {
            return "Q plant a House stake (Frontier)".into();
        }
        if self.arrived {
            return self.last_line.clone();
        }
        format!(
            "Q next · reserve {:.0} · nodes {} · stops {}",
            self.reserve,
            self.nodes.len(),
            self.route_stops
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn house_found_then_crate_arrives() {
        let mut f = VerticalFactory::default();
        assert!(!f.tutorial_complete());
        f.found_house();
        assert_eq!(f.advance(), "placed"); // extractor
        assert_eq!(f.advance(), "placed"); // depot
        assert_eq!(f.advance(), "placed"); // hauler
        assert_eq!(f.advance(), "stop");
        assert_eq!(f.advance(), "stop");
        assert_eq!(f.advance(), "arrived");
        assert!(f.tutorial_complete());
        assert!(f.last_line.contains("crate arrived"));
    }

    #[test]
    fn cannot_place_before_founding() {
        let mut f = VerticalFactory::default();
        assert_eq!(f.advance(), "unfounded");
        assert!(f.nodes.is_empty());
    }
}
