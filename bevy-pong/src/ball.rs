use bevy::prelude::*;

pub struct PongBallPlugin;

impl Plugin for PongBallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
        app.add_systems(Update, move_ball)
    }
}

const BALL_COLLISION_GROUP: Group = Group::GROUP_1;
const REFLECT_COLLISION_GROUP: Group = Group::GROUP_2;

const BALL_SIZE: f32 = 5.;
const BALL_VELOCITY: f32 = 4.0;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle{
    ball: Ball,
    collider: Collider,
    collision_group: CollisionGroup,
    rigid_body: RigidBody,
    velocity: Velocity,
    position: Position,
}

impl BallBundle {
    fn new(velocity: Vec2) -> {
        Self {
            ball: Ball,
            collider: Collider::ball(BALL_SIZE),
            collision_groupo: CollisionGroups::new(BALL_COLLISION_GROUP, REFLECT_COLLISION_GROUP),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity(velocity),
            position: Position(Vec2::new(0., 0.)),
        }
    }
}

fn spawn_ball(mut commands:Commands, mut meshes:RestMut<Assests<Mesh>>, mut materials:ResMut<Assests<ColorMaterial>>){

    let meshe = meshes.add(Circle::new(BALL_SIZE));
    let material = materials.add(ColorMaterial::from_color(Color::srgb(1., 0., 0.)));

    commands.spawn((
       BallBundle::new(Vec2::new(1., 0.)),
       Mesh2d(meshe),
       MeshMaterial2d(material),
       ActiveEvents::COLLISION_EVENTS, 
    ));
}

fn move_ball(mut ball::Query<(&mut Position, &Velocity), With<Ball>>){
    if let Ok((mut position, velocity)) = ball.get_sigle_mut(){
        position.0 += velocity.0 * BALL_SPEED;
    }
}
