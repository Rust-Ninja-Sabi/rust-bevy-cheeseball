use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_rapier3d::prelude::*;
use rand::Rng;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Resource)]
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

#[derive(Event)]
struct CreateEffectEvent(Vec3);

#[derive(Component)]
struct Timer{
    value:f32
}

#[derive(Component)]
struct ThirdPersonTarget;

#[derive(Component)]
struct ThirdPersonCamera{
    ideal_offset:Vec3,
    ideal_lookat:Vec3,
    current_lookat:Vec3,
    lookat_aviabel:bool,
    follow:f32
}
impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            ideal_offset: Vec3::new(0.0,2.0,6.0),
            ideal_lookat: Vec3::new(0.0,0.0,-4.0),
            current_lookat: Vec3::new(0.0,0.0,0.0),
            lookat_aviabel: false,
            follow: 1.2
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
        .insert_resource(Score::default())
        .add_event::<CreateEffectEvent>()
        //bevy itself
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("bevy_cheeseball {}", VERSION.unwrap_or("unknown")).to_string(),
                resolution: WindowResolution::new(800.0,  600.0),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        //.add_plugins(AtmospherePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // system once
        .add_systems(Startup,setup)
        // system frame
        .add_systems(Update,(input_user,
                             collision,
                             create_effect,
                             remove_effect,
                             scoreboard,
                             move_camera))
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
    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        Transform::from_xyz(0.0,1.0,0.0).looking_at(Vec3::new(0.,0.,-4.), Vec3::Y)
    ))
        .insert(ThirdPersonCamera{..Default::default()});

    // scoreboard
    commands.spawn((
        Text::new("Cheese:"),
        TextFont{
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 1.0)),
        TextLayout::new_with_justify(JustifyText::Center),
                   // Set the style of the Node itself.
       Node {
           position_type: PositionType::Absolute,
           top: Val::Px(5.0),
           left: Val::Px(5.0),
           ..default()
       }
    ))
        .insert(Cheesetext);

    commands.spawn((
        Text::new("Level:"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 1.0)),
        TextLayout::new_with_justify(JustifyText::Center),
                   // Set the style of the Node itself.
       Node {
           position_type: PositionType::Absolute,
           top: Val::Px(5.0),
           right: Val::Px(25.0),
           ..default()
       }
    )).insert(Leveltext);

    //light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 4.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        }
    ));
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
    });
    //walls
    let mut children_list:Vec<Entity> = Vec::new();
    let wall1 = commands
        .spawn((
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.4,0.4,12.0)))),
            MeshMaterial3d(materials.add( StandardMaterial{
                base_color: Color::srgb(0.5, 0.5, 0.5),
                double_sided: true,
                ..Default::default()
            })),
            Transform {
                translation: Vec3::new(-1.7, 0.2, 0.0),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            }
        ))
        .insert(Collider::cuboid(0.4/2.0, 0.4/2.0, 12.0/2.0))
        .id();
    children_list.push(wall1);
    let wall2 = commands.spawn((
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(0.4,0.4,12.0)))),
            MeshMaterial3d(materials.add( StandardMaterial{
                base_color: Color::srgb(0.5, 0.5, 0.5),
                double_sided: true,
                ..Default::default()
            })),
            Transform {
                translation: Vec3::new(1.7, 0.2, 0.0),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            }
    )).insert(Collider::cuboid(0.4/2.0, 0.4/2.0, 12.0/2.0))
        .id();

    children_list.push(wall2);
    let wall3 = commands
        .spawn((
            Mesh3d( meshes.add(Mesh::from(Cuboid::new(3.8,0.4,0.4)))),
            MeshMaterial3d( materials.add( StandardMaterial{
                base_color: Color::srgb(0.5, 0.5, 0.5),
                double_sided: true,
                ..Default::default()
            })),
            Transform {
                translation: Vec3::new(0.0, 0.2, -6.0),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            }
        ))
        .insert(Collider::cuboid(3.4/2.0, 0.4/2.0, 0.4/2.0))
        .id();
    children_list.push(wall3);
    //door
    let door1 = commands
        .spawn((
            Mesh3d(meshes.add(Mesh::from(Cuboid::new(2.0,0.4,0.4)))),
            MeshMaterial3d( materials.add( StandardMaterial{
                base_color: Color::srgb(0.0, 0.5, 0.0),
                double_sided: true,
                ..Default::default()
            })),
            Transform {
                translation: Vec3::new(0.0, 1.2, -5.6),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            }
        ))
        .insert(Collider::cuboid(2.0/2.0, 0.4/2.0, 0.4/2.0))
        .id();
    children_list.push(door1);
    let door2 = commands
        .spawn((
            Mesh3d( meshes.add(Mesh::from(Cuboid::new(0.4,1.2,0.4)))),
            MeshMaterial3d( materials.add( StandardMaterial{
                base_color: Color::srgb(0.0, 0.5, 0.0),
                double_sided: true,
                ..Default::default()
            })),
            Transform {
                translation: Vec3::new(-0.8, 0.6, -5.6),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            }
        ))
        .insert(Collider::cuboid(0.4/2.0, 1.2/2.0, 0.4/2.0))
        .id();
    children_list.push(door2);
    let door3 = commands
        .spawn((
            Mesh3d( meshes.add(Mesh::from(Cuboid::new(0.4,1.2,0.4)))),
            MeshMaterial3d( materials.add( StandardMaterial{
                base_color: Color::srgb(0.0, 0.5, 0.0),
                double_sided: true,
                ..Default::default()
            })),
            Transform {
                translation: Vec3::new(0.8, 0.6, -5.6),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            }
        ))
        .insert(Collider::cuboid(0.4/2.0, 1.2/2.0, 0.4/2.0))
        .id();
    children_list.push(door3);

    //platform
    commands
        .spawn((
            Mesh3d( meshes.add(Mesh::from(Cuboid::new(3.0,0.1,12.0)))),
            MeshMaterial3d( materials.add( StandardMaterial{
                base_color: Color::srgb(1.0, 0.8, 0.6),
                double_sided: true,
                ..Default::default()
            })),
            Transform {
                translation: Vec3::new(0.0, -2.0, -11.0),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            }
        ))
        .add_children (&children_list)
        .insert(RigidBody::Fixed)
        .insert(Sleeping::disabled())
        .insert(Collider::cuboid(3.0/2.0, 0.1/2.0, 12.0/2.0));

    //cheese
    let cheese_position = Vec3::new(0.0, -1.0, -9.0);

    commands.spawn((
                SceneRoot( asset_server.load("models/cheese.glb#Scene0")),
                Transform::from_translation(cheese_position)
    ))
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Collider::cylinder(0.15, 0.3))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Cheese{});
    //ball
    commands
    .spawn((
        Mesh3d( meshes.add(Mesh::from(Sphere{
            radius:0.5
        }))),
        MeshMaterial3d( materials.add( StandardMaterial{
            base_color: Color::srgb(0.0, 0.0, 1.0),
            ..Default::default()
        })),
        Transform {
            translation: Vec3::new(0.0, -1.0, -6.0),
            rotation: Quat::from_rotation_x(0.0),
            ..Default::default()
        }
    ))
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Collider::ball(0.5))
        .insert(ExternalForce {
            ..Default::default()
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ThirdPersonTarget{})
        .insert(Ball{});

}

const SPEED:f32= 1.0;

fn input_user(
    keyboard_input:Res<ButtonInput<KeyCode>>,
    mut query_forces: Query<&mut ExternalForce>,
){

    let x = if keyboard_input.pressed(KeyCode::ArrowLeft) {
        -SPEED
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        SPEED
    } else {
        0.0
    };

    let z = if keyboard_input.pressed(KeyCode::ArrowUp) {
        -SPEED
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
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
    mut cheese_query: Query<&mut Text, (With<Cheesetext>, Without<Leveltext>)>,
    mut level_query: Query<&mut Text, With<Leveltext>>,
) {
    let mut text = cheese_query.single_mut();
    text.0 = format!("Cheese: {}", score.cheese);

    let mut level_text = level_query.single_mut();
    level_text.0 = format!("Level: {}", score.level);
}

fn collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut score: ResMut<Score>,
    query_ball: Query<Entity, With<Ball>>,
    query_cheese: Query<(Entity, &Transform), With<Cheese>>,
    mut event_create_effect: EventWriter<CreateEffectEvent>,
    mut commands: Commands
){
    let entity_ball = query_ball.single();
    for e in collision_events.read(){
        //println!("{:?}",e);
        match e {
            CollisionEvent::Started(e1,e2,_) => {
                if e1 == &entity_ball || e2 == &entity_ball {
                    for (entity_cheese, cheese_transform) in query_cheese.iter(){
                        if e1 == &entity_cheese || e2 == &entity_cheese {
                            commands.entity(entity_cheese).despawn_recursive();
                            score.cheese += 1;
                            event_create_effect.send(CreateEffectEvent(Vec3::from(cheese_transform.translation)));
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_,_,_)=> {}
        }
    }
}

const EFFECT_SIZE:f32=0.1;
const EFFECT_TIME:f32=2.0;

fn create_effect(
    mut commands: Commands,
    mut event_create_effect: EventReader<CreateEffectEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{
    let mut rng = rand::thread_rng();
    for event in event_create_effect.read() {
        let pos = event.0;
        for x in -2..2 {
            for y in 0..2 {
            for z in -2..2 {
                commands
                    .spawn((
                        Mesh3d( meshes.add(Mesh::from(Cuboid::new(0.1, 0.1, 0.1)))),
                        MeshMaterial3d( materials.add(StandardMaterial {
                            metallic: 0.5,
                            emissive: Color::srgb(1.0, 0.5, 0.0).into(),
                            ..Default::default()
                        })),
                        Transform {
                            translation: Vec3::new(x as f32 * EFFECT_SIZE+pos.x,
                                                   y as f32 * EFFECT_SIZE+pos.y,
                                                   z as f32 * EFFECT_SIZE+pos.z),
                            rotation: Quat::from_rotation_x(0.0),
                            ..Default::default()
                        }
                    ))
                    .insert(RigidBody::Dynamic)
                    .insert(ExternalImpulse {
                        impulse: Vec3::new(rng.gen_range(-0.01..0.01),
                                           0.01,
                                           rng.gen_range(-0.01..0.01)),
                        torque_impulse: Vec3::new(0.0, 0.0, 0.0),
                    })
                    .insert(Timer{value:EFFECT_TIME})
                    .insert(Sleeping::disabled())
                    .insert(Collider::cuboid(0.1 / 2.0, 0.1 / 2.0, 0.1 / 2.0));
                 }
            }
        }
    }
}

fn remove_effect(
    mut commands: Commands,
    time:Res<Time>,
    mut query: Query<(Entity, &mut Timer)>
)
{
    for (entity, mut timer) in query.iter_mut() {
        timer.value -= time.delta_secs();
        if timer.value <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}


fn move_camera(
    time:Res<Time>,
    mut query_camera: Query<(&mut Transform, &mut ThirdPersonCamera), Without<ThirdPersonTarget>>,
    query_target: Query<&Transform, With<ThirdPersonTarget>>
){
    let (mut camera_transform, mut thridperson) = query_camera.single_mut();
    let target_transform = query_target.single();
    let t = thridperson.follow * time.delta_secs();

    let mut offset = thridperson.ideal_offset.clone();
    offset += target_transform.translation;
    offset = camera_transform.translation.lerp(offset,t);

    let mut lookat = thridperson.ideal_lookat.clone();
    lookat+= target_transform.translation;
    if thridperson.lookat_aviabel {
        lookat = thridperson.current_lookat.lerp(lookat, t);
    } else{
        thridperson.lookat_aviabel = true;
    }

    thridperson.current_lookat = lookat;

    let transform = Transform::from_translation(offset).looking_at(lookat, Vec3::Y);
    camera_transform.translation = transform.translation;
    camera_transform.rotation = transform.rotation
}