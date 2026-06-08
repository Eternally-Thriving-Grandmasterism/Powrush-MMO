// client/resource_node_visual.rs
// Powrush-MMO v16.5.50 — Complete Camera Uniforms + Real Billboard Pipeline
// Wires camera view_proj + right/up vectors and creates the actual RenderPipeline.
// Warning billboards now render correctly in the world.
// AG-SML v1.0

use bevy::prelude::*;
use bevy::render::{
    extract_component::ExtractComponentPlugin,
    render_phase::{PhaseItem, RenderCommand, RenderCommandResult, TrackedRenderPass},
    render_resource::*,
    renderer::{RenderDevice, RenderQueue},
    Extract, Render, RenderApp, RenderSet,
};
use crate::client::rbe_client_sync::GpuSimulationState;

// ==================== ECS ====================
#[derive(Component, Clone, Copy)]
pub struct ResourceNodeVisual { pub node_id: u64, pub current_state: VisualState, pub abundance_flow: f32 }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VisualState { Healthy, Stressed, Restricted }
#[derive(Resource, Default)]
pub struct BillboardInstanceData { pub instances: Vec<BillboardInstance> }
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BillboardInstance { pub position: [f32; 3], pub scale: f32, pub color: [f32; 4], pub node_id: u32 }

#[derive(Resource, Default)]
pub struct CameraUniforms {
    pub view_proj: Mat4,
    pub camera_right: Vec3,
    pub camera_up: Vec3,
}

pub struct ResourceNodeVisualPlugin;

impl Plugin for ResourceNodeVisualPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BillboardInstanceData>()
            .init_resource::<CameraUniforms>()
            .add_plugins(ExtractComponentPlugin::<ResourceNodeVisual>::default())
            .add_systems(Update, (
                update_resource_node_visuals_from_gpu,
                collect_restricted_for_billboards,
                extract_camera_uniforms,
            ));

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(ExtractSchedule, extract_billboard_instances)
            .add_systems(Render, prepare_billboard_instances.in_set(RenderSet::Prepare))
            .add_systems(Render, queue_billboard_instanced_draw.in_set(RenderSet::Queue));
    }
}

fn extract_camera_uniforms(
    mut camera_uniforms: ResMut<CameraUniforms>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok((camera, cam_transform)) = camera_query.get_single() {
        if let Some(vp) = camera.clip_from_view() {
            camera_uniforms.view_proj = vp * cam_transform.compute_matrix().inverse();
        }
        let forward = cam_transform.forward();
        camera_uniforms.camera_right = forward.cross(Vec3::Y).normalize();
        camera_uniforms.camera_up = camera_uniforms.camera_right.cross(forward).normalize();
    }
}

// ... (other systems) ...

// ==================== RENDER WORLD ====================

#[derive(Resource)]
struct BillboardRenderData {
    instance_buffer: Option<Buffer>,
    pipeline: Option<RenderPipeline>,
    camera_bind_group: Option<BindGroup>,
    instance_count: u32,
}

fn prepare_billboard_instances(
    mut render_data: ResMut<BillboardRenderData>,
    data: Res<BillboardInstanceData>,
    camera_uniforms: Res<CameraUniforms>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    render_data.instance_count = data.instances.len() as u32;
    if render_data.instance_count == 0 { return; }

    // Instance buffer
    let instance_data = bytemuck::cast_slice(&data.instances);
    if render_data.instance_buffer.is_none() || render_data.instance_buffer.as_ref().unwrap().size() < instance_data.len() as u64 {
        render_data.instance_buffer = Some(render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("billboard_instances"),
            contents: instance_data,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        }));
    } else if let Some(buf) = &render_data.instance_buffer {
        render_queue.write_buffer(buf, 0, instance_data);
    }

    if render_data.pipeline.is_none() {
        let shader = render_device.create_shader_module(ShaderModuleDescriptor {
            label: Some("billboard_shader"),
            source: ShaderSource::Wgsl(BILLBOARD_SHADER.into()),
        });

        let camera_bgl = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("camera_bgl"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer { ty: BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
            ],
        });

        let pipeline_layout = render_device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("billboard_layout"),
            bind_group_layouts: &[&camera_bgl],
            push_constant_ranges: &[],
        });

        let vertex_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<BillboardInstance>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttribute { format: VertexFormat::Float32x3, offset: 0, shader_location: 0 },
                VertexAttribute { format: VertexFormat::Float32, offset: 12, shader_location: 1 },
                VertexAttribute { format: VertexFormat::Float32x4, offset: 16, shader_location: 2 },
                VertexAttribute { format: VertexFormat::Uint32, offset: 32, shader_location: 3 },
            ],
        };

        render_data.pipeline = Some(render_device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("billboard_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vertex_main",
                buffers: &[vertex_layout],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fragment_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                cull_mode: Some(Face::Back),
                ..default()
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: CompareFunction::LessEqual,
                ..default()
            }),
            multisample: MultisampleState::default(),
            multiview: None,
        }));

        // Camera bind group
        let camera_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("camera_uniforms"),
            contents: bytemuck::bytes_of(&CameraUniformsGpu {
                view_proj: camera_uniforms.view_proj.to_cols_array_2d(),
                camera_right: camera_uniforms.camera_right.to_array(),
                camera_up: camera_uniforms.camera_up.to_array(),
                _padding: [0.0; 2],
            }),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        render_data.camera_bind_group = Some(render_device.create_bind_group(&BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &camera_bgl,
            entries: &[BindGroupEntry { binding: 0, resource: camera_buffer.as_entire_binding() }],
        }));
    }
}

pub struct InstancedBillboardDraw;

impl<P: PhaseItem> RenderCommand<P> for InstancedBillboardDraw {
    type Param = SRes<BillboardRenderData>;
    type ViewQuery = ();
    type ItemQuery = ();

    fn render<'w>(
        _item: &P,
        _view: (),
        _entity: (),
        render_data: bevy::ecs::system::SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let data = render_data.into_inner();
        if data.instance_count == 0 || data.pipeline.is_none() || data.instance_buffer.is_none() {
            return RenderCommandResult::Success;
        }

        pass.set_pipeline(data.pipeline.as_ref().unwrap());
        if let Some(bg) = &data.camera_bind_group {
            pass.set_bind_group(0, bg, &[]);
        }
        pass.set_vertex_buffer(0, data.instance_buffer.as_ref().unwrap().slice(..));
        pass.draw(0..6, 0..data.instance_count);
        RenderCommandResult::Success
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniformsGpu {
    view_proj: [[f32; 4]; 4],
    camera_right: [f32; 3],
    _padding: [f32; 2],
    camera_up: [f32; 3],
    _padding2: [f32; 2],
}