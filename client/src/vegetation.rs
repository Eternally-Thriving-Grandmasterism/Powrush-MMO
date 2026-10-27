//! CARD VP-VEG-1 — dense vegetation, instanced, LOD by preset.
//!
//! A Sanctuary field of tufts, shrubs, and trees between the yard and the
//! VP-SKY-2 conifer belt. Each part and LOD tier is one mesh and one
//! material. Every plant in that tier clones those handles, which is Bevy's
//! GPU instance batch. The field is not merged into one mesh (that is the
//! distant belt) and it does not add a render node.
//!
//! Density and mesh segments follow [`GraphicsPreset`]. Mobile, Low, and
//! Medium stay under [`CHEAP_INSTANCE_CAP`] and use one coarse mesh per
//! part. High and Ultra are the dense tiers: a finer mesh inside
//! [`VegPlan::near_radius`], the coarse mesh past it. Trunks use the earth
//! pass-through. Ground hues are not retuned.
//!
//! Contact: info@Rathor.ai

use std::f32::consts::TAU;

use bevy::ecs::system::SystemParam;
use bevy::mesh::{ConeAnchor, ConeMeshBuilder, CylinderAnchor, CylinderMeshBuilder};
use bevy::prelude::*;

use shared::local_settings::GraphicsPreset;

use crate::living_practice_loop::SoftPlayerRealm;
use crate::local_settings::LocalSettingsState;
use crate::painterly_mat::{
    paint_height_ramp, painterly_material, painterly_shade, remember_ramp, PainterlyMat,
    PainterlyRampMeshes, PainterlyRole,
};

/// Plants stay outside this radius so the well, the path stones, and the
/// five yard trees keep the middle of the plate.
pub const YARD_CLEAR: f32 = 8.0;
/// Boot camera (`main` places it at xz `(0, 14)`). The field leaves a pad
/// around that pose so the first frame is not inside a trunk.
pub const CAMERA_XZ: Vec2 = Vec2::new(0.0, 14.0);
pub const CAMERA_CLEAR: f32 = 3.5;
/// Sit just above the yard plate (`y = 0`) and the valley floor (`y = -0.02`).
/// On a lower VP-TERRAIN-1 shelf, [`seated_y`] adds the shelf drop.
const GROUND_LIFT: f32 = 0.03;
/// Trunk prototype height, metres. The canopy base overlaps the top.
pub const TRUNK_HEIGHT: f32 = 1.35;
const CANOPY_SIT: f32 = 0.78;
const VEG_SEED: u32 = 0x56E6_1D01;

/// Mobile, Low, and Medium instance counts stay at or under this.
pub const CHEAP_INSTANCE_CAP: u32 = 96;
/// High and Ultra plant counts are at least this multiple of Medium.
pub const DENSE_MIN_FACTOR: u32 = 4;

/// Tuft / shrub / canopy authored sRGB. [`PainterlyRole::Foliage`] walks
/// these toward valley green. They are not ground hues.
const TUFT_SRGB: [f32; 3] = [0.20, 0.42, 0.18];
const SHRUB_SRGB: [f32; 3] = [0.14, 0.36, 0.16];
const CANOPY_SRGB: [f32; 3] = [0.12, 0.34, 0.15];
/// Bark. Earth pass-through, so the hue is the one that ships.
const TRUNK_SRGB: [f32; 3] = [0.29, 0.20, 0.13];

/// What a plant is before it becomes instance parts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VegKind {
    Tuft,
    Shrub,
    Tree,
}

/// One shared mesh. A tree is a trunk plus a canopy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VegPart {
    Tuft = 0,
    Shrub = 1,
    Trunk = 2,
    Canopy = 3,
}

/// Coarse is the only tier on cheap presets. Fine is the inner ring on
/// High and Ultra.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VegLodTier {
    Coarse = 0,
    Fine = 1,
}

/// Preset → density and LOD. Cheap tiers set every `fine_*` count to 0.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VegPlan {
    pub tufts: u32,
    pub shrubs: u32,
    pub trees: u32,
    pub fine_tufts: u32,
    pub fine_shrubs: u32,
    pub fine_trees: u32,
    /// Radial segments of the far / cheap prototype.
    pub coarse_segments: u32,
    /// Radial segments of the inner prototype. Equals coarse when `fine_*` is 0.
    pub fine_segments: u32,
    /// Inner ring uses the fine mesh. `0` on cheap tiers.
    pub near_radius: f32,
    /// Outer edge of the field, metres from the yard origin.
    pub reach: f32,
    /// High and Ultra only.
    pub dense: bool,
}

impl VegPlan {
    pub const fn plants(self) -> u32 {
        self.tufts + self.shrubs + self.trees
    }

    /// Tuft, shrub, and tree canopy are one instance each. A trunk is the second.
    pub const fn instances(self) -> u32 {
        self.tufts + self.shrubs + self.trees * 2
    }

    pub const fn fine_plants(self) -> u32 {
        self.fine_tufts + self.fine_shrubs + self.fine_trees
    }
}

/// One planted tuft, shrub, or tree.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VegPlant {
    pub kind: VegKind,
    pub lod: VegLodTier,
    pub x: f32,
    pub z: f32,
    pub scale: f32,
    pub yaw: f32,
}

/// One GPU instance. Same [`VegPart`] and [`VegLodTier`] share one mesh.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VegInstance {
    pub part: VegPart,
    pub lod: VegLodTier,
    pub translation: Vec3,
    pub yaw: f32,
    pub scale: f32,
}

/// The batch key. One key is one mesh handle and one material handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VegBatchKey {
    pub part: VegPart,
    pub lod: VegLodTier,
}

/// Instances that clone one prototype.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VegBatch {
    pub key: VegBatchKey,
    pub indices: Vec<usize>,
}

/// Density and LOD for one graphics preset.
///
/// | Preset | Plants (tuft/shrub/tree) | Fine share | Segments coarse/fine | Reach |
/// |---|---|---|---|---|
/// | Mobile | 16/4/2 | none | 4/4 | 16 |
/// | Low | 24/8/4 | none | 4/4 | 22 |
/// | Medium | 40/12/6 | none | 5/5 | 28 |
/// | High | 280/72/28 | 80/24/8 | 4/8 | 48 |
/// | Ultra | 520/120/48 | 180/48/16 | 5/12 | 64 |
pub const fn vegetation_plan(preset: GraphicsPreset) -> VegPlan {
    match preset {
        GraphicsPreset::Mobile => VegPlan {
            tufts: 16,
            shrubs: 4,
            trees: 2,
            fine_tufts: 0,
            fine_shrubs: 0,
            fine_trees: 0,
            coarse_segments: 4,
            fine_segments: 4,
            near_radius: 0.0,
            reach: 16.0,
            dense: false,
        },
        GraphicsPreset::Low => VegPlan {
            tufts: 24,
            shrubs: 8,
            trees: 4,
            fine_tufts: 0,
            fine_shrubs: 0,
            fine_trees: 0,
            coarse_segments: 4,
            fine_segments: 4,
            near_radius: 0.0,
            reach: 22.0,
            dense: false,
        },
        GraphicsPreset::Medium => VegPlan {
            tufts: 40,
            shrubs: 12,
            trees: 6,
            fine_tufts: 0,
            fine_shrubs: 0,
            fine_trees: 0,
            coarse_segments: 5,
            fine_segments: 5,
            near_radius: 0.0,
            reach: 28.0,
            dense: false,
        },
        GraphicsPreset::High => VegPlan {
            tufts: 280,
            shrubs: 72,
            trees: 28,
            fine_tufts: 80,
            fine_shrubs: 24,
            fine_trees: 8,
            coarse_segments: 4,
            fine_segments: 8,
            near_radius: 18.0,
            reach: 48.0,
            dense: true,
        },
        GraphicsPreset::Ultra => VegPlan {
            tufts: 520,
            shrubs: 120,
            trees: 48,
            fine_tufts: 180,
            fine_shrubs: 48,
            fine_trees: 16,
            coarse_segments: 5,
            fine_segments: 12,
            near_radius: 24.0,
            reach: 64.0,
            dense: true,
        },
    }
}

/// Sanctuary only (`None` or `Some(0)`), same gate as the sky backdrop.
pub const fn vegetation_shown(realm: Option<u8>) -> bool {
    matches!(realm, None | Some(0))
}

pub const fn lod_segments(plan: VegPlan, lod: VegLodTier) -> u32 {
    match lod {
        VegLodTier::Coarse => plan.coarse_segments,
        VegLodTier::Fine => plan.fine_segments,
    }
}

pub const fn part_role(part: VegPart) -> PainterlyRole {
    match part {
        VegPart::Trunk => PainterlyRole::Earth,
        VegPart::Tuft | VegPart::Shrub | VegPart::Canopy => PainterlyRole::Foliage,
    }
}

pub const fn part_authored(part: VegPart) -> [f32; 3] {
    match part {
        VegPart::Tuft => TUFT_SRGB,
        VegPart::Shrub => SHRUB_SRGB,
        VegPart::Trunk => TRUNK_SRGB,
        VegPart::Canopy => CANOPY_SRGB,
    }
}

/// Prototype mesh. `segments` is the radial resolution (at least 3).
pub fn veg_mesh(part: VegPart, segments: u32) -> Mesh {
    let segments = segments.max(3);
    match part {
        VegPart::Tuft => {
            Mesh::from(ConeMeshBuilder::new(0.38, 0.62, segments).anchor(ConeAnchor::Base))
        }
        VegPart::Shrub => {
            Mesh::from(ConeMeshBuilder::new(0.72, 1.05, segments).anchor(ConeAnchor::Base))
        }
        VegPart::Trunk => Mesh::from(
            CylinderMeshBuilder::new(0.11, TRUNK_HEIGHT, segments).anchor(CylinderAnchor::Bottom),
        ),
        VegPart::Canopy => {
            Mesh::from(ConeMeshBuilder::new(0.95, 2.05, segments).anchor(ConeAnchor::Base))
        }
    }
}

/// Spawn mesh. Foliage bakes the painterly height ramp. Trunks stay flat
/// so the earth hue is not tinted.
pub fn prototype_mesh(part: VegPart, segments: u32, preset: GraphicsPreset) -> Mesh {
    let mut mesh = veg_mesh(part, segments);
    if part_role(part) == PainterlyRole::Foliage {
        paint_height_ramp(&mut mesh, &painterly_shade(PainterlyRole::Foliage, preset));
    }
    mesh
}

/// CARD PT-COLLIDE-1 — field trunk radius at scale 1, same as `VegPart::Trunk` in `veg_mesh`.
pub const TRUNK_RADIUS: f32 = 0.11;
/// Deterministic field for `preset`. Same preset, same plants.
pub fn vegetation_plants(preset: GraphicsPreset) -> Vec<VegPlant> {
    let plan = vegetation_plan(preset);
    let mut rng = Rng::new(VEG_SEED);
    let mut out = Vec::with_capacity(plan.plants() as usize);
    scatter(
        &mut rng,
        &mut out,
        VegKind::Tuft,
        plan.tufts,
        plan.fine_tufts,
        &plan,
    );
    scatter(
        &mut rng,
        &mut out,
        VegKind::Shrub,
        plan.shrubs,
        plan.fine_shrubs,
        &plan,
    );
    scatter(
        &mut rng,
        &mut out,
        VegKind::Tree,
        plan.trees,
        plan.fine_trees,
        &plan,
    );
    out
}

/// CARD PT-COLLIDE-1 — trunk circles (x/z, radius) of the spawned field; empty when hidden. Read by the push-out.
#[derive(Resource, Default, Clone, Debug)]
pub struct FieldTrunks(pub Vec<(Vec2, f32)>);

/// Expand plants into trunk / canopy / tuft / shrub instances.
pub fn vegetation_instances(plants: &[VegPlant]) -> Vec<VegInstance> {
    let mut out = Vec::with_capacity(plants.len() * 2);
    for plant in plants {
        match plant.kind {
            VegKind::Tuft => {
                push_instance(plant, VegPart::Tuft, seated_y(plant.x, plant.z), &mut out)
            }
            VegKind::Shrub => {
                push_instance(plant, VegPart::Shrub, seated_y(plant.x, plant.z), &mut out)
            }
            VegKind::Tree => {
                let ground = seated_y(plant.x, plant.z);
                push_instance(plant, VegPart::Trunk, ground, &mut out);
                push_instance(
                    plant,
                    VegPart::Canopy,
                    ground + TRUNK_HEIGHT * plant.scale * CANOPY_SIT,
                    &mut out,
                );
            }
        }
    }
    out
}

/// Group instance indices by part and LOD.
///
/// The spawn path clones one mesh and one material per batch. That is the
/// instance batch: many transforms, one prototype.
pub fn instance_batches(instances: &[VegInstance]) -> Vec<VegBatch> {
    let mut batches: Vec<VegBatch> = Vec::new();
    for (index, inst) in instances.iter().enumerate() {
        let key = VegBatchKey {
            part: inst.part,
            lod: inst.lod,
        };
        if let Some(batch) = batches.iter_mut().find(|batch| batch.key == key) {
            batch.indices.push(index);
        } else {
            batches.push(VegBatch {
                key,
                indices: vec![index],
            });
        }
    }
    batches.sort_by_key(|batch| (batch.key.part as u8, batch.key.lod as u8));
    batches
}

/// World Y of a plant base. Court and yard stay at [`GROUND_LIFT`].
/// A lower shelf keeps the same gap above its tread.
fn seated_y(x: f32, z: f32) -> f32 {
    GROUND_LIFT + crate::terrain::shelf_offset(x, z)
}

fn push_instance(plant: &VegPlant, part: VegPart, y: f32, out: &mut Vec<VegInstance>) {
    out.push(VegInstance {
        part,
        lod: plant.lod,
        translation: Vec3::new(plant.x, y, plant.z),
        yaw: plant.yaw,
        scale: plant.scale,
    });
}

fn scatter(
    rng: &mut Rng,
    out: &mut Vec<VegPlant>,
    kind: VegKind,
    total: u32,
    fine: u32,
    plan: &VegPlan,
) {
    let fine = fine.min(total);
    for i in 0..total {
        let lod = if i < fine {
            VegLodTier::Fine
        } else {
            VegLodTier::Coarse
        };
        let (inner, outer) = ring_for(plan, lod);
        let xz = ring_point(rng, inner, outer);
        out.push(VegPlant {
            kind,
            lod,
            x: xz.x,
            z: xz.y,
            scale: scale_for(kind, rng),
            yaw: rng.next_f32() * TAU,
        });
    }
}

fn ring_for(plan: &VegPlan, lod: VegLodTier) -> (f32, f32) {
    if plan.dense && lod == VegLodTier::Fine {
        (YARD_CLEAR, plan.near_radius)
    } else if plan.dense {
        (plan.near_radius, plan.reach)
    } else {
        (YARD_CLEAR, plan.reach)
    }
}

fn scale_for(kind: VegKind, rng: &mut Rng) -> f32 {
    let (lo, hi) = match kind {
        VegKind::Tuft => (0.75, 1.35),
        VegKind::Shrub => (0.85, 1.55),
        VegKind::Tree => (0.90, 1.70),
    };
    lo + rng.next_f32() * (hi - lo)
}

/// Uniform point in the annulus. Reject the boot-camera pad. If every try
/// lands in that pad, place the plant on the far side of the ring.
fn ring_point(rng: &mut Rng, inner: f32, outer: f32) -> Vec2 {
    for _ in 0..32 {
        let angle = rng.next_f32() * TAU;
        let t = rng.next_f32();
        let radius = (inner * inner + t * (outer * outer - inner * inner)).sqrt();
        let point = Vec2::new(angle.cos() * radius, angle.sin() * radius);
        if point.distance(CAMERA_XZ) >= CAMERA_CLEAR {
            return point;
        }
    }
    let mid = (inner + outer) * 0.5;
    Vec2::new(0.0, -1.0) * mid
}

/// Same xorshift the sky belt uses, so the field does not depend on that module.
struct Rng(u32);

impl Rng {
    fn new(seed: u32) -> Self {
        Self(seed.max(1))
    }

    fn next_f32(&mut self) -> f32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x as f32 / u32::MAX as f32
    }
}

fn instance_transform(inst: &VegInstance) -> Transform {
    Transform {
        translation: inst.translation,
        rotation: Quat::from_rotation_y(inst.yaw),
        scale: Vec3::splat(inst.scale),
    }
}

#[derive(Resource, Default)]
struct VegHeld {
    meshes: Vec<Handle<Mesh>>,
    materials: Vec<Handle<StandardMaterial>>,
}

#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
struct VegLive {
    preset: GraphicsPreset,
    realm: Option<u8>,
}

#[derive(Component, Clone, Copy, Debug)]
struct VegMark {
    /// The spawn test groups instance handles by these.
    #[allow(dead_code)]
    part: VegPart,
    /// The spawn test groups instance handles by these.
    #[allow(dead_code)]
    lod: VegLodTier,
}

pub struct VegetationPlugin;

impl Plugin for VegetationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VegHeld>()
            .add_systems(Update, sync_vegetation);
    }
}

#[derive(SystemParam)]
struct VegCtx<'w, 's> {
    commands: Commands<'w, 's>,
    meshes: Option<ResMut<'w, Assets<Mesh>>>,
    materials: Option<ResMut<'w, Assets<StandardMaterial>>>,
    settings: Option<Res<'w, LocalSettingsState>>,
    realm: Option<Res<'w, SoftPlayerRealm>>,
    ramps: Option<ResMut<'w, PainterlyRampMeshes>>,
    held: ResMut<'w, VegHeld>,
    live: Option<ResMut<'w, VegLive>>,
    existing: Query<'w, 's, Entity, With<VegMark>>,
}

fn sync_vegetation(mut ctx: VegCtx) {
    if ctx.meshes.is_none() || ctx.materials.is_none() {
        return;
    }
    let preset = ctx
        .settings
        .as_ref()
        .map(|settings| settings.inner.graphics_preset)
        .unwrap_or(GraphicsPreset::Medium);
    let realm = ctx
        .realm
        .as_ref()
        .map(|realm| realm.current)
        .unwrap_or(None);
    if ctx
        .live
        .as_ref()
        .is_some_and(|live| live.preset == preset && live.realm == realm)
    {
        return;
    }
    let stale: Vec<Entity> = ctx.existing.iter().collect();
    for entity in stale {
        ctx.commands.entity(entity).despawn();
    }
    let meshes = ctx.meshes.as_mut().unwrap();
    let materials = ctx.materials.as_mut().unwrap();
    release(meshes, materials, &mut ctx.ramps, &mut ctx.held);
    let trees = vegetation_plants(preset).into_iter().filter(|p| vegetation_shown(realm) && p.kind == VegKind::Tree);
    ctx.commands.insert_resource(FieldTrunks(trees.map(|p| (Vec2::new(p.x, p.z), TRUNK_RADIUS * p.scale)).collect()));
    if vegetation_shown(realm) {
        spawn_field(
            meshes,
            materials,
            &mut ctx.ramps,
            &mut ctx.held,
            &mut ctx.commands,
            preset,
        );
    }
    ctx.commands.insert_resource(VegLive { preset, realm });
}

fn release(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    ramps: &mut Option<ResMut<PainterlyRampMeshes>>,
    held: &mut VegHeld,
) {
    if let Some(ramps) = ramps.as_mut() {
        let drop_ids: Vec<_> = held.meshes.iter().map(|handle| handle.id()).collect();
        ramps
            .meshes
            .retain(|(handle, _)| !drop_ids.contains(&handle.id()));
    }
    for handle in held.meshes.drain(..) {
        meshes.remove(&handle);
    }
    for handle in held.materials.drain(..) {
        materials.remove(&handle);
    }
}

fn spawn_field(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    ramps: &mut Option<ResMut<PainterlyRampMeshes>>,
    held: &mut VegHeld,
    commands: &mut Commands,
    preset: GraphicsPreset,
) {
    let plan = vegetation_plan(preset);
    let instances = vegetation_instances(&vegetation_plants(preset));
    let batches = instance_batches(&instances);
    let mut materials_by_part: Vec<(VegPart, Handle<StandardMaterial>)> = Vec::new();
    for batch in &batches {
        let mesh = remember_mesh(
            meshes,
            ramps,
            held,
            batch.key.part,
            lod_segments(plan, batch.key.lod),
            preset,
        );
        let material = material_for(
            materials,
            held,
            &mut materials_by_part,
            batch.key.part,
            preset,
        );
        for index in &batch.indices {
            let inst = &instances[*index];
            let mut entity = commands.spawn((
                (
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    instance_transform(inst),
                ),
                VegMark {
                    part: inst.part,
                    lod: inst.lod,
                },
                Name::new("VP-VEG-1"),
                bevy::light::NotShadowCaster,
            ));
            if part_role(inst.part) == PainterlyRole::Foliage {
                entity.insert(PainterlyMat {
                    role: PainterlyRole::Foliage,
                    authored: part_authored(inst.part),
                });
            }
        }
    }
}

fn remember_mesh(
    meshes: &mut Assets<Mesh>,
    ramps: &mut Option<ResMut<PainterlyRampMeshes>>,
    held: &mut VegHeld,
    part: VegPart,
    segments: u32,
    preset: GraphicsPreset,
) -> Handle<Mesh> {
    let handle = meshes.add(prototype_mesh(part, segments, preset));
    if part_role(part) == PainterlyRole::Foliage {
        remember_ramp(ramps, &handle, PainterlyRole::Foliage);
    }
    held.meshes.push(handle.clone());
    handle
}

fn material_for(
    materials: &mut Assets<StandardMaterial>,
    held: &mut VegHeld,
    cache: &mut Vec<(VegPart, Handle<StandardMaterial>)>,
    part: VegPart,
    preset: GraphicsPreset,
) -> Handle<StandardMaterial> {
    if let Some((_, handle)) = cache.iter().find(|(cached, _)| *cached == part) {
        return handle.clone();
    }
    let handle = materials.add(painterly_material(
        part_role(part),
        part_authored(part),
        preset,
    ));
    held.materials.push(handle.clone());
    cache.push((part, handle.clone()));
    handle
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::painterly_mat::painterly_srgb;
    use std::collections::{HashMap, HashSet};

    fn field(preset: GraphicsPreset) -> Vec<VegInstance> {
        vegetation_instances(&vegetation_plants(preset))
    }

    fn draws(preset: GraphicsPreset) -> usize {
        instance_batches(&field(preset)).len()
    }

    #[test]
    fn low_and_medium_stay_cheap() {
        let medium = vegetation_plan(GraphicsPreset::Medium);
        for preset in [
            GraphicsPreset::Mobile,
            GraphicsPreset::Low,
            GraphicsPreset::Medium,
        ] {
            let plan = vegetation_plan(preset);
            assert!(!plan.dense, "{preset:?}");
            assert_eq!(plan.fine_plants(), 0, "{preset:?}");
            assert_eq!(plan.near_radius, 0.0, "{preset:?}");
            assert_eq!(plan.fine_segments, plan.coarse_segments, "{preset:?}");
            assert!(plan.coarse_segments <= 5, "{preset:?}");
            assert!(plan.instances() <= CHEAP_INSTANCE_CAP, "{preset:?}");
            assert!(plan.instances() < medium.instances() * DENSE_MIN_FACTOR);
            let instances = field(preset);
            assert_eq!(instances.len() as u32, plan.instances());
            assert!(instances.iter().all(|inst| inst.lod == VegLodTier::Coarse));
            assert_eq!(draws(preset), 4, "{preset:?}");
        }
        assert!(
            vegetation_plan(GraphicsPreset::Mobile).instances()
                < vegetation_plan(GraphicsPreset::Low).instances()
        );
        assert!(
            vegetation_plan(GraphicsPreset::Low).instances()
                < vegetation_plan(GraphicsPreset::Medium).instances()
        );
        assert!(
            vegetation_plan(GraphicsPreset::Low).reach
                < vegetation_plan(GraphicsPreset::Medium).reach
        );
    }

    #[test]
    fn high_and_ultra_are_dense() {
        let medium = vegetation_plan(GraphicsPreset::Medium);
        let high = vegetation_plan(GraphicsPreset::High);
        let ultra = vegetation_plan(GraphicsPreset::Ultra);
        for plan in [high, ultra] {
            assert!(plan.dense);
            assert!(plan.plants() >= medium.plants() * DENSE_MIN_FACTOR);
            assert!(plan.instances() >= medium.instances() * DENSE_MIN_FACTOR);
            assert!(plan.instances() > CHEAP_INSTANCE_CAP);
            assert!(plan.fine_plants() > 0);
            assert!(plan.fine_plants() < plan.plants());
            assert!(plan.fine_segments > plan.coarse_segments);
            assert!(plan.near_radius > YARD_CLEAR);
            assert!(plan.reach > plan.near_radius);
            // Far plants stay at or under Medium's segment count.
            assert!(plan.coarse_segments <= medium.coarse_segments);
        }
        assert!(ultra.plants() > high.plants());
        assert!(ultra.instances() > high.instances());
        assert!(ultra.fine_segments > high.fine_segments);
        assert!(ultra.reach > high.reach);
        assert_eq!(field(GraphicsPreset::High).len() as u32, high.instances());
        assert_eq!(field(GraphicsPreset::Ultra).len() as u32, ultra.instances());
        assert_eq!(draws(GraphicsPreset::High), 8);
        assert_eq!(draws(GraphicsPreset::Ultra), 8);
        let high_field = field(GraphicsPreset::High);
        assert!(high_field.iter().any(|inst| inst.lod == VegLodTier::Fine));
        assert!(high_field.iter().any(|inst| inst.lod == VegLodTier::Coarse));
    }

    #[test]
    fn lod_segments_follow_the_preset() {
        assert_eq!(
            lod_segments(vegetation_plan(GraphicsPreset::Low), VegLodTier::Coarse),
            4
        );
        assert_eq!(
            lod_segments(vegetation_plan(GraphicsPreset::Medium), VegLodTier::Coarse),
            5
        );
        assert_eq!(
            lod_segments(vegetation_plan(GraphicsPreset::High), VegLodTier::Fine),
            8
        );
        assert_eq!(
            lod_segments(vegetation_plan(GraphicsPreset::High), VegLodTier::Coarse),
            4
        );
        assert_eq!(
            lod_segments(vegetation_plan(GraphicsPreset::Ultra), VegLodTier::Fine),
            12
        );
        for part in [
            VegPart::Tuft,
            VegPart::Shrub,
            VegPart::Trunk,
            VegPart::Canopy,
        ] {
            let coarse = veg_mesh(part, 4).count_vertices();
            let fine = veg_mesh(part, 12).count_vertices();
            assert!(fine > coarse, "{part:?} {coarse} -> {fine}");
        }
        let canopy = prototype_mesh(VegPart::Canopy, 4, GraphicsPreset::Low);
        assert_eq!(
            canopy.attribute(Mesh::ATTRIBUTE_COLOR).map(|c| c.len()),
            Some(canopy.count_vertices())
        );
        assert!(prototype_mesh(VegPart::Trunk, 4, GraphicsPreset::Ultra)
            .attribute(Mesh::ATTRIBUTE_COLOR)
            .is_none());
    }

    #[test]
    fn instance_batches_share_one_mesh_per_part_and_lod() {
        let hand = [
            VegInstance {
                part: VegPart::Tuft,
                lod: VegLodTier::Coarse,
                translation: Vec3::ZERO,
                yaw: 0.0,
                scale: 1.0,
            },
            VegInstance {
                part: VegPart::Tuft,
                lod: VegLodTier::Coarse,
                translation: Vec3::X,
                yaw: 0.2,
                scale: 1.1,
            },
            VegInstance {
                part: VegPart::Tuft,
                lod: VegLodTier::Fine,
                translation: Vec3::Z,
                yaw: 0.4,
                scale: 1.2,
            },
            VegInstance {
                part: VegPart::Trunk,
                lod: VegLodTier::Coarse,
                translation: Vec3::Y,
                yaw: 0.1,
                scale: 1.0,
            },
            VegInstance {
                part: VegPart::Canopy,
                lod: VegLodTier::Coarse,
                translation: Vec3::Y,
                yaw: 0.1,
                scale: 1.0,
            },
        ];
        let batches = instance_batches(&hand);
        assert_eq!(batches.len(), 4);
        assert_eq!(
            batches
                .iter()
                .map(|batch| batch.indices.len())
                .sum::<usize>(),
            hand.len()
        );
        assert_eq!(
            batches[0].key,
            VegBatchKey {
                part: VegPart::Tuft,
                lod: VegLodTier::Coarse,
            }
        );
        assert_eq!(batches[0].indices, vec![0, 1]);

        for preset in GraphicsPreset::ALL {
            let instances = field(preset);
            let batches = instance_batches(&instances);
            let mut seen = vec![false; instances.len()];
            for batch in &batches {
                assert!(
                    batch.indices.len() >= 2,
                    "{preset:?} {:?} is a single",
                    batch.key
                );
                for index in &batch.indices {
                    assert!(!seen[*index]);
                    seen[*index] = true;
                    assert_eq!(instances[*index].part, batch.key.part);
                    assert_eq!(instances[*index].lod, batch.key.lod);
                }
            }
            assert!(seen.into_iter().all(|hit| hit));
            assert!(batches.len() < instances.len() / 4);
        }
    }

    #[test]
    fn field_stays_outside_the_yard_and_is_deterministic() {
        assert_eq!(field(GraphicsPreset::Ultra), field(GraphicsPreset::Ultra));
        assert_ne!(
            vegetation_plan(GraphicsPreset::Low).tufts,
            vegetation_plan(GraphicsPreset::High).tufts
        );
        for preset in GraphicsPreset::ALL {
            let plan = vegetation_plan(preset);
            for inst in field(preset) {
                let xz = Vec2::new(inst.translation.x, inst.translation.z);
                assert!(xz.length() + 1.0e-3 >= YARD_CLEAR, "{preset:?} {xz:?}");
                assert!(xz.length() <= plan.reach + 1.0e-3, "{preset:?} {xz:?}");
                assert!(
                    xz.distance(CAMERA_XZ) + 1.0e-3 >= CAMERA_CLEAR,
                    "{preset:?} {xz:?}"
                );
                assert!(inst.scale > 0.0);
                if inst.lod == VegLodTier::Fine {
                    assert!(xz.length() <= plan.near_radius + 1.0e-3);
                } else if plan.dense {
                    assert!(xz.length() + 1.0e-3 >= plan.near_radius);
                }
            }
        }
    }

    #[test]
    fn plants_sit_on_the_shaped_surface() {
        use crate::terrain::{play_footprint, shelf_offset, COURT_RADIUS, PLAY_FLOOR_Y};

        for preset in GraphicsPreset::ALL {
            let plants = vegetation_plants(preset);
            let instances = vegetation_instances(&plants);
            assert_eq!(instances.len() as u32, vegetation_plan(preset).instances());
            for inst in &instances {
                let ground = seated_y(inst.translation.x, inst.translation.z);
                match inst.part {
                    VegPart::Tuft | VegPart::Shrub | VegPart::Trunk => {
                        assert!(
                            (inst.translation.y - ground).abs() < 1.0e-4,
                            "{preset:?} {:?} {}",
                            inst.part,
                            inst.translation.y
                        );
                    }
                    VegPart::Canopy => {
                        let canopy = ground + TRUNK_HEIGHT * inst.scale * CANOPY_SIT;
                        assert!(
                            (inst.translation.y - canopy).abs() < 1.0e-4,
                            "{preset:?} canopy {}",
                            inst.translation.y
                        );
                    }
                }
                if play_footprint(inst.translation.x, inst.translation.z) {
                    let base = match inst.part {
                        VegPart::Canopy => {
                            inst.translation.y - TRUNK_HEIGHT * inst.scale * CANOPY_SIT
                        }
                        _ => inst.translation.y,
                    };
                    assert!(
                        (base - GROUND_LIFT).abs() < 1.0e-4,
                        "{preset:?} footprint plant moved"
                    );
                }
            }
            // Cheap reaches stay inside the court, so the old lift holds.
            if matches!(
                preset,
                GraphicsPreset::Mobile | GraphicsPreset::Low | GraphicsPreset::Medium
            ) {
                assert!(vegetation_plan(preset).reach < COURT_RADIUS);
                assert!(instances.iter().all(|inst| {
                    let base = match inst.part {
                        VegPart::Canopy => {
                            inst.translation.y - TRUNK_HEIGHT * inst.scale * CANOPY_SIT
                        }
                        _ => inst.translation.y,
                    };
                    (base - GROUND_LIFT).abs() < 1.0e-4
                }));
            }
        }

        let ultra = field(GraphicsPreset::Ultra);
        assert!(ultra.iter().any(|inst| {
            let r = Vec2::new(inst.translation.x, inst.translation.z).length();
            r > COURT_RADIUS && shelf_offset(inst.translation.x, inst.translation.z) < -0.5
        }));
        assert!(ultra
            .iter()
            .any(|inst| { inst.part == VegPart::Trunk && inst.translation.y < GROUND_LIFT - 0.5 }));
        // The drop is the shelf, not a new floor height on the yard.
        assert_eq!(seated_y(0.0, 0.0), GROUND_LIFT);
        assert!(seated_y(COURT_RADIUS + 2.0, 0.0) < PLAY_FLOOR_Y);
    }

    #[test]
    fn sanctuary_only() {
        assert!(vegetation_shown(None));
        assert!(vegetation_shown(Some(0)));
        for realm in [Some(1u8), Some(2), Some(3), Some(4)] {
            assert!(!vegetation_shown(realm));
        }
    }

    #[test]
    fn ground_and_earth_hues_stay() {
        // Valley earth is the yard / floor hue. This card does not retune it.
        assert_eq!(
            painterly_srgb(PainterlyRole::Ground, [0.33, 0.40, 0.20]),
            [0.33, 0.40, 0.20]
        );
        assert_eq!(painterly_srgb(PainterlyRole::Earth, TRUNK_SRGB), TRUNK_SRGB);
        assert_eq!(part_role(VegPart::Trunk), PainterlyRole::Earth);
        let leaf = painterly_srgb(PainterlyRole::Foliage, CANOPY_SRGB);
        assert!(leaf[1] > leaf[0] && leaf[1] > leaf[2]);
    }

    #[test]
    fn spawn_instances_share_handles_and_track_the_preset() {
        use bevy::asset::AssetPlugin;
        use shared::local_settings::LocalSettings;

        fn boot(preset: GraphicsPreset) -> App {
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, AssetPlugin::default()))
                .init_asset::<Mesh>()
                .init_asset::<StandardMaterial>()
                .add_plugins(VegetationPlugin)
                .insert_resource(SoftPlayerRealm { current: Some(0) });
            let mut inner = LocalSettings::default();
            inner.set_graphics_preset(preset);
            app.insert_resource(LocalSettingsState {
                inner,
                dirty: false,
            });
            app
        }

        fn tally(app: &mut App) -> (usize, usize, usize) {
            let world = app.world_mut();
            let mut query = world.query::<(&VegMark, &Mesh3d, &MeshMaterial3d<StandardMaterial>)>();
            let rows: Vec<_> = query
                .iter(world)
                .map(|(mark, mesh, material)| (*mark, mesh.0.id(), material.0.id()))
                .collect();
            let meshes: HashSet<_> = rows.iter().map(|row| row.1).collect();
            let materials: HashSet<_> = rows.iter().map(|row| row.2).collect();
            (rows.len(), meshes.len(), materials.len())
        }

        let mut app = boot(GraphicsPreset::Low);
        app.update();
        let low_plan = vegetation_plan(GraphicsPreset::Low);
        assert_eq!(tally(&mut app), (low_plan.instances() as usize, 4, 4));
        app.update();
        assert_eq!(tally(&mut app).0, low_plan.instances() as usize);

        app.world_mut()
            .resource_mut::<LocalSettingsState>()
            .inner
            .set_graphics_preset(GraphicsPreset::Ultra);
        app.update();
        let ultra = vegetation_plan(GraphicsPreset::Ultra);
        assert_eq!(tally(&mut app), (ultra.instances() as usize, 8, 4));

        let world = app.world_mut();
        let mut query = world.query::<(&VegMark, &Mesh3d, &MeshMaterial3d<StandardMaterial>)>();
        let rows: Vec<_> = query
            .iter(world)
            .map(|(mark, mesh, material)| (*mark, mesh.0.clone(), material.0.id()))
            .collect();
        let mut mesh_by_key: HashMap<VegBatchKey, AssetId<Mesh>> = HashMap::new();
        let mut material_by_part: HashMap<VegPart, AssetId<StandardMaterial>> = HashMap::new();
        for (mark, mesh, material) in &rows {
            let key = VegBatchKey {
                part: mark.part,
                lod: mark.lod,
            };
            if let Some(prev) = mesh_by_key.get(&key) {
                assert_eq!(*prev, mesh.id());
            } else {
                mesh_by_key.insert(key, mesh.id());
            }
            if let Some(prev) = material_by_part.get(&mark.part) {
                assert_eq!(*prev, *material);
            } else {
                material_by_part.insert(mark.part, *material);
            }
        }
        assert_eq!(mesh_by_key.len(), 8);
        assert_eq!(material_by_part.len(), 4);
        assert_ne!(
            mesh_by_key[&VegBatchKey {
                part: VegPart::Canopy,
                lod: VegLodTier::Fine,
            }],
            mesh_by_key[&VegBatchKey {
                part: VegPart::Canopy,
                lod: VegLodTier::Coarse,
            }]
        );
        let assets = world.resource::<Assets<Mesh>>();
        let fine = rows
            .iter()
            .find(|row| row.0.part == VegPart::Canopy && row.0.lod == VegLodTier::Fine)
            .unwrap();
        let coarse = rows
            .iter()
            .find(|row| row.0.part == VegPart::Canopy && row.0.lod == VegLodTier::Coarse)
            .unwrap();
        let fine_verts = assets.get(&fine.1).unwrap().count_vertices();
        let coarse_verts = assets.get(&coarse.1).unwrap().count_vertices();
        assert_eq!(
            fine_verts,
            veg_mesh(VegPart::Canopy, ultra.fine_segments).count_vertices()
        );
        assert_eq!(
            coarse_verts,
            veg_mesh(VegPart::Canopy, ultra.coarse_segments).count_vertices()
        );
        assert!(fine_verts > coarse_verts);

        app.world_mut().resource_mut::<SoftPlayerRealm>().current = Some(3);
        app.update();
        assert_eq!(tally(&mut app).0, 0);
        app.world_mut().resource_mut::<SoftPlayerRealm>().current = Some(0);
        app.update();
        assert_eq!(tally(&mut app).0, ultra.instances() as usize);
    }
}
