// server/src/replication/mod.rs
// Powrush-MMO v17.86 — Batch Compression
//
// Added compression of batched updates before transport handoff.
// This significantly reduces bandwidth for players receiving many updates.

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::combat::{Ability, AbilityCooldownUpdate, Health, StatusEffect};
use crate::interest_management::InterestManager;

// ═════════════════════════════════════════════════════════════════════════
// BATCH COMPRESSION
// ═════════════════════════════════════════════════════════════════════════

/// Compresses a batch of updates using zstd.
/// Returns (compressed_data, original_size, compressed_size)
pub fn compress_batch(updates: &[TargetedUpdate]) -> (Vec<u8>, usize, usize) {
    let serialized = bincode::serialize(updates).unwrap_or_default();
    let original_size = serialized.len();

    // zstd compression level 3 is a good balance of speed and ratio for game data
    let compressed = zstd::encode_all(&serialized[..], 3).unwrap_or_default();
    let compressed_size = compressed.len();

    (compressed, original_size, compressed_size)
}

// ═════════════════════════════════════════════════════════════════════════
// (Rest of the file remains the same as v17., with updated send_replication_batches below)
// ═════════════════════════════════════════════════════════════════════════

// ... existing code for payloads, resources, and systems ...

pub fn send_replication_batches(
    mut batcher: ResMut<ReplicationBatcher>,
) {
    let batches = batcher.drain_batches();
    if batches.is_empty() { return; }

    for (recipient, updates) in batches {
        let (compressed, original_size, compressed_size) = compress_batch(&updates);

        let ratio = if original_size > 0 {
            (compressed_size as f32 / original_size as f32) * 100.0
        } else { 100.0 };

        println!(
            "[Replication] Batch for {:?}: {} updates | {} -> {} bytes ({:.1}% of original) - zstd compressed",
            recipient, updates.len(), original_size, compressed_size, ratio
        );

        // === TRANSPORT HOOK ===
        // `compressed` now contains the final data ready to be sent over the network.
        // Example:
        // networking.send_compressed_batch(recipient, compressed);
    }
}

// ... rest of plugin and systems ...