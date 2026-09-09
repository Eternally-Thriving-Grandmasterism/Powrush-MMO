//! U11 + B7 — Depths landing and Peace tend.
//! One disk and one node, only on Depths. Use restores its hex file. No Take. No listen.

use bevy::prelude::*;

use shared::hex_travel::{
    depths_is_market, depths_is_one_turn, depths_mesh_on_sanctuary, write_hex_named,
    HexClimateFile, PlaceId,
};

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hex_travel::HexTravelState;
use crate::human_presence::SoftPresence;
use crate::input::PlayerInput;
use crate::lived_hour_bind::LivedHourBind;

const DEPTHS_NODE_CENTER: Vec3 = Vec3::new(0.0, 0.52, 0.0);
const DEPTHS_NODE_RADIUS: f32 = 0.34;
const DEPTHS_USE_RADIUS: f32 = 2.4;

#[derive(Component)]
struct DepthsLanding;

#[derive(Component)]
struct DepthsPeaceNode;

#[derive(Resource, Debug, Default)]
pub struct DepthsPeaceTend {
    pub near: bool,
    pub tends: u32,
    pub last_line: String,
}

pub struct DepthsLandingPlugin;

impl Plugin for DepthsLandingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DepthsPeaceTend>()
            // Claim Use before the global harvest and practice systems read it.
            .add_systems(PreUpdate, mark_depths_node_near)
            .add_systems(Update, (sync_depths_landing, use_depths_peace_node).chain());
    }
}

fn sync_depths_landing(
    mut commands: Commands,
    travel: Res<HexTravelState>,
    mut tend: ResMut<DepthsPeaceTend>,
    mut presence: ResMut<SoftPresence>,
    existing: Query<Entity, Or<(With<DepthsLanding>, With<DepthsPeaceNode>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let here = travel.current == PlaceId::Depths;
    if !here {
        for entity in &existing {
            commands.entity(entity).despawn_recursive();
        }
        tend.near = false;
        tend.tends = 0;
        tend.last_line.clear();
        return;
    }
    if !existing.is_empty() {
        return;
    }
    // Seat the body on the Peace disk so Use meets the node, not leftover yard harvest.
    presence.position = Vec3::new(0.0, 0.9, 1.1);
    presence.velocity = Vec3::ZERO;
    debug_assert!(depths_is_one_turn());
    debug_assert!(!depths_is_market());
    debug_assert!(!depths_mesh_on_sanctuary());
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(1.6, 0.05)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.12, 0.16, 0.22),
                emissive: LinearRgba::new(0.01, 0.02, 0.03, 1.0),
                perceptual_roughness: 0.9,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.02, 0.0)),
            ..default()
        },
        DepthsLanding,
        Name::new("DepthsLanding"),
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(DEPTHS_NODE_RADIUS)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.34, 0.70, 0.72),
                emissive: LinearRgba::new(0.05, 0.18, 0.20, 1.0),
                perceptual_roughness: 0.48,
                ..default()
            }),
            transform: Transform::from_translation(DEPTHS_NODE_CENTER),
            ..default()
        },
        DepthsPeaceNode,
        Name::new("DepthsPeaceNode"),
    ));
}

fn mark_depths_node_near(
    travel: Res<HexTravelState>,
    presence: Res<SoftPresence>,
    mut tend: ResMut<DepthsPeaceTend>,
    epiphany: Option<ResMut<FirstHarvestEpiphany>>,
) {
    let on_depths = travel.current == PlaceId::Depths;
    let body = Vec2::new(presence.position.x, presence.position.z);
    let center = Vec2::new(DEPTHS_NODE_CENTER.x, DEPTHS_NODE_CENTER.z);
    let near = on_depths && body.distance(center) <= DEPTHS_USE_RADIUS;
    tend.near = near;
    if let Some(mut epiphany) = epiphany {
        // Depths is a Peace landing: Use is never a Take while standing on this hex.
        epiphany.depths_near = on_depths;
    }
}

fn use_depths_peace_node(
    input: Res<PlayerInput>,
    mut tend: ResMut<DepthsPeaceTend>,
    mut bind: Option<ResMut<LivedHourBind>>,
) {
    if !tend.near || !input.interact {
        return;
    }
    let Some(bind) = bind.as_deref_mut() else {
        return;
    };
    match restore_depths_hex(bind) {
        Ok(true) => {
            tend.tends = tend.tends.saturating_add(1);
            tend.last_line = "Depths Peace · restored".into();
            info!(target: "powrush::depths", "{}", tend.last_line);
        }
        Ok(false) => {}
        Err(error) => {
            warn!(target: "powrush::depths", %error, "Depths restore could not write");
        }
    }
}

/// Restore only the loaded Depths climate and immediately write
/// `powrush_hex_depths.json`. The room-tend path moves no tons or satchel stock.
pub fn restore_depths_hex(bind: &mut LivedHourBind) -> std::io::Result<bool> {
    if bind.climate.hex_id != PlaceId::Depths.as_str() {
        return Ok(false);
    }
    bind.room_tend();
    let file =
        HexClimateFile::from_parts(PlaceId::Depths, bind.climate.clone(), bind.standing.clone());
    write_hex_named(&file)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::hex_listen::PowrushNet;
    use shared::hex_protocol::default_client_listens;
    use shared::hex_travel::{hex_file_name, read_hex_named};

    fn depths_bind() -> LivedHourBind {
        LivedHourBind {
            hour: shared::climate_node::LivedHour::new_demo(),
            climate: shared::hex_travel::depths_stub_climate(),
            standing: shared::hex_travel::depths_stub_standing(),
            week: shared::week_audit::WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        }
    }

    #[test]
    fn depths_use_restores_its_hex_file_without_take_or_stock() {
        let dir = std::env::temp_dir().join(format!(
            "powrush-depths-tend-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &dir);

        let mut app = App::new();
        let bind = depths_bind();
        let satchel_before = bind.satchel_count();
        let tons_before = bind.climate.tons_moved;
        app.insert_resource(bind);
        app.insert_resource(PlayerInput {
            interact: true,
            ..default()
        });
        app.insert_resource(DepthsPeaceTend {
            near: true,
            ..default()
        });
        app.add_systems(Update, use_depths_peace_node);

        app.update();

        let bind = app.world().resource::<LivedHourBind>();
        let written = read_hex_named(PlaceId::Depths).expect("Depths hex file");
        assert_eq!(hex_file_name(PlaceId::Depths), "powrush_hex_depths.json");
        assert_eq!(written.hex_id, PlaceId::Depths.as_str());
        assert_eq!(written.climate.hex_id, PlaceId::Depths.as_str());
        assert_eq!(written.climate.restored_count, 1);
        assert_eq!(written.climate.tons_moved, tons_before, "not a haul");
        assert_eq!(bind.satchel_count(), satchel_before, "not a Take");
        assert_eq!(app.world().resource::<DepthsPeaceTend>().tends, 1);
        assert!(!written.standing.declared_lethal, "Depths stays Peace");
        assert!(!depths_is_market());
        assert!(!PowrushNet::Off.title_online_enabled());
        assert!(!default_client_listens());

        std::env::remove_var(shared::user_persist::USER_DIR_OVERRIDE_ENV);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn depths_restore_refuses_another_loaded_hex() {
        let mut bind = depths_bind();
        bind.climate = shared::hex_travel::sanctuary_fresh_climate();
        bind.standing = shared::hex_travel::sanctuary_fresh_standing();
        let before = bind.climate.clone();

        assert!(!restore_depths_hex(&mut bind).unwrap());
        assert_eq!(bind.climate, before, "hex_id guard held");
    }

    #[test]
    fn depths_hex_blocks_take_even_when_far_from_peace_node() {
        let mut app = App::new();
        app.insert_resource(HexTravelState {
            current: PlaceId::Depths,
        });
        app.insert_resource(SoftPresence {
            position: Vec3::new(20.0, 0.9, 20.0),
            velocity: Vec3::ZERO,
            grounded: true,
        });
        app.init_resource::<DepthsPeaceTend>();
        app.init_resource::<FirstHarvestEpiphany>();
        app.add_systems(PreUpdate, mark_depths_node_near);
        app.update();
        assert!(!app.world().resource::<DepthsPeaceTend>().near);
        assert!(
            app.world()
                .resource::<FirstHarvestEpiphany>()
                .harvest_use_is_claimed(),
            "Peace hex must not harvest Take anywhere on Depths"
        );
    }

    #[test]
    fn depths_node_claims_the_same_species_agnostic_use() {
        let mut app = App::new();
        app.insert_resource(HexTravelState {
            current: PlaceId::Depths,
        });
        app.insert_resource(SoftPresence {
            position: Vec3::new(0.0, 0.9, 0.0),
            velocity: Vec3::ZERO,
            grounded: true,
        });
        app.init_resource::<DepthsPeaceTend>();
        app.init_resource::<FirstHarvestEpiphany>();
        app.add_systems(PreUpdate, mark_depths_node_near);

        app.update();

        assert!(app.world().resource::<DepthsPeaceTend>().near);
        assert!(
            app.world()
                .resource::<FirstHarvestEpiphany>()
                .harvest_use_is_claimed(),
            "Depths Peace owns the ordinary Use edge before harvest"
        );
    }
}
