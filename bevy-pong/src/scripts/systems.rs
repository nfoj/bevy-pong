use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEventFlags;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use std::f32::consts::PI;

use crate::controls::GameAction;
use crate::settings::{Difficult, GameSettings, PlayerType};
use crate::states::PausedState;

use super::components::*;
use super::observers::OnPointScored;
use super::Score;

#[derive(Component)]
pub struct Pong;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub enum ScoreField {
    Left,
    Right,
}

pub mod config {
    pub const WALL_THICKNESS: f32 = 10.0;
    pub const TOP_BUFFER: f32 = 100.0;
}

pub mod game {
    pub const MAX_SCORE: u32 = 5;
}

pub mod paddle {
    pub const WIDTH: f32 = 10.0;
    pub const HEIGHT: f32 = 100.0;
    pub const BUFFER: f32 = 40.0;
    pub const SPEED: F32 = 6.;
}

pub mod ball {
    pub const RADIUS: f32 = 10.0;
    pub const INITIAL_VELOCITY: (f32, f32) = (200.0, 100.0);
    pub const SPEED_INCREASE: f32 = 2.;
    pub const MAX_BALL_SPEED: f32 = 1000.;
}

pub mod setup {
    use super::*;

    pub fn game(
        mut commands: Commands,
        windows: Query<&Window>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut score: ResMut<Score>,
        mut next_state: ResMut<NextState<PausedState>>,
        settings: Res<GameSettings>,
    ) {
        score.reset();
        next_state.set(PausedState::Playing);

        let window = windows.single();
        let (width, height) = (window.resolution.width(), window.resolution.height());

        spawn_game_world(
            &mut commands,
            width,
            height,
            &mut meshes,
            &mut materials,
            settings,
        );
    }

    fn spawn_game_world(
        commands: &mut Commands,
        width: f32,
        height: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        settings: Res<GameSettings>,
    ) {
        commands
            .spawn((Pong, Transform::default(), Visibility::default()))
            .with_children(|builder| {
                create_board(builder, width, height, meshes, materials);
                create_players(builder, width, meshes, materials, settings);
                spawn_ball(builder, meshes, materials);
                create_score(builder, height);
            });
    }

    fn create_wall(
        commands: &mut ChildBuilder,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        width: f32,
        height: f32,
        transform: Transform,
    ) {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(width, height))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            transform,
            Collider::cuboid(width / 2.0, height / 2.0),
            RigidBody::Fixed,
        ));
    }

    fn create_board(
        builder: &mut ChildBuilder,
        screen_width: f32,
        screen_height: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        // horizontal walls
        for y_pos in [
            screen_height / 2.0 - config::WALL_THICKNESS - config::TOP_BUFFER,
            screen_height / -2.0 + config::WALL_THICKNESS,
        ] {
            create_wall(
                builder,
                meshes,
                materials,
                screen_width,
                config::WALL_THICKNESS,
                Transform::from_xyz(0.0, y_pos, 0.0),
            );
        }

        // scoring
        let sensor_height = screen_height - config::TOP_BUFFER - config::WALL_THICKNESS;
        for (x_pos, score_field) in [
            (
                screen_width / -2.0 + config::WALL_THICKNESS,
                ScoreField::Left,
            ),
            (
                screen_height / 2.0 - config::WALL_THICKNESS,
                ScoreField::Right,
            ),
        ] {
            builder.spawn((
                Transform::from_xyz(x_pos, config::TOP_BUFFER / -2.0, 0.0),
                Collider::cuboid(config::WALL_THICKNESS, sensor_height / 2.0),
                Sensor,
                score_field,
            ));
        }
    }

    fn create_paddle(
        builder: &mut ChildBuilder,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        transform: Transform,
        player_type: PlayerType,
        score_field: ScoreField,
    ) {
        builder.spawn((
            Mesh2d(meshes.add(Rectangle::new(paddle::WIDTH, paddle::HEIGHT))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            transform,
            Collider::cuboid(paddle::WIDTH / 2.0, paddle::HEIGHT / 2.0),
            RigidBody::KinematicPositionBased,
            KinematicCharacterController::default(),
            player_type,
            score_field,
        ));
    }

    fn create_players(
        builder: &mut ChildBuilder,
        screen_width: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        settings: Res<GameSettings>,
    ) {
        for (x_offset, player_type, score_field) in [
            (
                screen_width / -2.0 + paddle::BUFFER,
                settings.get_player1(),
                ScoreField::Left,
            ),
            (
                screen_width / 2.0 - paddle::BUFFER,
                settings.get_player2(),
                ScoreField::Right,
            ),
        ] {
            create_paddle(
                builder,
                meshes,
                materials,
                Transform::from_xyz(x_offset, config::TOP_BUFFER / -2.0, 0.0),
                player_type.clone(),
                score_field,
            );
        }
    }

    fn create_score(builder: &mut ChildBuilder, window_height: f32) {
        builder.spawn((
            Text2d::new("0 - 0"),
            TextColor(Color::WHITE),
            TextFont {
                font_size: 100.,
                ..default()
            },
        ));
    }

    pub fn spawn_ball(
        builder: &mut ChildBuilder,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        builder.spawn((
            Mesh2d(meshes.add(Circle::new(ball::RADIUS))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Ball,
            RigidBody::Dynamic,
            Ccd::enable(),
            Velocity {
                livel: vec2::new(ball::INITIAL_VELOCITY.0, ball::INITIAL_VELOCITY.1),
                angvel: 0.,
            },
            GravityScale(0.),
            Sleeping::disabled(),
            Collider::ball(ball::RADIUS),
            Restitution {
                coefficient: 0.99,
                combine_rule: CoefficientCombineRule::Max,
            },
            Friction {
                coefficient: 0.99,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

pub mod movement {
    use super::*;

    pub fn players(
        keys: Res<ActionState<GameAction>>,
        mut players: Query<(
            &mut KinematicCharacterController,
            &PlayerType,
            &Transform,
            &ScoreField,
        )>,
        balls: Query<&Transform, With<Ball>>,
    ) {
        let ball = balls.single();

        for (player, player_type, paddle_position, score_field) in players.iter_mut() {
            match player_type {
                PlayerType::Human => handle_player_input(player, score_field, &keys),
                PlayerType::Computer(difficulty) => {
                    handle_computer_movement(player, paddle_position, ball, *difficulty)
                }
            }
        }
    }
}
