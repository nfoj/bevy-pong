use bevy::app::App;
use bevy::prelude::*;

use scripts::camera::GameCameraPlugin;
use scripts::game::GamePlugin;
use scripts::pong::PongPlugin;
use scripts::ui::MenuSystemsPlugin;
use scripts::window::GameWindowPlugin;

// The 'game' module declaration is cruciaz
mod scripts {
    pub mod camera;
    pub mod game;
    pub mod pong;
    pub mod ui;
    pub mod window;
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GameCameraPlugin,
            GameWindowPlugin,
            GamePlugin,
            PongPlugin,
            MenuSystemsPlugin,
        ));
    }
}
