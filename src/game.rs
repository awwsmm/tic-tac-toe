use bevy::prelude::*;

use crate::*;

#[derive(States, Clone, Hash, PartialEq, Eq, Debug, Default)]
enum GameState {
    #[default]
    GameNotInProgress,
    XTurn,
    OTurn,
    GameOver
}

// Game is inside game module so private fields of Game cannot be accessed / mutated directly
mod game {
    use bevy::utils::{HashMap, HashSet};

    use crate::{Cell, Column, Line, Mark, Row};

    // All of Game's fields are private so that we can recalculate the winner when a new mark is made on the board
    // impl Default is required for impl Default on StateInfo
    #[derive(Default)]
    pub struct Game {
        marks: HashMap<Cell, Option<Mark>>,
        winner: Option<(Mark, Line)>,
        over: bool
    }

    impl Game {
        const WINNING_ARRANGEMENTS: [(fn(&(&Cell, &Option<Mark>)) -> bool, Line); 8] = [
            (|(Cell { row, .. }, _)| *row == Row::Top, Line::TopRow),
            (|(Cell { row, .. }, _)| *row == Row::Middle, Line::MiddleRow),
            (|(Cell { row, .. }, _)| *row == Row::Bottom, Line::BottomRow),
            (|(Cell { column, .. }, _)| *column == Column::Left, Line::LeftColumn),
            (|(Cell { column, .. }, _)| *column == Column::Middle, Line::MiddleColumn),
            (|(Cell { column, .. }, _)| *column == Column::Right, Line::RightColumn),
            (|(Cell { row, column }, _)| column.position() == row.position(), Line::UpDiagonal),
            (|(Cell { row, column }, _)| column.position() == -row.position(), Line::DownDiagonal),
        ];

        fn determine_winner(marks: &HashMap<Cell, Option<Mark>>) -> Option<(Mark, Line)> {
            for (arrangement, line) in Self::WINNING_ARRANGEMENTS {
                let marks = marks.iter()
                    .filter(arrangement)
                    .flat_map(|(_, mark)| *mark)
                    .collect::<Vec<Mark>>();

                let unique_marks = marks.iter().cloned()
                    .collect::<HashSet<Mark>>();

                if marks.len() == 3 && unique_marks.len() == 1 {
                    return Some((marks.get(0).unwrap().clone(), line))
                };
            }

            None
        }

        // behind a getter so the user cannot mutate this field directly
        pub fn winner(&self) -> Option<(Mark, Line)> {
            self.winner
        }

        // behind a getter so the user cannot mutate this field directly
        pub fn over(&self) -> bool {
            self.over
        }

        // behind a getter so the user cannot access / mutate marks directly
        pub fn get(&self, cell: Cell) -> Option<Mark> {
            self.marks.get(&cell).cloned().flatten()
        }

        // behind a setter so we can recalculate the winner immediately
        pub fn set(&mut self, cell: Cell, mark: Mark) {
            self.marks.insert(cell, Some(mark));
            self.winner = Game::determine_winner(&self.marks);
            self.over = self.winner.is_some() || self.marks.len() == 9;
        }
    }
}

#[derive(Resource, Default)]
struct StateInfo {
    game: game::Game,
    current_player: Mark,
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
            cell(parent, Cell::TOP_LEFT, UiRect::new(NONE, THIN, NONE, THIN));
            cell(parent, Cell::TOP_MIDDLE, UiRect::new(NONE, NONE, NONE, THIN));
            cell(parent, Cell::TOP_RIGHT, UiRect::new(THIN, NONE, NONE, THIN));

            // middle row
            cell(parent, Cell::MIDDLE_LEFT, UiRect::new(NONE, THIN, NONE, NONE));
            cell(parent, Cell::MIDDLE_MIDDLE, UiRect::new(NONE, NONE, NONE, NONE));
            cell(parent, Cell::MIDDLE_RIGHT, UiRect::new(THIN, NONE, NONE, NONE));

            // bottom row
            cell(parent, Cell::BOTTOM_LEFT, UiRect::new(NONE, THIN, THIN, NONE));
            cell(parent, Cell::BOTTOM_MIDDLE, UiRect::new(NONE, NONE, THIN, NONE));
            cell(parent, Cell::BOTTOM_RIGHT, UiRect::new(THIN, NONE, THIN, NONE));
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
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(1.0, 1.0, 1.0, 0.85).into(),
            ..default()
        }).with_children(|parent| {

            // top row
            parent.spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
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

                match info.game.winner() {
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
                    next_app_state.set(AppState::Menu);
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

fn generate_computer_input(game: &game::Game, computer: Mark, difficulty: Difficulty) -> Cell {

    // weight cells based on their advantage to the computer and their disadvantage to the human
    //
    //   1. +20 for any cell which lets the computer win this turn
    //   2. +10 for any cell which blocks a human win this turn
    //   3. +2 for the middle-middle space
    //   4. +1 for any corner space
    //
    // ...then, just pick the cell with the highest weight, after filtering out already-occupied cells

    let mut weights: [i8;9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    // scale weights based on difficulty, so the computer picks non-optimal moves

    let scale = match difficulty {
        Difficulty::Easy => -1, // purposefully pick the worst possible moves
        Difficulty::Medium => {
            // randomly pick best-possible and worst-possible moves
            let mut rng = thread_rng();
            [-1, 1].choose(&mut rng).expect("array is non-empty, so we should always get a value").clone()
        },
        Difficulty::Hard => 1, // pick the best possible moves
    };

    fn index(cell: Cell) -> usize {
        match cell {
            Cell::TOP_LEFT => 0,
            Cell::TOP_MIDDLE => 1,
            Cell::TOP_RIGHT => 2,
            Cell::MIDDLE_LEFT => 3,
            Cell::MIDDLE_MIDDLE => 4,
            Cell::MIDDLE_RIGHT => 5,
            Cell::BOTTOM_LEFT => 6,
            Cell::BOTTOM_MIDDLE => 7,
            Cell::BOTTOM_RIGHT => 8,
        }
    }

    Line::all().map(|line| {
        let cells: [Cell;3] = line.into();
        let cells_and_marks = cells.map(|cell| (cell, game.get(cell)));

        // case (1)
        match cells_and_marks {
            [(_, Some(a)), (_, Some(b)), (cell, None)] if a == b && b == computer => weights[index(cell)] += 20 * scale,
            [(_, Some(a)), (cell, None), (_, Some(b))] if a == b && b == computer => weights[index(cell)] += 20 * scale,
            [(cell, None), (_, Some(a)), (_, Some(b))] if a == b && b == computer => weights[index(cell)] += 20 * scale,
            _ => {}
        }

        // case (2)
        match cells_and_marks {
            [(_, Some(a)), (_, Some(b)), (cell, None)] if a == b && b != computer => weights[index(cell)] += 10 * scale,
            [(_, Some(a)), (cell, None), (_, Some(b))] if a == b && b != computer => weights[index(cell)] += 10 * scale,
            [(cell, None), (_, Some(a)), (_, Some(b))] if a == b && b != computer => weights[index(cell)] += 10 * scale,
            _ => {}
        }

        // case (3)
        match cells_and_marks {
            [_, (cell, None), _] if cell == Cell::MIDDLE_MIDDLE => weights[index(cell)] += 2 * scale,
            _ => {}
        }

        // case (4)
        match cells_and_marks {
            [(c1, None), _, (c2, None)] if c1.is_corner() => {
                weights[index(c1)] += 1 * scale;
                weights[index(c2)] += 1 * scale
            },
            [(cell, None), _, _] if cell.is_corner() => weights[index(cell)] += 1 * scale,
            [_, _, (cell, None)] if cell.is_corner() => weights[index(cell)] += 1 * scale,
            _ => {}
        }
    });

    info!("cell weights (higher is better): {:?}", weights);

    let (index, _) = weights.iter().enumerate()
        .filter(|(index, _)| game.get(Cell::all()[*index]).is_none())
        .max_by(|(_, &w1), (_, w2)| w1.cmp(w2)).expect("unable to find max weight");

    let chosen_cell = Cell::all()[index];

    info!("optimal cell for computer to choose is {:?} (on {:?} mode)", chosen_cell, difficulty);

    chosen_cell
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
    human_mark: Res<Mark>,
    difficulty: Res<Difficulty>,
) {

    // if the winner has already been decided, we should ignore user input until a new game is started
    if info.game.over() { return; }

    // either "X" or "O"
    let mark = info.current_player;

    let maybe_cell = match *game_mode {
        GameMode::OnePlayer if *human_mark != mark => Some(generate_computer_input(&info.game, mark, *difficulty)),
        _ => capture_user_input(windows, cameras, touch_input, mouse_button_input)
    };

    // the computer will always select a cell, but the human might not
    let Some(cell) = maybe_cell else { return; };

    // If the user / the computer did click on a cell...
    match info.game.get(cell) {
        Some(_) => warn!("this cell is already occupied"),
        None => {
            // ...get a handle to the cell clicked
            let (entity, cell) = cells.iter().filter(|(_, c)| c == &&cell).next().expect("could not find clicked cell in all cells");

            // ...and mark the cell as clicked by that player
            info.game.set(cell.clone(), mark);
            info!("{:?} was hit", cell);

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

            // If the game is over...
            if info.game.over() {
                match info.game.winner() {
                    None => {
                        info!("The game ends in a tie");
                    }
                    Some((mark, line)) => {
                        let [from, .., to]: [Cell;3] = line.into();
                        info!("The winner is {:?} along the line {:?} -> {:?}", mark, from, to);
                    }
                }

                next_game_state.set(GameState::GameOver)

            } else {
                // If the game is not over... keep playing
                match *current_game_state.get() {
                    GameState::XTurn => next_game_state.set(GameState::OTurn),
                    GameState::OTurn => next_game_state.set(GameState::XTurn),
                    GameState::GameOver => unreachable!("called capture_input() in GameOver state"),
                    GameState::GameNotInProgress => unreachable!("called capture_input() in GameNotInProgress state"),
                }
            }
        }
    }
}