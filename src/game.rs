use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use bevy::utils::hashbrown::hash_map::Entry;

use crate::*;

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
    current_player: Mark,
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
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::XTurn), start_x_turn)
        .add_systems(Update, capture_input.run_if(in_state(GameState::XTurn)))
        .add_systems(OnEnter(GameState::OTurn), start_o_turn)
        .add_systems(Update, capture_input.run_if(in_state(GameState::OTurn)))
        .add_systems(OnEnter(GameState::GameOver), game_over)
        .add_systems(Update, game_over_buttons.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), clear_entities::<Mark>)
        .add_systems(OnExit(GameState::GameOver), clear_entities::<GameOverOverlay>)
        .add_systems(OnExit(AppState::Game), clear_entities::<AppState>)
        .add_systems(OnExit(AppState::Game), clear_entities::<GameOverOverlay>);
}

fn start_x_turn(mut info: ResMut<StateInfo>) {
    info.current_player = Mark::X
}

fn start_o_turn(mut info: ResMut<StateInfo>) {
    info.current_player = Mark::O
}

fn start_game(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<GameState>>
) {

    next_game_state.set(GameState::XTurn);

    const GRID_SPACING: f32 = 250.0;

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
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
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
            background_color: Color::rgba(1.0, 1.0, 1.0, 0.85).into(),
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
                fn spawn_text(
                    parent: &mut ChildBuilder,
                    text: impl Into<String>,
                    font: Handle<Font>,
                    text_color: Color
                ) {
                    parent.spawn(TextBundle::from_section(
                        text,
                        TextStyle {
                            color: text_color,
                            font_size: 75.0,
                            font: font.clone(),
                            ..default()
                        }
                    ));
                }

                match info.winner {
                    None => {
                        spawn_text(parent, "It's a tie!", font.clone(), Color::BLACK);
                    }
                    Some((winner, _)) => {
                        spawn_text(parent, format!("{}", winner.to_string()), font.clone(), winner.color());
                        spawn_text(parent, " wins!", font.clone(), Color::BLACK);
                    }
                }
            });

            fn button(parent: &mut ChildBuilder, text: impl Into<String>, color: Color, marker: GameOverButton, font: Handle<Font>) {
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
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
                            font_size: 60.0,
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

fn capture_user_input(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    touch_input: Res<Touches>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) -> Option<Cell> {

    // expect() because we spawn only a single Camera2dBundle and expect Bevy to be able to provide it to us
    let (camera, camera_transform) = cameras.get_single().expect("expected exactly one camera");

    // get touch input from users on mobile
    let maybe_touch_coordinates: Option<Vec2> =
        touch_input.iter()
            .filter(|finger| touch_input.just_pressed(finger.id()))
            .next()
            .map(|finger| finger.position());

    // get mouse input from users on desktop
    let maybe_click_coordinates: Option<Vec2> =
        windows.get_single().iter()
            .filter(|_| mouse_button_input.just_pressed(MouseButton::Left))
            .next()
            .and_then(|window| window.cursor_position());

    maybe_touch_coordinates.or(maybe_click_coordinates)
        .and_then(|window_coordinates| camera.viewport_to_world_2d(camera_transform, window_coordinates))
        .and_then(|world_coordinates| Grid::hit_square(world_coordinates))
}

fn generate_computer_input() -> Cell {
    // TODO implement minimax algorithm for computer player
    //      https://www.neverstopbuilding.com/blog/minimax
    Cell::new(Row::random(), Column::random())
}

fn capture_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut info: ResMut<StateInfo>,
    cells: Query<(Entity, &Cell)>,
    touch_input: Res<Touches>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    game_mode: Res<GameMode>,
) {

    // if the winner has already been decided, we should ignore user input until a new game is started
    if info.winner.is_some() { return; }

    // either "X" or "O"
    let mark = info.current_player;

    let maybe_cell = match *game_mode {
        GameMode::OnePlayer { human_mark } if human_mark != mark => Some(generate_computer_input()),
        _ => capture_user_input(windows, cameras, touch_input, mouse_button_input)
    };

    // the computer will always select a cell, but the human might not
    let Some(cell) = maybe_cell else { return; };

    // If the user / the computer did click on a cell...
    match info.marks.entry(cell) {
        Entry::Occupied(_) => {
            warn!("this cell is already occupied")
        }
        Entry::Vacant(_) => {

            // ...get a handle to the cell clicked
            let (entity, cell) = cells.iter().filter(|(_, c)| c == &&cell).next().expect("could not find clicked cell in all cells");

            // ...and mark the cell as clicked by that player
            info.marks.insert(cell.clone(), Some(mark));
            info!("Hit the {:?} cell", cell);

            // draw the mark on the board
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        mark.to_string(),
                        TextStyle {
                            font_size: 200.0,
                            font: asset_server.load("fonts/larabie.otf"),
                            color: mark.color(),
                            ..default()
                        }
                    ),
                    mark // tag the entity with the Mark Component
                ));
            });

            // determine whether or not there is a winner
            info.winner = info.determine_winner();

            match info.winner {
                // if there is no winner...
                None => {
                    // ...keep playing
                    if info.marks.len() < 9 {
                        match *current_game_state.get() {
                            GameState::XTurn => next_game_state.set(GameState::OTurn),
                            GameState::OTurn => next_game_state.set(GameState::XTurn),
                            GameState::GameOver => unreachable!("called capture_input() in GameOver state"),
                            GameState::GameNotInProgress => unreachable!("called capture_input() in GameNotInProgress state"),
                        }
                        // ...unless there are now 9 marks on the board, in which case the game ends in a tie
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