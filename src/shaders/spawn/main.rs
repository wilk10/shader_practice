use bevy::{
    core::Bytes,
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{
            BlendFactor, BlendOperation, BlendState, ColorTargetState, ColorWrite, CompareFunction,
            CullMode, DepthBiasState, DepthStencilState, FrontFace, PipelineDescriptor,
            PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, StencilFaceState,
            StencilState,
        },
        render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
        renderer::{RenderResource, RenderResourceType, RenderResources},
        shader::{ShaderStage, ShaderStages},
        texture::TextureFormat,
    },
};

struct Rotator;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "af8c8bb6-bab2-48e9-9251-6b757d28afda"]
struct TimeComponent {
    value: f32,
}

#[derive(RenderResources, Default, TypeUuid)]
#[render_resources(from_self)]
#[uuid = "001a72b7-a79e-4768-bc30-34188f540716"]
#[repr(C)]
struct SpawnVfx {
    pub color_a: Color,
    pub color_b: Color,
    pub start_lerp: f32,
    pub end_lerp: f32,
}

impl RenderResource for SpawnVfx {
    fn resource_type(&self) -> Option<RenderResourceType> {
        Some(RenderResourceType::Buffer)
    }

    fn buffer_byte_len(&self) -> Option<usize> {
        Some(40)
    }

    fn write_buffer_bytes(&self, buffer: &mut [u8]) {
        let (color_a_buf, rest) = buffer.split_at_mut(16);
        self.color_a.write_bytes(color_a_buf);

        let (color_b_buf, rest) = rest.split_at_mut(16);
        self.color_b.write_bytes(color_b_buf);

        let (start_lerp_buf, rest) = rest.split_at_mut(4);
        self.start_lerp.write_bytes(start_lerp_buf);

        self.end_lerp.write_bytes(rest);
    }

    fn texture(&self) -> Option<&Handle<Texture>> {
        None
    }
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_asset::<SpawnVfx>()
        .add_startup_system(setup.system())
        .add_system(rotate_capsule.system())
        .add_system(animate_capsule.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut spawn_materials: ResMut<Assets<SpawnVfx>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor {
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: false, // changed this from true, to see objects behind it
            depth_compare: CompareFunction::LessEqual,
            stencil: StencilState {
                front: StencilFaceState::IGNORE,
                back: StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: DepthBiasState {
                constant: 0,
                slope_scale: 0.0,
                clamp: 0.0,
            },
            clamp_depth: false,
        }),
        color_target_states: vec![ColorTargetState {
            format: TextureFormat::default(),
            color_blend: BlendState {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::DstColor, /* changed this from OneMinusSrcAlpha, to get
                                                    * additive blending (hopefully) */
                operation: BlendOperation::Add,
            },
            alpha_blend: BlendState {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            write_mask: ColorWrite::ALL,
        }],
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::None,
            polygon_mode: PolygonMode::Fill,
        },
        ..PipelineDescriptor::new(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("spawn.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("spawn.frag"),
            ))),
        })
    });

    render_graph.add_system_node("spawn_vfx", AssetRenderResourcesNode::<SpawnVfx>::new(true));
    render_graph
        .add_node_edge("spawn_vfx", base::node::MAIN_PASS)
        .unwrap();

    render_graph.add_system_node(
        "time_component",
        RenderResourcesNode::<TimeComponent>::new(true),
    );
    render_graph
        .add_node_edge("time_component", base::node::MAIN_PASS)
        .unwrap();

    let spawn_material = spawn_materials.add(SpawnVfx {
        color_a: Color::rgb_u8(66, 113, 179),
        color_b: Color::rgb_u8(229, 209, 167),
        start_lerp: 0.0,
        end_lerp: 1.0,
    });

    let capsule = shape::Capsule {
        radius: 1.0,
        rings: 0,
        depth: 2.0,
        latitudes: 16,
        longitudes: 32,
        uv_profile: shape::CapsuleUvProfile::Aspect,
    };

    commands
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(capsule)),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle.clone(),
            )]),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(TimeComponent { value: 0.0 })
        .with(spawn_material)
        .with(Rotator)
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: standard_materials.add(Color::rgb_u8(159, 59, 87).into()),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..Default::default()
        })
        .spawn(LightBundle {
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .spawn(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, -2.0, -8.0).looking_at(Vec3::ZERO, -Vec3::Y),
            ..Default::default()
        });
}

fn rotate_capsule(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_ypr(
            0.0 * time.delta_seconds(),
            0.2 * time.delta_seconds(),
            0.1 * time.delta_seconds(),
        );
    }
}

fn animate_capsule(time: Res<Time>, mut query: Query<&mut TimeComponent>) {
    for mut time_component in query.iter_mut() {
        time_component.value = time.seconds_since_startup() as f32;
        println!("time_component.value: {:?}", time_component.value);
    }
}
