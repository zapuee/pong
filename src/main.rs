mod components;
mod player;
mod sys;
use bevy::prelude::*;
use player::PlayerBundle;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (
        sys::init::spawn_camera,
        sys::init::spawn_players,   
    ));
    app.add_systems(Update, (
        sys::rendering::render_paddles
    ));
    app.run();
}


