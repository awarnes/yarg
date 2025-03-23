use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_game)
        .add_systems(Update, (check_quit, move_player))
        .run();
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Sprite {
            image: asset_server.load("race_car.png"),
            custom_size: Some(Vec2::new(64., 64.)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player {
            settings: Settings::default(),
            handling: f32::to_radians(360.0),
            speed: 500.,
        },
    ));
}

#[derive(Component)]
struct Player {
    settings: Settings,
    handling: f32,
    speed: f32,
}

struct Settings {
    accelerate: KeyCode,
    decelerate: KeyCode,
    steer_right: KeyCode,
    steer_left: KeyCode,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            accelerate: KeyCode::KeyW,
            decelerate: KeyCode::KeyS,
            steer_left: KeyCode::KeyA,
            steer_right: KeyCode::KeyD,
        }
    }
}

fn check_quit(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.all_pressed([KeyCode::KeyQ, KeyCode::ControlLeft]) {
        exit.send(AppExit::Success);
    }
}

fn move_player(
    query: Single<(&Player, &mut Transform)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.into_inner();

    let mut rotation_factor: f32 = 0.0;
    let mut movement_factor: f32 = 0.0;

    if input.pressed(player.settings.accelerate) {
        movement_factor += 1.0;
    }

    if input.pressed(player.settings.decelerate) {
        movement_factor -= 1.0;
    }

    if movement_factor.abs() > 0.0 {
        if input.pressed(player.settings.steer_right) {
            rotation_factor -= 1.0;
        }

        if input.pressed(player.settings.steer_left) {
            rotation_factor += 1.0;
        }

        transform.rotate_z(rotation_factor * player.handling * time.delta_secs());
    }

    let movement_direction = transform.rotation * Vec3::Y;

    let movement_distance = movement_factor * player.speed * time.delta_secs();

    let translation_delta = movement_direction * movement_distance;

    transform.translation += translation_delta;
}
