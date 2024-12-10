use bevy::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app:mut App) {
        app.add_Plugins((
            DefaultPlugins.set(WindowsPlugin {
                primary_window: Some(Window{
                    title: "Pong".to_string(),
                    
                })
            })
        ))
    }
}
