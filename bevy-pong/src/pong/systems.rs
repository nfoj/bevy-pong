use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use leafwing_input_manager::prelude::*;

use std::f32::consts::PI;

use crate::core::controls::GameAction;
use crate::core::settings::{Difficulty, GameSettings, PlayerType};
use crate::core::states::PausedState;

use super::components::*;
use super::constants;
use super::observers::OnPointScored;
use super::Score;

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
        // Create horizontal walls
        for y_pos in [
            screen_height / 2.0 - constants::WALL_THICKNESS - constants::TOP_BUFFER,
            screen_height / -2.0 + constants::WALL_THICKNESS,
        ] {
            create_wall(
                builder,
                meshes,
                materials,
                screen_width,
                constants::WALL_THICKNESS,
                Transform::from_xyz(0.0, y_pos, 0.0),
            );
        }

        // Create scoring sensors
        let sensor_height = screen_height - constants::TOP_BUFFER - constants::WALL_THICKNESS;
        for (x_pos, score_field) in [
            (
                screen_width / -2.0 + constants::WALL_THICKNESS,
                ScoreField::Left,
            ),
            (
                screen_width / 2.0 - constants::WALL_THICKNESS,
                ScoreField::Right,
            ),
        ] {
            builder.spawn((
                Transform::from_xyz(x_pos, constants::TOP_BUFFER / -2.0, 0.0),
                Collider::cuboid(constants::WALL_THICKNESS, sensor_height / 2.0),
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
            Mesh2d(meshes.add(Rectangle::new(
                constants::paddle::WIDTH,
                constants::paddle::HEIGHT,
            ))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            transform,
            Collider::cuboid(
                constants::paddle::WIDTH / 2.0,
                constants::paddle::HEIGHT / 2.0,
            ),
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
                screen_width / -2.0 + constants::paddle::BUFFER,
                settings.get_player1(),
                ScoreField::Left,
            ),
            (
                screen_width / 2.0 - constants::paddle::BUFFER,
                settings.get_player2(),
                ScoreField::Right,
            ),
        ] {
            create_paddle(
                builder,
                meshes,
                materials,
                Transform::from_xyz(x_offset, constants::TOP_BUFFER / -2.0, 0.0),
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
            Transform::from_translation((window_height / 2.0 - 50.) * Vec3::Y),
        ));
    }

    pub fn spawn_ball(
        builder: &mut ChildBuilder,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        builder.spawn((
            Mesh2d(meshes.add(Circle::new(constants::ball::RADIUS))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Ball,
            RigidBody::Dynamic,
            Ccd::enabled(),
            Velocity {
                linvel: Vec2::new(
                    constants::ball::INITIAL_VELOCITY.0,
                    constants::ball::INITIAL_VELOCITY.1,
                ),
                angvel: 0.,
            },
            GravityScale(0.),
            Sleeping::disabled(),
            Collider::ball(constants::ball::RADIUS),
            Restitution {
                coefficient: 0.99,
                combine_rule: CoefficientCombineRule::Max,
            },
            Friction {
                coefficient: 0.01,
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

    fn handle_player_input(
        mut player: Mut<KinematicCharacterController>,
        score_field: &ScoreField,
        keys: &Res<ActionState<GameAction>>,
    ) {
        let direction = Vec2::new(
            0.,
            get_input_direction(score_field, keys) * constants::paddle::SPEED,
        );
        player.translation = Some(direction);
    }

    fn get_input_direction(score_field: &ScoreField, keys: &Res<ActionState<GameAction>>) -> f32 {
        let mut direction = 0.0;

        match score_field {
            ScoreField::Left => {
                if keys.pressed(&GameAction::Player1Up) {
                    direction += 1.0;
                }
                if keys.pressed(&GameAction::Player1Down) {
                    direction -= 1.0;
                }
            }
            ScoreField::Right => {
                if keys.pressed(&GameAction::Player2Up) {
                    direction += 1.0;
                }
                if keys.pressed(&GameAction::Player2Down) {
                    direction -= 1.0;
                }
            }
        }

        direction
    }

    fn handle_computer_movement(
        mut player: Mut<KinematicCharacterController>,
        paddle_position: &Transform,
        ball: &Transform,
        difficulty: Difficulty,
    ) {
        let direction = Vec2::new(0.0, ball.translation.y - paddle_position.translation.y);

        player.translation = Some(direction.clamp_length_max(difficulty.speed()));
    }
}

pub mod scoring {
    use super::*;

    pub fn detect_point(
        mut commands: Commands,
        mut collision_events: EventReader<CollisionEvent>,
        walls_query: Query<Entity, (With<ScoreField>, Without<PlayerType>)>,
    ) {
        for event in collision_events.read() {
            let (entity1, entity2, flags) = match event {
                CollisionEvent::Started(e1, e2, flags) => (e1, e2, flags),
                CollisionEvent::Stopped(_, _, _) => continue,
            };

            if *flags & CollisionEventFlags::SENSOR != CollisionEventFlags::SENSOR {
                continue;
            }

            if let Ok(wall) = walls_query
                .get(*entity1)
                .or_else(|_| walls_query.get(*entity2))
            {
                commands.trigger(OnPointScored(wall));
            }
        }
    }

    pub fn update_display(score: Res<Score>, mut score_text: Query<&mut Text2d>) {
        if score.is_changed() {
            for mut text in &mut score_text {
                text.0 = score.display_text();
            }
        }
    }
}

pub mod ball {
    use super::*;
    pub fn speed_up(
        mut collision_events: EventReader<CollisionEvent>,
        mut velocities: Query<&mut Velocity>,
    ) {
        for event in collision_events.read() {
            if let CollisionEvent::Started(entity1, entity2, _) = event {
                if let Ok(mut velocity) = velocities.get_mut(*entity1) {
                    adjust_velocity(&mut velocity);
                } else if let Ok(mut velocity) = velocities.get_mut(*entity2) {
                    adjust_velocity(&mut velocity);
                }
            }
        }
    }

    fn adjust_velocity(velocity: &mut Velocity) {
        velocity.linvel.y *= constants::ball::SPEED_INCREASE;
        velocity.linvel = velocity
            .linvel
            .clamp_length_max(constants::ball::MAX_BALL_SPEED);
    }

    pub fn paddle_collision(
        mut collision_events: EventReader<CollisionEvent>,
        mut ball_query: Query<(&Transform, &mut Velocity), With<Ball>>,
        paddle_query: Query<&Transform, With<PlayerType>>,
    ) {
        for event in collision_events.read() {
            if let CollisionEvent::Started(entity1, entity2, _) = event {
                if let Ok(paddle) = paddle_query
                    .get(*entity1)
                    .or_else(|_| paddle_query.get(*entity2))
                {
                    let (ball_transform, mut ball_velocity) = ball_query.single_mut();

                    let hit_position = (ball_transform.translation.y - paddle.translation.y)
                        / (constants::paddle::HEIGHT / 2.0);
                    let angle = hit_position * PI / 2.0;
                    let speed = ball_velocity.linvel.length();

                    ball_velocity.linvel.x = -ball_velocity.linvel.x;
                    ball_velocity.linvel.y = angle * speed;

                    ball_velocity.linvel = ball_velocity.linvel.normalize() * speed;
                }
            }
        }
    }
}

pub fn cleanup_game(mut commands: Commands, pong: Query<Entity, With<Pong>>) {
    for entity in pong.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub use ball::{paddle_collision as ball_paddle_collision, speed_up as speed_up_ball};
pub use movement::players as move_players;
pub use scoring::{detect_point, update_display as update_score_display};
pub use setup::game as setup_game;
