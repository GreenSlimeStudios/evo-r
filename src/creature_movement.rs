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
    mut objects: Query<(&mut Velocity, &mut Transform, &Leg)>,
    part_datas: Query<&EntityData>,
    keys: Res<Input<KeyCode>>,
) {
    for part_data in &part_datas {
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

            match part_data.data[leg.id.1][leg.id.2].rotation_limit {
                Some(limit) => {
                    if radian_to_angle(transform.rotation.z)
                        < radian_to_angle(Quat::from_rotation_z(angle_to_radian(limit.0)).z)
                    {
                        println!("limit 1");
                        transform.rotation = Quat::from_rotation_z(angle_to_radian(limit.0));
                        leg_velocity.angvel = 0.6;
                        // transform.rotation.z = limit.0;
                        // transform.rotation.x = 0.0;
                        // transform.rotation.y = 0.0;
                        // leg_velocity.angvel = 0.0;
                    }
                    if radian_to_angle(transform.rotation.z)
                        > radian_to_angle(Quat::from_rotation_z(angle_to_radian(limit.1)).z)
                    {
                        println!("limit 2");
                        // if radian_to_angle(transform.rotation.z) > limit.1 {
                        transform.rotation = Quat::from_rotation_z(angle_to_radian(limit.1));
                        leg_velocity.angvel = -0.6;
                        // transform.rotation = Quat::from_rotation_z(limit.1);
                        // transform.rotation.z = limit.1;
                        // transform.rotation.x = 0.0;
                        // transform.rotation.y = 0.0;
                        // leg_velocity.angvel = 0.0;
                    }
                }
                None => (),
            }
        }
    }
}
