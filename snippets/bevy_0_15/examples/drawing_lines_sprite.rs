use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

// ANCHOR: content
fn line_segment(
    start: Vec2,
    end: Vec2,
    thickness: f32,
    color: Color,
) -> impl Bundle {
    let length = start.distance(end);
    let diff = start - end;
    let theta = diff.y.atan2(diff.x);
    let midpoint = (start + end) / 2.;

    let transform = Transform::from_xyz(midpoint.x, midpoint.y, 0.)
        .with_rotation(Quat::from_rotation_z(theta));

    (
        Sprite {
            color,
            custom_size: Some(Vec2::new(length, thickness)),
            ..default()
        },
        transform,
    )
}

fn setup(mut commands: Commands) {
    commands.spawn(line_segment(
        Vec2::new(0.0, 0.0),
        Vec2::new(100.0, 200.0),
        3.0,
        Color::WHITE,
    ));
}
// ANCHOR_END: content
