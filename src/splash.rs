use bevy::prelude::*;

use crate::*;

pub fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::Splash), setup)
        .add_systems(Update, start.run_if(in_state(AppState::Splash)))
        .add_systems(OnExit(AppState::Splash), clear_entities::<AppState>);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/larabie.otf");

    fn word(parent: &mut ChildBuilder, word: [char; 3], font: Handle<Font>) {
        fn letter(parent: &mut ChildBuilder, letter: char, font: Handle<Font>) {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        border: when_debugging(UiRect::all(Val::Px(1.0))),
                        width: Val::Px(100.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    border_color: when_debugging(Color::BLUE.into()),
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
                    border: when_debugging(UiRect::all(Val::Px(1.0))),
                    ..default()
                },
                border_color: when_debugging(Color::RED.into()),
                ..default()
            }).with_children(|parent| {

            letter(parent, word[0], font.clone());
            letter(parent, word[1], font.clone());
            letter(parent, word[2], font.clone());
        });
    }

    draw_screen(&mut commands, AppState::Splash).with_children(|parent| {
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

                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            margin: UiRect::top(Val::Px(100.0)),
                            border: when_debugging(UiRect::all(Val::Px(1.0))),
                            ..default()
                        },
                        border_color: when_debugging(Color::INDIGO.into()),
                        background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                        ..default()
                    },
                    AppState::Splash
                )).with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "START",
                            TextStyle {
                                font,
                                font_size: 60.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        )
                    );
                });
            });
    });
}

fn start(
    mut query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for interaction in &mut query {
        match interaction {
            Interaction::Pressed => {
                app_state.set(AppState::Game)
            },
            Interaction::Hovered => {},
            Interaction::None => {},
        }
    }
}

