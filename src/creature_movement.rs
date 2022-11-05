use crate::*;
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
                    if transform.rotation.z < limit.0 - 0.01 {
                        transform.rotation.z = limit.0;
                        transform.rotation.x = 0.0;
                        transform.rotation.y = 0.0;
                        leg_velocity.angvel = 0.0;
                    }
                    if transform.rotation.z > limit.1 + 0.01 {
                        // transform.rotation = Quat::from_rotation_z(limit.1);
                        transform.rotation.z = limit.1;
                        transform.rotation.x = 0.0;
                        transform.rotation.y = 0.0;
                        leg_velocity.angvel = 0.0;
                    }
                }
                None => (),
            }
        }
    }
}
