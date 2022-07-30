use bevy::prelude::*;
use bevy_atmosphere::*;

use std::f32::consts::{FRAC_PI_2,PI};

fn main() {
    App::new()
        //add config resources
        .insert_resource(Msaa {samples: 4})
        .insert_resource(WindowDescriptor{
            title: "cheeseball".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(bevy_atmosphere::AtmosphereMat::default())
       // .insert_resource(Score::default())
       // .insert_resource(CountLaser{value:0})
        //bevy itself
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_atmosphere::AtmospherePlugin {
            dynamic: false,
            sky_radius: 11.0,
        })
        // system once
        .add_startup_system(setup)
        // system frame
        //.add_system(input_ship)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //camera
    //commands.spawn_bundle(PerspectiveCameraBundle{
    //    ..Default::default()
    //});
    commands.spawn_bundle(PerspectiveCameraBundle{
        transform: Transform::from_xyz(0.0,1.0,0.0).looking_at(Vec3::new(0.,0.,-4.), Vec3::Y),
        ..Default::default()
    });
    //light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 4.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
    });
    //walls
    let mut children_list:Vec<Entity> = Vec::new();
    let wall1 = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.4,0.4,12.0))),
            material: materials.add( StandardMaterial{
                base_color: Color::rgb(0.5, 0.5, 0.5),
                double_sided: true,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3::new(-1.7, 0.2, 0.0),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            },
            ..Default::default()
        }).id();
    children_list.push(wall1);
    let wall2 = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.4,0.4,12.0))),
            material: materials.add( StandardMaterial{
                base_color: Color::rgb(0.5, 0.5, 0.5),
                double_sided: true,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3::new(1.7, 0.2, 0.0),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            },
            ..Default::default()
        }).id();
    children_list.push(wall2);
    let wall3 = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(3.4,0.4,0.4))),
            material: materials.add( StandardMaterial{
                base_color: Color::rgb(0.5, 0.5, 0.5),
                double_sided: true,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.2, -6.0),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            },
            ..Default::default()
        }).id();
    children_list.push(wall3);
    //door
    let door1 = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(2.0,0.4,0.4))),
            material: materials.add( StandardMaterial{
                base_color: Color::rgb(0.0, 0.5, 0.0),
                double_sided: true,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 1.2, -6.0),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            },
            ..Default::default()
        }).id();
    children_list.push(door1);
    let door2 = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.4,1.2,0.4))),
            material: materials.add( StandardMaterial{
                base_color: Color::rgb(0.0, 0.5, 0.0),
                double_sided: true,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3::new(-0.8, 0.6, -6.0),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            },
            ..Default::default()
        }).id();
    children_list.push(door2);
    let door3 = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.4,1.2,0.4))),
            material: materials.add( StandardMaterial{
                base_color: Color::rgb(0.0, 0.5, 0.0),
                double_sided: true,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3::new(0.8, 0.6, -6.0),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            },
            ..Default::default()
        }).id();
    children_list.push(door3);

    //platform
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(3.0,0.1,12.0))),
            material: materials.add( StandardMaterial{
                base_color: Color::rgb(1.0, 0.8, 0.6),
                double_sided: true,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, -2.0, -11.0),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .push_children(&children_list);
    //cheese
    let cheese_position = Vec3::new(0.0, -1.0, -8.0);

    commands.spawn_bundle((
        Transform::from_translation(cheese_position),
        GlobalTransform::identity(),
    ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/cheese.glb#Scene0"));
        });
    //ball
    commands
    .spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere{
            radius:0.5,
            sectors:32,
            stacks:32
        })),
        material: materials.add( StandardMaterial{
            base_color: Color::rgb(0.0, 0.0, 1.0),
            ..Default::default()
        }),
        transform: Transform {
            translation: Vec3::new(0.0, -1.0, -5.0),
            rotation: Quat::from_rotation_x(0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}