use crate::*;

pub struct PartSelectionPlugin;
impl Plugin for PartSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SelectedEntity>()
            .add_system(edit_selected_parts_system);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct SelectedEntity {
    #[reflect(ignore)]
    pub parent: bool,
    #[reflect(ignore)]
    pub parts: Option<Vec<(usize, usize)>>,
}
fn edit_selected_parts_system(
    mut commands: Commands,
    mut parts: Query<(&mut EntityData, &mut EntityParts)>,
    mut parents: Query<(Entity, &mut ParentData)>,
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
                    if entity_selector.parent == true {
                        if keys.just_pressed(KeyCode::Left) {
                            if keys.pressed(KeyCode::LControl) {
                                parent_data.position.x -= 10.0;
                                change_pos(&mut part_data, Vec2::new(-10.0, 0.0));
                            } else {
                                parent_data.size.x -= 10.0;
                            }
                        }
                        if keys.just_pressed(KeyCode::Right) {
                            if keys.pressed(KeyCode::LControl) {
                                parent_data.position.x += 10.0;
                                change_pos(&mut part_data, Vec2::new(10.0, 0.0));
                            } else {
                                parent_data.size.x += 10.0;
                            }
                        }
                        if keys.just_pressed(KeyCode::Up) {
                            if keys.pressed(KeyCode::LControl) {
                                parent_data.position.y += 10.0;
                                change_pos(&mut part_data, Vec2::new(0.0, 10.0));
                            } else {
                                parent_data.size.y += 10.0;
                            }
                        }
                        if keys.just_pressed(KeyCode::Down) {
                            if keys.pressed(KeyCode::LControl) {
                                parent_data.position.y -= 10.0;
                                change_pos(&mut part_data, Vec2::new(0.0, -10.0));
                            } else {
                                parent_data.size.y -= 10.0;
                            }
                        }
                        // break;
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
                                                    part_data.data[i][j].transform.y += 10.0;
                                                } else {
                                                    part_data.data[i][j]
                                                        .extra_joint_parent_offset
                                                        .y += 10.0;
                                                }
                                            } else if keys.pressed(KeyCode::LAlt) {
                                                part_data.data[i][j].joint_offset.y -= 10.0;
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
                                                    part_data.data[i][j].transform.x -= 10.0;
                                                } else {
                                                    part_data.data[i][j]
                                                        .extra_joint_parent_offset
                                                        .y -= 10.0;
                                                }
                                                // part_data.data[i][j].extra_joint_parent_offset.y -=
                                                //     10.0;
                                            } else if keys.pressed(KeyCode::LAlt) {
                                                part_data.data[i][j].joint_offset.y += 10.0;
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
                                                    part_data.data[i][j].transform.x -= 10.0;
                                                } else {
                                                    part_data.data[i][j]
                                                        .extra_joint_parent_offset
                                                        .x -= 10.0;
                                                }
                                                // part_data.data[i][j].extra_joint_parent_offset.x -=
                                                //     10.0;
                                            } else if keys.pressed(KeyCode::LAlt) {
                                                part_data.data[i][j].joint_offset.x += 10.0;
                                            } else {
                                                part_data.data[i][j].part_size.x -= 10.0;
                                            }
                                        };
                                        if keys.just_pressed(KeyCode::Right) {
                                            if keys.pressed(KeyCode::LControl) {
                                                if j == 0 {
                                                    part_data.data[i][j].joint_parrent_offset.x +=
                                                        10.0;
                                                    part_data.data[i][j].transform.x += 10.0;
                                                } else {
                                                    part_data.data[i][j]
                                                        .extra_joint_parent_offset
                                                        .x += 10.0;
                                                }
                                                // part_data.data[i][j].extra_joint_parent_offset.x +=
                                                //     10.0;
                                            } else if keys.pressed(KeyCode::LAlt) {
                                                part_data.data[i][j].joint_offset.x -= 10.0;
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
fn change_pos(part_data: &mut EntityData, pos_offset: Vec2) {
    for i in 0..part_data.data.len() {
        for j in 0..part_data.data[i].len() {
            part_data.data[i][j].transform.x += pos_offset.x;
            part_data.data[i][j].transform.y += pos_offset.y;
        }
    }
}
