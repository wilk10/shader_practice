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

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "93fb26fc-6c05-489b-9029-601edf703b6b"]
struct FireTexture {
    pub texture: Handle<Texture>,
}

#[derive(RenderResources, Default, TypeUuid)]
#[render_resources(from_self)]
#[uuid = "539fe49d-df51-48c1-bbfc-d68eb1716354"]
#[repr(C)]
struct FireMaterial {
    pub base_color: Color,
    pub power: f32,
    pub detail_level: f32,
    pub bottom_threshold: f32,
}

impl RenderResource for FireMaterial {
    fn resource_type(&self) -> Option<RenderResourceType> {
        Some(RenderResourceType::Buffer)
    }

    fn buffer_byte_len(&self) -> Option<usize> {
        Some(28)
    }

    fn write_buffer_bytes(&self, buffer: &mut [u8]) {
        let (base_color_buf, rest) = buffer.split_at_mut(16);
        self.base_color.write_bytes(base_color_buf);

        let (power_buf, rest) = rest.split_at_mut(4);
        self.power.write_bytes(power_buf);

        let (detail_level_buf, rest) = rest.split_at_mut(4);
        self.detail_level.write_bytes(detail_level_buf);

        self.bottom_threshold.write_bytes(rest);
    }

    fn texture(&self) -> Option<&Handle<Texture>> {
        None
    }
}

struct LoadingTexture(Option<Handle<Texture>>);

struct FirePipeline(Handle<PipelineDescriptor>);

struct FireMaterialHandle(Handle<FireMaterial>);

pub fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_asset::<FireMaterial>()
        .add_asset::<FireTexture>()
        .add_startup_system(setup.system())
        .add_system(spawn_fire.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut fire_materials: ResMut<Assets<FireMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    commands.insert_resource(LoadingTexture(Some(
        asset_server.load("fire.png"),
    )));

    let fire_material = fire_materials.add(FireMaterial {
        base_color: Color::rgba_u8(179, 111, 76, 180),
        power: 0.5,
        detail_level: 10.0,
        bottom_threshold: -0.5,
    });
    commands.insert_resource(FireMaterialHandle(fire_material));

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("fire.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("fire.frag"),
        ))),
    }));
    commands.insert_resource(FirePipeline(pipeline_handle));

    render_graph.add_system_node(
        "fire_texture",
        AssetRenderResourcesNode::<FireTexture>::new(true),
    );
    render_graph
        .add_node_edge("fire_texture", base::node::MAIN_PASS)
        .unwrap();

    render_graph.add_system_node(
        "fire_material",
        AssetRenderResourcesNode::<FireMaterial>::new(true),
    );
    render_graph
        .add_node_edge("fire_material", base::node::MAIN_PASS)
        .unwrap();

    commands.spawn(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn spawn_fire(
    mut commands: Commands,
    fire_pipeline: Res<FirePipeline>,
    fire_material: Res<FireMaterialHandle>,
    mut loading_texture: ResMut<LoadingTexture>,
    mut textures: ResMut<Assets<Texture>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut fire_textures: ResMut<Assets<FireTexture>>,
) {
    let (handle, texture) = match loading_texture.0.as_ref() {
        Some(handle) => {
            if let Some(texture) = textures.get_mut(handle) {
                (loading_texture.0.take().unwrap(), texture)
            } else {
                return;
            }
        }
        None => return,
    };

    // Create a new array texture asset from the loaded texture.
    let array_layers = 3;
    texture.reinterpret_stacked_2d_as_array(array_layers);
    let fire_texture = fire_textures.add(FireTexture { texture: handle });

    commands
        .spawn(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(5.0, 5.0),
                flip: true,
            })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                fire_pipeline.0.clone(),
            )]),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(fire_material.0.clone())
        .with(fire_texture);
}