use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(move_objects)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..default()
    });
}
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Leg;

pub fn setup_physics(mut commands: Commands, mut reapier_config: ResMut<RapierConfiguration>) {
    reapier_config.gravity = Vec2::new(0.0, -200.0);
    /*
     * Ground
     */
    let ground_size = 5000.0;
    let ground_height = 10.0;

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            0.0 * -ground_height,
            0.0,
        )))
        .insert(Collider::cuboid(ground_size, ground_height));

    let child_entity = commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(0.0, 30.0, 0.0),
            ..default()
        })
        .insert(Name::new("sussy"))
        .insert(Velocity {
            angvel: 0.01,
            linvel: Vec2::new(1.0, 1.0),
        })
        .insert(Collider::cuboid(10.0, 30.0))
        .insert(RigidBody::Dynamic)
        .insert(Leg)
        .id();
    let child_entity2 = commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(0.0, 30.0, 0.0),
            ..default()
        })
        .insert(Name::new("sussy"))
        .insert(Velocity {
            angvel: 0.01,
            linvel: Vec2::new(1.0, 1.0),
        })
        .insert(Collider::cuboid(10.0, 30.0))
        .insert(RigidBody::Dynamic)
        .insert(Leg)
        .id();
    let child_entity3 = commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(0.0, 200.0, 0.0),
            ..default()
        })
        .insert(Name::new("sussy"))
        .insert(Velocity {
            angvel: 0.01,
            linvel: Vec2::new(1.0, 1.0),
        })
        .insert(Collider::cuboid(10.0, 30.0))
        .insert(RigidBody::Dynamic)
        .insert(Leg)
        .id();

    let parent_entity = commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(0.0, 150.0, 0.0),
            ..default()
        })
        .insert(Name::new("sussy_parent"))
        .insert(Velocity {
            angvel: 0.01,
            linvel: Vec2::new(1.0, 1.0),
        })
        .insert(Collider::cuboid(60.0, 30.0))
        .insert(RigidBody::Dynamic)
        .id();

    let joint = RevoluteJointBuilder::new().local_anchor2(Vec2::new(-40.0, -80.0));
    let joint2 = RevoluteJointBuilder::new().local_anchor2(Vec2::new(40.0, -80.0));
    let joint3 = RevoluteJointBuilder::new().local_anchor2(Vec2::new(0.0, 120.0));

    commands.entity(parent_entity).with_children(|cmd| {
        cmd.spawn().insert(ImpulseJoint::new(child_entity, joint));
        cmd.spawn().insert(ImpulseJoint::new(child_entity2, joint2));
        cmd.spawn().insert(ImpulseJoint::new(child_entity3, joint3));
    });

    /*
     * Create the cubes
     */
    // let num = 8;
    // let rad = 10.0;

    // let shift = rad * 2.0 + rad;
    // let centerx = shift * (num / 2) as f32;
    // let centery = shift / 2.0;

    // let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

    // for j in 0usize..20 {
    //     for i in 0..num {
    //         let x = i as f32 * shift - centerx + offset;
    //         let y = j as f32 * shift + centery + 30.0;

    //         commands
    //             .spawn_bundle(TransformBundle::from(Transform::from_xyz(x, y, 0.0)))
    //             .insert(RigidBody::Dynamic)
    //             .insert(Velocity::zero())
    //             .insert(Collider::cuboid(rad, rad));
    //     }

    //     offset -= 0.05 * rad * (num as f32 - 1.0);
    // }
}
fn move_objects(mut objects: Query<&mut Velocity, With<Leg>>, keys: Res<Input<KeyCode>>) {
    for mut object in &mut objects {
        if keys.pressed(KeyCode::D) {
            object.angvel -= 1.0;
        }
        if keys.pressed(KeyCode::A) {
            object.angvel += 1.0;
        }
        if keys.pressed(KeyCode::W) {
            object.linvel.y += 10.0;
        }
        if object.angvel > 100.0 {
            object.angvel = 100.0;
        }
        if object.angvel < -100.0 {
            object.angvel = -100.0;
        }
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Player;
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut rb_vels) in player_info.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vec2::ZERO {
            move_delta /= move_delta.length();
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta;
    }
}
