use bevy::{
    audio::Volume,
    color::palettes::css::{BLACK, BLUE, GREEN, RED},
    prelude::*,
};

//
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::{ActiveEvents, Collider, CollisionEvent, CollisionGroups, Group, RigidBody},
};

//
use crate::positions::Position;
pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<PongSounds>()
            .add_event::<Score>()
            .add_plugins(RapierPhysicsPlugin::<NoUseData>::pixels_per_meter(1.))
            .add_systems(
                Startup,
                (
                    spawn_ball,
                    spawn_paddles,
                    spawn_gutters,
                    spawn_scoreboards,
                    load_assets,
                )
                    .chain(),
            )
            .add_systems(Update, (update_scoreboards, handle_player_input).chain())
            .add_systems(
                FixedUpdate,
                (
                    move_ball,
                    detect_scoring,
                    update_score,
                    reset_ball,
                    move_ai,
                    move_paddles,
                    handle_ball_collisions,
                )
                    .chain(),
            );
    }
}

//
#[derive(Resource, Default)]
struct PongSounds {
    on_hit: Handle<AudioSource>,
    on_score: Handle<AudioSource>,
}

fn load_assets(asset_serve: Res<AssetServer>, mut pong_sounds: ResMut<PongSounds>) {
    pong_sounds.on_hit = asset_serve.load(path);
    pong_sounds.on_score = asset_serve.load(path);
}

//
#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Ball;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    collider: Collider,
    collision_group: CollisionGroups,
    rigid_body: RigidBody,
    velocity: Velocity,
    position: Position,
}

const BALL_COLLISION_GROUP: Group = Group::GROUP_1;
const REFLECT_COLISION_GROUP: Group = Group::GROUP_2;
const BALL_SIZE: f32 = 5.;
const BALL_SPEED: f32 = 4.;

impl BallBundle {
    fn new(velocity: Vec2) -> Self {
        Self {
            ball: Ball,
            collider: Collider::ball(BALL_SIZE),
            collision_group: CollisionGroups::new(BALL_COLLISION_GROUP, REFLECT_COLISION_GROUP),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity(velocity),
            position: Position(Vec2::new(0., 0.)),
        }
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let meshe = meshes.add(Circle::new(BALL_SIZE));
    let material = materials.add(ColorMaterial::from_color(Color::srgb(1., 0., 0.)));

    commands.spawn((
        BallBundle::new(Vec2::new(1., 0.)),
        Mesh2d(meshe),
        MeshMaterial2d(material),
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn move_ball(mut ball: Query<(&mut Position, &Velocity), With<Ball>>) {
    if let Ok((mut position, velocity)) = ball.get_single_mut() {
        position.0 += velocity.0 * BALL_SPEED;
    }
}

//
#[derive(Component)]
struct Paddle;

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    collider: Collider,
    collision_group: CollisionGroups,
    rigid_body: RigidBody,
    position: Position,
    velocity: Velocity,
}

const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 50.;

impl PaddleBundle {
    fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            collider: Collider::cuboid(PADDLE_WIDTH / 2., PADDLE_HEIGHT / 2.),
            collision_group: CollisionGroups::new(REFLECT_COLISION_GROUP, BALL_COLLISION_GROUP),
            position: Position(Vec2::new(x, y)),
            rigid_body: RigidBody::Fixed,
            velocity: Velocity(Vec2 { x: 0., y: (0.) }),
        }
    }
}

fn spawn_paddles (mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, window: Query<&Window>,) {
    let window = match window.get_single() {
        Ok(window) => window,
        Err(_) => return,
    };

    let window_width = window.resolution.width();
    let padding = 50.;
    let right_paddle_x = window_width / 2. - padding;
    let left_paddle_x = -window_width / 2. + padding;

    let meshe = meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
    let player_color = materials.add(ColorMaterial::from_color(Color::srgb(1., 1., 0.)));
    let ai_color = Handle<ColorMaterial> = materials.add(ColorMaterial::from_color(Color::srgb(1., 1., 0.)));
    comands.spawn((
        Player,
        PaddleBundle::new(right_paddle_x, -0.),
        Mesh2d(meshe.clone()),
        MeshMaterial2d(player_color),
    ));

    commands.spawn((
        Ai,
        PaddleBundle::new(left_paddle_x, -0.),
        Mesh2d(meshe),
        MeshMaterial2d(ai_color),
    ));
}

fn handle_ball_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut ball: Query<(&mut Velocity, &Position), With<Ball>>,
    paddle: Query<&Position, With<Paddle>>,
    gutter: Query<&Position, With<Gutter>>,
    mut commands: Commands,
    pong_sounds: Res<PongSounds>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity_a, entity_b, _flags) => {
                if let Ok((mut ball_velocity, ball_position)) = ball.get_mut(*entity_b) {
                    commands.spawn((
                        AudioPlayer::new(pong_sounds.on_hit.clone_weak()),
                        PlaybackSettings::ONCE.with_volume(Volume::new(0.5)),
                    ));
                    if let Ok(paddle) = paddle.get(*entity_a) {
                        ball_velocity.0.x = (ball_position.0 - paddle.0).x.signum();
                    } else if let Ok(gutter) = gutter.get(*entity_a) {
                        ball_velocity.0.y = (ball_position.0 - gutter.0).y.signum();
                    }
                } else if let Ok((mut ball_velocity, ball_position)) = ball.get_mut(*entity_a) {
                    commands.spawn((
                        AudioPlayer::new(pong_sounds.on_hit.clone_weak()),
                        PlaybackSettings::ONCE.with_volume(Volume::new(0.5)),
                    ));
                    if let Ok(paddle) = paddle.get(*entity_b) {
                        ball_velocity.0.x = (ball_position.0 - paddle.0).x.signum();
                    } else if let Ok(gutter) = gutter.get(*entity_b) {
                        ball_velocity.0.y = (ball_position.0 - gutter.0).y.signum();
                    }
                }
            }
            _ => return,
        }
    }
}
