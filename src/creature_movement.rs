use crate::{
    utils::{angle_to_radian, radian_to_angle},
    *,
};
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
            let parent_part_rotation: f32 = match part_data.data[leg.id.1][leg.id.2].parent_id {
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

            match part_data.data[leg.id.1][leg.id.2].rotation_limit {
                Some(limit) => {
                    if radian_to_angle(transform.rotation.z)
                        < radian_to_angle(
                            Quat::from_rotation_z(angle_to_radian(limit.0) + parent_part_rotation)
                                .z,
                        )
                    {
                        println!("limit 1");
                        transform.rotation =
                            Quat::from_rotation_z(angle_to_radian(limit.0) + parent_part_rotation);
                        leg_velocity.angvel = 0.4;
                        // leg_velocity.angvel = leg_velocity.angvel.abs();
                        // transform.rotation.z = limit.0;
                        // transform.rotation.x = 0.0;
                        // transform.rotation.y = 0.0;
                        // leg_velocity.angvel = 0.0;
                    }
                    if radian_to_angle(transform.rotation.z)
                        > radian_to_angle(
                            Quat::from_rotation_z(angle_to_radian(limit.1) + parent_part_rotation)
                                .z,
                        )
                    {
                        println!("limit 2");
                        // if radian_to_angle(transform.rotation.z) > limit.1 {
                        transform.rotation =
                            Quat::from_rotation_z(angle_to_radian(limit.1) + parent_part_rotation);
                        leg_velocity.angvel = -0.4;
                        // leg_velocity.angvel = -leg_velocity.angvel.abs();
                        // transform.rotation = Quat::from_rotation_z(limit.1);
                        // transform.rotation.z = limit.1;
                        // transform.rotation.x = 0.0;
                        // transform.rotation.y = 0.0;
                        // leg_velocity.angvel = 0.0;
                    }
                }
                None => (),
            }
            // }
        }
    }
}
