use crate::*;
use bevy::prelude::*;

pub struct CreatureConstructorPlugin;
impl Plugin for CreatureConstructorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ParentData>()
            .register_type::<Joint>()
            .register_type::<Leg>()
            .register_type::<EntityData>()
            .register_type::<EntityParts>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Leg {
    pub id: (usize, usize, usize),
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Joint;
pub fn create_part_data(
    parent_id: (usize, usize),
    extra_joint_parent_offset: Vec2,
    parent_data: PartData,
    part_size: Vec2,
    joint_offset: Option<Vec2>,
    id: (usize, usize),
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
    };
}
pub fn construct_entity(
    id: usize,
    entity_selector: &SelectedEntity,
    part_datas: &mut Vec<Vec<PartData>>,
    parts: &mut Vec<Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>>>,
    mut parent: (Entity, &ParentData),
    commands: &mut Commands,
) {
    delete_entities(commands, parts, parent.0);
    parent.0 = spawn_parent(parent.1, commands, entity_selector.parent, id);

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
}
pub fn construct_entities(
    group_size: usize,
    entity_selector: &SelectedEntity,
    part_datas: &mut Vec<Vec<PartData>>,
    parts: &mut Query<&mut EntityParts>,
    parents: Query<(Entity, &ParentData)>,
    commands: &mut Commands,
) {
    let mut parent_data: ParentData = ParentData {
        id: 0,
        position: Vec3::ZERO,
        size: Vec2::ZERO,
    };
    for parent in &parents {
        parent_data = parent.1.clone();
        for mut parts in &mut *parts {
            delete_entities(commands, &mut parts.parts, parent.0);
        }
    }
    for mut parts in &mut *parts {
        let parts = &mut parts.parts;

        println!("creating part datas");
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
                        );
                    }
                }
            }
        }

        for id in 0..group_size {
            println!("constructing the parts for parent no. {}", id);
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

            println!("attaching the parts together | parent no. {}", id);
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
#[derive(Clone, Copy)]
pub struct PartData {
    pub parent_id: (usize, usize),
    pub extra_joint_parent_offset: Vec2,
    pub joint_parrent_offset: Vec2,
    pub joint_offset: Vec2,
    pub transform: Vec3,
    pub part_size: Vec2,
    pub id: (usize, usize),
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

pub fn create_part(
    parent_body_id: usize,
    part_data: &PartData,
    commands: &mut Commands,
    is_part_selected: bool,
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
fn delete_entities(
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
