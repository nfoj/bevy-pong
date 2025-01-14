# Notes!

- Camera

```
  use bevy::prelude::*;

  pub struct PongCameraPlugin;

  impl Plugin for PongCameraPlugin {
      fn build(&self, app: &mut App) {
          app.add_systems(Startup, spawn_camera);
      }
  }

  fn spawn_camera(mut commands: Commands) {
      commands.spawn_empty().insert(Camera2d::default());
  }
  
```

- Note:
  - spawn_camera: commands.spawn(Camera2d::default());



- Window

```
  use bevy::prelude::*;

  pub struct PongWindowPlugin;

  impl Plugin for PongWindowPlugin {
      fn build(&self, app: &mut App) {
          app.insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
              .add_plugins(DefaultPlugins.set(WindowPlugin {
                  primary_window: Some(Window {
                      title: "Pong".into(),
                      resizable: true,
                      resolution: (800., 600.).into(),
                      desired_maximum_frame_latency: core::num::NonZero::new(1u32),

                      ..default()
                  }),

                  ..default()
              }));
      }
  }
  
```

- Note: 

  - Background Color: app.insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))

  - Window Plugin:
    - title: "Title ..."
    - resizable: true or false (Ignore window size, occupy available space.)
    - resolution: windows size (Width., Height.) - float
    - frame latency: maximum desired frame latency as a non-zero value of 1.
      - desired_maximum_frame_latency
      - :core::num::NonZero (must never be zero)
      - ::new(1u32) (latency 1)

  - ..default(): all other unmentioned items assume the default value.



