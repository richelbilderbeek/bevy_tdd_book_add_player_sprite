use crate::app::*;
use bevy::prelude::*;
mod app;

fn main() {
    let initial_player_position = Vec2::new(0.0, 0.0);
    let initial_player_size = Vec2::new(64.0, 32.0);
    let mut app = create_app(initial_player_position, initial_player_size);
    let add_camera_fn = |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    };
    app.add_systems(Startup, add_camera_fn);
    app.add_plugins(DefaultPlugins);
    app.run();
}
