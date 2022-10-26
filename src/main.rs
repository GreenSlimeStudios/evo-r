mod entity_constructor;
mod entity_modification;
mod entity_selection;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use entity_constructor::*;
use entity_modification::*;
use entity_selection::*;

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
        .add_plugin(PartSelectionPlugin)
        .add_plugin(CreatureConstructorPlugin)
        .add_plugin(CreatureModificationPlugin)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(move_objects)
        .add_system(move_camera_system)
        .add_system(toggle_gravity)
        // .add_system(edit_selected_parts_system)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..default()
    });
}

// pub struct PartConstructorData {
//     // joint_entity: Entity,
//     // joint_builder: RevoluteJointBuilder,
//     pub parts: Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>,
// }

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

    let parent_data: ParentData = ParentData {
        position: entity_pos,
        size: Vec2::new(100.0, 30.0),
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
        .insert(ParentData {
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
    mut parent: Query<(&mut Transform, &mut Velocity, &ParentData), With<ParentData>>,
    mut legs: Query<(&mut Transform, &mut Velocity), Without<ParentData>>,
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
