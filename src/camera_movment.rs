use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CameraAdditionsPlugin;
impl Plugin for CameraAdditionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_points_system)
            .add_system(move_camera_system);
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

fn spawn_points_system(mut commands: Commands) {
    for i in -10..1000 {
        commands
            .spawn_bundle(TransformBundle {
                local: Transform::from_xyz((100 * i) as f32, -40.0, 0.0),
                ..default()
            })
            .insert(Collider::cuboid(10.0, -20.0));
    }
}
