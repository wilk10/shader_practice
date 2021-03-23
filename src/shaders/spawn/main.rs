use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph, RenderResourcesNode},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};

use shader_practice::shaders::spawn::frag::FRAGMENT_SHADER;
use shader_practice::shaders::spawn::vert::VERTEX_SHADER;

struct Rotator;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "af8c8bb6-bab2-48e9-9251-6b757d28afda"]
struct TimeComponent{
    value: f32,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "001a72b7-a79e-4768-bc30-34188f540716"]
struct SpawnVfx {
    pub color_a: Color,
    pub color_b: Color,
    pub start_lerp: f32,
    pub end_lerp: f32,
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
    mut materials: ResMut<Assets<SpawnVfx>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    render_graph.add_system_node(
        "spawn_vfx",
        AssetRenderResourcesNode::<SpawnVfx>::new(true),
    );
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

    let material = materials.add(SpawnVfx {
        color_a: Color::rgb_u8(66, 113, 179),
        color_b: Color::rgb_u8(229, 209, 167),
        start_lerp: 0.0,
        end_lerp: 1.0,
    });
    
    commands
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle.clone(),
            )]),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(TimeComponent{value: 0.0})
        .with(material)
        .with(Rotator)
        .spawn(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, -8.0)
                .looking_at(Vec3::ZERO, -Vec3::Y),
            ..Default::default()
        });
        
}

fn rotate_capsule(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_ypr(
            0.0 * time.delta_seconds(),
            0.2 * time.delta_seconds(),
            0.1 * time.delta_seconds()
        );
    }
}

fn animate_capsule(time: Res<Time>, mut query: Query<&mut TimeComponent>) {
    for mut time_component in query.iter_mut() {
        time_component.value = time.seconds_since_startup() as f32;
        println!("time_component.value: {:?}", time_component.value);
    }
}
