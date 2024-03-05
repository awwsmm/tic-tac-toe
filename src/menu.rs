use bevy::prelude::*;

use crate::*;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::Menu), setup)
        .add_systems(Update, update_mark.run_if(in_state(AppState::Menu)))
        .add_systems(Update, hover_mark_button.run_if(in_state(AppState::Menu)))
        .add_systems(Update, hover_difficulty_button.run_if(in_state(AppState::Menu)))
        .add_systems(Update, hover_start_button.run_if(in_state(AppState::Menu)))
        .add_systems(Update, update_difficulty.run_if(in_state(AppState::Menu)))
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

                fn button<C: Component>(
                    text: impl Into<String>,
                    marker: C,
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
                        marker
                    )).with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                text,
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
                        button("One Player", GameMode::OnePlayer, parent, font.clone(), 60.0);

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
                            button("Easy", Difficulty::Easy, parent, font.clone(), 40.0);
                            button("Medium", Difficulty::Medium, parent, font.clone(), 40.0);
                            button("Hard", Difficulty::Hard, parent, font.clone(), 40.0);
                        });

                        parent.spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceEvenly,
                                margin: UiRect::bottom(Val::Px(30.0)),
                                ..default()
                            },
                            ..default()
                        }).with_children(|parent| {
                            button("Human X", Mark::X, parent, font.clone(), 40.0);
                            button("Human O", Mark::O, parent, font.clone(), 40.0);
                        });

                        button("Two Players", GameMode::TwoPlayers, parent, font.clone(), 60.0);
                    });
            });
    });
}

fn hover_difficulty_button(
    mut buttons: Query<(&Interaction, &mut BorderColor, &Difficulty)>,
    selected: Res<Difficulty>,
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

fn hover_mark_button(
    mut buttons: Query<(&Interaction, &mut BorderColor, &Mark)>,
    selected: Res<Mark>,
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

fn update_mark(
    query: Query<(&Interaction, &Mark), Changed<Interaction>>,
    mut mark: ResMut<Mark>,
) {
    for (interaction, new_mark) in &query {
        if let Interaction::Pressed = interaction {
            *mark = new_mark.clone();
            info!("Updated human mark to: {:?}", mark.clone());
        }
    }
}

fn update_difficulty(
    query: Query<(&Interaction, &Difficulty), Changed<Interaction>>,
    mut difficulty: ResMut<Difficulty>,
) {
    for (interaction, new_difficulty) in &query {
        if let Interaction::Pressed = interaction {
            *difficulty = new_difficulty.clone();
            info!("Updated difficulty to: {:?}", difficulty.clone());
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
            *game_mode = mode.clone();
            app_state.set(AppState::Game)
        }
    }
}
