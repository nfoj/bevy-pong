use bevy::prelude::*;

pub struct PongWindowPlugin;

impl Plugin for PongWindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pong".into(),
                    resizable: false,
                    resolution: (1200., 600.).into(),
                    desired_maximum_frame_latency: core::num::NonZero::new(1u32),

                    ..default()
                }),

                ..default()
            }));
    }
}
