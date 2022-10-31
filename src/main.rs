mod camera_movment;
mod collision_map;
mod creature_movement;
mod entity_constructor;
mod entity_modification;
mod entity_selection;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use camera_movment::*;
use collision_map::*;
use creature_movement::*;
use entity_constructor::*;
use entity_modification::*;
use entity_selection::*;

const GROUP_SIZE: usize = 10;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            title: "evo-r".to_string(),
            fit_canvas_to_parent: true,
            ..default()
        })
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        // .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(CollisionMapPlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(PartSelectionPlugin)
        .add_plugin(CreatureConstructorPlugin)
        .add_plugin(CreatureModificationPlugin)
        .add_plugin(CreatureMovmentPlugin)
        .add_plugin(CameraAdditionsPlugin)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..default()
    }); // .insert(Body);
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
        id: 0,
        position: entity_pos,
        size: Vec2::new(100.0, 30.0),
    };

    // let mut parents: Vec<Entity> = Vec::new();

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
            id: parent_data.id,
            size: parent_data.size,
            position: parent_data.position,
        })
        .id();

    let mut part_datas: Vec<Vec<PartData>> = Vec::new();
    let mut parts: Vec<Vec<Vec<(Entity, RevoluteJointBuilder, Entity, RevoluteJointBuilder)>>> =
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
        0,
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
        .insert(EntityParts { parts })
        .insert(SelectedEntity {
            parent: true,
            parts: None,
        })
        .insert(Name::new("entity data"));
}

pub fn to_vec2(vec3: &Vec3) -> Vec2 {
    Vec2::new(vec3.x, vec3.y)
}
