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
    });
}

pub fn setup_physics(
    mut commands: Commands,
    mut reapier_config: ResMut<RapierConfiguration>,
    rotation_indicators: Query<Entity, With<RotationIndicator>>,
) {
    reapier_config.gravity = Vec2::new(0.0, 0.0);

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
        &rotation_indicators,
    );

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(
            0.0,
            0.0 * -ground_height,
            0.0,
        )))
        .insert(Collider::cuboid(ground_size, ground_height))
        .insert(CustomFilterTag::GroupB);

    // reapier_config.gravity = Vec2::new(0.0, -250.0);
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
