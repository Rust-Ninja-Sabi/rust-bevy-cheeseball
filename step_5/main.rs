use bevy::prelude::*;
use bevy_atmosphere::*;
use bevy_rapier3d::prelude::*;

struct Score {
    cheese:i32,
    level:i32
}
impl Default for Score{
    fn default() -> Self {
        Self {
            cheese:0,
            level:1,
        }
    }
}

#[derive(Component)]
struct Cheesetext;

#[derive(Component)]
struct Leveltext;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Cheese;


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
        .insert_resource(Score::default())
        //bevy itself
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_atmosphere::AtmospherePlugin {
            dynamic: false,
            sky_radius: 11.0,
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        // system once
        .add_startup_system(setup)
        // system frame
        .add_system(input_user)
        .add_system(collision)
        .add_system(scoreboard)
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

    commands.spawn_bundle(UiCameraBundle::default());
    // scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Cheese:",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            },
            Default::default(),
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Cheesetext);

    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "Level:",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.5, 0.5, 1.0),
            },
            Default::default(),
        ),
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                right: Val::Px(25.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Leveltext);

    //light
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
        })
        .insert(Collider::cuboid(0.4/2.0, 0.4/2.0, 12.0/2.0))
        .id();
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
        })
        .insert(Collider::cuboid(0.4/2.0, 0.4/2.0, 12.0/2.0))
        .id();
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
        })
        .insert(Collider::cuboid(3.4/2.0, 0.4/2.0, 0.4/2.0))
        .id();
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
        })
        .insert(Collider::cuboid(2.0/2.0, 0.4/2.0, 0.4/2.0))
        .id();
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
        })
        .insert(Collider::cuboid(0.4/2.0, 1.2/2.0, 0.4/2.0))
        .id();
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
        })
        .insert(Collider::cuboid(0.4/2.0, 1.2/2.0, 0.4/2.0))
        .id();
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
        .push_children(&children_list)
        .insert(RigidBody::Fixed)
        .insert(Sleeping::disabled())
        .insert(Collider::cuboid(3.0/2.0, 0.1/2.0, 12.0/2.0));

    //cheese
    let cheese_position = Vec3::new(0.0, -1.0, -9.0);

    commands.spawn_bundle((
        Transform::from_translation(cheese_position),
        GlobalTransform::identity(),
    ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/cheese.glb#Scene0"));
        })
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Collider::cylinder(0.15, 0.3))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Cheese{});
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
            translation: Vec3::new(0.0, -1.0, -6.0),
            rotation: Quat::from_rotation_x(0.0),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Collider::ball(0.5))
        .insert(ExternalForce {
            ..Default::default()
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Ball{});

}

const SPEED:f32= 1.0;

fn input_user(
    keyboard_input:Res<Input<KeyCode>>,
    mut query_forces: Query<&mut ExternalForce>,
){

    let x = if keyboard_input.pressed(KeyCode::Left) {
        -SPEED
    } else if keyboard_input.pressed(KeyCode::Right) {
        SPEED
    } else {
        0.0
    };

    let z = if keyboard_input.pressed(KeyCode::Up) {
        -SPEED
    } else if keyboard_input.pressed(KeyCode::Down) {
        SPEED
    } else {
        0.0
    };

    if x != 0.0 || z != 0.0 {
        for mut ext_force in query_forces.iter_mut() {
            ext_force.force = Vec3::new(x,0.0, z);
        }
    }
}

fn scoreboard(
    score: Res<Score>,
    mut cheese_query: Query<(&mut Text, With<Cheesetext>, Without<Leveltext>)>,
    mut level_query: Query<&mut Text, With<Leveltext>>,
) {
    let (mut text,_,_) = cheese_query.single_mut();
    text.sections[0].value = format!("Cheese: {}", score.cheese);

    let mut level_text = level_query.single_mut();
    level_text.sections[0].value = format!("Level: {}", score.level);
}

fn collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut score: ResMut<Score>,
    query_ball: Query<Entity, With<Ball>>,
    query_cheese: Query<Entity, With<Cheese>>,
    mut commands: Commands
){
    let entity_ball = query_ball.single();
    for e in collision_events.iter(){
        //println!("{:?}",e);
        match e {
            CollisionEvent::Started(e1,e2,_) => {
                if e1 == &entity_ball || e2 == &entity_ball {
                    for entity_cheese in query_cheese.iter(){
                        if e1 == &entity_cheese || e2 == &entity_cheese {
                            commands.entity(entity_cheese).despawn_recursive();
                            score.cheese += 1;
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_,_,_)=> {}
        }
    }
}