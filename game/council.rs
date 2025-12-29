use std::collections::HashMap;

/// Council Voting System — Grace-Aligned Governance
/// Votes weighted by grace points + empathy score
/// Mercy coforging: high grace = higher influence

pub struct CouncilSystem {
    /// Player grace points (from RBE/contribution)
    grace_points: HashMap<String, u64>,
    /// Player empathy score (from forgiveness/community feedback, 0.0–1.0)
    empathy_score: HashMap<String, f64>,
    /// Current proposals: proposal_id -> (description, votes_for, votes_against)
    proposals: HashMap<u64, (String, u64, u64)>,
    /// Elected council members (player_id list)
    council_members: Vec<String>,
    next_proposal_id: u64,
}

impl CouncilSystem {
    pub fn new() -> Self {
        Self {
            grace_points: HashMap::new(),
            empathy_score: HashMap::new(),
            proposals: HashMap::new(),
            council_members: Vec::new(),
            next_proposal_id: 1,
        }
    }

    /// Add grace points (from quests, building, service)
    pub fn add_grace(&mut self, player_id: &str, points: u64) {
        *self.grace_points.entry(player_id.to_string()).or_insert(0) += points;
    }

    /// Set empathy score (from community forgiveness/feedback, 0.0–1.0)
    pub fn set_empathy(&mut self, player_id: &str, score: f64) {
        self.empathy_score.insert(player_id.to_string(), score.clamp(0.0, 1.0));
    }

    /// Calculate voting weight: grace * empathy multiplier
    pub fn voting_weight(&self, player_id: &str) -> u64 {
        let grace = *self.grace_points.get(player_id).unwrap_or(&0);
        let empathy = *self.empathy_score.get(player_id).unwrap_or(&1.0);
        // Empathy boosts weight — max 2x for perfect empathy
        ((grace as f64) * (1.0 + empathy)) as u64
    }

    /// Submit new proposal
    pub fn submit_proposal(&mut self, description: &str) -> u64 {
        let id = self.next_proposal_id;
        self.proposals.insert(id, (description.to_string(), 0, 0));
        self.next_proposal_id += 1;
        println!("Proposal {} submitted: {}", id, description);
        id
    }

    /// Vote on proposal (for/against)
    pub fn vote(&mut self, player_id: &str, proposal_id: u64, in_favor: bool) {
        if let Some((_, votes_for, votes_against)) = self.proposals.get_mut(&proposal_id) {
            let weight = self.voting_weight(player_id);
            if in_favor {
                *votes_for += weight;
                println!("{} voted FOR proposal {} (weight: {})", player_id, proposal_id, weight);
            } else {
                *votes_against += weight;
                println!("{} voted AGAINST proposal {} (weight: {})", player_id, proposal_id, weight);
            }
        } else {
            println!("Proposal {} not found", proposal_id);
        }
    }

    /// Tally and resolve proposal
    pub fn resolve_proposal(&self, proposal_id: u64) -> Option<bool> {
        if let Some((desc, votes_for, votes_against)) = self.proposals.get(&proposal_id) {
            println!("Proposal {} tally: FOR {} | AGAINST {}", proposal_id, votes_for, votes_against);
            if votes_for > votes_against {
                println!("Proposal PASSED: {}", desc);
                Some(true)
            } else if votes_against > votes_for {
                println!("Proposal REJECTED: {}", desc);
                Some(false)
            } else {
                println!("Proposal TIED — grace meditation required");
                None
            }
        } else {
            println!("Proposal {} not found", proposal_id);
            None
        }
    }

    /// Elect council (top grace holders)
    pub fn elect_council(&mut self, seats: usize) {
        let mut candidates: Vec<_> = self.grace_points.iter().collect();
        candidates.sort_by(|a, b| b.1.cmp(a.1)); // Highest grace first
        self.council_members = candidates.into_iter().take(seats).map(|(id, _)| id.clone()).collect();
        println!("APAGI Council elected: {:?}", self.council_members);
    }
}

// Example usage
pub fn example_council() {
    let mut council = CouncilSystem::new();
    
    // Players earn grace
    council.add_grace("player_123", 5000);
    council.add_grace("player_456", 3000);
    council.add_grace("player_789", 8000);
    
    // Set empathy (from community)
    council.set_empathy("player_123", 0.9); // High empathy
    council.set_empathy("player_456", 0.5);
    council.set_empathy("player_789", 1.0); // Max empathy
    
    // Submit proposal
    let prop_id = council.submit_proposal("Build Eternal Library in Desert Biome");
    
    // Vote
    council.vote("player_123", prop_id, true);
    council.vote("player_456", prop_id, false);
    council.vote("player_789", prop_id, true);
    
    // Resolve
    council.resolve_proposal(prop_id);
    
    // Elect council
    council.elect_council(3);
}
