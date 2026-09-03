//! Hybrid Matrix v0 — Slice 13 (v23.2.17)
//!
//! Double vision under a visible ledger. Stability is the tell.
//! Not a second body, not racial modules, not an F-key.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HybridMatrix {
    pub stability: f32,
    pub attuned: bool,
    pub last_line: String,
}

impl Default for HybridMatrix {
    fn default() -> Self {
        Self {
            stability: 1.0,
            attuned: false,
            last_line: String::new(),
        }
    }
}

impl HybridMatrix {
    pub fn reveal(&mut self) {
        if self.last_line.is_empty() {
            self.last_line = "Hybrid · stability 1 · E Attune (double vision)".into();
        }
    }

    /// See both songs. Does not fuse a module. Does not fire combat.
    pub fn attune(&mut self) -> &'static str {
        if self.attuned {
            return "idle";
        }
        self.attuned = true;
        self.last_line = "Double vision — the ledger still holds · stability 1".into();
        "attuned"
    }

    pub fn slab_line(&self) -> String {
        if self.last_line.is_empty() {
            "Hybrid · stability 1 · E Attune (double vision)".into()
        } else {
            self.last_line.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_stable_not_a_module() {
        let h = HybridMatrix::default();
        assert_eq!(h.stability, 1.0);
        assert!(!h.attuned);
        let blob = format!("{h:?}").to_lowercase();
        assert!(!blob.contains("juke"));
        assert!(!blob.contains("swarm"));
        assert!(!blob.contains("f-key"));
    }

    #[test]
    fn attune_once() {
        let mut h = HybridMatrix::default();
        assert_eq!(h.attune(), "attuned");
        assert!(h.attuned);
        assert_eq!(h.stability, 1.0);
        assert_eq!(h.attune(), "idle");
    }
}
