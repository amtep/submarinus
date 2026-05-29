use bevy::prelude::*;
use bevy::window::WindowMode;

mod bubbles;
mod constants;
mod enemies;
mod level;
mod math;
mod player;
mod random;
mod torpedoes;

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
            random::plugin,
            player::plugin,
            level::plugin,
            bubbles::plugin,
            torpedoes::plugin,
            enemies::plugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
