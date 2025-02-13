use bevy::app::App;
use bevy::prelude::*;

use scripts::camera::GameCameraPlugin;
use scripts::controls::GameControlsPlugin;
use scripts::settings::GameSettings;
use scripts::states::GameStatesPlugin;
use scripts::window::GameWindowPlugin;

// The 'game' module declaration is crucial
mod scripts {
    pub mod camera;
    pub mod commands;
    pub mod controls;
    pub mod settings;
    pub mod states;
    pub mod window;
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameCameraPlugin,
            GameWindowPlugin,
            GameStatesPlugin,
            GameControlsPlugin,
        ));
    }
}
