//! Cross-realm bridging challenges — high-road transfer practice seeds
//! v21.91.0
//!
//! Same underlying principle, different surface features across realms.
//! Designed for deliberate abstraction (bridging), not surface cloning (hugging).
//!
//! Contact: info@Rathor.ai | TOLC 8. Yoi ⚡

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

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
            Self::MercyFirstHighStakes => "mercy-first prioritization when stakes are high"
            Self::CrossRealmAbundanceSharing => "cross-realm abundance sharing with sustainability"
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

    pub fn activate(&mut self, id: u64) -> bool {
        if self.challenges.iter().any(|c| c.id == id && !c.completed) {
            self.active_id = Some(id);
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
        }
    }

    pub fn active(&self) -> Option<&CrossRealmChallenge> {
        self.active_id
            .and_then(|id| self.challenges.iter().find(|c| c.id == id))
    }

    pub fn pending(&self) -> impl Iterator<Item = &CrossRealmChallenge> {
        self.challenges.iter().filter(|c| !c.completed)
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
    reg.seed_defaults();
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

pub struct CrossRealmChallengePlugin;

impl Plugin for CrossRealmChallengePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CrossRealmChallengeRegistry>()
            .add_systems(
                Update,
                (
                    cross_realm_challenge_seed_system,
                    cross_realm_challenge_pulse_system,
                )
                    .chain(),
            );
        info!("CrossRealmChallengePlugin — high-road bridging seeds active");
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
