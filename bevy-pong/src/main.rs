use bevy::prelude::*;
use pong::AppPlugin;

mod scripts {
    pub mod camera;
    pub mod game;
    pub mod pong;
    pub mod ui;
    pub mod window;
}

use scripts::camera::GameCameraPlugin;
use scripts::game::GamePlugin;
use scripts::pong::PongPlugin;
use scripts::ui::MenuSystemsPlugin;
use scripts::window::GameWindowPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameWindowPlugin, GamePlugin, PongPlugin, MenuSystemsPlugin));
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, AppPlugin)).run();
}
