use bevy::prelude::*;
use core::f32::consts::PI;
use bevy_inspector_egui::WorldInspectorPlugin;
pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

#[derive(Component)]
pub struct Miner {
    spawner_timer: Timer
}#[derive(Component)]

pub struct Tower {
    spawner_timer: Timer
}

#[derive(Component,Reflect,Default)]
#[reflect(Component)]
pub struct Lifetime{
    timer:Timer
}

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
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_system(miner_spawner)
        .add_system(resource_despawn)
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
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Ground"));
    
        commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 50.0).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            spawner_timer: Timer::from_seconds(1.0, true),
        })
        .insert(Name::new("Tower"));
    /*commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 100.0).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Miner {
            spawner_timer: Timer::from_seconds(1.0, true),
        })
        .insert(Name::new("Miner"));*/
    /*commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Torus {
                radius: 1.0,
                ring_radius: 0.2,
                ..default()
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 100.0).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Name::new("Donut"));*/
    //light
    commands
        .spawn_bundle(PointLightBundle {
            point_light: PointLight {
                intensity: 1000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(7.0, 5.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<&mut Tower>,
    time:Res<Time>
) {
    for mut tower in &mut towers {
        tower.spawner_timer.tick(time.delta());
        if tower.spawner_timer.just_finished() {
            let spawn_transform =
                Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                material: materials.add(Color::rgb(0.8, 0.4, 0.4).into()),
                transform: spawn_transform,
                ..default()
            })
            .insert(Lifetime{
                timer: Timer::from_seconds(5.0,false)
            })
            .insert(Name::new("Bullet"));
        }
    }
}
/*fn miner_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut miners: Query<&mut Miner>,
    time:Res<Time>
) {
    for mut miner in &mut miners {
        miner.spawner_timer.tick(time.delta());
        if miner.spawner_timer.just_finished() {
            let spawn_transform =
                Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));
            commands.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                material: materials.add(Color::rgb(0.8, 0.4, 0.4).into()),
                transform: spawn_transform,
                ..default()
            })
            .insert(Lifetime{
                timer: Timer::from_seconds(5.0,false)
            })
            .insert(Name::new("Stone"));
        }
    }
}*/

fn resource_despawn(
    mut commands: Commands,
    mut resource: Query<(Entity,&mut Lifetime)>,
    time:Res<Time>
){
    for(entity,mut lifetime) in &mut resource{
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished(){
            commands.entity(entity).despawn_recursive();
        }
    }
}
