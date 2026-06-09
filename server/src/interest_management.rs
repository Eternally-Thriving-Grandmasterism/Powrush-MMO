// server/src/interest_management.rs
// ... (existing code preserved)

impl InterestManager {
    // ... existing methods ...

    /// Returns players who are interested in the given entity.
    /// This is the key method for interest-based replication (reverse query).
    ///
    /// In a full implementation this would query the spatial layer.
    /// For now it provides a working foundation that can be optimized.
    pub fn get_interested_players(&self, entity_id: u64) -> Vec<u64> {
        // Placeholder implementation
        // Real version should query spatial.get_players_interested_in(entity_id)
        // or maintain a reverse interest map.
        //
        // For development we return an empty list or self if entity is a player.
        // This will be replaced with real spatial queries.
        vec![]
    }

    // ... rest of impl ...
}