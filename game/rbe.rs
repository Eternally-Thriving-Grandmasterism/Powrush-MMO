use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

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

/// ========================================================================
/// ServerInventoryComponent v16.2 — Professional Authoritative RBE Inventory
/// Appended respectfully to game/rbe.rs as part of eternal iterative PR cycle
/// Deriving from:
/// - Ra-Thor sovereign_core persistence patterns + eternal mercy flow
/// - PATSAGi Councils 7 Living Mercy Gates (abundance for all, no tyranny/hoarding)
/// - Real production MMO authoritative server + client prediction sync (InventoryUpdate messages)
/// - InterestManager + lag compensation integration ready (future)
/// - Lean separation: game/ owns RBE mechanics + persistence; server/ owns tick wiring
/// 
/// All mutations mercy-gated at call site (server harvest/trade handlers)
/// Persistence: crash-safe per-player .inv files in data/inventories/
/// Extendable to full Ra-Thor lattice sync or DB without breaking changes
/// ========================================================================

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ServerInventoryComponent {
    pub resources: HashMap<String, f32>,
    pub abundance_score: f32,
    pub last_updated_ms: u64,
}

impl ServerInventoryComponent {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            abundance_score: 0.0,
            last_updated_ms: 0,
        }
    }

    /// Add harvested or traded resources — call after PATSAGi validate_harvest / validate_trade
    pub fn add_resource(&mut self, resource_type: &str, amount: f32, current_time_ms: u64) {
        *self.resources.entry(resource_type.to_string()).or_insert(0.0) += amount;
        self.abundance_score = (self.abundance_score + amount * 0.01).min(100000.0); // cap for balance
        self.last_updated_ms = current_time_ms;
    }

    /// Remove for crafting/trade — returns success
    pub fn remove_resource(&mut self, resource_type: &str, amount: f32) -> bool {
        if let Some(current) = self.resources.get_mut(resource_type) {
            if *current >= amount {
                *current -= amount;
                if *current < 0.0001 { *current = 0.0; }
                return true;
            }
        }
        false
    }

    /// PATSAGi Council validation for any inventory mutation (sustainability + universal thriving)
    pub fn validate_patsagi_action(&self, action: &str, amount: f32) -> Result<(bool, String, f32), String> {
        if amount <= 0.0 {
            return Ok((false, "Mercy violation: non-positive amounts break eternal flow.".to_string(), 0.0));
        }
        // Simple anti-hoarding for abundance for all
        if self.abundance_score > 50000.0 && action.contains("hoard") {
            return Ok((false, "PATSAGi Council: High abundance detected. Redirect to sharing/trading for universal thriving. (Radical Love Gate)".to_string(), -0.5));
        }
        let valence = 0.8; // positive for sustainable
        Ok((true, format!("PATSAGi Council approves {} of {:.1} — abundance flows to all sentience. (Abundance Gate)", action, amount), valence))
    }

    /// Sovereign persistence hook — file-based, bincode encoded, per-player
    /// Call after any mutation in authoritative tick or on disconnect for safety
    pub fn persist_to_file(&self, player_id: u64) -> Result<(), String> {
        let dir = Path::new("data/inventories");
        if !dir.exists() {
            fs::create_dir_all(dir).map_err(|e| format!("Failed to create inventories dir: {}", e))?;
        }
        let path = dir.join(format!("player_{}.inv", player_id));
        let encoded = bincode::serialize(self)
            .map_err(|e| format!("Bincode serialize failed: {}", e))?;
        fs::write(&path, encoded)
            .map_err(|e| format!("Failed to persist inventory for player {}: {}", player_id, e))?;
        Ok(())
    }

    /// Load hook — called on player connect for session resume (sovereign)
    pub fn load_from_file(player_id: u64) -> Result<Self, String> {
        let path = Path::new("data/inventories").join(format!("player_{}.inv", player_id));
        if !path.exists() {
            return Ok(Self::new());
        }
        let data = fs::read(&path)
            .map_err(|e| format!("Failed to read inventory file: {}", e))?;
        let inv: Self = bincode::deserialize(&data)
            .map_err(|e| format!("Bincode deserialize failed: {}", e))?;
        Ok(inv)
    }
}

// Example integration note (for server/src/main.rs or server_tick_loop):
// let inv = player_inventories.entry(player_id).or_insert_with(ServerInventoryComponent::new);
// // after harvest validation
// if let Ok((approved, reason, _)) = inv.validate_patsagi_action("harvest", amount) {
//     if approved { inv.add_resource(&node.resource_type, amount, now); let _ = inv.persist_to_file(player_id); }
// }
