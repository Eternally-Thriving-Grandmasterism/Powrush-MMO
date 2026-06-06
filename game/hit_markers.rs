// game/hit_markers.rs
// Powrush-MMO — Hit Markers + Sound Effects
// AG-SML v1.0 License

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HitSound {
    Normal,
    Critical,
    Headshot,
    WeakHit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitMarkerData {
    pub show: bool,
    pub damage: f32,
    pub is_critical: bool,
    pub hit_location: Option<(f32, f32, f32)>,
    pub target_id: u64,
    pub play_sound: bool,
    pub sound_type: Option<HitSound>,
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

pub struct HitMarkerManager {
    markers: VecDeque<HitMarker>,
    pending_sounds: VecDeque<HitSound>,
    max_markers: usize,
}

impl HitMarkerManager {
    pub fn new(max_markers: usize) -> Self {
        Self {
            markers: VecDeque::with_capacity(max_markers),
            pending_sounds: VecDeque::new(),
            max_markers,
        }
    }

    pub fn add_hit_marker(&mut self, data: &HitMarkerData, world_position: Option<(f32, f32, f32)>) {
        if data.show {
            let marker = HitMarker::new(data, world_position);
            if self.markers.len() >= self.max_markers {
                self.markers.pop_front();
            }
            self.markers.push_back(marker);
        }

        if data.play_sound {
            if let Some(sound_type) = data.sound_type {
                self.pending_sounds.push_back(sound_type);
            } else {
                self.pending_sounds.push_back(HitSound::Normal);
            }
        }
    }

    pub fn update(&mut self, delta_time: f32, camera: Option<&crate::game::hit_markers::Camera>) {
        for marker in &mut self.markers {
            marker.update(delta_time);

            if let (Some(cam), Some(world_pos)) = (camera, marker.world_position) {
                marker.screen_position = cam.world_to_screen(world_pos);
            }
        }

        self.markers.retain(|m| m.is_alive());
    }

    pub fn take_next_sound(&mut self) -> Option<HitSound> {
        self.pending_sounds.pop_front()
    }

    pub fn get_markers(&self) -> &VecDeque<HitMarker> {
        &self.markers
    }

    pub fn clear(&mut self) {
        self.markers.clear();
        self.pending_sounds.clear();
    }
}