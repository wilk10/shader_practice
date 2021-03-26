use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, RenderGraph, RenderResourcesNode},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "af8c8bb6-bab2-48e9-9251-6b757d28afda"]
struct TimeComponent {
    value: f32,
}

pub fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(animate_shader.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("time.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("time.frag"),
        ))),
    }));

    render_graph.add_system_node(
        "time_component",
        RenderResourcesNode::<TimeComponent>::new(true),
    );
    render_graph
        .add_node_edge("time_component", base::node::MAIN_PASS)
        .unwrap();

    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(5.0, 5.0),
                flip: true,
            })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(TimeComponent { value: 0.0 });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, -8.0).looking_at(Vec3::ZERO, -Vec3::Y),
        ..Default::default()
    });
}

fn animate_shader(time: Res<Time>, mut query: Query<&mut TimeComponent>) {
    for mut time_component in query.iter_mut() {
        time_component.value = time.seconds_since_startup() as f32;
    }
}
