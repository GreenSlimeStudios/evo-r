use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
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
        .add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(move_objects)
        .add_system(move_camera_system)
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

pub struct PartConstructorData {
    // joint_entity: Entity,
    // joint_builder: RevoluteJointBuilder,
    parts: Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>,
}

pub fn setup_physics(mut commands: Commands, mut reapier_config: ResMut<RapierConfiguration>) {
    reapier_config.gravity = Vec2::new(0.0, -200.0);
    /*
     * Ground
     */
    let ground_size = 50000.0;
    let ground_height = 10.0;

    commands.insert_resource(PhysicsHooksWithQueryResource(Box::new(
        SameUserDataFilter {},
    )));
    let entity_pos: Vec3 = Vec3::new(0.0, 200.0, 0.0);

    let parent_entity = commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(entity_pos.x, entity_pos.y, entity_pos.z),
            ..default()
        })
        .insert(Name::new("sussy_parent"))
        .insert(Velocity {
            angvel: 0.01,
            linvel: Vec2::new(1.0, 1.0),
        })
        .insert(Collider::cuboid(60.0, 20.0))
        .insert(RigidBody::Dynamic)
        .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
        .insert(CustomFilterTag::GroupA)
        .id();

    let mut parts: Vec<PartConstructorData> = Vec::new();

    parts.push(PartConstructorData {
        parts: vec![
            create_part(
                PartData {
                    joint_parrent_offset: Vec2::new(0.0, 60.0),
                    joint_offset: Vec2::new(0.0, 30.0),
                    transform: entity_pos,
                    part_size: Vec2::new(10.0, 30.0),
                },
                &mut commands,
            ),
            create_part(
                PartData {
                    joint_parrent_offset: Vec2::new(0.0, 40.0),
                    joint_offset: Vec2::new(0.0, -50.0),
                    transform: entity_pos,
                    part_size: Vec2::new(10.0, 60.0),
                },
                &mut commands,
            ),
            create_part(
                PartData {
                    joint_parrent_offset: Vec2::new(40.0, 10.0),
                    joint_offset: Vec2::new(0.0, -30.0),
                    transform: entity_pos,
                    part_size: Vec2::new(10.0, 40.0),
                },
                &mut commands,
            ),
        ],
    });
    parts.push(PartConstructorData {
        parts: vec![
            // create_part(
            //     PartData {
            //         joint_parrent_offset: Vec2::new(0.0, -70.0),
            //         joint_offset: Vec2::new(0.0, 15.0),
            //         transform: entity_pos,
            //         part_size: Vec2::new(10.0, 30.0),
            //     },
            //     &mut commands,
            // ),
            create_part(
                PartData {
                    joint_parrent_offset: Vec2::new(0.0, 40.0),
                    joint_offset: Vec2::new(0.0, -50.0),
                    transform: entity_pos,
                    part_size: Vec2::new(10.0, 60.0),
                },
                &mut commands,
            ),
            create_part(
                PartData {
                    joint_parrent_offset: Vec2::new(-40.0, -10.0),
                    joint_offset: Vec2::new(0.0, -30.0),
                    transform: entity_pos,
                    part_size: Vec2::new(10.0, 40.0),
                },
                &mut commands,
            ),
        ],
    });

    for i in 0..parts.len() {
        for j in 0..parts[i].parts.len() {
            commands.entity(parts[i].parts[j].0).with_children(|cmd| {
                cmd.spawn()
                    .insert(ImpulseJoint::new(parts[i].parts[j].2, parts[i].parts[j].3));
            });

            commands
                .entity(if j + 1 != parts[i].parts.len() {
                    parts[i].parts[j + 1].2
                } else {
                    parent_entity
                })
                .with_children(|cmd| {
                    cmd.spawn()
                        .insert(ImpulseJoint::new(parts[i].parts[j].0, parts[i].parts[j].1));
                });
        }
    }

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            0.0 * -ground_height,
            0.0,
        )))
        .insert(Collider::cuboid(ground_size, ground_height))
        // .insert(ActiveHooks::FILTER_CONTACT_PAIRS);
        .insert(CustomFilterTag::GroupB);

    for i in 0..final_parts.len() {
        connect_to_parrent(
            parent_entity,
            final_parts[i].0,
            final_parts[i].1,
            &mut commands,
        )
        // commands.entity(parent_entity).with_children(|cmd| {
        //     cmd.spawn()
        //         .insert(ImpulseJoint::new(final_parts[i].0, final_parts[i].1));
        // });
    }
}
fn move_objects(mut objects: Query<&mut Velocity, With<Leg>>, keys: Res<Input<KeyCode>>) {
    for mut object in &mut objects {
        if keys.pressed(KeyCode::D) {
            object.angvel -= 1.5;
        }
        if keys.pressed(KeyCode::A) {
            object.angvel += 1.5;
        }
        if keys.pressed(KeyCode::W) {
            object.linvel.y += 10.0;
        }
        if object.angvel > 50.0 {
            object.angvel = 50.0;
        }
        if object.angvel < -50.0 {
            object.angvel = -50.0;
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
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Body;

fn move_camera_system(
    body_transforms: Query<&GlobalTransform, With<Body>>,
    mut cameras: Query<&mut Transform, With<Camera2d>>,
) {
    for mut camera_transform in &mut cameras {
        for body_transform in &body_transforms {
            camera_transform.translation.x = body_transform.translation().x;
        }
    }
}

pub struct PartData {
    joint_parrent_offset: Vec2,
    joint_offset: Vec2,
    transform: Vec3,
    part_size: Vec2,
}

fn create_entity(part_data: PartData, commands: &mut Commands) -> (Entity, RevoluteJointBuilder) {
    let mut entity: Entity = commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(
                part_data.transform.x + part_data.joint_parrent_offset.x,
                part_data.transform.y + part_data.joint_parrent_offset.y,
                part_data.transform.z,
            ),
            ..default()
        })
        .insert(Name::new("joint"))
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(RigidBody::Dynamic)
        // .insert(Leg)
        .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
        .insert(CustomFilterTag::GroupA)
        .id();

    let joint_to_parrent =
        RevoluteJointBuilder::new().local_anchor2(part_data.joint_parrent_offset);
    let joint_to_joint = RevoluteJointBuilder::new().local_anchor1(part_data.joint_offset);

    let mut part_entity = commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(
                part_data.transform.x + part_data.joint_parrent_offset.x,
                // + part_data.part_size.x / 2.0,
                part_data.transform.y + part_data.joint_parrent_offset.y,
                // + part_data.part_size.y / 2.0,
                part_data.transform.z,
            ),
            ..default()
        })
        .insert(Name::new("sussy"))
        .insert(Velocity {
            angvel: 0.01,
            linvel: Vec2::new(1.0, 1.0),
        })
        .insert(Collider::cuboid(
            part_data.part_size.x,
            part_data.part_size.y,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Leg)
        .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
        .insert(CustomFilterTag::GroupA)
        .id();

    commands.entity(entity).with_children(|cmd| {
        cmd.spawn()
            .insert(ImpulseJoint::new(part_entity, joint_to_joint));
    });

    (entity, joint_to_parrent)
}
fn create_part(
    part_data: PartData,
    commands: &mut Commands,
) -> (Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder) {
    let entity: Entity = commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(
                part_data.transform.x + part_data.joint_parrent_offset.x,
                part_data.transform.y + part_data.joint_parrent_offset.y,
                part_data.transform.z,
            ),
            ..default()
        })
        .insert(Name::new("joint"))
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(RigidBody::Dynamic)
        // .insert(Leg)
        .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
        .insert(CustomFilterTag::GroupA)
        .id();

    let joint_to_parrent =
        RevoluteJointBuilder::new().local_anchor2(part_data.joint_parrent_offset);
    let joint_to_joint = RevoluteJointBuilder::new().local_anchor1(part_data.joint_offset);

    let part_entity = commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(
                part_data.transform.x + part_data.joint_parrent_offset.x,
                // + part_data.part_size.x / 2.0,
                part_data.transform.y + part_data.joint_parrent_offset.y,
                // + part_data.part_size.y / 2.0,
                part_data.transform.z,
            ),
            ..default()
        })
        .insert(Name::new("sussy"))
        .insert(Velocity {
            angvel: 0.01,
            linvel: Vec2::new(1.0, 1.0),
        })
        .insert(Collider::cuboid(
            part_data.part_size.x,
            part_data.part_size.y,
        ))
        .insert(RigidBody::Dynamic)
        .insert(Leg)
        .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
        .insert(CustomFilterTag::GroupA)
        .id();

    // commands.entity(entity).with_children(|cmd| {
    //     cmd.spawn()
    //         .insert(ImpulseJoint::new(part_entity, joint_to_joint));
    // });

    (entity, joint_to_parrent, part_entity, joint_to_joint)
}

fn connect_to_parrent(
    parent_entity: Entity,
    child_entity: Entity,
    joint: RevoluteJointBuilder,
    commands: &mut Commands,
) {
    commands.entity(parent_entity).with_children(|cmd| {
        cmd.spawn().insert(ImpulseJoint::new(child_entity, joint));
    });
}
