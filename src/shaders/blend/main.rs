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

struct Rotator;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
struct BlendColors {
    pub color_a: Color,
    pub color_b: Color,
    pub start_lerp: f32,
    pub end_lerp: f32,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_asset::<BlendColors>()
        .add_startup_system(setup.system())
        .add_system(rotate_sphere.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BlendColors>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, include_str!("blend.vert"))),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, include_str!("blend.frag")))),
    }));

    render_graph.add_system_node(
        "blend_colors",
        AssetRenderResourcesNode::<BlendColors>::new(true),
    );
    render_graph
        .add_node_edge("blend_colors", base::node::MAIN_PASS)
        .unwrap();
    let material = materials.add(BlendColors {
        color_a: Color::rgb(0.0, 0.0, 1.0),
        color_b: Color::rgb(1.0, 0.0, 0.0),
        start_lerp: 0.25,
        end_lerp: 0.75,
    });
    
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
        .with(material.clone())
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::new(3.0, 3.0), flip: true })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(3.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(material)
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
