use bevy::prelude::*;
use bevy::window::WindowMode;

mod level;
mod player;

pub fn app() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    ..default()
                }),
                ..default()
            }),
            player::plugin,
            level::plugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
