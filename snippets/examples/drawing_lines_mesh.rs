use bevy::{
    prelude::*,
    render::{
        mesh::Indices, render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
    sprite::MaterialMesh2dBundle,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// ANCHOR: content
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(LineSegment {
                start: Vec2::new(0., 0.),
                end: Vec2::new(100., 200.),
                thickness: 3.0,
            }))
            .into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        ..default()
    });
}

pub struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
    pub thickness: f32,
}

impl From<LineSegment> for Mesh {
    fn from(segment: LineSegment) -> Self {
        let LineSegment {
            start,
            end,
            thickness,
        } = segment;

        let dir = (start - end).normalize();
        let perp = Vec2::new(-dir.y, dir.x);

        let s1 = start + perp * -thickness;
        let s2 = start + perp * thickness;
        let e1 = end + perp * -thickness;
        let e2 = end + perp * thickness;

        let vertices = [
            ([s1.x, s1.y, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
            ([s2.x, s2.y, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
            ([e1.x, e1.y, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
            ([e2.x, e2.y, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
        ];

        let indices = Indices::U32(vec![0, 1, 2, 1, 3, 2]);

        let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
        let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
        let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_indices(indices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }
}
// ANCHOR_END: content
