use crate::utils::*;
use crate::*;
use bevy::prelude::*;

pub struct CreatureMovmentPlugin;

impl Plugin for CreatureMovmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_parts);
    }
}

fn move_parts(
    mut objects: Query<(&mut Velocity, &mut Transform, &Leg), Without<ParentData>>,
    part_datas: Query<&EntityData>,
    keys: Res<Input<KeyCode>>,
    parents: Query<(&Transform, &ParentData)>,
) {
    let rotations: Vec<(f32, (usize, usize, usize))> =
        objects.iter().map(|f| (f.1.rotation.z, f.2.id)).collect();

    for part_data in &part_datas {
        // for (parent_tr, parent) in &parents {
        for (mut leg_velocity, mut transform, leg) in &mut objects {
            if keys.pressed(KeyCode::D) {
                leg_velocity.angvel -= 2.0;
            }
            if keys.pressed(KeyCode::A) {
                leg_velocity.angvel += 2.0;
            }
            if keys.pressed(KeyCode::W) {
                leg_velocity.linvel.y += 10.0;
            }
            if leg_velocity.angvel > 50.0 {
                leg_velocity.angvel = 50.0;
            }
            if leg_velocity.angvel < -50.0 {
                leg_velocity.angvel = -50.0;
            }
            let mut parent_part_rotation: f32 = match part_data.data[leg.id.1][leg.id.2].parent_id {
                (0, 0) => {
                    parents
                        .iter()
                        .find(|f| f.1.id == leg.id.0)
                        .unwrap()
                        .0
                        .rotation
                        .z
                }
                v => {
                    rotations
                        .iter()
                        .find(|f| f.1 == (leg.id.0, v.0, v.1))
                        .unwrap()
                        .0
                } // v => part_data.data[v.0][v.1].,
            };
            // parent_part_rotation = angle_to_radian(radian_to_angle(parent_part_rotation));
            parent_part_rotation = 0.0;

            match part_data.data[leg.id.1][leg.id.2].rotation_limit {
                Some(limit) => {
                    let mut angle: f32 =
                        transform.rotation.z.asin() * 180.0 * 2.0 / std::f32::consts::PI;
                    if angle < 0.0 {
                        angle += 360.0;
                    }
                    if is_angle_between(angle, limit.0, limit.1) {
                        return;
                    }
                    let mut is_cross_limit_1: bool = false;
                    let mut is_cross_limit_2: bool = false;
                    if is_surpasing_limit_1(angle, change_angle(limit.0, 1.0), limit.1) {
                        is_cross_limit_1 = true;
                    }
                    if is_surpasing_limit_2(angle, limit.0, change_angle(limit.1, -1.0)) {
                        is_cross_limit_2 = true;
                    }

                    if is_cross_limit_1 && is_cross_limit_2 == false {
                        transform.rotation = Quat::from_rotation_z(
                            angle_to_radian_full(limit.0) + parent_part_rotation,
                        );
                        leg_velocity.angvel = 0.4;
                    } else if is_cross_limit_2 && is_cross_limit_1 == false {
                        transform.rotation = Quat::from_rotation_z(
                            angle_to_radian_full(limit.1) + parent_part_rotation,
                        );
                        leg_velocity.angvel = -0.4;
                    } else if is_cross_limit_1 && is_cross_limit_2 {
                        let mut dist1 = (limit.0 - angle).abs();
                        if dist1 > 180.0 {
                            dist1 = (angle - limit.0).abs();
                        }
                        let mut dist2 = (limit.1 - angle).abs();
                        if dist2 > 180.0 {
                            dist2 = (angle - limit.1).abs();
                        }
                        if dist1 < dist2 {
                            transform.rotation = Quat::from_rotation_z(
                                angle_to_radian_full(limit.0) + parent_part_rotation,
                            );
                            leg_velocity.angvel = 0.4;
                        } else {
                            transform.rotation = Quat::from_rotation_z(
                                angle_to_radian_full(limit.1) + parent_part_rotation,
                            );
                            leg_velocity.angvel = -0.4;
                        }
                    }
                }
                None => (),
            }
            // }
        }
    }
}
