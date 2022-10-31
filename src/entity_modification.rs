use crate::*;

pub struct CreatureModificationPlugin;

impl Plugin for CreatureModificationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_leg_system)
            .add_system(respawn_entity_system)
            .add_system(reset_entity_system)
            .add_system(toggle_gravity_system);
    }
}
fn toggle_gravity_system(
    entity_selectors: Query<&SelectedEntity>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    parents: Query<(Entity, &ParentData)>,
    mut part_datas: Query<&mut EntityData>,
    mut parts: Query<&mut EntityParts>,
    mut reapier_config: ResMut<RapierConfiguration>,
    mut parent: Query<(&mut Transform, &mut Velocity, &ParentData), With<ParentData>>,
    mut legs: Query<(&mut Transform, &mut Velocity), Without<ParentData>>,
) {
    if keys.just_pressed(KeyCode::G) {
        if reapier_config.gravity == Vec2::ZERO {
            reapier_config.gravity = Vec2::new(0.0, -250.0);
        } else {
            reapier_config.gravity = Vec2::ZERO;
            for entity_selector in &entity_selectors {
                for mut part_data in &mut part_datas {
                    // for mut parts in &mut parts {
                    construct_entities(
                        if reapier_config.gravity == Vec2::ZERO {
                            1
                        } else {
                            GROUP_SIZE
                        },
                        entity_selector,
                        &mut part_data.data,
                        &mut parts,
                        parents,
                        &mut commands,
                    );
                    break;
                    // }
                }
                // }
                break;
            }
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

fn add_leg_system(
    reapier_config: Res<RapierConfiguration>,
    windows: Res<Windows>,
    mut commands: Commands,
    mut parent: Query<
        (
            &GlobalTransform,
            Entity,
            &ParentData,
            &mut ColliderDebugColor,
        ),
        Without<Leg>,
    >,
    mut parts: Query<(&mut EntityData, &mut EntityParts)>,
    buttons: Res<Input<MouseButton>>,
    mut legs: Query<(&GlobalTransform, &Leg, &mut ColliderDebugColor)>,
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
        if
        // (buttons.just_pressed(MouseButton::Left) || buttons.just_pressed(MouseButton::Right))
        // &&
        reapier_config.gravity == Vec2::ZERO {
            for mut entity_selector in &mut selected_entity {
                for (mut entity_data, mut entity_parts) in &mut parts {
                    for (parent_transform, parent_entity, parent_data, mut parent_collider_color) in
                        &mut parent
                    {
                        if (position.x - parent_transform.translation().x).abs()
                            < parent_data.size.x
                            && (position.y - parent_transform.translation().y).abs()
                                < parent_data.size.y
                        {
                            parent_collider_color.0 = Color::rgb(0.0, 0.5, 0.0);
                            if buttons.just_pressed(MouseButton::Left) {
                                entity_data.data.push(Vec::new());
                                let index1: usize = entity_data.data.len() - 1;
                                let index2: usize = entity_data.data[index1].len();
                                entity_data.data[index1].push(PartData {
                                    parent_id: (0, 0),
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
                                    0,
                                    &entity_selector,
                                    &mut entity_data.data,
                                    &mut entity_parts.parts,
                                    (parent_entity, &parent_data),
                                    &mut commands,
                                );
                            } else if buttons.just_pressed(MouseButton::Right)
                                || keys.just_pressed(KeyCode::S)
                            {
                                entity_selector.parts = None;
                                entity_selector.parent = true;
                                construct_entity(
                                    0,
                                    &entity_selector,
                                    &mut entity_data.data,
                                    &mut entity_parts.parts,
                                    (parent_entity, &parent_data),
                                    &mut commands,
                                );
                            }
                            // break;
                        } else {
                            if parent_collider_color.0 == Color::rgb(0.0, 0.5, 0.0) {
                                if entity_selector.parent {
                                    parent_collider_color.0 = Color::rgb(2.0, 2.0, 2.0);
                                } else {
                                    parent_collider_color.0 = Color::rgb(1.0, 0.0, 1.0);
                                }
                            }

                            for (leg_trasform, leg, mut collider_color) in &mut legs {
                                let mut parent_leg_data =
                                    entity_data.data[leg.id.1][leg.id.2].clone();
                                if parent_leg_data.id.1 == 0 {
                                    parent_leg_data = PartData {
                                        parent_id: (0, 0),
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
                                    - leg_trasform.translation().y)
                                    // - parent_leg_data.joint_parrent_offset.y
                                    // - parent_leg_data.part_size.y
                                    // - parent_leg_data.joint_offset.y
                                        .abs()
                                        // - parent_leg_data.part_size.y
                                        < parent_leg_data.part_size.y + 10.0
                                {
                                    collider_color.0 = Color::rgb(0.0, 1.0, 0.0);
                                    // OOOOOOOOOOOOOOOOOOOOOOOOOOO
                                    if buttons.just_pressed(MouseButton::Left) {
                                        let index2: usize = entity_data.data[leg.id.0].len();
                                        entity_data.data[leg.id.0].push(
                                            create_part_data(
                                                parent_leg_data.id,
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
                                            0,
                                            &entity_selector,
                                            &mut entity_data.data,
                                            &mut entity_parts.parts,
                                            (parent_entity, &parent_data),
                                            &mut commands,
                                        );
                                        break;
                                    } else if buttons.just_pressed(MouseButton::Right)
                                        || keys.just_pressed(KeyCode::S)
                                    {
                                        entity_selector.parent = false;
                                        if keys.pressed(KeyCode::LControl) {
                                            match &mut entity_selector.parts {
                                                None => {
                                                    entity_selector.parts =
                                                        Some(vec![(leg.id.1, leg.id.2)]);
                                                }
                                                Some(vec) => {
                                                    if vec.contains(&(leg.id.1, leg.id.2)) {
                                                        let mut index: usize = 0;
                                                        for k in 0..vec.len() {
                                                            if vec[k] == (leg.id.1, leg.id.2) {
                                                                index = k;
                                                                break;
                                                            }
                                                        }
                                                        vec.remove(index);
                                                    } else {
                                                        vec.push((leg.id.1, leg.id.2));
                                                    }
                                                }
                                            }
                                        } else {
                                            entity_selector.parts =
                                                Some(vec![((leg.id.1, leg.id.2))])
                                        }
                                        construct_entity(
                                            0,
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
                                } else {
                                    if collider_color.0 == Color::rgb(0.0, 1.0, 0.0) {
                                        match &entity_selector.parts {
                                            Some(v) => {
                                                if v.contains(&(leg.id.1, leg.id.2)) {
                                                    collider_color.0 = Color::rgb(2.0, 2.0, 2.0);
                                                } else {
                                                    collider_color.0 = Color::rgb(1.0, 0.0, 1.0);
                                                }
                                            }
                                            None => {
                                                collider_color.0 = Color::rgb(1.0, 0.0, 1.0);
                                            }
                                        }
                                        // if entity_selector.parts.contains(&leg.id) {
                                        // } else {
                                        //     collider_color.0 = Color::rgb(2.0, 2.0, 2.0);
                                        // }
                                    }
                                }
                            }
                        }
                        // break;
                    }
                }
            }
        }
    } else {
        // cursor is not inside the window
    }
}

fn reset_entity_system(
    mut parts: Query<(&mut EntityData, &mut EntityParts)>,
    parents: Query<(Entity, &ParentData)>,
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    entity_selectors: Query<&SelectedEntity>,
    reapier_config: Res<RapierConfiguration>,
) {
    if reapier_config.gravity != Vec2::ZERO {
        return;
    }
    if keys.just_pressed(KeyCode::Q) == false {
        return;
    }
    for entity_selector in &entity_selectors {
        for parent in &parents {
            for (mut part_data, mut parts) in &mut parts {
                part_data.data.clear();

                construct_entity(
                    0,
                    entity_selector,
                    &mut part_data.data,
                    &mut parts.parts,
                    (parent.0, &parent.1),
                    &mut commands,
                );
            }
            break;
        }
    }
}
fn respawn_entity_system(
    entity_selectors: Query<&SelectedEntity>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    parents: Query<(Entity, &ParentData)>,
    mut part_datas: Query<&mut EntityData>,
    mut parts: Query<&mut EntityParts>,
    // parts: ResMut<Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>>>,
    reapier_config: Res<RapierConfiguration>,
) {
    if keys.pressed(KeyCode::R) {
        // for parent_entity in &parents {
        for entity_selector in &entity_selectors {
            for mut part_data in &mut part_datas {
                // for mut parts in &mut parts {
                construct_entities(
                    if reapier_config.gravity == Vec2::ZERO {
                        1
                    } else {
                        GROUP_SIZE
                    },
                    entity_selector,
                    &mut part_data.data,
                    &mut parts,
                    parents,
                    &mut commands,
                );
                break;
                // }
            }
            // }
            break;
        }
    }
}
