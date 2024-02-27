use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::*;

pub fn plugin(app: &mut App) {
    app
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
    windows: Query<&Window>
) {
    for event in mouse_button_input_events.read() {
        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            if let Some((row, column)) = Grid::hit_square(most_recent_mouse_position.pos) {
                info!("Hit the {:?} {:?} square", row, column)
            }
        }
    }

    for event in cursor_moved_events.read() {
        let window = windows.single();
        let (ww, wh) = (window.resolution.width(), window.resolution.height());

        let x = event.position.x - ww / 2.0;
        let y = -event.position.y + wh / 2.0;

        most_recent_mouse_position.pos = Vec2::new(x, y);
    }
}
