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
