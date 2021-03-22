use bevy::{
    prelude::*,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        shader::{ShaderStage, ShaderStages},
    },
};

use shader_practice::shaders::uv::frag::FRAGMENT_SHADER;
use shader_practice::shaders::uv::vert::VERTEX_SHADER;

struct Rotator;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(rotate_sphere.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));
    
    commands
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 1.0, subdivisions: 10 })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle.clone(),
            )]),
            transform: Transform::from_xyz(-3.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(Rotator)
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::new(3.0, 3.0), flip: true })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(3.0, 0.0, 0.0),
            ..Default::default()
        })
        .spawn(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, -8.0)
                .looking_at(Vec3::ZERO, -Vec3::Y),
            ..Default::default()
        });
        
}

fn rotate_sphere(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(3.0 * time.delta_seconds());
    }
}
