use std::fmt::Formatter;

use bevy::asset::AssetMetaCheck;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use dimension_macro_derive::Dimension;
use rand::prelude::*;

mod menu;
mod game;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Dimension, Component)]
enum Row {
    Bottom,
    Middle,
    Top,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Dimension, Component)]
enum Column {
    Left,
    Middle,
    Right
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
            Self::BottomRow => [Cell::BOTTOM_LEFT, Cell::BOTTOM_MIDDLE, Cell::BOTTOM_RIGHT],
            Self::MiddleRow => [Cell::MIDDLE_LEFT, Cell::MIDDLE_MIDDLE, Cell::MIDDLE_RIGHT],
            Self::TopRow => [Cell::TOP_LEFT, Cell::TOP_MIDDLE, Cell::TOP_RIGHT],
            Self::LeftColumn => [Cell::TOP_LEFT, Cell::MIDDLE_LEFT, Cell::BOTTOM_LEFT],
            Self::MiddleColumn => [Cell::TOP_MIDDLE, Cell::MIDDLE_MIDDLE, Cell::BOTTOM_MIDDLE],
            Self::RightColumn => [Cell::TOP_RIGHT, Cell::MIDDLE_RIGHT, Cell::BOTTOM_RIGHT],
            Self::UpDiagonal => [Cell::BOTTOM_LEFT, Cell::MIDDLE_MIDDLE, Cell::TOP_RIGHT],
            Self::DownDiagonal => [Cell::TOP_LEFT, Cell::MIDDLE_MIDDLE, Cell::BOTTOM_RIGHT],
        }
    }
}

impl Line {
    fn all() -> [Self;8] {
        [
            Self::BottomRow,
            Self::MiddleRow,
            Self::TopRow,
            Self::LeftColumn,
            Self::MiddleColumn,
            Self::RightColumn,
            Self::UpDiagonal,
            Self::DownDiagonal,
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Component)]
struct Cell {
    row: Row,
    column: Column
}

impl Cell {
    const fn new(row: Row, column: Column) -> Self {
        Self { row, column }
    }

    const TOP_LEFT: Self = Self::new(Row::Top, Column::Left);
    const TOP_MIDDLE: Self = Self::new(Row::Top, Column::Middle);
    const TOP_RIGHT: Self = Self::new(Row::Top, Column::Right);
    const MIDDLE_LEFT: Self = Self::new(Row::Middle, Column::Left);
    const MIDDLE_MIDDLE: Self = Self::new(Row::Middle, Column::Middle);
    const MIDDLE_RIGHT: Self = Self::new(Row::Middle, Column::Right);
    const BOTTOM_LEFT: Self = Self::new(Row::Bottom, Column::Left);
    const BOTTOM_MIDDLE: Self = Self::new(Row::Bottom, Column::Middle);
    const BOTTOM_RIGHT: Self = Self::new(Row::Bottom, Column::Right);

    fn all() -> [Self;9] {
        [
            Self::TOP_LEFT,
            Self::TOP_MIDDLE,
            Self::TOP_RIGHT,
            Self::MIDDLE_LEFT,
            Self::MIDDLE_MIDDLE,
            Self::MIDDLE_RIGHT,
            Self::BOTTOM_LEFT,
            Self::BOTTOM_MIDDLE,
            Self::BOTTOM_RIGHT,
        ]
    }

    fn is_corner(&self) -> bool {
        *self == Self::TOP_LEFT || *self == Self::TOP_RIGHT || *self == Self::BOTTOM_LEFT || *self == Self::BOTTOM_RIGHT
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
    Menu,
    Game,
}

// when used as a Resource, Mark is the human player (the other player is the computer)
#[derive(Component, Default, PartialEq, Eq, Debug, Clone, Copy, Hash)]
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

#[derive(Resource, Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
enum HumanMark {
    #[default]
    HumanX,
    HumanO
}

impl HumanMark {
    fn is(&self, mark: Mark) -> bool {
        match self {
            HumanMark::HumanX if mark == Mark::X => true,
            HumanMark::HumanO if mark == Mark::O => true,
            _ => false
        }
    }
}

#[derive(Resource, Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Difficulty {
    Easy,
    Medium,
    #[default]
    Hard,
}

#[derive(Resource, Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
enum GameMode {
    OnePlayer,
    #[default]
    TwoPlayers,
}

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never) // https://github.com/bevyengine/bevy/issues/10157#issuecomment-1849092112
        .insert_resource(GameMode::default())
        .insert_resource(HumanMark::default())
        .insert_resource(Difficulty::default())
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins((menu::plugin, game::plugin))
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
