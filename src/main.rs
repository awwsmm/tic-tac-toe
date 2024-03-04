use std::fmt::Formatter;

use bevy::asset::AssetMetaCheck;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use dimension_macro_derive::Dimension;
use rand::prelude::*;

mod splash;
mod game;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Dimension, Component)]
enum Row {
    Bottom,
    Middle,
    Top,
}

impl Row {
    fn random() -> Self {
        let mut rng = thread_rng();
        Self::values().iter().choose(&mut rng).unwrap().clone()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Dimension, Component)]
enum Column {
    Left,
    Middle,
    Right
}

impl Column {
    fn random() -> Self {
        let mut rng = thread_rng();
        Self::values().iter().choose(&mut rng).unwrap().clone()
    }
}

#[derive(Clone, Copy)]
enum Line {
    BottomRow,
    MiddleRow,
    TopRow,
    LeftColumn,
    MiddleColumn,
    RightColumn,
    UpDiagonal,
    DownDiagonal,
}

impl Into<[Cell;3]> for Line {
    fn into(self) -> [Cell; 3] {
        match self {
            Line::BottomRow => [Cell::BOTTOM_LEFT, Cell::BOTTOM_MIDDLE, Cell::BOTTOM_RIGHT],
            Line::MiddleRow => [Cell::MIDDLE_LEFT, Cell::MIDDLE_MIDDLE, Cell::MIDDLE_RIGHT],
            Line::TopRow => [Cell::TOP_LEFT, Cell::TOP_MIDDLE, Cell::TOP_RIGHT],
            Line::LeftColumn => [Cell::TOP_LEFT, Cell::MIDDLE_LEFT, Cell::BOTTOM_LEFT],
            Line::MiddleColumn => [Cell::TOP_MIDDLE, Cell::MIDDLE_MIDDLE, Cell::BOTTOM_MIDDLE],
            Line::RightColumn => [Cell::TOP_RIGHT, Cell::MIDDLE_RIGHT, Cell::BOTTOM_RIGHT],
            Line::UpDiagonal => [Cell::BOTTOM_LEFT, Cell::MIDDLE_MIDDLE, Cell::TOP_RIGHT],
            Line::DownDiagonal => [Cell::TOP_LEFT, Cell::MIDDLE_MIDDLE, Cell::BOTTOM_RIGHT],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Component)]
struct Cell {
    row: Row,
    column: Column
}

impl Cell {
    const TOP_LEFT: Cell = Cell::new(Row::Top, Column::Left);
    const TOP_MIDDLE: Cell = Cell::new(Row::Top, Column::Middle);
    const TOP_RIGHT: Cell = Cell::new(Row::Top, Column::Right);
    const MIDDLE_LEFT: Cell = Cell::new(Row::Middle, Column::Left);
    const MIDDLE_MIDDLE: Cell = Cell::new(Row::Middle, Column::Middle);
    const MIDDLE_RIGHT: Cell = Cell::new(Row::Middle, Column::Right);
    const BOTTOM_LEFT: Cell = Cell::new(Row::Bottom, Column::Left);
    const BOTTOM_MIDDLE: Cell = Cell::new(Row::Bottom, Column::Middle);
    const BOTTOM_RIGHT: Cell = Cell::new(Row::Bottom, Column::Right);

    const fn new(row: Row, column: Column) -> Self {
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

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Component)]
enum AppState {
    #[default]
    Splash,
    Game,
}

// X is always the first player
#[derive(Resource, Component, Default, PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Mark {
    #[default]
    X,
    O
}

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mark::X => write!(f, "X"),
            Mark::O => write!(f, "O"),
        }
    }
}

impl Mark {
    fn color(&self) -> Color {
        match self {
            Mark::X => Color::RED,
            Mark::O => Color::BLUE,
        }
    }
}

#[derive(Resource, Component, Clone, Copy)]
enum GameMode {
    OnePlayer {
        human_mark: Mark // human goes first by default
    },
    TwoPlayers,
}

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never) // https://github.com/bevyengine/bevy/issues/10157#issuecomment-1849092112
        .insert_resource(GameMode::TwoPlayers)
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .init_state::<AppState>()
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

fn draw_screen<'a>(commands: &'a mut Commands, state: AppState) -> EntityCommands<'a> {
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