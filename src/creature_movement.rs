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
    let part_data = &part_datas.single();
    // println!("wot??");
    let mut counter: usize = 0;
    for (mut leg_velocity, mut transform, leg) in &mut objects {
        // println!("{} - {}", counter, leg.id.1);
        counter += 1;
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
        // let mut parent_part_rotation = 0.0;
        let mut p_angle: f32 = match part_data.data[leg.id.1][leg.id.2].parent_id {
            (0, 0) => angle_from_sin(
                parents
                    .iter()
                    .find(|f| f.1.id == leg.id.0)
                    .unwrap()
                    .0
                    .rotation
                    .z,
            ),
            v => angle_from_sin(
                rotations
                    .iter()
                    .find(|f| f.1 == (leg.id.0, v.0, v.1))
                    .unwrap()
                    .0,
            ), // v => part_data.data[v.0][v.1].,
        };
        // println!("{}", p_angle);
        // p_angle = 0.0;

        match part_data.data[leg.id.1][leg.id.2].rotation_limit {
            Some(limit) => {
                let angle = angle_from_sin(transform.rotation.z);
                if is_angle_between(
                    angle,
                    change_angle(limit.0, p_angle),
                    change_angle(limit.1, p_angle),
                ) {
                    continue;
                }
                let mut is_cross_limit_1: bool = false;
                let mut is_cross_limit_2: bool = false;
                if is_surpasing_limit_1(
                    angle,
                    change_angle(limit.0, p_angle),
                    change_angle(limit.1, p_angle),
                ) {
                    // if is_surpasing_limit_1(angle, change_angle(limit.0, 1.0), limit.1) {
                    is_cross_limit_1 = true;
                }
                if is_surpasing_limit_2(
                    angle,
                    change_angle(limit.0, p_angle),
                    change_angle(limit.1, p_angle),
                ) {
                    // if is_surpasing_limit_2(angle, limit.0, change_angle(limit.1, -1.0)) {
                    is_cross_limit_2 = true;
                }

                if is_cross_limit_1 && is_cross_limit_2 == false {
                    transform.rotation = Quat::from_rotation_z(angle_to_radian_full(change_angle(
                        limit.0,
                        p_angle + 1.0,
                    )));
                    leg_velocity.angvel = 1.0;
                } else if is_cross_limit_2 && is_cross_limit_1 == false {
                    transform.rotation = Quat::from_rotation_z(angle_to_radian_full(change_angle(
                        limit.1,
                        p_angle - 1.0,
                    )));
                    leg_velocity.angvel = -1.0;
                } else if is_cross_limit_1 && is_cross_limit_2 {
                    let dist1 = calculate_distance(change_angle(limit.0, p_angle), angle);
                    let dist2 = calculate_distance(change_angle(limit.1, p_angle), angle);

                    // let mut dist1 = (change_angle(limit.0, p_angle) - angle).abs();
                    // if dist1 > 180.0 {
                    //     dist1 = (angle - change_angle(limit.0, p_angle)).abs();
                    // }
                    // let mut dist2 = (change_angle(limit.1, p_angle) - angle).abs();
                    // if dist2 > 180.0 {
                    //     dist2 = (angle - change_angle(limit.1, p_angle)).abs();
                    // }

                    if dist1 < dist2 {
                        transform.rotation = Quat::from_rotation_z(angle_to_radian_full(
                            change_angle(limit.0, p_angle + 1.0),
                        ));
                        leg_velocity.angvel = 1.0;
                    } else {
                        transform.rotation = Quat::from_rotation_z(angle_to_radian_full(
                            change_angle(limit.1, p_angle - 1.0),
                        ));
                        leg_velocity.angvel = -1.00;
                    }
                }
            }
            None => (),
            // }
        }
    }
}
