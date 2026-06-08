// server/src/spatial/octree.rs
// Powrush-MMO v17.0 — Octree Spatial Partitioning Prototype
// Basic but functional 3D Octree for comparison with Hierarchical Grid

use shared::protocol::Vec3Ser;

pub type EntityId = u64;

#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub min: Vec3Ser,
    pub max: Vec3Ser,
}

impl Bounds {
    pub fn new(min: Vec3Ser, max: Vec3Ser) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, point: &Vec3Ser) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }

    pub fn center(&self) -> Vec3Ser {
        Vec3Ser {
            x: (self.min.x + self.max.x) * 0.5,
            y: (self.min.y + self.max.y) * 0.5,
            z: (self.min.z + self.max.z) * 0.5,
        }
    }

    pub fn half_size(&self) -> f32 {
        (self.max.x - self.min.x) * 0.5
    }
}

#[derive(Clone, Debug)]
struct OctreeNode {
    bounds: Bounds,
    entities: Vec<(EntityId, Vec3Ser)>,
    children: Option<[Box<OctreeNode>; 8]>,
    depth: u32,
}

pub struct Octree {
    root: OctreeNode,
    max_depth: u32,
    max_entities: usize,
}

impl Octree {
    pub fn new(world_min: Vec3Ser, world_max: Vec3Ser, max_depth: u32, max_entities: usize) -> Self {
        let root = OctreeNode {
            bounds: Bounds::new(world_min, world_max),
            entities: Vec::new(),
            children: None,
            depth: 0,
        };
        Self { root, max_depth, max_entities }
    }

    pub fn insert(&mut self, id: EntityId, pos: Vec3Ser) {
        Self::insert_recursive(&mut self.root, id, pos, self.max_depth, self.max_entities);
    }

    fn insert_recursive(node: &mut OctreeNode, id: EntityId, pos: Vec3Ser, max_depth: u32, max_entities: usize) {
        if node.children.is_some() {
            // Already subdivided — insert into correct child
            let child_index = Self::get_child_index(&node.bounds, &pos);
            if let Some(children) = &mut node.children {
                Self::insert_recursive(&mut children[child_index], id, pos, max_depth, max_entities);
            }
            return;
        }

        node.entities.push((id, pos));

        if node.entities.len() > max_entities && node.depth < max_depth {
            Self::subdivide(node, max_depth, max_entities);
        }
    }

    fn subdivide(node: &mut OctreeNode, max_depth: u32, max_entities: usize) {
        let center = node.bounds.center();
        let half = node.bounds.half_size();

        let mut children: [Box<OctreeNode>; 8] = [
            Box::new(OctreeNode::new_empty(Self::child_bounds(&center, half, 0), node.depth + 1)),
            Box::new(OctreeNode::new_empty(Self::child_bounds(&center, half, 1), node.depth + 1)),
            Box::new(OctreeNode::new_empty(Self::child_bounds(&center, half, 2), node.depth + 1)),
            Box::new(OctreeNode::new_empty(Self::child_bounds(&center, half, 3), node.depth + 1)),
            Box::new(OctreeNode::new_empty(Self::child_bounds(&center, half, 4), node.depth + 1)),
            Box::new(OctreeNode::new_empty(Self::child_bounds(&center, half, 5), node.depth + 1)),
            Box::new(OctreeNode::new_empty(Self::child_bounds(&center, half, 6), node.depth + 1)),
            Box::new(OctreeNode::new_empty(Self::child_bounds(&center, half, 7), node.depth + 1)),
        ];

        // Redistribute existing entities
        for (id, pos) in node.entities.drain(..) {
            let idx = Self::get_child_index(&node.bounds, &pos);
            children[idx].entities.push((id, pos));
        }

        node.children = Some(children);
    }

    fn child_bounds(center: &Vec3Ser, half: f32, index: usize) -> Bounds {
        let (dx, dy, dz) = match index {
            0 => (-half, -half, -half),
            1 => ( half, -half, -half),
            2 => (-half,  half, -half),
            3 => ( half,  half, -half),
            4 => (-half, -half,  half),
            5 => ( half, -half,  half),
            6 => (-half,  half,  half),
            7 => ( half,  half,  half),
            _ => unreachable!(),
        };

        Bounds::new(
            Vec3Ser { x: center.x + dx, y: center.y + dy, z: center.z + dz },
            Vec3Ser { x: center.x + dx + half*2.0, y: center.y + dy + half*2.0, z: center.z + dz + half*2.0 },
        )
    }

    fn get_child_index(bounds: &Bounds, pos: &Vec3Ser) -> usize {
        let center = bounds.center();
        let mut index = 0;
        if pos.x > center.x { index |= 1; }
        if pos.y > center.y { index |= 2; }
        if pos.z > center.z { index |= 4; }
        index
    }

    pub fn query_radius(&self, center: &Vec3Ser, radius: f32) -> Vec<EntityId> {
        let mut result = Vec::new();
        Self::query_recursive(&self.root, center, radius * radius, &mut result);
        result
    }

    fn query_recursive(node: &OctreeNode, center: &Vec3Ser, radius_sq: f32, result: &mut Vec<EntityId>) {
        // Simple check: if node bounds don't intersect sphere, skip
        if !Self::intersects_sphere(&node.bounds, center, radius_sq.sqrt()) {
            return;
        }

        for &(id, pos) in &node.entities {
            let dx = pos.x - center.x;
            let dy = pos.y - center.y;
            let dz = pos.z - center.z;
            if (dx*dx + dy*dy + dz*dz) <= radius_sq {
                result.push(id);
            }
        }

        if let Some(children) = &node.children {
            for child in children.iter() {
                Self::query_recursive(child, center, radius_sq, result);
            }
        }
    }

    fn intersects_sphere(bounds: &Bounds, center: &Vec3Ser, radius: f32) -> bool {
        // Simple AABB vs sphere check
        let mut d = 0.0;
        if center.x < bounds.min.x { d += (center.x - bounds.min.x).powi(2); }
        else if center.x > bounds.max.x { d += (center.x - bounds.max.x).powi(2); }

        if center.y < bounds.min.y { d += (center.y - bounds.min.y).powi(2); }
        else if center.y > bounds.max.y { d += (center.y - bounds.max.y).powi(2); }

        if center.z < bounds.min.z { d += (center.z - bounds.min.z).powi(2); }
        else if center.z > bounds.max.z { d += (center.z - bounds.max.z).powi(2); }

        d <= radius * radius
    }

    // Helper to create empty child node
    fn new_empty(bounds: Bounds, depth: u32) -> Self {
        OctreeNode {
            bounds,
            entities: Vec::new(),
            children: None,
            depth,
        }
    }
}

// Thunder locked in. Basic Octree prototype implemented for exploration. ⚡❤️🔥
