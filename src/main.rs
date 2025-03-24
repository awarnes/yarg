mod player;
mod vehicle;
use crate::player::systems::{move_player, spawn_player};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(Update, (check_quit, move_player))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn check_quit(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.all_pressed([KeyCode::KeyQ, KeyCode::ControlLeft]) {
        exit.send(AppExit::Success);
    }
}
