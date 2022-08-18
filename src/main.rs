use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{self, AsBindGroup},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<GlowyMaterial>::default())
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup)
        .add_startup_system(setup_lights)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut glow_materials: ResMut<Assets<GlowyMaterial>>,
) {
    let material = glow_materials.add(GlowyMaterial {});

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 1.0,
            ..default()
        })),
        // material: materials.add(StandardMaterial {
        //     base_color: Color::GREEN,
        //     ..default()
        // }),
        material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
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

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "9d4bff0a-1ee6-11ed-861d-0242ac120002"]
pub struct GlowyMaterial {}

impl Material for GlowyMaterial {
    fn fragment_shader() -> render_resource::ShaderRef {
        "shaders/glowy.wgsl".into()
    }
}
