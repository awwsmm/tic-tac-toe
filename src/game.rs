use std::fmt::Formatter;

use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy::utils::hashbrown::hash_map::Entry;

use crate::*;

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

#[derive(States, Clone, Hash, PartialEq, Eq, Debug, Default)]
enum GameState {
    #[default]
    GameNotInProgress,
    XTurn,
    OTurn,
    GameOver
}

#[derive(Resource, Default)]
struct StateInfo {
    marks: HashMap<Cell, Option<Mark>>,
    winner: Option<(Mark, (Cell, Cell))>,
    current_player: Option<Mark>
}

impl StateInfo {
    fn determine_winner(&self) -> Option<(Mark, (Cell, Cell))> {
        let winning_arrangements: [(fn(&(&Cell, &Option<Mark>)) -> bool, (Cell, Cell)); 8] = [
            (|(Cell { row, .. }, _)| *row == Row::Top, (Cell::new(Row::Top, Column::Left), Cell::new(Row::Top, Column::Right))),
            (|(Cell { row, .. }, _)| *row == Row::Middle, (Cell::new(Row::Middle, Column::Left), Cell::new(Row::Middle, Column::Right))),
            (|(Cell { row, .. }, _)| *row == Row::Bottom, (Cell::new(Row::Bottom, Column::Left), Cell::new(Row::Bottom, Column::Right))),
            (|(Cell { column, .. }, _)| *column == Column::Left, (Cell::new(Row::Top, Column::Left), Cell::new(Row::Bottom, Column::Left))),
            (|(Cell { column, .. }, _)| *column == Column::Middle, (Cell::new(Row::Top, Column::Middle), Cell::new(Row::Bottom, Column::Middle))),
            (|(Cell { column, .. }, _)| *column == Column::Right, (Cell::new(Row::Top, Column::Right), Cell::new(Row::Bottom, Column::Right))),
            (|(Cell { row, column }, _)| column.position() == row.position(), (Cell::new(Row::Bottom, Column::Left), Cell::new(Row::Top, Column::Right))),
            (|(Cell { row, column }, _)| column.position() == -row.position(), (Cell::new(Row::Top, Column::Left), Cell::new(Row::Bottom, Column::Right))),
        ];

        for (arrangement, line) in winning_arrangements {
            let marks = self.marks.iter()
                .filter(arrangement)
                .flat_map(|(_, mark)| *mark)
                .collect::<Vec<Mark>>();

            let unique_marks = marks.iter().cloned()
                .collect::<HashSet<Mark>>();

            if marks.len() == 3 && unique_marks.len() == 1 {
                return Some((marks.get(0).unwrap().clone(), line));
            }
        }

        None
    }
}

pub fn plugin(app: &mut App) {
    app
        .insert_resource(Mark::default())
        .insert_resource(StateInfo::default())
        .add_systems(OnEnter(AppState::Game), start_game)
        .add_systems(PostUpdate, save_most_recent_mouse_position.run_if(in_state(AppState::Game)))
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::XTurn), start_x_turn)
        .add_systems(Update, capture_clicks.run_if(in_state(GameState::XTurn)))
        .add_systems(Update, capture_touches.run_if(in_state(GameState::XTurn)))
        .add_systems(OnEnter(GameState::OTurn), start_o_turn)
        .add_systems(Update, capture_clicks.run_if(in_state(GameState::OTurn)))
        .add_systems(Update, capture_touches.run_if(in_state(GameState::OTurn)))
        .add_systems(OnEnter(GameState::GameOver), game_over)
        .add_systems(Update, game_over_buttons.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), clear_entities::<Mark>)
        .add_systems(OnExit(GameState::GameOver), clear_entities::<GameOverOverlay>)
        .add_systems(OnExit(AppState::Game), clear_entities::<AppState>)
        .add_systems(OnExit(AppState::Game), clear_entities::<GameOverOverlay>);
}

fn start_x_turn(mut info: ResMut<StateInfo>) {
    info.current_player = Some(Mark::X)
}

fn start_o_turn(mut info: ResMut<StateInfo>) {
    info.current_player = Some(Mark::O)
}

fn start_game(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>
) {

    next_game_state.set(GameState::XTurn);

    const GRID_SPACING: f32 = 200.0;

    fn cell<'a>(parent: &'a mut ChildBuilder, cell: Cell, border: UiRect) -> EntityCommands<'a> {
        parent.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_row: GridPlacement::start((-cell.row.position() + 2) as i16),
                    grid_column: GridPlacement::start((cell.column.position() + 2) as i16),
                    justify_items: JustifyItems::Center,
                    align_items: AlignItems::Center,
                    border,
                    ..default()
                },
                border_color: Color::BLACK.into(),
                ..default()
            },
            cell
        ))
    }

    draw_screen(&mut commands, AppState::Game).with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::flex(1.0), GridTrack::flex(1.0)],
                grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::flex(1.0), GridTrack::flex(1.0)],
                width: Val::Px(3.0 * GRID_SPACING),
                height: Val::Px(3.0 * GRID_SPACING),
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            const NONE: Val = Val::ZERO;
            const THIN: Val = Val::Px(6.0);

            // top row
            cell(parent, Cell::new(Row::Top, Column::Left), UiRect::new(NONE, THIN, NONE, THIN));
            cell(parent, Cell::new(Row::Top, Column::Middle), UiRect::new(NONE, NONE, NONE, THIN));
            cell(parent, Cell::new(Row::Top, Column::Right), UiRect::new(THIN, NONE, NONE, THIN));

            // middle row
            cell(parent, Cell::new(Row::Middle, Column::Left), UiRect::new(NONE, THIN, NONE, NONE));
            cell(parent, Cell::new(Row::Middle, Column::Middle), UiRect::new(NONE, NONE, NONE, NONE));
            cell(parent, Cell::new(Row::Middle, Column::Right), UiRect::new(THIN, NONE, NONE, NONE));

            // bottom row
            cell(parent, Cell::new(Row::Bottom, Column::Left), UiRect::new(NONE, THIN, THIN, NONE));
            cell(parent, Cell::new(Row::Bottom, Column::Middle), UiRect::new(NONE, NONE, THIN, NONE));
            cell(parent, Cell::new(Row::Bottom, Column::Right), UiRect::new(THIN, NONE, THIN, NONE));
        });
    });
}

#[derive(Component)]
enum GameOverButton {
    PlayAgain,
    BackToMenu
}

#[derive(Component)]
struct GameOverOverlay {}

fn game_over(
    mut commands: Commands,
    info: Res<StateInfo>,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/larabie.otf");

    // entire screen
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                left: Val::ZERO,
                top: Val::ZERO,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
            z_index: ZIndex::Global(1),
            ..default()
        },
        GameOverOverlay {}
    )).with_children(|parent| {

        // inner window
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(61.8),
                border: when_debugging(UiRect::all(Val::Px(1.0))),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(1.0, 1.0, 1.0, 0.75).into(),
            border_color: when_debugging(Color::RED.into()),
            ..default()
        }).with_children(|parent| {

            // top row
            parent.spawn(NodeBundle {
                style: Style {
                    border: when_debugging(UiRect::all(Val::Px(1.0))),
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                border_color: when_debugging(Color::GREEN.into()),
                ..default()
            }).with_children(|parent| {
                match info.winner {
                    None => {
                        parent.spawn(TextBundle::from_section(
                            "It's a tie!",
                            TextStyle {
                                color: Color::BLACK,
                                font_size: 50.0,
                                font: font.clone(),
                                ..default()
                            }
                        ));
                    }
                    Some((winner, _)) => {
                        parent.spawn(TextBundle::from_section(
                            format!("{}", winner.to_string()),
                            TextStyle {
                                color: info.winner.unwrap().0.color(),
                                font_size: 50.0,
                                font: font.clone(),
                                ..default()
                            }
                        ));
                        parent.spawn(TextBundle::from_section(
                            " wins!",
                            TextStyle {
                                color: Color::BLACK,
                                font_size: 50.0,
                                font: font.clone(),
                                ..default()
                            }
                        ));
                    }
                }
            });

            // bottom row
            parent.spawn(NodeBundle {
                style: Style {
                    border: when_debugging(UiRect::all(Val::Px(1.0))),
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                border_color: when_debugging(Color::BLUE.into()),
                ..default()
            }).with_children(|parent| {
                fn button(parent: &mut ChildBuilder, text: impl Into<String>, color: Color, marker: GameOverButton, font: Handle<Font>) {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                            ..default()
                        },
                        marker
                    )).with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            text,
                            TextStyle {
                                color,
                                font_size: 30.0,
                                font: font.clone(),
                                ..default()
                            }
                        ));
                    });
                }

                button(parent, "play again", Color::BLUE, GameOverButton::PlayAgain, font.clone());
                button(parent, "back to menu", Color::RED, GameOverButton::BackToMenu, font.clone());
            });
        });
    });
}

fn game_over_buttons(
    buttons: Query<(&Interaction, &GameOverButton), (Changed<Interaction>, With<Button>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut info: ResMut<StateInfo>,
) {
    for (interaction, button) in buttons.iter() {
        if let Interaction::Pressed = interaction {
            match button {
                GameOverButton::PlayAgain => {
                    *info = StateInfo::default();
                    next_game_state.set(GameState::XTurn);
                }
                GameOverButton::BackToMenu => {
                    *info = StateInfo::default();
                    next_game_state.set(GameState::GameNotInProgress);
                    next_app_state.set(AppState::Splash);
                }
            }
        }
    }

}

fn save_most_recent_mouse_position(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut most_recent_mouse_position: ResMut<MostRecentMousePosition>,
    windows: Query<&Window>,
) {
    match windows.get_single() {
        Ok(window) => {
            let (ww, wh) = (window.resolution.width(), window.resolution.height());

            // update the most_recent_mouse_position, using game coordinates (origin at center of screen)
            for event in cursor_moved_events.read() {
                let x = event.position.x - ww / 2.0;
                let y = -event.position.y + wh / 2.0;

                most_recent_mouse_position.pos = Vec2::new(x, y);
            }
        }
        Err(_) => {
            warn!("Tried to save_most_recent_mouse_position, but there is no Window")
        }
    }
}

fn process_input(
    info: &mut ResMut<StateInfo>,
    cell: Cell,
    cells: &Query<Entity, With<Cell>>,
    query: &Query<&Cell>,
    commands: &mut Commands,
    font: Handle<Font>,
    current_game_state: &Res<State<GameState>>,
    next_game_state: &mut ResMut<NextState<GameState>>,
) {
    match info.marks.entry(cell) {
        Entry::Occupied(_) => {
            warn!("this cell is already occupied")
        }
        Entry::Vacant(_) => {
            let entities = cells.iter().filter(|e| query.get(*e).unwrap() == &cell).collect::<Vec<Entity>>();

            match entities.get(0) {
                None => {
                    info!("pressed back to menu -- error here")
                }
                Some(entity) => {
                    let cell = query.get(*entity).unwrap().clone();
                    let mark = info.current_player.unwrap();

                    commands.entity(*entity).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                mark.to_string(),
                                TextStyle {
                                    font_size: 200.0,
                                    font: font.clone(),
                                    color: mark.color(),
                                    ..default()
                                }
                            ),
                            mark
                        ));
                    });

                    info.marks.insert(cell, Some(mark));
                    info!("Hit the {:?} cell", cell);
                    info.winner = info.determine_winner();

                    match info.winner {
                        None => {
                            if info.marks.len() < 9 {
                                match *current_game_state.get() {
                                    GameState::XTurn => next_game_state.set(GameState::OTurn),
                                    GameState::OTurn => next_game_state.set(GameState::XTurn),
                                    GameState::GameOver => unreachable!("entered capture_clicks() in GameOver state"),
                                    GameState::GameNotInProgress => unreachable!("entered capture_clicks() in GameNotInProgress state"),
                                }
                            } else {
                                info!("The game ends in a tie");
                                next_game_state.set(GameState::GameOver)
                            }
                        }
                        Some((mark, (from, to))) => {
                            info!("The winner is {:?} along the line {:?} -> {:?}", mark, from, to);
                            next_game_state.set(GameState::GameOver)
                        }
                    }
                }
            }
        }
    }
}

fn capture_touches(
    touches: Res<Touches>,
    mut commands: Commands,
    mut info: ResMut<StateInfo>,
    asset_server: Res<AssetServer>,
    cells: Query<Entity, With<Cell>>,
    query: Query<&Cell>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>
) {
    if info.winner.is_none() {
        let font = asset_server.load("fonts/larabie.otf");
        for finger in touches.iter() {
            if touches.just_pressed(finger.id()) {
                if let Some(cell) = Grid::hit_square(finger.position()) {
                    process_input(
                        &mut info,
                        cell,
                        &cells,
                        &query,
                        &mut commands,
                        font.clone(),
                        &current_game_state,
                        &mut next_game_state,
                    )
                }
            }
        }
    }
}

fn capture_clicks(
    mouse_button_input_events: Res<ButtonInput<MouseButton>>,
    most_recent_mouse_position: Res<MostRecentMousePosition>,
    mut commands: Commands,
    mut info: ResMut<StateInfo>,
    asset_server: Res<AssetServer>,
    cells: Query<Entity, With<Cell>>,
    query: Query<&Cell>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>
) {
    if info.winner.is_none() {
        let font = asset_server.load("fonts/larabie.otf");

        if mouse_button_input_events.just_pressed(MouseButton::Left) {
            if let Some(cell) = Grid::hit_square(most_recent_mouse_position.pos) {
                process_input(
                    &mut info,
                    cell,
                    &cells,
                    &query,
                    &mut commands,
                    font.clone(),
                    &current_game_state,
                    &mut next_game_state,
                )
            }
        }
    }
}
