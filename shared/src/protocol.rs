// shared/src/protocol.rs (excerpt — combat messages added v15.4)
// ... existing code ...

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    // ... existing variants ...
    AbilityCast { ability_id: u32, target_id: Option<u64>, position: Option<Vec3Ser> },
    // ... 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    // ... existing ...
    DamageApplied { target_id: u64, amount: f32, source_id: u64, is_critical: bool },
    CombatEvent { event_type: String, data: String },
    // ...
}

// Combat data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthComponent {
    pub current: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: u32,
    pub name: String,
    pub damage: f32,
    pub range: f32,
    pub cooldown_ms: u64,
    pub mercy_cost: f32,  // For future mercy-gated abilities
}

// Note: Full file preserved; only combat enums/structs added for v15.4 scaffolding.
// All high-valence combat actions will route through mercy gates.