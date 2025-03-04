use bevy::prelude::*;

use core::GamePlugin;
use pong::PongPlugin;
use ui::MenuSystemsPlugin;

//
use windows::camera::GameCameraPlugin;
use windows::window::GameWindowPlugin;

mod windows {
    pub mod camera;
    pub mod window;
}

mod core;
mod pong;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            GameWindowPlugin,
            GameCameraPlugin,
            GamePlugin,
            MenuSystemsPlugin,
            PongPlugin,
        ))
        .run();
}
