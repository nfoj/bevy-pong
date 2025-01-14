use bevy::app::App;
use bevy::prelude::*;

use camera::PongCameraPlugin;
use window::PongWindowPlugin;

mod camera;
mod window;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PongCameraPlugin, PongWindowPlugin));
    }
}
