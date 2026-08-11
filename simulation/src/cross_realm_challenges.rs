//! Cross-realm bridging challenges — high-road transfer practice seeds
//! v21.91.2 — Soft progress + mercy-gated completion across realm surfaces
//!
//! Same underlying principle, different surface features across realms.
//! Designed for deliberate abstraction (bridging), not surface cloning (hugging).
//!
//! Contact: info@Rathor.ai | TOLC 8. Yoi ⚡

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::info;

use crate::council::decision::CouncilDecisions;
use crate::multi_realm_harness::{MultiRealmHarness, RealmId};

/// Portable principle the challenge is designed to train.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChallengePrinciple {
    ResourceAllocationUnderUncertainty,
    PeacefulResolutionIncompleteInfo,
    MercyFirstHighStakes,
    CrossRealmAbundanceSharing,
    OpportunityCostUnderTimePressure,
}

impl ChallengePrinciple {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ResourceAllocationUnderUncertainty => {
                "resource allocation under uncertainty"
            }
            Self::PeacefulResolutionIncompleteInfo => {
                "peaceful resolution under incomplete information"
            }
            Self::MercyFirstHighStakes => "mercy-first prioritization when stakes are high",
            Self::CrossRealmAbundanceSharing => "cross-realm abundance sharing with sustainability",
            Self::OpportunityCostUnderTimePressure => {
                "opportunity-cost decision-making under time pressure"
            }
        }
    }

    pub fn tags(&self) -> Vec<&'static str> {
        match self {
            Self::ResourceAllocationUnderUncertainty => vec!["rbe", "allocation", "uncertainty"],
            Self::PeacefulResolutionIncompleteInfo => vec!["peace", "ethics", "council"],
            Self::MercyFirstHighStakes => vec!["mercy", "tolc", "priority"],
            Self::CrossRealmAbundanceSharing => vec!["abundance", "rbe", "multi_realm"],
            Self::OpportunityCostUnderTimePressure => vec!["timing", "priority", "strategy"],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossRealmChallenge {
    pub id: u64,
    pub title: String,
    pub principle: ChallengePrinciple,
    /// Realms that present *different surfaces* of the same principle.
    pub realm_surfaces: Vec<RealmSurfaceVariant>,
    pub difficulty: f32,
    pub mercy_floor: f32,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmSurfaceVariant {
    pub realm_id: RealmId,
    pub surface_label: String,
    pub prompt: String,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct CrossRealmChallengeRegistry {
    pub challenges: Vec<CrossRealmChallenge>,
    pub active_id: Option<u64>,
    pub completed_count: u64,
    pub seeded: bool,
    /// True once id=1 was auto-activated on first multi-realm seed.
    pub bootstrap_activated: bool,
    /// Realms where active challenge already received a mercy-aligned resolve.
    pub progress_realms: HashSet<RealmId>,
    pub last_progress_history_len: usize,
}

impl CrossRealmChallengeRegistry {
    pub fn seed_defaults(&mut self) {
        if self.seeded {
            return;
        }
        self.challenges = default_challenge_seeds();
        self.seeded = true;
        info!(
            target: "ra_thor::cross_realm",
            count = self.challenges.len(),
            "Cross-realm bridging challenges seeded"
        );
    }

    /// Seed + auto-activate challenge id=1 for player-visible practice.
    pub fn seed_and_bootstrap_practice(&mut self) {
        self.seed_defaults();
        if self.bootstrap_activated || self.active_id.is_some() {
            return;
        }
        if self.activate(1) {
            self.bootstrap_activated = true;
            if let Some(c) = self.active() {
                info!(
                    target: "ra_thor::cross_realm",
                    id = c.id,
                    title = %c.title,
                    principle = c.principle.as_str(),
                    surfaces = c.realm_surfaces.len(),
                    "Bootstrap practice active — Caps Across Climates (high-road)"
                );
            }
        }
    }

    pub fn activate(&mut self, id: u64) -> bool {
        if self.challenges.iter().any(|c| c.id == id && !c.completed) {
            self.active_id = Some(id);
            self.progress_realms.clear();
            true
        } else {
            false
        }
    }

    pub fn mark_completed(&mut self, id: u64) {
        if let Some(c) = self.challenges.iter_mut().find(|c| c.id == id) {
            if !c.completed {
                c.completed = true;
                self.completed_count = self.completed_count.saturating_add(1);
            }
        }
        if self.active_id == Some(id) {
            self.active_id = None;
            self.progress_realms.clear();
        }
    }

    pub fn active(&self) -> Option<&CrossRealmChallenge> {
        self.active_id
            .and_then(|id| self.challenges.iter().find(|c| c.id == id))
    }

    pub fn pending(&self) -> impl Iterator<Item = &CrossRealmChallenge> {
        self.challenges.iter().filter(|c| !c.completed)
    }

    /// Record a mercy-aligned resolve on a realm surface of the active challenge.
    /// Completes when every surface realm has been touched at least once.
    pub fn note_surface_progress(&mut self, realm_id: RealmId, mercy: f32) -> bool {
        let Some(active) = self.active().cloned() else {
            return false;
        };
        if mercy < active.mercy_floor {
            return false;
        }
        let is_surface = active
            .realm_surfaces
            .iter()
            .any(|s| s.realm_id == realm_id);
        if !is_surface {
            return false;
        }
        self.progress_realms.insert(realm_id);
        let needed: HashSet<RealmId> = active
            .realm_surfaces
            .iter()
            .map(|s| s.realm_id)
            .collect();
        if needed.is_subset(&self.progress_realms) {
            info!(
                target: "ra_thor::cross_realm",
                id = active.id,
                title = %active.title,
                surfaces_cleared = self.progress_realms.len(),
                "High-road challenge completed — all surfaces practiced under mercy floor"
            );
            self.mark_completed(active.id);
            true
        } else {
            info!(
                target: "ra_thor::cross_realm",
                id = active.id,
                realm = realm_id,
                progress = self.progress_realms.len(),
                needed = needed.len(),
                "Challenge surface progress"
            );
            false
        }
    }
}

fn default_challenge_seeds() -> Vec<CrossRealmChallenge> {
    vec![
        CrossRealmChallenge {
            id: 1,
            title: "Caps Across Climates".into(),
            principle: ChallengePrinciple::ResourceAllocationUnderUncertainty,
            realm_surfaces: vec![
                RealmSurfaceVariant {
                    realm_id: 0,
                    surface_label: "Sanctuary soft cap".into(),
                    prompt: "Sanctuary yields are stable — set a mercy-aligned harvest cap without starving growth.".into(),
                },
                RealmSurfaceVariant {
                    realm_id: 2,
                    surface_label: "Verdant surge".into(),
                    prompt: "Verdant Bloom is flooding nodes — allocate under surplus without collapse of sustainability.".into(),
                },
                RealmSurfaceVariant {
                    realm_id: 4,
                    surface_label: "Horizon scarcity".into(),
                    prompt: "Voidfarer Horizon is thin — allocate under uncertainty with incomplete scouting.".into(),
                },
            ],
            difficulty: 0.55,
            mercy_floor: 0.60,
            completed: false,
        },
        CrossRealmChallenge {
            id: 2,
            title: "Treaty Under Fog".into(),
            principle: ChallengePrinciple::PeacefulResolutionIncompleteInfo,
            realm_surfaces: vec![
                RealmSurfaceVariant {
                    realm_id: 1,
                    surface_label: "Lattice negotiation".into(),
                    prompt: "Synthetic Lattice offers partial telemetry only — resolve without full data.".into(),
                },
                RealmSurfaceVariant {
                    realm_id: 3,
                    surface_label: "Chorus mediation".into(),
                    prompt: "Harmonic Chorus hears conflicting faction songs — choose peaceful resolution.".into(),
                },
            ],
            difficulty: 0.62,
            mercy_floor: 0.65,
            completed: false,
        },
        CrossRealmChallenge {
            id: 3,
            title: "Mercy at the Brink".into(),
            principle: ChallengePrinciple::MercyFirstHighStakes,
            realm_surfaces: vec![
                RealmSurfaceVariant {
                    realm_id: 0,
                    surface_label: "Sanctuary crisis vote".into(),
                    prompt: "High-stakes council vote — prioritize mercy when strength tempts overreach.".into(),
                },
                RealmSurfaceVariant {
                    realm_id: 2,
                    surface_label: "Verdant overharvest".into(),
                    prompt: "Abundance tempts greed — hold mercy floor while pressure rises.".into(),
                },
            ],
            difficulty: 0.70,
            mercy_floor: 0.75,
            completed: false,
        },
        CrossRealmChallenge {
            id: 4,
            title: "Shared Flow".into(),
            principle: ChallengePrinciple::CrossRealmAbundanceSharing,
            realm_surfaces: vec![
                RealmSurfaceVariant {
                    realm_id: 2,
                    surface_label: "Verdant surplus".into(),
                    prompt: "Export surplus without draining sustainability — train portable abundance schema.".into(),
                },
                RealmSurfaceVariant {
                    realm_id: 4,
                    surface_label: "Horizon receive".into(),
                    prompt: "Receive shared flow under stress — same principle, inverted surface.".into(),
                },
            ],
            difficulty: 0.58,
            mercy_floor: 0.62,
            completed: false,
        },
        CrossRealmChallenge {
            id: 5,
            title: "Clock and Cost".into(),
            principle: ChallengePrinciple::OpportunityCostUnderTimePressure,
            realm_surfaces: vec![
                RealmSurfaceVariant {
                    realm_id: 1,
                    surface_label: "Lattice tick pressure".into(),
                    prompt: "Multiple policies compete in one tick — choose opportunity cost wisely.".into(),
                },
                RealmSurfaceVariant {
                    realm_id: 3,
                    surface_label: "Chorus tempo".into(),
                    prompt: "Harmonic windows close fast — prioritize under time pressure.".into(),
                },
            ],
            difficulty: 0.66,
            mercy_floor: 0.60,
            completed: false,
        },
    ]
}

pub fn cross_realm_challenge_seed_system(
    mut reg: ResMut<CrossRealmChallengeRegistry>,
    harness: Res<MultiRealmHarness>,
) {
    if harness.realms.is_empty() {
        return;
    }
    reg.seed_and_bootstrap_practice();
}

/// Soft log of active challenge surfaces (host / debug visibility).
pub fn cross_realm_challenge_pulse_system(
    reg: Res<CrossRealmChallengeRegistry>,
    mut last: Local<Option<u64>>,
) {
    let Some(active) = reg.active() else {
        return;
    };
    if *last == Some(active.id) {
        return;
    }
    *last = Some(active.id);
    info!(
        target: "ra_thor::cross_realm",
        id = active.id,
        title = %active.title,
        principle = active.principle.as_str(),
        surfaces = active.realm_surfaces.len(),
        "Active cross-realm bridging challenge"
    );
}

/// Mercy-gated progress against active challenge surfaces from resolved council history.
pub fn cross_realm_challenge_progress_system(
    mut reg: ResMut<CrossRealmChallengeRegistry>,
    decisions: Option<Res<CouncilDecisions>>,
) {
    let Some(decisions) = decisions else {
        return;
    };
    if reg.active_id.is_none() {
        return;
    }
    if decisions.resolved_history.len() <= reg.last_progress_history_len {
        return;
    }
    let start = reg.last_progress_history_len;
    reg.last_progress_history_len = decisions.resolved_history.len();
    for d in &decisions.resolved_history[start..] {
        let _ = reg.note_surface_progress(d.realm_id, d.mercy_factor);
        if reg.active_id.is_none() {
            break;
        }
    }
}

pub struct CrossRealmChallengePlugin;

impl Plugin for CrossRealmChallengePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CrossRealmChallengeRegistry>()
            .add_systems(
                Update,
                (
                    cross_realm_challenge_seed_system,
                    cross_realm_challenge_pulse_system,
                    cross_realm_challenge_progress_system,
                )
                    .chain(),
            );
        info!("CrossRealmChallengePlugin — bootstrap id=1 + mercy surface progress");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_five_challenges() {
        let mut reg = CrossRealmChallengeRegistry::default();
        reg.seed_defaults();
        assert_eq!(reg.challenges.len(), 5);
        assert!(reg.activate(1));
        assert_eq!(reg.active().unwrap().id, 1);
        reg.mark_completed(1);
        assert_eq!(reg.completed_count, 1);
        assert!(reg.active().is_none());
    }

    #[test]
    fn bootstrap_activates_id_1_once() {
        let mut reg = CrossRealmChallengeRegistry::default();
        reg.seed_and_bootstrap_practice();
        assert!(reg.seeded);
        assert!(reg.bootstrap_activated);
        assert_eq!(reg.active_id, Some(1));
        assert_eq!(reg.active().unwrap().title, "Caps Across Climates");

        reg.seed_and_bootstrap_practice();
        assert_eq!(reg.active_id, Some(1));
        assert!(reg.bootstrap_activated);
    }

    #[test]
    fn surface_progress_completes_when_all_realms_hit() {
        let mut reg = CrossRealmChallengeRegistry::default();
        reg.seed_and_bootstrap_practice();
        assert!(!reg.note_surface_progress(0, 0.7));
        assert!(!reg.note_surface_progress(2, 0.7));
        assert!(reg.note_surface_progress(4, 0.7));
        assert!(reg.active().is_none());
        assert_eq!(reg.completed_count, 1);
    }

    #[test]
    fn below_mercy_floor_no_progress() {
        let mut reg = CrossRealmChallengeRegistry::default();
        reg.seed_and_bootstrap_practice();
        assert!(!reg.note_surface_progress(0, 0.4));
        assert!(reg.progress_realms.is_empty());
    }

    #[test]
    fn principles_have_tags() {
        for p in [
            ChallengePrinciple::ResourceAllocationUnderUncertainty,
            ChallengePrinciple::MercyFirstHighStakes,
        ] {
            assert!(!p.tags().is_empty());
            assert!(!p.as_str().is_empty());
        }
    }
}
