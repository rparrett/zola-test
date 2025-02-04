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
    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(
            -std::f32::consts::FRAC_PI_2,
        )),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// ANCHOR: content
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_scene, setup))
        .add_observer(decorate_red_fox)
        .run();
}

/// A marker component that we will add to the particular fox that we want to be red.
#[derive(Component)]
struct RedFox;

fn decorate_red_fox(
    trigger: Trigger<SceneInstanceReady>,
    instances: Query<&SceneInstance, With<RedFox>>,
    spawner: Res<SceneSpawner>,
    mut material_handles: Query<&mut MeshMaterial3d<StandardMaterial>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(instance) = instances.get(trigger.entity()) else {
        return;
    };

    for instance_entity in spawner.iter_instance_entities(**instance) {
        let Ok(mut original_material_handle) =
            material_handles.get_mut(instance_entity)
        else {
            continue;
        };

        let Some(original_material_asset) =
            material_assets.get(&*original_material_handle)
        else {
            continue;
        };

        // This creates a new material asset every time this code runs, so
        // potentially multiple new distinct material assets for every `RedFox`
        // you spawn.
        //
        // This can be a performance concern, so you may want to find a way to
        // reuse these materials.

        let mut new_material = original_material_asset.clone();
        new_material.base_color = basic::RED.into();

        *original_material_handle = material_assets.add(new_material).into();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fox_handle = asset_server.load("models/animated/Fox.glb#Scene0");

    commands.spawn((
        SceneRoot(fox_handle.clone()),
        Transform::from_scale(Vec3::splat(0.02)).with_translation(Vec3::NEG_X),
        RedFox,
    ));

    commands.spawn((
        SceneRoot(fox_handle),
        Transform::from_scale(Vec3::splat(0.02)).with_translation(Vec3::X),
    ));
}
// ANCHOR_END: content
