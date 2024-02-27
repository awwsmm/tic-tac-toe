use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::{HashMap, HashSet};
use bevy::utils::hashbrown::hash_map::Entry;

use crate::*;

#[derive(Resource, Component, Default, PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Mark {
    #[default]
    X,
    O
}

#[derive(Resource, Default)]
struct State {
    marks: HashMap<Cell, Option<Mark>>,
    winner: Option<(Mark, (Cell, Cell))>
}

impl State {
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

impl Mark {
    fn next(&self) -> Self {
        if *self == Self::X { Self::O } else { Self::X }
    }
}

pub fn plugin(app: &mut App) {
    app
        .insert_resource(Mark::default())
        .insert_resource(State::default())
        .add_systems(OnEnter(GameState::Game), setup)
        .add_systems(Update, capture_clicks.run_if(in_state(GameState::Game)));
}

fn setup(mut commands: Commands) {

    fn line(start: (f32, f32), end: (f32, f32)) -> SpriteBundle {
        let thickness = 10.0;

        let start = Vec2::new(start.0, start.1);
        let end = Vec2::new(end.0, end.1);

        let length = start.distance(end);
        let angle = Vec2::new(1.0, 0.0).angle_between(end - start);
        let rotation = Quat::from_rotation_z(angle);

        let transform = Transform {
            translation: start.extend(0.0),
            rotation,
            ..default()
        };

        let color = Color::BLACK;

        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(length, thickness)),
                anchor: Anchor::CenterLeft,
                ..default()
            },
            transform,
            ..default()
        }
    }

    // create the grid
    commands.spawn(line((-HALFSIZE, -3.0 * HALFSIZE), (-HALFSIZE, 3.0 * HALFSIZE)));
    commands.spawn(line((HALFSIZE, -3.0 * HALFSIZE), (HALFSIZE, 3.0 * HALFSIZE)));
    commands.spawn(line((-3.0 * HALFSIZE, HALFSIZE), (3.0 * HALFSIZE, HALFSIZE)));
    commands.spawn(line((-3.0 * HALFSIZE, -HALFSIZE), (3.0 * HALFSIZE, -HALFSIZE)));
}

fn capture_clicks(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut most_recent_mouse_position: ResMut<MostRecentMousePosition>,
    windows: Query<&Window>,
    mut commands: Commands,
    mut player_turn: ResMut<Mark>,
    mut state: ResMut<State>,
    asset_server: Res<AssetServer>
) {
    if state.winner.is_none() {
        let font = asset_server.load("fonts/larabie.otf");

        let window = windows.single();
        let (ww, wh) = (window.resolution.width(), window.resolution.height());

        for event in mouse_button_input_events.read() {
            if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
                if let Some(cell) = Grid::hit_square(most_recent_mouse_position.pos) {
                    match state.marks.entry(cell) {
                        Entry::Occupied(_) => {
                            warn!("this cell is already occupied")
                        }
                        Entry::Vacant(_) => {
                            let (origin_x, origin_y) = (ww / 2.0, wh / 2.0);
                            let Vec2 { x: x_min, y: x_max } = cell.column.x_range();
                            let Vec2 { x: y_min, y: y_max } = cell.row.y_range();

                            // draw an "X" or an "O" on the board
                            commands.spawn((
                                NodeBundle {
                                    style: Style {
                                        position_type: PositionType::Absolute,
                                        left: Val::Px(origin_x + x_min),
                                        top: Val::Px(origin_y - y_max),
                                        width: Val::Px(x_max - x_min),
                                        height: Val::Px(y_max - y_min),
                                        border: when_debugging(UiRect::all(Val::Px(1.0))),
                                        justify_content: JustifyContent::Center,
                                        align_content: AlignContent::Center,
                                        ..default()
                                    },
                                    border_color: when_debugging(Color::RED.into()),
                                    ..default()
                                },
                                *player_turn
                            )).with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    if *player_turn == Mark::X { "X" } else { "O" },
                                    TextStyle {
                                        font_size: 200.0,
                                        font: font.clone(),
                                        color: if *player_turn == Mark::X { Color::RED } else { Color::BLUE },
                                        ..default()
                                    }
                                ));
                            });

                            state.marks.insert(cell, Some(*player_turn));
                            *player_turn = player_turn.next();
                            info!("Hit the {:?} cell", cell);

                            state.winner = state.determine_winner();
                            info!("The winner is {:?}", state.winner);
                        }
                    }
                }
            }
        }

        // update the most_recent_mouse_position, using game coordinates (origin at center of screen)
        for event in cursor_moved_events.read() {
            let x = event.position.x - ww / 2.0;
            let y = -event.position.y + wh / 2.0;

            most_recent_mouse_position.pos = Vec2::new(x, y);
        }
    }
}
