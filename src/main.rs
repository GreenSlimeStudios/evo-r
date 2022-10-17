use bevy::{ecs::system::EntityCommands, prelude::*};
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
        // .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<&CustomFilterTag>::pixels_per_meter(
            100.0,
        ))
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

    commands.insert_resource(PhysicsHooksWithQueryResource(Box::new(
        SameUserDataFilter {},
    )));

    let mut parts: Vec<(Entity, RevoluteJointBuilder)> = Vec::new();
    let mut offsets: Vec<(Entity, RevoluteJointBuilder)> = Vec::new();

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            0.0 * -ground_height,
            0.0,
        )))
        .insert(Collider::cuboid(ground_size, ground_height))
        // .insert(ActiveHooks::FILTER_CONTACT_PAIRS);
        .insert(CustomFilterTag::GroupB);

    offsets.push((
        commands
            .spawn_bundle(TransformBundle {
                local: Transform::from_xyz(-40.0, 130.0, 0.0),
                ..default()
            })
            .insert(Name::new("Transform_placeholder"))
            .insert(Collider::cuboid(5.0, 5.0))
            .insert(RigidBody::Dynamic)
            .insert(Leg)
            .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
            .insert(CustomFilterTag::GroupA)
            .id(),
        RevoluteJointBuilder::new().local_anchor2(Vec2::new(-50.0, -15.0)),
    ));
    offsets.push((
        commands
            .spawn_bundle(TransformBundle {
                local: Transform::from_xyz(-40.0, 130.0, 0.0),
                ..default()
            })
            .insert(Name::new("Transform_placeholder"))
            .insert(Collider::cuboid(5.0, 5.0))
            .insert(RigidBody::Dynamic)
            .insert(Leg)
            .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
            .insert(CustomFilterTag::GroupA)
            .id(),
        RevoluteJointBuilder::new().local_anchor2(Vec2::new(50.0, -15.0)),
    ));
    offsets.push((
        commands
            .spawn_bundle(TransformBundle {
                local: Transform::from_xyz(-40.0, 130.0, 0.0),
                ..default()
            })
            .insert(Name::new("Transform_placeholder"))
            .insert(Collider::cuboid(5.0, 5.0))
            .insert(RigidBody::Dynamic)
            .insert(Leg)
            .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
            .insert(CustomFilterTag::GroupA)
            .id(),
        RevoluteJointBuilder::new().local_anchor2(Vec2::new(0.0, -15.0)),
    ));

    parts.push((
        commands
            .spawn_bundle(TransformBundle {
                local: Transform::from_xyz(-40.0, 130.0, 0.0),
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
            .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
            .insert(CustomFilterTag::GroupA)
            .id(),
        RevoluteJointBuilder::new().local_anchor1(Vec2::new(0.0, -50.0)),
    ));

    parts.push((
        commands
            .spawn_bundle(TransformBundle {
                local: Transform::from_xyz(40.0, 130.0, 0.0),
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
            .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
            .insert(CustomFilterTag::GroupA)
            .id(),
        RevoluteJointBuilder::new().local_anchor1(Vec2::new(0.0, -50.0)),
    ));
    parts.push((
        commands
            .spawn_bundle(TransformBundle {
                local: Transform::from_xyz(40.0, 130.0, 0.0),
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
            .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
            .insert(CustomFilterTag::GroupA)
            .id(),
        RevoluteJointBuilder::new().local_anchor1(Vec2::new(0.0, -50.0)),
    ));
    // let child_entity3 = commands
    //     .spawn_bundle(TransformBundle {
    //         local: Transform::from_xyz(0.0, 200.0, 0.0),
    //         ..default()
    //     })
    //     .insert(Name::new("sussy"))
    //     .insert(Velocity {
    //         angvel: 0.01,
    //         linvel: Vec2::new(1.0, 1.0),
    //     })
    //     .insert(Collider::cuboid(10.0, 30.0))
    //     .insert(RigidBody::Dynamic)
    //     .insert(Leg)
    //     .id();

    let parent_entity = commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(0.0, 250.0, 0.0),
            ..default()
        })
        .insert(Name::new("sussy_parent"))
        .insert(Velocity {
            angvel: 0.01,
            linvel: Vec2::new(1.0, 1.0),
        })
        .insert(Collider::cuboid(60.0, 30.0))
        .insert(RigidBody::Dynamic)
        .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
        .insert(CustomFilterTag::GroupA)
        .id();

    let joint = RevoluteJointBuilder::new().local_anchor2(Vec2::new(-40.0, -80.0));
    let joint2 = RevoluteJointBuilder::new().local_anchor2(Vec2::new(40.0, -80.0));
    let joint3 = RevoluteJointBuilder::new().local_anchor2(Vec2::new(0.0, 120.0));

    for i in 0..offsets.len() {
        commands.entity(parent_entity).with_children(|cmd| {
            cmd.spawn()
                .insert(ImpulseJoint::new(offsets[i].0, offsets[i].1));
        });
    }
    for i in 0..offsets.len() {
        commands.entity(offsets[i].0).with_children(|cmd| {
            cmd.spawn()
                .insert(ImpulseJoint::new(parts[i].0, parts[i].1));
        });
    }
}
fn move_objects(mut objects: Query<&mut Velocity, With<Leg>>, keys: Res<Input<KeyCode>>) {
    for mut object in &mut objects {
        if keys.pressed(KeyCode::D) {
            object.angvel -= 2.0;
        }
        if keys.pressed(KeyCode::A) {
            object.angvel += 2.0;
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

pub fn to_vec2(vec3: &Vec3) -> Vec2 {
    Vec2::new(vec3.x, vec3.y)
}

#[derive(PartialEq, Eq, Clone, Copy, Component)]
enum CustomFilterTag {
    GroupA,
    GroupB,
}

// A custom filter that allows contacts only between rigid-bodies with the
// same user_data value.
// Note that using collision groups would be a more efficient way of doing
// this, but we use custom filters instead for demonstration purpose.
struct SameUserDataFilter;
impl<'a> PhysicsHooksWithQuery<&'a CustomFilterTag> for SameUserDataFilter {
    fn filter_contact_pair(
        &self,
        context: PairFilterContextView,
        tags: &Query<&'a CustomFilterTag>,
    ) -> Option<SolverFlags> {
        if tags.get(context.collider1()).ok().copied()
            != tags.get(context.collider2()).ok().copied()
        {
            Some(SolverFlags::COMPUTE_IMPULSES)
        } else {
            None
        }
    }
}
