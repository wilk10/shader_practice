use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};

use shader_practice::shaders::candy::frag::FRAGMENT_SHADER;
use shader_practice::shaders::candy::vert::VERTEX_SHADER;

struct Rotator;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "7d342b4f-59a7-47a9-bb1a-f4f8b9fb0bb6"]
struct Candy {
    pub color_a: Color,
    pub color_b: Color,
    pub start_lerp: f32,
    pub end_lerp: f32,
}

fn main() {
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
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    render_graph.add_system_node(
        "candy",
        AssetRenderResourcesNode::<Candy>::new(true),
    );
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
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle.clone(),
            )]),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(Rotator)
        .with(material)
        .spawn(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, -8.0)
                .looking_at(Vec3::ZERO, -Vec3::Y),
            ..Default::default()
        });
        
}

fn rotate_capsule(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_ypr(
            2.0 * time.delta_seconds(),
            0.1 * time.delta_seconds(),
            0.5 * time.delta_seconds()
        );
    }
}
