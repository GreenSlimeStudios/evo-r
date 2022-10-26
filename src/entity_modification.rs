use crate::*;

pub struct CreatureModificationPlugin;

impl Plugin for CreatureModificationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_leg_system)
            .add_system(respawn_entity_system)
            .add_system(reset_entity_system);
    }
}

fn add_leg_system(
    reapier_config: Res<RapierConfiguration>,
    windows: Res<Windows>,
    mut commands: Commands,
    parent: Query<(&GlobalTransform, Entity, &ParentData)>,
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
                                let mut parent_leg_data =
                                    entity_data.data[leg.id.0][leg.id.1].clone();
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

fn reset_entity_system(
    mut parts: Query<(&mut EntityData, &mut EntityParts)>,
    parents: Query<(Entity, &ParentData)>,
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
fn respawn_entity_system(
    entity_selectors: Query<&SelectedEntity>,
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    parent_entity: Query<(Entity, &ParentData)>,
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