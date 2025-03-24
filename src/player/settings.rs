use bevy::prelude::*;
pub struct Settings {
    pub accelerate: KeyCode,
    pub decelerate: KeyCode,
    pub steer_right: KeyCode,
    pub steer_left: KeyCode,
    pub ebrake: KeyCode,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            accelerate: KeyCode::KeyW,
            decelerate: KeyCode::KeyS,
            steer_left: KeyCode::KeyA,
            steer_right: KeyCode::KeyD,
            ebrake: KeyCode::Space,
        }
    }
}
