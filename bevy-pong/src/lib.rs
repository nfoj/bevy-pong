use bevy::app::App;
use bevy::prelude::*;

use camera::PongCameraPlugin;

mod camera;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PongCameraPlugin,));
    }
}
