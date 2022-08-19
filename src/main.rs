use bevy::{
    asset::AssetServerSettings,
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{self, AsBindGroup},
};

use bevy::window::{PresentMode, WindowMode};
use bevy_inspector_egui::{WorldInspectorPlugin, Inspectable, RegisterInspectable};

fn main() {
    App::new()
        .insert_resource(AssetServerSettings{
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(WindowDescriptor{

            width: 800.0,
            height: 600.0,
            position: WindowPosition::At(Vec2::new(3500., 1500.)),
            // position: WindowPosition::Centered(MonitorSelection::Number(2)),
            resize_constraints: Default::default(),
            scale_factor_override: None,
            title: "Shaders!".to_string(),
            present_mode: PresentMode::AutoVsync,
            resizable: false,
            decorations: false,
            cursor_visible: true,
            cursor_locked: false,
            mode: WindowMode::Windowed,
            transparent: false,
            canvas: None,
            fit_canvas_to_parent: false
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(MaterialPlugin::<GlowyMaterial>::default())
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .register_inspectable::<GlowyMaterial>()
        .register_inspectable::<Handle<GlowyMaterial>>()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(sphere_setup)
        .add_startup_system(setup_lights)
        .run();
}

fn sphere_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut glow_materials: ResMut<Assets<GlowyMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>
) {

    commands.spawn()
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.0,
                ..default()
            })),
            // material: materials.add(StandardMaterial {
            //     base_color: Color::GREEN,
            //         ..default()
            // }),
            material: glow_materials.add(GlowyMaterial{
                image: Some(assets.load("textures/splatter1.png")),
                env_texture: Some(assets.load("textures/mountain_midday.hdr")),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
            })
        .insert(Name::new("Sphere"));

}

fn setup_lights(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid, Inspectable,Reflect)]
#[uuid = "9d4bff0a-1ee6-11ed-861d-0242ac120002"]
pub struct GlowyMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    image: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    env_texture: Option<Handle<Image>>
}

impl Default for GlowyMaterial {
    fn default() -> Self {
        Self {
            color: Color::MIDNIGHT_BLUE,
            image: default(),
            env_texture: default()
        }
    }
}
impl Material for GlowyMaterial {
    fn fragment_shader() -> render_resource::ShaderRef {
        "shaders/glowy.wgsl".into()
    }
}
