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
            Self::BottomRow => [Cell::BottomLeft, Cell::BottomMiddle, Cell::BottomRight],
            Self::MiddleRow => [Cell::MiddleLeft, Cell::MiddleMiddle, Cell::MiddleRight],
            Self::TopRow => [Cell::TopLeft, Cell::TopMiddle, Cell::TopRight],
            Self::LeftColumn => [Cell::TopLeft, Cell::MiddleLeft, Cell::BottomLeft],
            Self::MiddleColumn => [Cell::TopMiddle, Cell::MiddleMiddle, Cell::BottomMiddle],
            Self::RightColumn => [Cell::TopRight, Cell::MiddleRight, Cell::BottomRight],
            Self::UpDiagonal => [Cell::BottomLeft, Cell::MiddleMiddle, Cell::TopRight],
            Self::DownDiagonal => [Cell::TopLeft, Cell::MiddleMiddle, Cell::BottomRight],
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

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Cell {
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleLeft,
    MiddleMiddle,
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

impl Cell {
    fn all() -> [Self;9] {
        [
            Self::TopLeft,
            Self::TopMiddle,
            Self::TopRight,
            Self::MiddleLeft,
            Self::MiddleMiddle,
            Self::MiddleRight,
            Self::BottomLeft,
            Self::BottomMiddle,
            Self::BottomRight,
        ]
    }

    fn row(&self) -> Row {
        match self {
            Cell::TopLeft => Row::Top,
            Cell::TopMiddle => Row::Top,
            Cell::TopRight => Row::Top,
            Cell::MiddleLeft => Row::Middle,
            Cell::MiddleMiddle => Row::Middle,
            Cell::MiddleRight => Row::Middle,
            Cell::BottomLeft => Row::Bottom,
            Cell::BottomMiddle => Row::Bottom,
            Cell::BottomRight => Row::Bottom,
        }
    }

    fn column(&self) -> Column {
        match self {
            Cell::TopLeft => Column::Left,
            Cell::TopMiddle => Column::Middle,
            Cell::TopRight => Column::Right,
            Cell::MiddleLeft => Column::Left,
            Cell::MiddleMiddle => Column::Middle,
            Cell::MiddleRight => Column::Right,
            Cell::BottomLeft => Column::Left,
            Cell::BottomMiddle => Column::Middle,
            Cell::BottomRight => Column::Right,
        }
    }

    fn from(row: Row, column: Column) -> Cell {
        match row {
            Row::Bottom => match column {
                Column::Left => Cell::BottomLeft,
                Column::Middle => Cell::BottomMiddle,
                Column::Right => Cell::BottomRight,
            }
            Row::Middle => match column {
                Column::Left => Cell::MiddleLeft,
                Column::Middle => Cell::MiddleMiddle,
                Column::Right => Cell::MiddleRight,
            }
            Row::Top => match column {
                Column::Left => Cell::TopLeft,
                Column::Middle => Cell::TopMiddle,
                Column::Right => Cell::TopRight,
            }
        }
    }

    fn is_corner(&self) -> bool {
        *self == Self::TopLeft || *self == Self::TopRight || *self == Self::BottomLeft || *self == Self::BottomRight
    }
}

struct Grid {}

impl Grid {
    fn hit_square(pos: Vec2) -> Option<Cell> {
        match (Row::containing(pos.y), Column::containing(pos.x)) {
            (None, _) | (_, None) => None,
            (Some(row), Some(col)) => Some(Cell::from(row, col))
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
