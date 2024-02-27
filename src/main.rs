use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

mod splash;
mod game;

const GRID_SPACING: f32 = 200.0;
const HALFSIZE: f32 = GRID_SPACING / 2.0;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Row {
    Top,
    Middle,
    Bottom
}

impl Row {
    fn y_range(&self) -> Vec2 {
        match self {
            Row::Top => Vec2::new(HALFSIZE, 3.0*HALFSIZE),
            Row::Middle => Vec2::new(-HALFSIZE, HALFSIZE),
            Row::Bottom => Vec2::new(-3.0*HALFSIZE, -HALFSIZE)
        }
    }

    fn contains(&self, y: f32) -> bool {
        let Vec2 { x: min, y: max } = self.y_range();
        min <= y && y < max
    }

    fn in_row(y: f32) -> Option<Row> {
        if Row::Top.contains(y) {
            Some(Row::Top)
        } else if Row::Middle.contains(y) {
            Some(Row::Middle)
        } else if Row::Bottom.contains(y) {
            Some(Row::Bottom)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Column {
    Left,
    Middle,
    Right
}

impl Column {
    fn x_range(&self) -> Vec2 {
        match self {
            Column::Left => Vec2::new(-3.0*HALFSIZE, -HALFSIZE),
            Column::Middle => Vec2::new(-HALFSIZE, HALFSIZE),
            Column::Right => Vec2::new(HALFSIZE, 3.0*HALFSIZE)
        }
    }

    fn contains(&self, x: f32) -> bool {
        let Vec2 { x: min, y: max } = self.x_range();
        min <= x && x < max
    }

    fn in_column(x: f32) -> Option<Column> {
        if Column::Left.contains(x) {
            Some(Column::Left)
        } else if Column::Middle.contains(x) {
            Some(Column::Middle)
        } else if Column::Right.contains(x) {
            Some(Column::Right)
        } else {
            None
        }
    }
}

struct Grid {}

impl Grid {
    fn hit_square(pos: Vec2) -> Option<(Row, Column)> {
        match (Row::in_row(pos.y), Column::in_column(pos.x)) {
            (None, _) | (_, None) => None,
            (Some(row), Some(col)) => Some((row, col))
        }
    }
}

#[derive(Resource, Debug)]
struct MostRecentMousePosition {
    pos: Vec2
}

impl Default for MostRecentMousePosition {
    fn default() -> Self {
        Self {
            pos: Vec2::splat(f32::NAN)
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Component)]
enum GameState {
    #[default]
    Splash,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(MostRecentMousePosition::default())
        .init_state::<GameState>() // start in GameState::default()
        .add_systems(Startup, setup)
        .add_plugins((splash::plugin, game::plugin))
        .run();
}

fn setup(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
) {
    let mut window = windows.single_mut();
    window.resolution.set(800.0, 800.0);
    commands.spawn(Camera2dBundle::default());
}

fn draw_screen<'a>(commands: &'a mut Commands, state: GameState) -> EntityCommands<'a> {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            state
        ))
}

fn clear_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

const DEBUG_UI: bool = false;

fn when_debugging<T: Default>(t: T) -> T {
    if DEBUG_UI { t } else { T::default() }
}