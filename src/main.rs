//use crate::app::create_app;
use crate::app::*;
use bevy::prelude::*;
mod app;

fn main() {
    let mut app = create_app(Vec3::new(0.0, 0.0, 0.0));
    let setup_camera_fun = |mut commands: Commands| {
        commands.spawn(Camera2dBundle::default());
    };
    app.add_systems(Startup, setup_camera_fun);
    app.add_plugins(DefaultPlugins);
    app.run();
}
