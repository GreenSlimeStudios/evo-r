use crate::*;
use bevy::prelude::*;
use utils::*;

pub struct CreatureConstructorPlugin;
impl Plugin for CreatureConstructorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ParentData>()
            .register_type::<Joint>()
            .register_type::<Leg>()
            .register_type::<EntityData>()
            .register_type::<EntityParts>()
            .register_type::<RotationIndicator>()
            .add_system(indicator_positioning_system);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Leg {
    pub id: (usize, usize, usize),
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Joint {
    pub id: (usize, usize, usize),
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct RotationIndicator {
    pub leg_id: (usize, usize),
    pub left: bool,
}

/// Creates a part data representing a creature part based on the parent ~leg part data.
pub fn create_part_data(
    parent_id: (usize, usize),
    extra_joint_parent_offset: Vec2,
    parent_data: PartData,
    part_size: Vec2,
    joint_offset: Option<Vec2>,
    id: (usize, usize),
    rotation_limit: Option<(f32, f32)>,
) -> PartData {
    let joint_offset: Vec2 = match joint_offset {
        Some(offset) => offset,
        None => Vec2::new(0.0, part_size.y),
    };

    return PartData {
        parent_id,
        extra_joint_parent_offset,
        id,
        joint_offset,
        part_size,
        transform: Vec3::new(
            parent_data.transform.x
                + parent_data.joint_parrent_offset.x
                + parent_data.extra_joint_parent_offset.x,
            parent_data.transform.y + parent_data.joint_parrent_offset.y - parent_data.part_size.y
                + parent_data.extra_joint_parent_offset.y,
            0.0,
        ),
        joint_parrent_offset: Vec2::new(
            0.0 + extra_joint_parent_offset.x,
            -parent_data.part_size.y + extra_joint_parent_offset.y,
        ),
        rotation_limit,
    };
}

fn indicator_positioning_system(
    // mut commands: Commands,
    joints: Query<(&GlobalTransform, &Joint)>,
    part_data: Query<&EntityData>,
    mut rotation_indicators: Query<(&mut Transform, &RotationIndicator)>,
) {
    for part_data in &part_data {
        for (joint_transform, joint) in &joints {
            for i in 0..2 {
                for (mut indicator_transform, _indicator) in
                    rotation_indicators.iter_mut().find(|x| {
                        x.1.leg_id == (joint.id.1, joint.id.2)
                            && x.1.left == if i == 0 { true } else { false }
                    })
                {
                    indicator_transform.translation = joint_transform.translation();
                    // println!("sussy");
                    match part_data.data[joint.id.1][joint.id.2].rotation_limit {
                        Some(limit) => {
                            // println!("{} {}", limit.0, limit.1);
                            // indicator_transform.rotation =
                            //     Quat::from_rotation_z(if i == 0 { limit.0 } else { limit.1 });
                            indicator_transform.rotation =
                                Quat::from_rotation_z(angle_to_radian_full(if i == 0 {
                                    limit.0
                                } else {
                                    limit.1
                                }));
                            // indicator_transform.rotation.z = if i == 0 { limit.0 } else { limit.1 };
                            // indicator_transform.rotation.x = 0.0;
                            // indicator_transform.rotation.y = 0.0;
                        }
                        None => {
                            // indicator_transform.rotation = Quat::from_rotation_z(0.0);
                        }
                    }
                }
            }
        }
    }
}

/// Deletes the edited creature and constructs it from the part data.
pub fn construct_entity(
    id: usize,
    entity_selector: &SelectedEntity,
    part_datas: &mut Vec<Vec<PartData>>,
    parts: &mut Vec<Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>>>,
    mut parent: (Entity, &ParentData),
    commands: &mut Commands,
    rotation_indicators: &Query<Entity, With<RotationIndicator>>,
) {
    delete_creature_instance(commands, parts, parent.0);
    delete_rotation_indicators(rotation_indicators, commands);
    parent.0 = spawn_parent(parent.1, commands, entity_selector.parent, id);

    // Constructs the parts
    parts.push(Vec::new());
    for i in 0..part_datas.len() {
        parts[0].push(Vec::new());
        for j in 0..part_datas[i].len() {
            let is_part_selected: bool = match &entity_selector.parts {
                Some(v) => v.contains(&(i, j)),
                None => false,
            };
            let current_data: PartData = part_datas[i][j].clone();
            let parent_leg_data: Option<PartData> = match j {
                0 => None,
                _ => Some(part_datas[current_data.parent_id.0][current_data.parent_id.1].clone()),
            };
            match parent_leg_data {
                None => (),
                Some(data) => {
                    part_datas[i][j] = create_part_data(
                        current_data.parent_id,
                        current_data.extra_joint_parent_offset,
                        data,
                        current_data.part_size,
                        Some(current_data.joint_offset),
                        current_data.id,
                        current_data.rotation_limit,
                    );
                }
            }

            parts[0][i].push(create_part(
                0,
                &part_datas[i][j],
                commands,
                is_part_selected,
            ));
        }
    }

    // Attaches the parts to eachother forming the whole creature
    for i in 0..parts[0].len() {
        for j in 0..parts[0][i].len() {
            commands.entity(parts[0][i][j].0).with_children(|cmd| {
                cmd.spawn()
                    .insert(ImpulseJoint::new(parts[0][i][j].2, parts[0][i][j].3));
            });

            commands
                .entity(if part_datas[i][j].id.1 != 0 {
                    parts[0][part_datas[i][j].parent_id.0][part_datas[i][j].parent_id.1].2
                } else {
                    parent.0
                })
                .with_children(|cmd| {
                    cmd.spawn()
                        .insert(ImpulseJoint::new(parts[0][i][j].0, parts[0][i][j].1));
                });
        }
    }
    // // spawn rotaion limit indicator
    for i in 0..part_datas.len() {
        for j in 0..part_datas[i].len() {
            // commands.entity(parts[0][i][j].0).with_children(|cmd| {
            commands
                .spawn_bundle(TransformBundle {
                    local: Transform::from_xyz(
                        part_datas[i][j].transform.x + part_datas[i][j].joint_parrent_offset.x,
                        part_datas[i][j].transform.y + part_datas[i][j].joint_parrent_offset.y,
                        0.0,
                    ),
                    ..default()
                })
                .insert(RotationIndicator {
                    leg_id: (part_datas[i][j].id),
                    left: true,
                })
                .insert(Collider::cuboid(1.0, 10.0))
                .insert(ColliderDebugColor {
                    0: Color::rgb(0.5, 0.5, 0.0),
                })
                .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
                .insert(CustomFilterTag::GroupA)
                .insert(Name::new("Left Rotation Indicator"));

            commands
                .spawn_bundle(TransformBundle {
                    local: Transform::from_xyz(
                        part_datas[i][j].transform.x,
                        part_datas[i][j].transform.y,
                        0.0,
                    ),
                    ..default()
                })
                .insert(RotationIndicator {
                    leg_id: (part_datas[i][j].id),
                    left: false,
                })
                .insert(Collider::cuboid(1.0, 10.0))
                .insert(ColliderDebugColor {
                    0: Color::rgb(0.5, 0.5, 0.5),
                })
                .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
                .insert(CustomFilterTag::GroupA)
                .insert(Name::new("Right Rotation Indicator"));
            // cmd.spawn().insert(ImpulseJoint::new(
            //     left_rotation,
            //     RevoluteJointBuilder::new().local_anchor1(Vec2::new(0.0, -5.0)),
            // ));
            // cmd.spawn().insert(ImpulseJoint::new(
            //     right_rotation,
            //     RevoluteJointBuilder::new().local_anchor1(Vec2::new(0.0, -5.0)),
            // ));
            // });
        }
    }
}

/// Deletes all of the creatures and constructs them from the part data.
pub fn construct_entities(
    group_size: usize,
    entity_selector: &SelectedEntity,
    part_datas: &mut Vec<Vec<PartData>>,
    parts: &mut Query<&mut EntityParts>,
    parents: Query<(Entity, &ParentData)>,
    commands: &mut Commands,
    rotation_indicators: &Query<Entity, With<RotationIndicator>>,
) {
    let mut parent_data: ParentData = ParentData {
        id: 0,
        position: Vec3::ZERO,
        size: Vec2::ZERO,
    };
    // Deleting the existing creatures
    for parent in &parents {
        parent_data = parent.1.clone();
        for mut parts in &mut *parts {
            delete_creature_instance(commands, &mut parts.parts, parent.0);
        }
    }
    delete_rotation_indicators(rotation_indicators, commands);

    for mut parts in &mut *parts {
        let parts = &mut parts.parts;
        // Creating the part data
        for i in 0..part_datas.len() {
            for j in 0..part_datas[i].len() {
                let current_data: PartData = part_datas[i][j].clone();
                let parent_leg_data: Option<PartData> = match j {
                    0 => None,
                    _ => {
                        Some(part_datas[current_data.parent_id.0][current_data.parent_id.1].clone())
                    }
                };
                match parent_leg_data {
                    None => (),
                    Some(data) => {
                        part_datas[i][j] = create_part_data(
                            current_data.parent_id,
                            current_data.extra_joint_parent_offset,
                            data,
                            current_data.part_size,
                            Some(current_data.joint_offset),
                            current_data.id,
                            current_data.rotation_limit,
                        );
                    }
                }
            }
        }

        for id in 0..group_size {
            // Constructing accual parts and joints from the part data
            parts.push(Vec::new());
            for i in 0..part_datas.len() {
                parts[id].push(Vec::new());
                for j in 0..part_datas[i].len() {
                    let is_part_selected: bool = match &entity_selector.parts {
                        Some(v) => v.contains(&(i, j)),
                        None => false,
                    };
                    parts[id][i].push(create_part(
                        id,
                        &part_datas[i][j],
                        commands,
                        is_part_selected,
                    ));
                }
            }

            // Attaching the parts and joints together to create the creature
            let parent: Entity = spawn_parent(&parent_data, commands, entity_selector.parent, id);

            for i in 0..parts[id].len() {
                for j in 0..parts[id][i].len() {
                    commands.entity(parts[id][i][j].0).with_children(|cmd| {
                        cmd.spawn()
                            .insert(ImpulseJoint::new(parts[id][i][j].2, parts[id][i][j].3));
                    });

                    commands
                        .entity(if part_datas[i][j].id.1 != 0 {
                            parts[id][part_datas[i][j].parent_id.0][part_datas[i][j].parent_id.1].2
                        } else {
                            parent
                        })
                        .with_children(|cmd| {
                            cmd.spawn()
                                .insert(ImpulseJoint::new(parts[id][i][j].0, parts[id][i][j].1));
                        });
                }
            }
        }
    }
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct ParentData {
    pub id: usize,
    pub position: Vec3,
    pub size: Vec2,
}
/// Represents a part of the creature, used for part construction.
#[derive(Clone, Copy)]
pub struct PartData {
    pub parent_id: (usize, usize),
    pub extra_joint_parent_offset: Vec2,
    pub joint_parrent_offset: Vec2,
    pub joint_offset: Vec2,
    pub transform: Vec3,
    pub part_size: Vec2,
    pub id: (usize, usize),
    pub rotation_limit: Option<(f32, f32)>,
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
    pub parts: Vec<Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>>>,
}

/// This function creates a part with its joint from a specified part data
pub fn create_part(
    parent_body_id: usize,
    part_data: &PartData,
    commands: &mut Commands,
    is_part_selected: bool,
) -> (Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder) {
    // Creating the Joint
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
        .insert(Joint {
            id: (parent_body_id, part_data.id.0, part_data.id.1),
        })
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(RigidBody::Dynamic)
        .insert(ColliderDebugColor {
            0: if is_part_selected {
                Color::rgb(255.0, 0.0, 0.0)
            } else {
                Color::rgb(1.0, 0.0, 1.0)
            },
        })
        .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
        .insert(CustomFilterTag::GroupA)
        .id();

    let joint_to_parrent =
        RevoluteJointBuilder::new().local_anchor2(part_data.joint_parrent_offset);
    let joint_to_joint = RevoluteJointBuilder::new().local_anchor1(part_data.joint_offset);

    // Creating the part
    let part_entity = commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_xyz(
                part_data.transform.x - part_data.joint_offset.x,
                // + part_data.joint_parrent_offset.x / 2.0,
                // + part_data.part_size.x / 2.0,
                part_data.transform.y - part_data.joint_offset.y,
                // + part_data.joint_parrent_offset.y / 2.0,
                // - part_data.part_size.y,
                // + part_data.part_size.y / 2.0,
                part_data.transform.z,
            ),
            // .with_rotation(Quat::from_rotation_z(0.0)),
            ..default()
        })
        .insert(Name::new("sussy"))
        .insert(Velocity::zero())
        .insert(Collider::cuboid(
            part_data.part_size.x,
            part_data.part_size.y + 10.0,
        ))
        .insert(ColliderDebugColor {
            0: if is_part_selected {
                Color::rgb(255.0, 0.0, 0.0)
            } else {
                Color::rgb(1.0, 0.0, 1.0)
            },
        })
        .insert(RigidBody::Dynamic)
        .insert(Leg {
            id: (parent_body_id, part_data.id.0, part_data.id.1),
        })
        .insert(ActiveHooks::FILTER_CONTACT_PAIRS)
        .insert(CustomFilterTag::GroupA)
        .id();

    (entity, joint_to_parrent, part_entity, joint_to_joint)
}

/// Deletes the rotation_indicators to make space for new ones.
fn delete_rotation_indicators(
    rotation_indicators: &Query<Entity, With<RotationIndicator>>,
    commands: &mut Commands,
) {
    for indicator in rotation_indicators {
        commands.entity(indicator).despawn_recursive();
    }
}

/// Deletes a specified creature.
fn delete_creature_instance(
    commands: &mut Commands,
    parts: &mut Vec<Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>>>,
    parent: Entity,
) {
    commands.entity(parent).despawn_recursive();
    for i in 0..parts.len() {
        for j in 0..parts[i].len() {
            for k in 0..parts[i][j].len() {
                commands.entity(parts[i][j][k].0).despawn_recursive();
                commands.entity(parts[i][j][k].2).despawn_recursive();
            }
        }
    }
    parts.clear();
}

/// Spawns the most upper part (body) of the creature.
fn spawn_parent(
    parent_data: &ParentData,
    commands: &mut Commands,
    is_parent_selected: bool,
    id: usize,
) -> Entity {
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
        .insert(ColliderDebugColor {
            0: if is_parent_selected {
                Color::rgb(255.0, 0.0, 0.0)
            } else {
                Color::rgb(1.0, 0.0, 1.0)
            },
        })
        .insert(ParentData {
            id,
            size: parent_data.size,
            position: parent_data.position,
        })
        .insert(Body)
        .id()
}
