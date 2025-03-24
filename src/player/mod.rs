use bevy::prelude::*;

pub mod settings;
pub mod systems;

use crate::vehicle::Vehicle;
use settings::Settings;

#[derive(Component)]
pub struct Player {
    pub settings: Settings,
    pub vehicle: Vehicle,
    // pub handling: f32,
    // pub speed: f32,
}
