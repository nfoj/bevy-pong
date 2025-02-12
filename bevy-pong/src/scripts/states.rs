use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[derive]
    Main,
    Controls,
    StartGame,
    Playing,
    EndGame,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PausedState {
    #[default]
    Playing,
    Paused,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MainSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ControlsSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct StartGameSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayingSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PausedSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EndGameSet;

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_state::<PausedState>()
            .configure_sets(
                Update,
                (
                    MainSet.run_if(in_state(GameState::Main)),
                    ControlsSet.run_if(in_state(GameState::Controls)),
                    StartGameSet.run_if(in_state(GameState::StartGame)),
                    PlayingSet
                        .run_if(in_state(GameState::Playing))
                        .run_if(in_state(PausedState::Paused)),
                    PausedSet
                        .run_if(in_state(GameState::Playing))
                        .run_if(in_state(PausedState::Paused)),
                    EndGameSet.run_if(in_state(GameState::EndGame)),
                ),
            )
            .configure_sets(
                FixedUpdate,
                PlayingSet
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PausedState::Playing)),
            );
    }
}
