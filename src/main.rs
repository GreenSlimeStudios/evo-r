use std::ops::Index;

use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::{na::ComplexField, parry::transformation::voxelization, prelude::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            title: "evo-r".to_string(),
            ..default()
        })
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
        .add_system(add_leg_system)
        .add_system(reset_entity)
        .add_system(toggle_gravity)
        .add_system(edit_selected_parts_system)
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
pub struct Leg {
    pub id: (usize, usize),
}

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
        size: Vec2::new(200.0, 30.0),
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

    // part_datas.push(Vec::new());
    // part_datas[0].push(PartData {
    //     joint_parrent_offset: Vec2::new(40.0, 10.0),
    //     joint_offset: Vec2::new(0.0, 40.0), // y 10 lower than in part size y
    //     transform: Vec3::new(entity_pos.x, entity_pos.y, 0.0),
    //     part_size: Vec2::new(10.0, 40.0), // y is the same as in the child entity joint offset y
    // });

    // let part_data: PartData = part_datas[0][0].clone();
    // part_datas[0].push(
    //     create_part_data(part_data, Vec2 { x: 10.0, y: 60.0 }, None),
    //     // PartData {
    //     //     joint_parrent_offset: Vec2::new(0.0, -40.0), // y is the same as in the parent entity part size y
    //     //     joint_offset: Vec2::new(0.0, 60.0),          // y 10 lower than in part size y
    //     //     transform: Vec3::new(entity_pos.x + 40.0, entity_pos.y + 10.0 - 40.0, 0.0),
    //     //     part_size: Vec2::new(10.0, 60.0),
    //     // },
    // );
    // let part_data: PartData = part_datas[0][1].clone();
    // part_datas[0].push(create_part_data(part_data, Vec2 { x: 10.0, y: 30.0 }, None));
    // part_datas.push(Vec::new());
    // part_datas[1].push(PartData {
    //     joint_parrent_offset: Vec2::new(-40.0, -10.0),
    //     joint_offset: Vec2::new(0.0, 40.0), // y 10 lower than in part size y
    //     transform: Vec3::new(entity_pos.x, entity_pos.y, 0.0),
    //     part_size: Vec2::new(10.0, 40.0), // y is the same as in the child entity joint offset y
    // });
    // part_datas[1].push(PartData {
    //     joint_parrent_offset: Vec2::new(0.0, -40.0), // y is the same as in the parent entity part size y
    //     joint_offset: Vec2::new(0.0, 60.0),          // y 10 lower than in part size y
    //     transform: Vec3::new(entity_pos.x - 40.0, entity_pos.y - 10.0 - 40.0, 0.0),
    //     part_size: Vec2::new(10.0, 60.0),
    // });
    // part_datas.push(Vec::new());
    // part_datas[2].push(PartData {
    //     joint_parrent_offset: Vec2::new(0.0, 10.0),
    //     joint_offset: Vec2::new(0.0, 40.0), // y 10 lower than in part size y
    //     transform: Vec3::new(entity_pos.x, entity_pos.y, 0.0),
    //     part_size: Vec2::new(10.0, 40.0), // y is the same as in the child entity joint offset y
    // });
    // part_datas[2].push(PartData {
    //     joint_parrent_offset: Vec2::new(0.0, -40.0), // y is the same as in the parent entity part size y
    //     joint_offset: Vec2::new(0.0, 60.0),          // y 10 lower than in part size y
    //     transform: Vec3::new(entity_pos.x, entity_pos.y + 10.0 - 40.0, 0.0),
    //     part_size: Vec2::new(10.0, 60.0),
    // });
    // part_datas.push(Vec::new());
    // part_datas[3].push(PartData {
    //     joint_parrent_offset: Vec2::new(0.0, -10.0),
    //     joint_offset: Vec2::new(0.0, 40.0), // y 10 lower than in part size y
    //     transform: Vec3::new(entity_pos.x, entity_pos.y, 0.0),
    //     part_size: Vec2::new(10.0, 40.0), // y is the same as in the child entity joint offset y
    // });
    // part_datas[3].push(PartData {
    //     joint_parrent_offset: Vec2::new(0.0, -40.0), // y is the same as in the parent entity part size y
    //     joint_offset: Vec2::new(0.0, 60.0),          // y 10 lower than in part size y
    //     transform: Vec3::new(entity_pos.x, entity_pos.y - 10.0 - 40.0, 0.0),
    //     part_size: Vec2::new(10.0, 60.0),
    // });

    construct_entity(
        &SelectedEntity {
            parent: true,
            parts: None,
        },
        &mut part_datas,
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

    // reapier_config.gravity = Vec2::new(0.0, -250.0);
    // commands.insert_resource(part_datas);
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(EntityData { data: part_datas })
        .insert(EntityParts { parts: parts })
        .insert(SelectedEntity {
            parent: false,
            parts: None,
        })
        .insert(Name::new("entity data"));
}
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct SelectedEntity {
    #[reflect(ignore)]
    pub parent: bool,
    #[reflect(ignore)]
    pub parts: Option<Vec<(usize, usize)>>,
}

fn create_part_data(
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
        extra_joint_parent_offset,
        id,
        joint_offset,
        part_size,
        transform: Vec3::new(
            parent_data.transform.x + parent_data.joint_parrent_offset.x,
            parent_data.transform.y + parent_data.joint_parrent_offset.y - parent_data.part_size.y,
            0.0,
        ),
        joint_parrent_offset: Vec2::new(
            0.0 + extra_joint_parent_offset.x,
            -parent_data.part_size.y + extra_joint_parent_offset.y,
        ),
    };
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
fn toggle_gravity(
    mut reapier_config: ResMut<RapierConfiguration>,
    keys: Res<Input<KeyCode>>,
    mut parent: Query<(&mut Transform, &mut Velocity, &Parent), With<Parent>>,
    mut legs: Query<(&mut Transform, &mut Velocity), Without<Parent>>,
) {
    if keys.just_pressed(KeyCode::G) {
        if reapier_config.gravity == Vec2::ZERO {
            reapier_config.gravity = Vec2::new(0.0, -250.0);
        } else {
            reapier_config.gravity = Vec2::ZERO;
        }
    }
    if reapier_config.gravity == Vec2::ZERO {
        for (mut parent_transform, mut parent_velocity, parent_data) in &mut parent {
            parent_transform.translation = parent_data.position;
            parent_transform.rotation = Quat::from_rotation_y(0.0);
            parent_velocity.angvel = 0.0;
            parent_velocity.linvel = Vec2::ZERO;
        }
        for (mut transform, mut velocity) in &mut legs {
            transform.rotation = Quat::from_rotation_y(0.0);
            velocity.angvel = 0.0;
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

#[derive(Clone, Copy)]
pub struct PartData {
    extra_joint_parent_offset: Vec2,
    joint_parrent_offset: Vec2,
    joint_offset: Vec2,
    transform: Vec3,
    part_size: Vec2,
    id: (usize, usize),
}

fn create_part(
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
        // .insert(Leg)
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
        .insert(Leg { id: part_data.id })
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
    entity_selector: &SelectedEntity,
    mut part_datas: &mut Vec<Vec<PartData>>,
    parts: &mut Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>>,
    mut parent: (Entity, &Parent),
    commands: &mut Commands,
) {
    delete_entities(commands, parts, parent.0);
    parent.0 = spawn_parent(parent.1, commands, entity_selector.parent);

    for i in 0..part_datas.len() {
        parts.push(Vec::new());
        for j in 0..part_datas[i].len() {
            let is_part_selected: bool = match &entity_selector.parts {
                Some(v) => v.contains(&(i, j)),
                None => false,
            };
            let parent_leg_data: Option<PartData> = match j {
                0 => None,
                _ => Some(part_datas[i][j - 1].clone()),
            };
            match parent_leg_data {
                None => (),
                Some(data) => {
                    let current_data: PartData = part_datas[i][j].clone();
                    part_datas[i][j] = create_part_data(
                        current_data.extra_joint_parent_offset,
                        data,
                        current_data.part_size,
                        Some(current_data.joint_offset),
                        current_data.id,
                    );
                    // PartData {
                }
            }

            parts[i].push(create_part(&part_datas[i][j], commands, is_part_selected));
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
pub struct Parent {
    pub position: Vec3,
    pub size: Vec2,
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
    entity_selectors: Query<&SelectedEntity>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    parent_entity: Query<(Entity, &Parent)>,
    mut part_datas: Query<&mut EntityData>,
    mut parts: Query<&mut EntityParts>,
    // parts: ResMut<Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>>>,
) {
    if keys.pressed(KeyCode::R) {
        for entity_selector in &entity_selectors {
            for parent_entity in &parent_entity {
                for mut part_data in &mut part_datas {
                    for mut parts in &mut parts {
                        construct_entity(
                            entity_selector,
                            &mut part_data.data,
                            &mut parts.parts,
                            parent_entity,
                            &mut commands,
                        );
                    }
                }
            }
        }
    }
}

fn spawn_parent(parent_data: &Parent, commands: &mut Commands, is_parent_selected: bool) -> Entity {
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
        .insert(Parent {
            size: parent_data.size,
            position: parent_data.position,
        })
        .id()
}

fn add_leg_system(
    reapier_config: Res<RapierConfiguration>,
    windows: Res<Windows>,
    mut commands: Commands,
    parent: Query<(&GlobalTransform, Entity, &Parent)>,
    mut parts: Query<(&mut EntityData, &mut EntityParts)>,
    buttons: Res<Input<MouseButton>>,
    legs: Query<(&GlobalTransform, &Leg)>,
    keys: Res<Input<KeyCode>>,
    mut selected_entity: Query<&mut SelectedEntity>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(_position) = window.cursor_position() {
        // cursor is inside the window, position given
        let position = Vec2::new(
            _position.x - window.width() / 2.0,
            _position.y - window.height() / 2.0 + 20.0,
        );
        // println!("{}", position);
        // for (tower, transform) in &targets {
        if keys.pressed(KeyCode::R) {
            return;
        }
        if (buttons.just_pressed(MouseButton::Left) || buttons.just_pressed(MouseButton::Right))
            && reapier_config.gravity == Vec2::ZERO
        {
            for mut entity_selector in &mut selected_entity {
                for (mut entity_data, mut entity_parts) in &mut parts {
                    for (parent_transform, parent_entity, parent_data) in &parent {
                        if (position.x - parent_transform.translation().x).abs()
                            < parent_data.size.x
                            && (position.y - parent_transform.translation().y).abs()
                                < parent_data.size.y
                        {
                            if buttons.just_pressed(MouseButton::Left) {
                                entity_data.data.push(Vec::new());
                                let index1: usize = entity_data.data.len() - 1;
                                let index2: usize = entity_data.data[index1].len();
                                entity_data.data[index1].push(PartData {
                                    extra_joint_parent_offset: Vec2::ZERO,
                                    id: (index1, index2),
                                    joint_parrent_offset: position
                                        - to_vec2(&parent_transform.translation()),
                                    joint_offset: Vec2::new(0.0, 40.0),
                                    transform: Vec3::new(position.x, position.y, 0.0),
                                    part_size: Vec2::new(10.0, 40.0),
                                });
                                entity_selector.parts = Some(vec![(index1, index2)]);
                                entity_selector.parent = false;
                                construct_entity(
                                    &entity_selector,
                                    &mut entity_data.data,
                                    &mut entity_parts.parts,
                                    (parent_entity, &parent_data),
                                    &mut commands,
                                );
                            } else if buttons.just_pressed(MouseButton::Right) {
                                entity_selector.parts = None;
                                entity_selector.parent = true;
                                construct_entity(
                                    &entity_selector,
                                    &mut entity_data.data,
                                    &mut entity_parts.parts,
                                    (parent_entity, &parent_data),
                                    &mut commands,
                                );
                            }
                            break;
                        } else {
                            for (leg_trasform, leg) in &legs {
                                let mut parent_leg_data = entity_data.data[leg.id.0]
                                    [entity_data.data[leg.id.0].len() - 1]
                                    .clone();
                                if parent_leg_data.id.1 == 0 {
                                    parent_leg_data = PartData {
                                        extra_joint_parent_offset: Vec2::ZERO,
                                        id: parent_leg_data.id,
                                        joint_offset: parent_leg_data.joint_offset,
                                        joint_parrent_offset: parent_leg_data.joint_parrent_offset,
                                        part_size: parent_leg_data.part_size,
                                        transform: Vec3::new(
                                            (parent_leg_data.transform.x - parent_data.size.x)
                                                / parent_data.size.x.abs()
                                                + parent_data.position.x,
                                            (parent_leg_data.transform.y - parent_data.size.y)
                                                / parent_data.size.y.abs()
                                                + parent_data.position.y
                                                - parent_leg_data.part_size.y * 2.0,
                                            0.0,
                                        ),
                                    }
                                };
                                if (
                                    position.x
                                        - leg_trasform.translation().x
                                        - parent_leg_data.joint_offset.x
                                    // - parent_leg_data.joint_parrent_offset.x
                                )
                                    .abs()
                                    < parent_leg_data.part_size.x
                                    && (position.y
                                    - leg_trasform.translation().y
                                    // - parent_leg_data.joint_parrent_offset.y
                                    // - parent_leg_data.part_size.y
                                    - parent_leg_data.joint_offset.y)
                                        .abs()
                                        < parent_leg_data.part_size.y
                                {
                                    if buttons.just_pressed(MouseButton::Left) {
                                        let index2: usize = entity_data.data[leg.id.0].len();
                                        entity_data.data[leg.id.0].push(
                                            create_part_data(
                                                Vec2::ZERO,
                                                parent_leg_data,
                                                Vec2::new(10.0, 30.0),
                                                None,
                                                (leg.id.0, index2),
                                            ), // PartData {
                                               // id: (leg.id.0, index2),
                                               // joint_parrent_offset: position
                                               //     - to_vec2(&parent_transform.translation),
                                               // joint_offset: Vec2::new(0.0, 40.0),
                                               // transform: Vec3::new(position.x, position.y, 0.0),
                                               // part_size: Vec2::new(10.0, 40.0),
                                               // }
                                        );
                                        entity_selector.parent = false;
                                        entity_selector.parts = Some(vec![(leg.id.0, index2)]);
                                        construct_entity(
                                            &entity_selector,
                                            &mut entity_data.data,
                                            &mut entity_parts.parts,
                                            (parent_entity, &parent_data),
                                            &mut commands,
                                        );
                                        break;
                                    } else if buttons.just_pressed(MouseButton::Right) {
                                        entity_selector.parent = false;
                                        if keys.pressed(KeyCode::LControl) {
                                            match &mut entity_selector.parts {
                                                None => {
                                                    entity_selector.parts = Some(vec![leg.id]);
                                                }
                                                Some(vec) => {
                                                    if vec.contains(&leg.id) {
                                                        let mut index: usize = 0;
                                                        for k in 0..vec.len() {
                                                            if vec[k] == leg.id {
                                                                index = k;
                                                                break;
                                                            }
                                                        }
                                                        vec.remove(index);
                                                    } else {
                                                        vec.push(leg.id);
                                                    }
                                                }
                                            }
                                        } else {
                                            entity_selector.parts = Some(vec![(leg.id)])
                                        }
                                        construct_entity(
                                            &entity_selector,
                                            &mut entity_data.data,
                                            &mut entity_parts.parts,
                                            (parent_entity, &parent_data),
                                            &mut commands,
                                        );
                                        break;
                                    }
                                    // entity_selector.parent = false;
                                    // entity_selector.parts = None;
                                    // construct_entity(
                                    //     &entity_selector,
                                    //     &entity_data.data,
                                    //     &mut entity_parts.parts,
                                    //     (parent_entity, &parent_data),
                                    //     &mut commands,
                                    // );
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        // cursor is not inside the window
    }
}

fn reset_entity(
    mut parts: Query<(&mut EntityData, &mut EntityParts)>,
    parents: Query<(Entity, &Parent)>,
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    entity_selectors: Query<&SelectedEntity>,
) {
    if keys.just_pressed(KeyCode::Q) == false {
        return;
    }
    for entity_selector in &entity_selectors {
        for parent in &parents {
            for (mut part_data, mut parts) in &mut parts {
                part_data.data.clear();

                construct_entity(
                    entity_selector,
                    &mut part_data.data,
                    &mut parts.parts,
                    (parent.0, &parent.1),
                    &mut commands,
                );
            }
        }
    }
}

fn edit_selected_parts_system(
    mut commands: Commands,
    mut parts: Query<(&mut EntityData, &mut EntityParts)>,
    mut parents: Query<(Entity, &mut Parent)>,
    keys: Res<Input<KeyCode>>,
    entity_selectors: Query<&SelectedEntity>,
) {
    if keys.just_pressed(KeyCode::Down)
        || keys.just_pressed(KeyCode::Up)
        || keys.just_pressed(KeyCode::Left)
        || keys.just_pressed(KeyCode::Right)
    {
        for entity_selector in &entity_selectors {
            for (parent_entity, mut parent_data) in &mut parents {
                for (mut part_data, mut parts) in &mut parts {
                    if entity_selector.parent {
                        if keys.just_pressed(KeyCode::Left) {
                            parent_data.size.x -= 10.0;
                        }
                        if keys.just_pressed(KeyCode::Right) {
                            parent_data.size.x += 10.0;
                        }
                        if keys.just_pressed(KeyCode::Up) {
                            parent_data.size.y += 10.0;
                        }
                        if keys.just_pressed(KeyCode::Down) {
                            parent_data.size.y -= 10.0;
                        }
                    }

                    for i in 0..part_data.data.len() {
                        for j in 0..part_data.data[i].len() {
                            match &entity_selector.parts {
                                None => (),
                                Some(v) => {
                                    if v.contains(&(i, j)) {
                                        if keys.just_pressed(KeyCode::Up) {
                                            if keys.pressed(KeyCode::LControl) {
                                                if j == 0 {
                                                    part_data.data[i][j].joint_parrent_offset.y +=
                                                        10.0;
                                                } else {
                                                    part_data.data[i][j]
                                                        .extra_joint_parent_offset
                                                        .y += 10.0;
                                                }
                                            } else if keys.pressed(KeyCode::LAlt) {
                                                part_data.data[i][j].joint_offset.y += 10.0;
                                            } else {
                                                part_data.data[i][j].part_size.y += 10.0;
                                                part_data.data[i][j].joint_offset.y += 10.0;
                                            }
                                        };
                                        if keys.just_pressed(KeyCode::Down) {
                                            if keys.pressed(KeyCode::LControl) {
                                                if j == 0 {
                                                    part_data.data[i][j].joint_parrent_offset.y -=
                                                        10.0;
                                                } else {
                                                    part_data.data[i][j]
                                                        .extra_joint_parent_offset
                                                        .y -= 10.0;
                                                }
                                                // part_data.data[i][j].extra_joint_parent_offset.y -=
                                                //     10.0;
                                            } else if keys.pressed(KeyCode::LAlt) {
                                                part_data.data[i][j].joint_offset.y -= 10.0;
                                            } else {
                                                part_data.data[i][j].part_size.y -= 10.0;
                                                part_data.data[i][j].joint_offset.y -= 10.0;
                                            }
                                        };
                                        if keys.just_pressed(KeyCode::Left) {
                                            if keys.pressed(KeyCode::LControl) {
                                                if j == 0 {
                                                    part_data.data[i][j].joint_parrent_offset.x -=
                                                        10.0;
                                                } else {
                                                    part_data.data[i][j]
                                                        .extra_joint_parent_offset
                                                        .x -= 10.0;
                                                }
                                                // part_data.data[i][j].extra_joint_parent_offset.x -=
                                                //     10.0;
                                            } else if keys.pressed(KeyCode::LAlt) {
                                                part_data.data[i][j].joint_offset.x -= 10.0;
                                            } else {
                                                part_data.data[i][j].part_size.x -= 10.0;
                                            }
                                        };
                                        if keys.just_pressed(KeyCode::Right) {
                                            if keys.pressed(KeyCode::LControl) {
                                                if j == 0 {
                                                    part_data.data[i][j].joint_parrent_offset.x +=
                                                        10.0;
                                                } else {
                                                    part_data.data[i][j]
                                                        .extra_joint_parent_offset
                                                        .x += 10.0;
                                                }
                                                // part_data.data[i][j].extra_joint_parent_offset.x +=
                                                //     10.0;
                                            } else if keys.pressed(KeyCode::LAlt) {
                                                part_data.data[i][j].joint_offset.x += 10.0;
                                            } else {
                                                part_data.data[i][j].part_size.x += 10.0;
                                            }
                                        };
                                    }
                                }
                            }
                        }
                    }
                    construct_entity(
                        &entity_selector,
                        &mut part_data.data,
                        &mut parts.parts,
                        (parent_entity, &parent_data),
                        &mut commands,
                    );
                }
            }
        }
    }
}
