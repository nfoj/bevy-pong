use bevy::app::App;
use bevy::prelude::*;

use ball::PongBallPlugin;
use camera::PongCameraPlugin;
use window::PongWindowPlugin;

mod ball;
mod camera;
mod window;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PongCameraPlugin, PongWindowPlugin, PongBallPlugin));
    }
}
