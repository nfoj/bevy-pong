use bevy::prelude::*;

pub struct GameWindowPlugin;

impl Plugin for GameWindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pong".to_string(),
                    resizable: true,
                    resolution: (1200., 600.).into(),
                    ..default()
                }),
                ..default()
            }));
    }
}
