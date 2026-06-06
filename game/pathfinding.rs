// game/pathfinding.rs
// Powrush-MMO — A* Pathfinding with Obstacle Support
// AG-SML v1.0 License

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct PathNode {
    pub position: GridPos,
    pub g_cost: f32,
    pub h_cost: f32,
    pub parent: Option<GridPos>,
}

impl PathNode {
    fn f_cost(&self) -> f32 {
        self.g_cost + self.h_cost
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost().partial_cmp(&self.f_cost()).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PathfindingGrid {
    pub width: i32,
    pub height: i32,
    pub blocked: HashSet<GridPos>,
}

impl PathfindingGrid {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            blocked: HashSet::new(),
        }
    }

    pub fn is_walkable(&self, pos: GridPos) -> bool {
        pos.x >= 0 && pos.x < self.width &&
        pos.y >= 0 && pos.y < self.height &&
        !self.blocked.contains(&pos)
    }

    pub fn add_obstacle(&mut self, pos: GridPos) {
        self.blocked.insert(pos);
    }

    pub fn remove_obstacle(&mut self, pos: GridPos) {
        self.blocked.remove(&pos);
    }

    pub fn find_path(&self, start: GridPos, goal: GridPos) -> Option<Vec<GridPos>> {
        if !self.is_walkable(goal) {
            return None;
        }

        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
        let mut g_score: HashMap<GridPos, f32> = HashMap::new();

        g_score.insert(start, 0.0);

        open_set.push(PathNode {
            position: start,
            g_cost: 0.0,
            h_cost: heuristic(start, goal),
            parent: None,
        });

        while let Some(current) = open_set.pop() {
            if current.position == goal {
                return Some(reconstruct_path(&came_from, goal));
            }

            for neighbor in get_neighbors(current.position) {
                if !self.is_walkable(neighbor) {
                    continue;
                }

                let tentative_g = g_score.get(&current.position).unwrap_or(&f32::MAX) + 1.0;

                if tentative_g < *g_score.get(&neighbor).unwrap_or(&f32::MAX) {
                    came_from.insert(neighbor, current.position);
                    g_score.insert(neighbor, tentative_g);

                    open_set.push(PathNode {
                        position: neighbor,
                        g_cost: tentative_g,
                        h_cost: heuristic(neighbor, goal),
                        parent: Some(current.position),
                    });
                }
            }
        }

        None
    }
}

fn heuristic(a: GridPos, b: GridPos) -> f32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as f32
}

fn get_neighbors(pos: GridPos) -> Vec<GridPos> {
    vec![
        GridPos { x: pos.x + 1, y: pos.y },
        GridPos { x: pos.x - 1, y: pos.y },
        GridPos { x: pos.x, y: pos.y + 1 },
        GridPos { x: pos.x, y: pos.y - 1 },
    ]
}

fn reconstruct_path(came_from: &HashMap<GridPos, GridPos>, mut current: GridPos) -> Vec<GridPos> {
    let mut path = vec![current];
    while let Some(&prev) = came_from.get(&current) {
        path.push(prev);
        current = prev;
    }
    path.reverse();
    path
}