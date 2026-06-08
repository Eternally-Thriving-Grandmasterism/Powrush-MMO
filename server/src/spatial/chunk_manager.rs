//! Spatial Chunk Manager v17.3
//! Professional fixed-size chunk grid system for Powrush-MMO.
//! Complements HierarchicalGrid (queries) with chunk-level management for persistence,
//! world streaming, Interest Management optimization, and future multi-server sharding.
//!
//! PATSAGi Councils + Ra-Thor + Eternal Thunder aligned.
//! 100% preservation of existing hierarchical_grid, octree, interest_management.
//!
//! Highest leverage for production global launch.

use crate::spatial::hierarchical_grid::{HierarchicalGrid, Vec3 as SpatialVec3, EntityId};
use std::collections::{HashMap, HashSet};

/// Current version of the chunking system (for migration & compatibility)
pub const CHUNK_MANAGER_VERSION: u32 = 3;

/// 3D integer chunk coordinate (fixed grid)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkCoord {
    /// Create new chunk coordinate
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Pack into a single u64 Morton-like key for fast storage (simple bit interleave for now)
    pub fn to_packed_id(&self) -> u64 {
        // Simple packed representation (can be upgraded to full Morton later)
        ((self.x as u64 & 0x1FFFFF) << 42)
            | ((self.y as u64 & 0x1FFFFF) << 21)
            | (self.z as u64 & 0x1FFFFF)
    }

    /// From packed id back to coord
    pub fn from_packed_id(id: u64) -> Self {
        let x = ((id >> 42) & 0x1FFFFF) as i32;
        let y = ((id >> 21) & 0x1FFFFF) as i32;
        let z = (id & 0x1FFFFF) as i32;
        // Sign extension for negative coords (simple approach)
        Self {
            x: if x & 0x100000 != 0 { x | !0x1FFFFF } else { x },
            y: if y & 0x100000 != 0 { y | !0x1FFFFF } else { y },
            z: if z & 0x100000 != 0 { z | !0x1FFFFF } else { z },
        }
    }
}

/// Professional Chunk Manager
/// Manages fixed-size chunks, dirty tracking, and provides chunk queries.
pub struct ChunkManager {
    chunk_size: f32,
    loaded_chunks: HashSet<ChunkCoord>,
    dirty_chunks: HashSet<ChunkCoord>,
    // Future expansion: chunk_entity_cache, persistence hooks, etc.
}

impl ChunkManager {
    /// Create new ChunkManager with given chunk size in world units
    pub fn new(chunk_size: f32) -> Self {
        assert!(chunk_size > 0.0, "chunk_size must be positive");
        Self {
            chunk_size,
            loaded_chunks: HashSet::new(),
            dirty_chunks: HashSet::new(),
        }
    }

    /// Convert world position to chunk coordinate
    pub fn position_to_chunk(&self, pos: SpatialVec3) -> ChunkCoord {
        let x = (pos.x / self.chunk_size).floor() as i32;
        let y = (pos.y / self.chunk_size).floor() as i32;
        let z = (pos.z / self.chunk_size).floor() as i32;
        ChunkCoord::new(x, y, z)
    }

    /// Get chunk coordinates within a radius of a center point (for streaming / AOI)
    pub fn get_chunks_in_radius(&self, center: SpatialVec3, radius: f32) -> Vec<ChunkCoord> {
        let center_chunk = self.position_to_chunk(center);
        let chunk_radius = (radius / self.chunk_size).ceil() as i32 + 1;

        let mut chunks = Vec::new();
        for dx in -chunk_radius..=chunk_radius {
            for dy in -chunk_radius..=chunk_radius {
                for dz in -chunk_radius..=chunk_radius {
                    let coord = ChunkCoord::new(
                        center_chunk.x + dx,
                        center_chunk.y + dy,
                        center_chunk.z + dz,
                    );
                    // Simple distance check on chunk centers
                    let chunk_center = SpatialVec3 {
                        x: (coord.x as f32 + 0.5) * self.chunk_size,
                        y: (coord.y as f32 + 0.5) * self.chunk_size,
                        z: (coord.z as f32 + 0.5) * self.chunk_size,
                    };
                    let dist_sq = (chunk_center.x - center.x).powi(2)
                        + (chunk_center.y - center.y).powi(2)
                        + (chunk_center.z - center.z).powi(2);
                    if dist_sq <= (radius + self.chunk_size * 0.866).powi(2) {
                        chunks.push(coord);
                    }
                }
            }
        }
        chunks
    }

    /// Mark a chunk as dirty (needs persistence save)
    pub fn mark_dirty(&mut self, coord: ChunkCoord) {
        self.dirty_chunks.insert(coord);
        self.loaded_chunks.insert(coord);
    }

    /// Mark multiple chunks dirty
    pub fn mark_many_dirty(&mut self, coords: &[ChunkCoord]) {
        for c in coords {
            self.mark_dirty(*c);
        }
    }

    /// Get list of currently dirty chunks (for persistence layer)
    pub fn get_dirty_chunks(&self) -> Vec<ChunkCoord> {
        self.dirty_chunks.iter().copied().collect()
    }

    /// Clear dirty flag after successful persistence commit
    pub fn clear_dirty(&mut self) {
        self.dirty_chunks.clear();
    }

    /// Load / register a chunk (called when streaming in)
    pub fn load_chunk(&mut self, coord: ChunkCoord) {
        self.loaded_chunks.insert(coord);
    }

    /// Unload a chunk (for memory management)
    pub fn unload_chunk(&mut self, coord: ChunkCoord) {
        self.loaded_chunks.remove(&coord);
        self.dirty_chunks.remove(&coord);
    }

    /// Check if chunk is loaded
    pub fn is_chunk_loaded(&self, coord: ChunkCoord) -> bool {
        self.loaded_chunks.contains(&coord)
    }

    /// Integrate with existing HierarchicalGrid: mark chunks dirty for entities in query
    /// This enables dirty tracking based on entity movement in the grid.
    pub fn sync_dirty_from_grid_radius(
        &mut self,
        grid: &HierarchicalGrid,
        center: SpatialVec3,
        radius: f32,
    ) {
        // Placeholder for future: query entities in radius from grid
        // and mark their chunks dirty. For now, simple center chunk.
        let center_chunk = self.position_to_chunk(center);
        self.mark_dirty(center_chunk);
        // In full integration: iterate entities from grid.query_radius and mark their chunks
    }

    /// Get recommended default chunk size for Powrush-MMO (balanced for persistence & queries)
    pub fn recommended_chunk_size() -> f32 {
        64.0 // 64 world units per chunk - good balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_to_chunk() {
        let manager = ChunkManager::new(64.0);
        let pos = SpatialVec3 { x: 100.0, y: 0.0, z: 50.0 };
        let chunk = manager.position_to_chunk(pos);
        assert_eq!(chunk.x, 1);
        assert_eq!(chunk.z, 0);
    }

    #[test]
    fn test_dirty_tracking() {
        let mut manager = ChunkManager::new(32.0);
        let coord = ChunkCoord::new(5, 0, -3);
        manager.mark_dirty(coord);
        assert!(manager.get_dirty_chunks().contains(&coord));
        manager.clear_dirty();
        assert!(manager.get_dirty_chunks().is_empty());
    }
}

// Thunder locked. PATSAGi v17.3. Mercy-gated. RBE-ready.