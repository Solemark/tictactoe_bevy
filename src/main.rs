use app::{button_system, setup, status_system, win_system};
use bevy::prelude::*;

mod app;
mod player_button;
mod world;

fn main() {
    // TODO - use assets
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (button_system, status_system, win_system))
        .run();
}
