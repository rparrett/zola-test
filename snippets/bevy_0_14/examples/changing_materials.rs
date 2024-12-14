use bevy::{
    color::palettes::basic,
    prelude::*,
    scene::{SceneInstance, SceneInstanceReady},
};

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(
            -std::f32::consts::FRAC_PI_2,
        )),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// ANCHOR: content
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<RedMaterial>()
        .add_systems(Startup, (setup_scene, setup))
        .add_systems(PostUpdate, decorate_red_fox)
        .run();
}

/// A marker component that we will add to the particular fox that we want to be red.
#[derive(Component)]
struct RedFox;

#[derive(Resource)]
struct RedMaterial(Handle<StandardMaterial>);
impl FromWorld for RedMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let red: Color = basic::RED.into();
        Self(materials.add(StandardMaterial::from(red)))
    }
}

fn decorate_red_fox(
    mut events: EventReader<SceneInstanceReady>,
    instances: Query<&SceneInstance, With<RedFox>>,
    spawner: Res<SceneSpawner>,
    mut material_query: Query<&mut Handle<StandardMaterial>>,
    red_material: Res<RedMaterial>,
) {
    for event in events.read() {
        let Ok(instance) = instances.get(event.parent) else {
            continue;
        };

        for instance_entity in spawner.iter_instance_entities(**instance) {
            let Ok(mut original_material) =
                material_query.get_mut(instance_entity)
            else {
                continue;
            };

            *original_material = red_material.0.clone();
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/animated/Fox.glb#Scene0"),
            transform: Transform::from_scale(Vec3::splat(0.02))
                .with_translation(Vec3::NEG_X),
            ..default()
        },
        RedFox,
    ));

    commands.spawn(SceneBundle {
        scene: asset_server.load("models/animated/Fox.glb#Scene0"),
        transform: Transform::from_scale(Vec3::splat(0.02))
            .with_translation(Vec3::X),
        ..default()
    });
}
// ANCHOR_END: content
