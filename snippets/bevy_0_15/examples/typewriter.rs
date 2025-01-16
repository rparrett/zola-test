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
    commands.spawn(Camera2d::default());
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
    mut query: Query<(Entity, &mut Typewriter), With<Typewriter>>,
    mut writer: TextUiWriter,
) {
    for (entity, mut typewriter) in query.iter_mut() {
        if !typewriter.0.tick(time.delta()).just_finished() {
            return;
        }

        while !writer.text(entity, 1).is_empty() {
            // Remove a char from the span containing hidden characters and place
            // it in the section for visible characters.
            let first_hidden = writer.text(entity, 1).remove(0);

            writer.text(entity, 0).push(first_hidden);

            // I think the effect is nicer if we don't pause on spaces.
            if first_hidden != ' ' {
                break;
            }
        }
    }
}

fn setup(mut commands: Commands) {
    let container = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .id();

    let bg = commands
        .spawn((
            Node {
                width: Val::Px(680.0),
                height: Val::Px(300.0),
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor(Srgba::gray(0.2).into()),
        ))
        .id();

    let typewriter = commands
        .spawn((
            Typewriter::new(0.1),
            // The (initially empty) span that will contain visible text.
            Text::default(),
            Node {
                width: Val::Percent(100.),
                ..default()
            },
        ))
        .with_child((
            // The span that contains (initially hidden) text that will be
            // slowly revealed.
            TextSpan::new(TEXT),
            TextColor(Color::NONE),
        ))
        .id();

    commands.entity(container).add_child(bg);
    commands.entity(bg).add_child(typewriter);
}
// ANCHOR_END: content
