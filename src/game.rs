use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::hashbrown::hash_map::Entry;
use bevy::utils::HashMap;

use crate::*;

#[derive(Resource, Default, PartialEq, Debug, Clone, Copy)]
enum Mark {
    #[default]
    X,
    O
}

#[derive(Resource, Default)]
struct State {
    marks: HashMap<(Row, Column), Option<Mark>>
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
    let font = asset_server.load("fonts/larabie.otf");

    let window = windows.single();
    let (ww, wh) = (window.resolution.width(), window.resolution.height());

    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            if let Some((row, column)) = Grid::hit_square(most_recent_mouse_position.pos) {

                match state.marks.entry((row, column)) {
                    Entry::Occupied(_) => {
                        println!("this cell is already occupied")
                    }
                    Entry::Vacant(_) => {

                        let (origin_x, origin_y) = (ww / 2.0, wh / 2.0);
                        let Vec2 { x: x_min, y: x_max } = column.x_range();
                        let Vec2 { x: y_min, y: y_max } = row.y_range();

                        commands.spawn(NodeBundle {
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
                        }).with_children(|parent| {
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

                        state.marks.insert((row, column), Some(*player_turn));
                        *player_turn = player_turn.next();

                        info!("Hit the {:?} {:?} square", row, column)
                    }
                }
            }
        }
    }

    // update the most_recent_mouse_position
    for event in cursor_moved_events.read() {
        let x = event.position.x - ww / 2.0;
        let y = -event.position.y + wh / 2.0;

        most_recent_mouse_position.pos = Vec2::new(x, y);
    }
}
