use bevy::asset::AssetMetaCheck;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

mod menu;
mod game;

#[derive(States, Component, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Menu,
    Game,
}

#[derive(Resource, Component, Clone, Copy, Default, PartialEq, Eq)]
enum HumanMark {
    #[default]
    HumanX,
    HumanO
}

impl std::fmt::Display for HumanMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            HumanMark::HumanX => "Human X",
            HumanMark::HumanO => "Human O",
        })
    }
}

#[derive(Resource, Component, Clone, Copy, Default, PartialEq, Eq)]
enum Difficulty {
    Easy,
    Medium,
    #[default]
    Hard,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        })
    }
}

#[derive(Resource, Component, Clone, Copy, Default, PartialEq, Eq)]
enum GameMode {
    OnePlayer,
    #[default]
    TwoPlayers,
}

impl std::fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            GameMode::OnePlayer => "One Player",
            GameMode::TwoPlayers => "Two Players",
        })
    }
}

trait Enumerated {
    type Item;
    const CARDINALITY: usize;
    fn variants() -> Vec<Self::Item>;
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
