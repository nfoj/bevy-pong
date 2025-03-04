mod components;
mod constants;
mod observers;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::states::{GameState, PausedState, PlayingSet};
use observers::*;
use systems::*;

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .configure_sets(
                FixedUpdate,
                (PhysicsSet::StepSimulation
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PausedState::Playing)),),
            )
            .add_event::<OnPointScored>()
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_observer(score_point)
            .add_observer(reset_ball)
            .add_observer(end_game)
            .add_systems(OnEnter(GameState::Playing), setup_game)
            .add_systems(OnExit(GameState::Playing), cleanup_game)
            .add_systems(FixedUpdate, move_players.in_set(PlayingSet))
            .add_systems(
                Update,
                (
                    speed_up_ball,
                    ball_paddle_collision,
                    detect_point,
                    update_score_display,
                )
                    .in_set(PlayingSet),
            );
    }
}

pub use resources::Score;
