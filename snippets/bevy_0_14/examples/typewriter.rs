use bevy::prelude::*;

const TEXT: &str = "This is some text that runs on for quite a while and occupies multiple lines. \
Let's add even more so we'll wrap onto a third line.";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup))
        .add_systems(Update, update_typewriters)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// ANCHOR: content
#[derive(Component)]
struct Typewriter(Timer);

impl Typewriter {
    fn new(delay: f32) -> Self {
        Self(Timer::from_seconds(delay, TimerMode::Repeating))
    }
}

fn update_typewriters(
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut Typewriter), With<Typewriter>>,
) {
    for (mut text, mut typewriter) in query.iter_mut() {
        if !typewriter.0.tick(time.delta()).just_finished() {
            return;
        }

        while !text.sections[1].value.is_empty() {
            // Remove a char from the section containing hidden characters and place
            // it in the section for visible characters.
            let first_hidden = text.sections[1].value.remove(0);

            text.sections[0].value.push(first_hidden);

            if first_hidden != ' ' {
                break;
            }
        }
    }
}

fn setup(mut commands: Commands) {
    let container = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .id();

    let bg = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(680.0),
                height: Val::Px(300.0),
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            border_radius: BorderRadius::all(Val::Px(10.)),
            background_color: Srgba::gray(0.2).into(),
            ..default()
        })
        .id();

    let typewriter = commands
        .spawn((
            Typewriter::new(0.1),
            TextBundle {
                style: Style {
                    width: Val::Percent(100.),
                    ..default()
                },
                text: Text::from_sections([
                    // The (initially empty) section that will contain visible text.
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle::default(),
                    },
                    // The section that contains hidden text.
                    TextSection {
                        value: TEXT.into(),
                        style: TextStyle {
                            color: Color::NONE,
                            ..default()
                        },
                    },
                ]),
                ..default()
            },
        ))
        .id();

    commands.entity(container).add_child(bg);
    commands.entity(bg).add_child(typewriter);
}
// ANCHOR_END: content
