// game/hit_markers.rs
// Powrush-MMO — Client-Side Hit Markers with 3D World Projection
// AG-SML v1.0 License

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitMarkerData {
    pub show: bool,
    pub damage: f32,
    pub is_critical: bool,
    pub hit_location: Option<(f32, f32, f32)>,
    pub target_id: u64,
}

#[derive(Debug, Clone)]
pub struct HitMarker {
    pub damage: f32,
    pub is_critical: bool,
    pub world_position: Option<(f32, f32, f32)>,
    pub screen_position: Option<(f32, f32)>,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub alpha: f32,
    pub offset_y: f32,
}

impl HitMarker {
    pub fn new(data: &HitMarkerData, world_position: Option<(f32, f32, f32)>) -> Self {
        Self {
            damage: data.damage,
            is_critical: data.is_critical,
            world_position,
            screen_position: None,
            lifetime: 1.2,
            max_lifetime: 1.2,
            alpha: 1.0,
            offset_y: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.lifetime -= delta_time;
        self.alpha = (self.lifetime / self.max_lifetime).clamp(0.0, 1.0);
        self.offset_y += 35.0 * delta_time;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub view_matrix: [[f32; 4]; 4],
    pub projection_matrix: [[f32; 4]; 4],
    pub screen_width: f32,
    pub screen_height: f32,
}

impl Camera {
    pub fn world_to_screen(&self, world_pos: (f32, f32, f32)) -> Option<(f32, f32)> {
        let world = [world_pos.0, world_pos.1, world_pos.2, 1.0];

        let view_x = self.view_matrix[0][0] * world[0] + self.view_matrix[0][1] * world[1] +
                     self.view_matrix[0][2] * world[2] + self.view_matrix[0][3] * world[3];
        let view_y = self.view_matrix[1][0] * world[0] + self.view_matrix[1][1] * world[1] +
                     self.view_matrix[1][2] * world[2] + self.view_matrix[1][3] * world[3];
        let view_z = self.view_matrix[2][0] * world[0] + self.view_matrix[2][1] * world[1] +
                     self.view_matrix[2][2] * world[2] + self.view_matrix[2][3] * world[3];
        let view_w = self.view_matrix[3][0] * world[0] + self.view_matrix[3][1] * world[1] +
                     self.view_matrix[3][2] * world[2] + self.view_matrix[3][3] * world[3];

        let clip_x = self.projection_matrix[0][0] * view_x + self.projection_matrix[0][1] * view_y +
                     self.projection_matrix[0][2] * view_z + self.projection_matrix[0][3] * view_w;
        let clip_y = self.projection_matrix[1][0] * view_x + self.projection_matrix[1][1] * view_y +
                     self.projection_matrix[1][2] * view_z + self.projection_matrix[1][3] * view_w;
        let clip_w = self.projection_matrix[3][0] * view_x + self.projection_matrix[3][1] * view_y +
                     self.projection_matrix[3][2] * view_z + self.projection_matrix[3][3] * view_w;

        if clip_w <= 0.0 {
            return None;
        }

        let ndc_x = clip_x / clip_w;
        let ndc_y = clip_y / clip_w;

        let screen_x = (ndc_x + 1.0) * 0.5 * self.screen_width;
        let screen_y = (1.0 - ndc_y) * 0.5 * self.screen_height;

        Some((screen_x, screen_y))
    }
}

pub struct HitMarkerManager {
    markers: VecDeque<HitMarker>,
    max_markers: usize,
}

impl HitMarkerManager {
    pub fn new(max_markers: usize) -> Self {
        Self {
            markers: VecDeque::with_capacity(max_markers),
            max_markers,
        }
    }

    pub fn add_hit_marker(&mut self, data: &HitMarkerData, world_position: Option<(f32, f32, f32)>) {
        if !data.show {
            return;
        }

        let marker = HitMarker::new(data, world_position);
        if self.markers.len() >= self.max_markers {
            self.markers.pop_front();
        }
        self.markers.push_back(marker);
    }

    pub fn update(&mut self, delta_time: f32, camera: Option<&Camera>) {
        for marker in &mut self.markers {
            marker.update(delta_time);

            if let (Some(cam), Some(world_pos)) = (camera, marker.world_position) {
                marker.screen_position = cam.world_to_screen(world_pos);
            }
        }

        self.markers.retain(|m| m.is_alive());
    }

    pub fn get_markers(&self) -> &VecDeque<HitMarker> {
        &self.markers
    }

    pub fn clear(&mut self) {
        self.markers.clear();
    }
}