use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::sprite::Anchor;

const GRID_SPACING: f32 = 200.0;
const HALFSIZE: f32 = GRID_SPACING / 2.0;

#[derive(Debug)]
enum Row {
    Top,
    Middle,
    Bottom
}

impl Row {
    fn y_range(&self) -> Vec2 {
        match self {
            Row::Top => Vec2::new(HALFSIZE, 3.0*HALFSIZE),
            Row::Middle => Vec2::new(-HALFSIZE, HALFSIZE),
            Row::Bottom => Vec2::new(-3.0*HALFSIZE, -HALFSIZE)
        }
    }

    fn contains(&self, y: f32) -> bool {
        let Vec2 { x: min, y: max } = self.y_range();
        min <= y && y < max
    }

    fn in_row(y: f32) -> Option<Row> {
        if Row::Top.contains(y) {
            Some(Row::Top)
        } else if Row::Middle.contains(y) {
            Some(Row::Middle)
        } else if Row::Bottom.contains(y) {
            Some(Row::Bottom)
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum Column {
    Left,
    Middle,
    Right
}

impl Column {
    fn x_range(&self) -> Vec2 {
        match self {
            Column::Left => Vec2::new(-3.0*HALFSIZE, -HALFSIZE),
            Column::Middle => Vec2::new(-HALFSIZE, HALFSIZE),
            Column::Right => Vec2::new(HALFSIZE, 3.0*HALFSIZE)
        }
    }

    fn contains(&self, x: f32) -> bool {
        let Vec2 { x: min, y: max } = self.x_range();
        min <= x && x < max
    }

    fn in_column(x: f32) -> Option<Column> {
        if Column::Left.contains(x) {
            Some(Column::Left)
        } else if Column::Middle.contains(x) {
            Some(Column::Middle)
        } else if Column::Right.contains(x) {
            Some(Column::Right)
        } else {
            None
        }
    }
}

struct Grid {}

impl Grid {
    fn hit_square(pos: Vec2) -> Option<(Row, Column)> {
        match (Row::in_row(pos.y), Column::in_column(pos.x)) {
            (None, _) | (_, None) => None,
            (Some(row), Some(col)) => Some((row, col))
        }
    }
}

#[derive(Resource, Debug)]
struct MostRecentMousePosition {
    pos: Vec2
}

impl Default for MostRecentMousePosition {
    fn default() -> Self {
        Self {
            pos: Vec2::splat(f32::NAN)
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(MostRecentMousePosition::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_click_system)
        .run();
}

fn setup(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
) {
    let mut window = windows.single_mut();
    window.resolution.set(800.0, 800.0);

    commands.spawn(Camera2dBundle::default());

    // create the grid
    commands.spawn(line((-HALFSIZE, -3.0*HALFSIZE), (-HALFSIZE, 3.0*HALFSIZE)));
    commands.spawn(line((HALFSIZE, -3.0*HALFSIZE), (HALFSIZE, 3.0*HALFSIZE)));
    commands.spawn(line((-3.0*HALFSIZE, HALFSIZE), (3.0*HALFSIZE, HALFSIZE)));
    commands.spawn(line((-3.0*HALFSIZE, -HALFSIZE), (3.0*HALFSIZE, -HALFSIZE)));
}

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

fn mouse_click_system(
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