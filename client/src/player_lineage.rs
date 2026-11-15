/*!
 * Player Lineage — v23.0.0
 *
 * Human, Cydruid, Quellorian, Draek, Ambrosian.
 * C cycles. Felt in tend / take / freshness / body tint.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::harvest_feel::SoftRbePool;
use crate::hex_travel::PlacesPlate;
use crate::hud_edit_mode::HudEditMode;
use crate::human_presence::HumanPresence;
use crate::living_freshness::LivingFreshness;
use crate::local_settings::LocalSettingsState;
use crate::title_screen::{HouseLabel, LaunchDoor, PersonaCreatorState};
use crate::world_answer::{AnswerKind, WorldAnswer};
use shared::local_settings::{LocalSettings, PeaceKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lineage {
    Human,
    Cydruid,
    Quellorian,
    Draek,
    Ambrosian,
}

impl Lineage {
    pub fn next(self) -> Self {
        match self {
            Self::Human => Self::Cydruid,
            Self::Cydruid => Self::Quellorian,
            Self::Quellorian => Self::Draek,
            Self::Draek => Self::Ambrosian,
            Self::Ambrosian => Self::Human,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Human => "Human",
            Self::Cydruid => "Cydruid",
            Self::Quellorian => "Quellorian",
            Self::Draek => "Draek",
            Self::Ambrosian => "Ambrosian",
        }
    }

    pub fn tint(self) -> Color {
        match self {
            Self::Human => Color::srgb(0.20, 0.26, 0.24),
            Self::Cydruid => Color::srgb(0.16, 0.42, 0.22),
            Self::Quellorian => Color::srgb(0.55, 0.78, 0.92),
            Self::Draek => Color::srgb(0.42, 0.16, 0.28),
            Self::Ambrosian => Color::srgb(0.78, 0.72, 0.88),
        }
    }
}

#[derive(Resource, Debug)]
pub struct PlayerLineage {
    pub current: Lineage,
}

impl Default for PlayerLineage {
    fn default() -> Self {
        Self {
            current: Lineage::Human,
        }
    }
}

pub struct PlayerLineagePlugin;

impl Plugin for PlayerLineagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerLineage>()
            .add_systems(Update, (cycle_lineage, paint_body, lineage_care));
    }
}

fn peace_key_c_is_bound(s: &LocalSettings) -> bool {
    [s.key_move_up,s.key_move_down,s.key_move_left,s.key_move_right,s.key_jump,s.key_sprint,s.key_use,s.key_satchel,s.key_hide,s.key_allocate].into_iter().any(|k| k==PeaceKey::C)
}

fn lineage_c_is_free(settings_open: bool, text_entry: bool, panel_open: bool, c_peace_bound: bool) -> bool {
    !settings_open && !text_entry && !panel_open && !c_peace_bound
}

fn cycle_lineage(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut lineage: ResMut<PlayerLineage>,
    mut inv_line: Option<ResMut<crate::human_inventory::HumanInventory>>,
    time: Res<Time>,
    house: Option<Res<HouseLabel>>, door: Option<Res<LaunchDoor>>, persona: Option<Res<PersonaCreatorState>>,
    places: Option<Res<PlacesPlate>>, hud_edit: Option<Res<HudEditMode>>, settings: Option<Res<LocalSettingsState>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyC) || !lineage_c_is_free(
        house.as_deref().is_some_and(|h| h.settings_open),
        door.as_deref().is_some_and(|d| *d == LaunchDoor::NameHouse) || persona.as_deref().is_some_and(|p| p.open),
        places.as_deref().is_some_and(|p| p.open) || hud_edit.as_deref().is_some_and(|h| h.active),
        settings.as_deref().is_some_and(|s| peace_key_c_is_bound(&s.inner)),
    ) { return; }
    lineage.current = lineage.current.next();
    if let Some(mut inv) = inv_line {
        inv.pickup_until = time.elapsed_secs_f64() + 2.2;
        inv.pickup_line = format!("lineage · {}", lineage.current.name());
    }
    info!(target: "powrush::lineage", race = lineage.current.name(), "C cycled");
}

fn paint_body(
    lineage: Res<PlayerLineage>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<&MeshMaterial3d<StandardMaterial>, With<HumanPresence>>,
    roots: Query<Entity, (With<HumanPresence>, Without<Mesh3d>)>,
    children: Query<&Children>,
    scene_mats: Query<&MeshMaterial3d<StandardMaterial>>,
) {
    let tint = lineage.current.tint();
    let lineage_changed = lineage.is_changed();
    // Primitive lane: one shared root handle paints the whole stacked person.
    if lineage_changed {
        for handle in &q {
            if let Some(mut mat) = materials.get_mut(handle) {
                mat.base_color = tint;
            }
        }
    }
    // Authored lane: glTF body materials live on HumanPresence descendants and
    // arrive asynchronously after spawn, so the first tint also lands when a
    // scene material appears (base_color still not the lineage tint). The
    // primitive root carries Mesh3d, so its gold accent child is never walked.
    for root in &roots {
        for desc in children.iter_descendants(root) {
            let Ok(handle) = scene_mats.get(desc) else {
                continue;
            };
            let Some(mut mat) = materials.get_mut(handle) else {
                continue;
            };
            if lineage_changed || mat.base_color != tint {
                mat.base_color = tint;
            }
        }
    }
}

fn lineage_care(
    lineage: Res<PlayerLineage>,
    answer: Res<WorldAnswer>,
    mut pool: ResMut<SoftRbePool>,
    mut fresh: Option<ResMut<LivingFreshness>>,
) {
    if !answer.is_changed() || answer.kind == AnswerKind::Idle {
        return;
    }
    match (lineage.current, answer.kind) {
        (Lineage::Cydruid, AnswerKind::Tend) => {
            pool.harmony += 0.12;
            pool.joy += 0.06;
        }
        (Lineage::Draek, AnswerKind::Take) => {
            pool.vitality += 0.18;
        }
        (Lineage::Draek, AnswerKind::Tend) => {
            pool.harmony += 0.16;
            pool.joy += 0.10;
        }
        (Lineage::Quellorian, AnswerKind::Tend | AnswerKind::Flow) => {
            pool.harmony += 0.08;
        }
        (Lineage::Ambrosian, _) => {
            if let Some(ref mut f) = fresh {
                f.age = (f.age * 0.72).max(0.0);
            }
        }
        (Lineage::Human, _) => {
            pool.joy += 0.03;
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::local_settings::LocalSettings;

    #[test]
    fn default_c_free_cycles_all_five_and_wraps() {
        assert!(lineage_c_is_free(false, false, false, false));
        let mut l = Lineage::Human;
        for _ in 0..5 {
            l = l.next();
        }
        assert_eq!(l, Lineage::Human);
        assert!(!peace_key_c_is_bound(&LocalSettings::peace_defaults()));
    }

    #[test]
    fn ui_gates_block_c() {
        assert!(!lineage_c_is_free(true, false, false, false));
        assert!(!lineage_c_is_free(false, true, false, false));
        assert!(!lineage_c_is_free(false, false, true, false));
    }

    #[test]
    fn peace_c_rebind_blocks_until_defaults() {
        let mut s = LocalSettings::peace_defaults();
        s.key_move_right = PeaceKey::C;
        assert!(!lineage_c_is_free(false, false, false, peace_key_c_is_bound(&s)));
        s = LocalSettings::peace_defaults();
        s.key_use = PeaceKey::C;
        assert!(!lineage_c_is_free(false, false, false, peace_key_c_is_bound(&s)));
        s = LocalSettings::peace_defaults();
        assert!(lineage_c_is_free(false, false, false, peace_key_c_is_bound(&s)));
    }
}
