use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
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
        .add_system(respawn_entity_system)
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

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Joint;

pub struct PartConstructorData {
    // joint_entity: Entity,
    // joint_builder: RevoluteJointBuilder,
    pub parts: Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>,
}

pub fn setup_physics(mut commands: Commands, mut reapier_config: ResMut<RapierConfiguration>) {
    reapier_config.gravity = Vec2::new(0.0, 0.0);

    /*
     * Ground
     */
    let ground_size = 50000.0;
    let ground_height = 10.0;

    commands.insert_resource(PhysicsHooksWithQueryResource(Box::new(
        SameUserDataFilter {},
    )));
    let entity_pos: Vec3 = Vec3::new(0.0, 300.0, 0.0);

    let parent_data: Parent = Parent {
        position: entity_pos,
        size: Vec2::new(60.0, 60.0),
    };

    let parent_entity = commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(entity_pos.x, entity_pos.y, entity_pos.z),
            ..default()
        })
        .insert(Name::new("sussy_parent"))
        .insert(Velocity::zero())
        .insert(Collider::cuboid(parent_data.size.x, parent_data.size.y))
        .insert(RigidBody::Dynamic)
        .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
        .insert(CustomFilterTag::GroupA)
        .insert(Parent {
            size: parent_data.size,
            position: parent_data.position,
        })
        .id();

    let mut part_datas: Vec<Vec<PartData>> = Vec::new();
    let mut parts: Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>> =
        Vec::new();

    part_datas.push(Vec::new());
    part_datas[0].push(PartData {
        joint_parrent_offset: Vec2::new(40.0, 10.0),
        joint_offset: Vec2::new(0.0, -40.0), // y 10 lower than in part size y
        transform: Vec3::new(entity_pos.x, entity_pos.y, 0.0),
        part_size: Vec2::new(10.0, 40.0), // y is the same as in the child entity joint offset y
    });
    part_datas[0].push(PartData {
        joint_parrent_offset: Vec2::new(0.0, 40.0), // y is the same as in the parent entity part size y
        joint_offset: Vec2::new(0.0, -60.0),        // y 10 lower than in part size y
        transform: Vec3::new(entity_pos.x + 40.0, entity_pos.y + 10.0 + 40.0, 0.0),
        part_size: Vec2::new(10.0, 60.0),
    });
    part_datas.push(Vec::new());
    part_datas[1].push(PartData {
        joint_parrent_offset: Vec2::new(-40.0, -10.0),
        joint_offset: Vec2::new(0.0, -40.0), // y 10 lower than in part size y
        transform: Vec3::new(entity_pos.x, entity_pos.y, 0.0),
        part_size: Vec2::new(10.0, 40.0), // y is the same as in the child entity joint offset y
    });
    part_datas[1].push(PartData {
        joint_parrent_offset: Vec2::new(0.0, 40.0), // y is the same as in the parent entity part size y
        joint_offset: Vec2::new(0.0, -60.0),        // y 10 lower than in part size y
        transform: Vec3::new(entity_pos.x - 40.0, entity_pos.y - 10.0 + 40.0, 0.0),
        part_size: Vec2::new(10.0, 60.0),
    });
    part_datas.push(Vec::new());
    part_datas[2].push(PartData {
        joint_parrent_offset: Vec2::new(0.0, 10.0),
        joint_offset: Vec2::new(0.0, -40.0), // y 10 lower than in part size y
        transform: Vec3::new(entity_pos.x, entity_pos.y, 0.0),
        part_size: Vec2::new(10.0, 40.0), // y is the same as in the child entity joint offset y
    });
    part_datas[2].push(PartData {
        joint_parrent_offset: Vec2::new(0.0, 40.0), // y is the same as in the parent entity part size y
        joint_offset: Vec2::new(0.0, -60.0),        // y 10 lower than in part size y
        transform: Vec3::new(entity_pos.x, entity_pos.y + 10.0 + 40.0, 0.0),
        part_size: Vec2::new(10.0, 60.0),
    });
    part_datas.push(Vec::new());
    part_datas[3].push(PartData {
        joint_parrent_offset: Vec2::new(0.0, -10.0),
        joint_offset: Vec2::new(0.0, -40.0), // y 10 lower than in part size y
        transform: Vec3::new(entity_pos.x, entity_pos.y, 0.0),
        part_size: Vec2::new(10.0, 40.0), // y is the same as in the child entity joint offset y
    });
    part_datas[3].push(PartData {
        joint_parrent_offset: Vec2::new(0.0, 40.0), // y is the same as in the parent entity part size y
        joint_offset: Vec2::new(0.0, -60.0),        // y 10 lower than in part size y
        transform: Vec3::new(entity_pos.x, entity_pos.y - 10.0 + 40.0, 0.0),
        part_size: Vec2::new(10.0, 60.0),
    });

    construct_entity(
        &part_datas,
        &mut parts,
        (parent_entity, &parent_data),
        &mut commands,
    );

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            0.0 * -ground_height,
            0.0,
        )))
        .insert(Collider::cuboid(ground_size, ground_height))
        // .insert(ActiveHooks::FILTER_CONTACT_PAIRS);
        .insert(CustomFilterTag::GroupB);

    reapier_config.gravity = Vec2::new(0.0, -250.0);
    // commands.insert_resource(part_datas);
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(EntityData { data: part_datas })
        .insert(EntityParts { parts: parts });
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
        if object.angvel > 50.0 {
            object.angvel = 50.0;
        }
        if object.angvel < -50.0 {
            object.angvel = -50.0;
        }
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

fn create_part(
    part_data: &PartData,
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
        .insert(Joint)
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
                part_data.transform.y + part_data.joint_parrent_offset.y + part_data.part_size.y,
                // + part_data.part_size.y / 2.0,
                part_data.transform.z,
            ),
            ..default()
        })
        .insert(Name::new("sussy"))
        .insert(Velocity::zero())
        .insert(Collider::cuboid(
            part_data.part_size.x,
            part_data.part_size.y + 10.0,
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

fn delete_entities(
    commands: &mut Commands,
    parts: &mut Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>>,
    parent: Entity,
) {
    commands.entity(parent).despawn_recursive();
    for i in 0..parts.len() {
        for j in 0..parts[i].len() {
            commands.entity(parts[i][j].0).despawn_recursive();
            commands.entity(parts[i][j].2).despawn_recursive();
        }
    }
    parts.clear();
}

fn construct_entity(
    part_datas: &Vec<Vec<PartData>>,
    parts: &mut Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>>,
    mut parent: (Entity, &Parent),
    commands: &mut Commands,
) {
    delete_entities(commands, parts, parent.0);
    parent.0 = spawn_parent(parent.1, commands);

    for i in 0..part_datas.len() {
        parts.push(Vec::new());
        for j in 0..part_datas[i].len() {
            parts[i].push(create_part(&part_datas[i][j], commands));
        }
    }

    for i in 0..parts.len() {
        parts[i].reverse();
        for j in 0..parts[i].len() {
            commands.entity(parts[i][j].0).with_children(|cmd| {
                cmd.spawn()
                    .insert(ImpulseJoint::new(parts[i][j].2, parts[i][j].3));
            });

            commands
                .entity(if j + 1 != parts[i].len() {
                    parts[i][j + 1].2
                } else {
                    parent.0
                })
                .with_children(|cmd| {
                    cmd.spawn()
                        .insert(ImpulseJoint::new(parts[i][j].0, parts[i][j].1));
                });
        }
        parts[i].reverse();
    }
}
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Parent {
    position: Vec3,
    size: Vec2,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EntityData {
    #[reflect(ignore)]
    pub data: Vec<Vec<PartData>>,
}
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EntityParts {
    #[reflect(ignore)]
    pub parts: Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>>,
}

fn respawn_entity_system(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    parent_entity: Query<(Entity, &Parent)>,
    part_datas: Query<&EntityData>,
    mut parts: Query<&mut EntityParts>,
    // parts: ResMut<Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>>>,
) {
    if keys.pressed(KeyCode::R) {
        for parent_entity in &parent_entity {
            for part_data in &part_datas {
                for mut parts in &mut parts {
                    println!("check 4");
                    construct_entity(
                        &part_data.data,
                        &mut parts.parts,
                        parent_entity,
                        &mut commands,
                    );
                }
            }
        }
    }
}

fn spawn_parent(parent_data: &Parent, commands: &mut Commands) -> Entity {
    commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(
                parent_data.position.x,
                parent_data.position.y,
                parent_data.position.z,
            ),
            ..default()
        })
        .insert(Name::new("sussy_parent"))
        .insert(Velocity::zero())
        .insert(Collider::cuboid(parent_data.size.x, parent_data.size.y))
        .insert(RigidBody::Dynamic)
        .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
        .insert(CustomFilterTag::GroupA)
        .insert(Parent {
            size: parent_data.size,
            position: parent_data.position,
        })
        .id()
}
