pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn move_player(
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
