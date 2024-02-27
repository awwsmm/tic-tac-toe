use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use dimension_macro_derive::Dimension;

mod splash;
mod game;

const GRID_SPACING: f32 = 200.0;
const HALFSIZE: f32 = GRID_SPACING / 2.0;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Dimension)]
enum Row {
    Bottom,
    Middle,
    Top,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Dimension)]
enum Column {
    Left,
    Middle,
    Right
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cell {
    row: Row,
    column: Column
}

impl Cell {
    fn new(row: Row, column: Column) -> Self {
        Self { row, column }
    }
}

struct Grid {}

impl Grid {
    fn hit_square(pos: Vec2) -> Option<Cell> {
        match (Row::containing(pos.y), Column::containing(pos.x)) {
            (None, _) | (_, None) => None,
            (Some(row), Some(col)) => Some(Cell::new(row, col))
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
    window.resizable = false;
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

fn clear_entities<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

const DEBUG_UI: bool = false;

fn when_debugging<T: Default>(t: T) -> T {
    if DEBUG_UI { t } else { T::default() }
}