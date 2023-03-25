use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Game".to_string(),
            resizable: true,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    //cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 100.0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    //light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform:Transform::from_xyz(7.0, 5.0, 4.0),
        ..default()
    });
}
