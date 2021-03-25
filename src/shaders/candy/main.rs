use bevy::{
    core::Bytes,
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::{RenderResource, RenderResourceType, RenderResources},
        shader::{ShaderStage, ShaderStages},
    },
};

struct Rotator;

#[derive(Default, RenderResources, TypeUuid)]
#[render_resources(from_self)]
#[uuid = "7d342b4f-59a7-47a9-bb1a-f4f8b9fb0bb6"]
#[repr(C)]
struct Candy {
    pub color_a: Color,
    pub color_b: Color,
    pub start_lerp: f32,
    pub end_lerp: f32,
}

impl RenderResource for Candy {
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

pub fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_asset::<Candy>()
        .add_startup_system(setup.system())
        .add_system(rotate_capsule.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Candy>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("candy.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("candy.frag"),
        ))),
    }));

    render_graph.add_system_node("candy", AssetRenderResourcesNode::<Candy>::new(true));
    render_graph
        .add_node_edge("candy", base::node::MAIN_PASS)
        .unwrap();
    let material = materials.add(Candy {
        color_a: Color::rgb(1.0, 1.0, 1.0),
        color_b: Color::rgb(1.0, 0.0, 0.0),
        start_lerp: 0.1,
        end_lerp: 0.9,
    });

    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Rotator)
        .insert(material);

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, -8.0).looking_at(Vec3::ZERO, -Vec3::Y),
        ..Default::default()
    });
}

fn rotate_capsule(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_ypr(
            2.0 * time.delta_seconds(),
            0.1 * time.delta_seconds(),
            0.5 * time.delta_seconds(),
        );
    }
}
