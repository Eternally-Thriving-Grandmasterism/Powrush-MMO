use std::collections::HashMap;

/// Resource-Based Economy System
/// Basics: Unlimited (replicated freely)
/// Advanced: Earned through contribution/grace

pub struct RbeSystem {
    /// Player contribution score (grace points)
    pub grace_points: HashMap<String, u64>,  // player_id -> points
    /// Basic resources — always available
    pub basic_resources: Vec<String>,
    /// Advanced unlock thresholds
    pub advanced_thresholds: HashMap<String, u64>,
}

impl RbeSystem {
    pub fn new() -> Self {
        let mut advanced = HashMap::new();
        advanced.insert("custom_home".to_string(), 1000);
        advanced.insert("personal_vehicle".to_string(), 5000);
        advanced.insert("art_studio".to_string(), 2000);

        Self {
            grace_points: HashMap::new(),
            basic_resources: vec![
                "food".to_string(),
                "water".to_string(),
                "shelter".to_string(),
                "energy".to_string(),
                "basic_tools".to_string(),
            ],
            advanced_thresholds: advanced,
        }
    }

    /// Replicate basic resource — always succeeds
    pub fn replicate_basic(&self, resource: &str) -> bool {
        self.basic_resources.contains(&resource.to_string())
    }

    /// Add grace points for contribution (quests, building, teaching)
    pub fn add_grace(&mut self, player_id: &str, points: u64) {
        *self.grace_points.entry(player_id.to_string()).or_insert(0) += points;
    }

    /// Check/unlock advanced item
    pub fn unlock_advanced(&self, player_id: &str, item: &str) -> bool {
        if let Some(required) = self.advanced_thresholds.get(item) {
            if let Some(player_points) = self.grace_points.get(player_id) {
                return player_points >= required;
            }
        }
        false
    }

    /// Get player grace level (for council eligibility)
    pub fn grace_level(&self, player_id: &str) -> u64 {
        *self.grace_points.get(player_id).unwrap_or(&0)
    }
}

// Example usage in game loop
pub fn example_usage() {
    let mut rbe = RbeSystem::new();
    
    // Player replicates food — always free
    assert!(rbe.replicate_basic("food"));
    
    // Player completes education quest — earns grace
    rbe.add_grace("player_123", 500);
    
    // Check advanced unlock
    if rbe.unlock_advanced("player_123", "art_studio") {
        println!("Art studio unlocked — create freely!");
    }
    
    // Grace high enough for council vote
    if rbe.grace_level("player_123") > 2000 {
        println!("Eligible for APAGI Council!");
    }
}
