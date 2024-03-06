use bevy::prelude::*;

use crate::{AppState, clear_entities, draw_screen, Enumerated};
use crate::settings::{Difficulty, GameMode, HumanMark, Setting};

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::Menu), setup)
        .add_systems(Update, update_setting::<HumanMark>.run_if(in_state(AppState::Menu)))
        .add_systems(Update, hover_setting_button::<HumanMark>.run_if(in_state(AppState::Menu)))
        .add_systems(Update, hover_setting_button::<Difficulty>.run_if(in_state(AppState::Menu)))
        .add_systems(Update, hover_start_button.run_if(in_state(AppState::Menu)))
        .add_systems(Update, update_setting::<Difficulty>.run_if(in_state(AppState::Menu)))
        .add_systems(Update, start.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), clear_entities::<AppState>);
}

#[derive(Component)]
struct StartGame;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/larabie.otf");

    fn word(parent: &mut ChildBuilder, word: [char; 3], font: Handle<Font>) {
        fn letter(parent: &mut ChildBuilder, letter: char, font: Handle<Font>) {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(100.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        letter,
                        TextStyle {
                            font,
                            font_size: 100.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    )
                );
            });
        }

        parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    width: Val::Px(300.0),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                letter(parent, word[0], font.clone());
                letter(parent, word[1], font.clone());
                letter(parent, word[2], font.clone());
            });
    }

    draw_screen(&mut commands, AppState::Menu).with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                word(parent, ['T', 'I', 'C'], font.clone());
                word(parent, ['T', 'A', 'C'], font.clone());
                word(parent, ['T', 'O', 'E'], font.clone());

                fn button<S: Setting>(
                    setting: S,
                    parent: &mut ChildBuilder,
                    font: Handle<Font>,
                    font_size: f32
                ) {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                border: UiRect::all(Val::Px(2.0)),
                                padding: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                            ..default()
                        },
                        AppState::Menu,
                        setting
                    )).with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                setting.to_string(),
                                TextStyle {
                                    font,
                                    font_size,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            )
                        );
                    });
                }

                parent
                    .spawn(NodeBundle {
                        style: Style {
                            height: Val::Px(300.0),
                            margin: UiRect::top(Val::Px(50.0)),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::SpaceEvenly,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        button(GameMode::OnePlayer, parent, font.clone(), 60.0);

                        fn settings_row<S: Setting>(parent: &mut ChildBuilder, font: Handle<Font>) where S: Enumerated<Item = S> {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::SpaceEvenly,
                                    ..default()
                                },
                                ..default()
                            }).with_children(|parent| {
                                for variant in S::variants() {
                                    button(variant, parent, font.clone(), 40.0);
                                }
                            });
                        }

                        settings_row::<Difficulty>(parent, font.clone());
                        settings_row::<HumanMark>(parent, font.clone());

                        // just a little bit of space to visually separate 1P and 2P modes
                        parent.spawn(NodeBundle {
                            style: Style { height: Val::Px(20.0), ..default() },
                            ..default()
                        });

                        button(GameMode::TwoPlayers, parent, font.clone(), 60.0);
                    });
            });
    });
}

fn hover_setting_button<T: Setting>(
    mut buttons: Query<(&Interaction, &mut BorderColor, &T)>,
    selected: Res<T>,
) {
    for (interaction, mut color, value) in buttons.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                *color = Color::rgba(0.0, 0.0, 0.0, 0.5).into();
            }
            _ if *value == *selected => {
                *color = Color::rgba(0.0, 0.0, 0.0, 1.0).into();
            }
            _ => { // deselect
                *color = Color::rgba(0.0, 0.0, 0.0, 0.0).into();
            }
        }
    }
}

// different from hover_setting_button because we don't want to show the "selected" game mode
fn hover_start_button(
    mut buttons: Query<(&Interaction, &mut BorderColor), With<GameMode>>,
) {
    for (interaction, mut color) in buttons.iter_mut() {
        match interaction {
            Interaction::Hovered => {
                *color = Color::rgba(0.0, 0.0, 0.0, 0.5).into();
            }
            _ => { // deselect
                *color = Color::rgba(0.0, 0.0, 0.0, 0.0).into();
            }
        }
    }
}

fn update_setting<T: Setting>(
    query: Query<(&Interaction, &T), Changed<Interaction>>,
    mut setting: ResMut<T>,
) {
    for (interaction, new_setting) in &query {
        if let Interaction::Pressed = interaction {
            *setting = *new_setting;
            info!("New setting: {}", *setting);
        }
    }
}

// When the user presses the "One Player" / "Two Players" button, start the game in OnePlayer / TwoPlayers mode
fn start(
    mut query: Query<(&Interaction, &GameMode), Changed<Interaction>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_mode: ResMut<GameMode>,
) {
    for (interaction, mode) in &mut query {
        if let Interaction::Pressed = interaction {
            *game_mode = *mode;
            app_state.set(AppState::Game)
        }
    }
}
