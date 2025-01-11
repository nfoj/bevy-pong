use bevy::prelude::*;
use bevy::sprite::MeshMaterial2d;

const BALL_SPEED: f32 = 5.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 60.;

#[derive(Compoment)]
struct Paddle;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    position: Position,
    velocity: Velocity,
}

//
impl BallBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            ball: Ball,
            velocity: Velocity(Vec2::new(x, y)),
            position: Position(Vec2::new(0., 0.)),
            paddle: Paddle,
        }
    }
}

//
fn spawn_ball(
    //
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(5.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(1., 0., 0.)))),
    ));
}

//
fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * BALL_SPEED;
    }
}

//
fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2d::default());
}

//
fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

//
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera))
        .add_systems(Update, project_positions)
        .run();
}
