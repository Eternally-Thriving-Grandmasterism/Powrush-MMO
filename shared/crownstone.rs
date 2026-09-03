//! Crownstone set-piece — Slice 11 (v23.2.15)
//!
//! A witnessed stone. Integrity visible. Path stays Unset.
//! Not the Hivelord trilemma, not Brood Spire, not an F-key.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// Destroy / purify / sabotage stay named and idle. This slice does not fire them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CrownPath {
    #[default]
    Unset,
    Purify,
    Destroy,
    Sabotage,
}

impl CrownPath {
    pub fn label(self) -> &'static str {
        match self {
            CrownPath::Unset => "Unset",
            CrownPath::Purify => "Purify",
            CrownPath::Destroy => "Destroy",
            CrownPath::Sabotage => "Sabotage",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrownstoneState {
    pub integrity: f32,
    pub owner: Option<String>,
    pub corruption: f32,
    pub path: CrownPath,
    pub witnessed: bool,
    pub last_line: String,
}

impl Default for CrownstoneState {
    fn default() -> Self {
        Self {
            integrity: 1.0,
            owner: None,
            corruption: 0.0,
            path: CrownPath::Unset,
            witnessed: false,
            last_line: String::new(),
        }
    }
}

impl CrownstoneState {
    pub fn reveal(&mut self) {
        if self.last_line.is_empty() {
            self.last_line = "Crownstone · integrity 1 · path Unset · E Witness".into();
        }
    }

    /// See the stone. Does not pick a path. Does not fire a kill.
    pub fn witness(&mut self) -> &'static str {
        if self.witnessed {
            return "idle";
        }
        self.witnessed = true;
        self.last_line = "The stone is seen — path waits · integrity 1 · no owner".into();
        "witnessed"
    }

    pub fn slab_line(&self) -> String {
        if self.last_line.is_empty() {
            "Crownstone · integrity 1 · path Unset · E Witness".into()
        } else {
            self.last_line.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_path_is_unset() {
        let c = CrownstoneState::default();
        assert_eq!(c.path, CrownPath::Unset);
        assert!(!c.witnessed);
        assert_eq!(c.integrity, 1.0);
        assert!(c.owner.is_none());
        assert_eq!(c.corruption, 0.0);
        let blob = format!("{c:?}");
        assert!(!blob.to_lowercase().contains("hivelord"));
    }

    #[test]
    fn witness_once_leaves_path_unset() {
        let mut c = CrownstoneState::default();
        c.reveal();
        assert_eq!(c.witness(), "witnessed");
        assert!(c.witnessed);
        assert_eq!(c.path, CrownPath::Unset);
        assert_eq!(c.witness(), "idle");
        assert!(c.last_line.contains("path waits"));
    }
}
