use bevy::prelude::*;

use crate::player::Player;
use crate::player::settings::Settings;
use crate::vehicle::Vehicle;

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
            vehicle: Vehicle::default(),
        },
    ));
}

pub fn move_player(
    query: Single<(&mut Player, &mut Transform)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.into_inner();

    let mut vehicle = player.vehicle;

    let mut input_throttle: f32 = 0.0;
    let mut input_brake: f32 = 0.0;
    let mut input_ebrake: f32 = 0.0;
    let mut input_steer: f32 = 0.0;

    if input.pressed(player.settings.accelerate) {
        input_throttle = 1.0;
    }

    if input.pressed(player.settings.decelerate) {
        input_brake = 1.0;
    }

    if input.pressed(player.settings.ebrake) {
        input_ebrake = 1.0;
    }

    if input_throttle.abs() > 0.0 || input_brake.abs() > 0.0 {
        if input.pressed(player.settings.steer_right) {
            input_steer -= 1.0;
        }

        if input.pressed(player.settings.steer_left) {
            input_steer += 1.0;
        }
    }

    // Smooth steering
    vehicle.steer = if input_steer != 0.0 {
        clamp(
            vehicle.steer + input_steer * time.delta_secs() * 2.0,
            -1.0,
            1.0,
        )
    } else {
        if vehicle.steer > 0.0 {
            0.0f32.max(vehicle.steer - time.delta_secs() * 1.0)
        } else if vehicle.steer < 0.0 {
            0.0f32.min(vehicle.steer + time.delta_secs() * 1.0)
        } else {
            0.0
        }
    };

    vehicle.steer_angle = vehicle.steer * vehicle.max_steer;

    // Pre-calculate heading vector
    let heading_sin = vehicle.heading.sin();
    let heading_cos = vehicle.heading.cos();

    // Get local velocity
    vehicle.local_velocity.x = heading_cos * vehicle.velocity.x + heading_sin * vehicle.velocity.y;
    vehicle.local_velocity.y = heading_cos * vehicle.velocity.y - heading_sin * vehicle.velocity.x;

    // Calculate weight on axles based on center of gravity and weight shift
    let axle_weight_front = vehicle.mass
        * (vehicle.axel_weight_ratio_front * vehicle.gravity
            - vehicle.weight_transfer * vehicle.local_acceleration.x * vehicle.center_height
                / vehicle.wheel_base);
    let axle_weight_rear = vehicle.mass
        * (vehicle.axel_weight_ratio_rear * vehicle.gravity
            + vehicle.weight_transfer * vehicle.local_acceleration.x * vehicle.center_height
                / vehicle.wheel_base);

    // Resulting velocity of wheels as result of yaw rate of car body
    // v = yaw_rate * r where r is distance from axel to center of gravity
    let yaw_speed_front = vehicle.center_to_front_axle * vehicle.yaw_rate;
    let yaw_speed_rear = -vehicle.center_to_rear_axle * vehicle.yaw_rate;

    // Calculate slip angles for front and rear wheels
    let slip_angle_front = (vehicle.local_velocity.y + yaw_speed_front)
        .atan2(vehicle.local_velocity.x.abs())
        - vehicle.local_velocity.x.signum() * vehicle.steer_angle;
    let slip_angle_rear =
        (vehicle.local_velocity.y + yaw_speed_rear).atan2(vehicle.local_velocity.x.abs());

    // reduce rear grip when ebrake is on
    let tire_grip_rear = vehicle.tire_grip * (1.0 - input_ebrake * (1.0 - vehicle.lock_grip));

    let friction_force_front_cy = clamp(
        -vehicle.corner_stiffness_front * slip_angle_front,
        -vehicle.tire_grip,
        vehicle.tire_grip,
    ) * axle_weight_front;
    let friction_force_rear_cy = clamp(
        -vehicle.corner_stiffness_rear * slip_angle_rear,
        -tire_grip_rear,
        tire_grip_rear,
    ) * axle_weight_rear;

    // Get amount of brake and throttle from inputs
    // TODO: Don't go backwards when just using ebrake
    let brake = vehicle
        .brake_force
        .min(input_brake * vehicle.brake_force + input_ebrake * vehicle.ebrake_force);
    let throttle = input_throttle * vehicle.engine_force;

    // Resulting force in local car coordinates
    // This is for RWD only
    let traction_force_cx = throttle - brake * vehicle.local_velocity.x.signum();

    let traction_force_cy = 0.0;

    let drag_force_cx = -vehicle.roll_resistance * vehicle.local_velocity.x
        - vehicle.air_resistance * vehicle.local_velocity.x * vehicle.local_velocity.x.abs();
    let drag_force_cy = -vehicle.roll_resistance * vehicle.local_velocity.y
        - vehicle.air_resistance * vehicle.local_velocity.y * vehicle.local_velocity.y.abs();

    // Total foce in car coordinates
    let total_force_cx = drag_force_cx + traction_force_cx;
    let total_force_cy = drag_force_cy
        + traction_force_cy
        + vehicle.steer_angle.cos() * friction_force_front_cy
        + friction_force_rear_cy;

    println!("x: {:?}", total_force_cx);
    println!("y: {:?}", total_force_cy);

    // acceleration along car axes
    vehicle.local_acceleration.x = total_force_cx / vehicle.mass; // forward/reverse acceleration
    vehicle.local_acceleration.y = total_force_cy / vehicle.mass; // sideways acceleration

    // accleration in world coordinates
    vehicle.acceleration.x =
        heading_cos * vehicle.local_acceleration.x - heading_sin * vehicle.local_acceleration.y;
    vehicle.acceleration.y =
        heading_sin * vehicle.local_acceleration.x + heading_cos * vehicle.local_acceleration.y;

    // update velocity
    vehicle.velocity.x += vehicle.acceleration.x;
    vehicle.velocity.y += vehicle.acceleration.y;

    vehicle.absolute_velocity = vehicle.velocity.length();

    // calculate rotational forces
    let mut angular_torque = (friction_force_front_cy + traction_force_cy)
        * vehicle.center_to_front_axle
        - friction_force_rear_cy * vehicle.center_to_rear_axle;

    // Unstable at slow speeds, so just stop
    if vehicle.absolute_velocity.abs() < 0.5 && throttle == 0.0 {
        vehicle.velocity = Vec2::ZERO;
        vehicle.absolute_velocity = 0.0;

        angular_torque = 0.0;
        vehicle.yaw_rate = 0.0;
    }

    let angular_acceleration = angular_torque / vehicle.inertia;

    vehicle.yaw_rate += angular_acceleration * time.delta_secs();
    vehicle.heading += vehicle.yaw_rate;

    transform.rotate_z(vehicle.heading);

    // println!("velocity: {:?}", vehicle.velocity);
    // println!("heading: {:?}", vehicle.heading);

    let movement_direction = transform.rotation * Vec3::Y;
    transform.translation += movement_direction * vehicle.velocity.extend(0.0) * time.delta_secs();

    println!("translation: {:?}", transform.translation);
    // transform.translation.x += vehicle.velocity.y;
}

fn clamp(input: f32, min: f32, max: f32) -> f32 {
    input.max(min).min(max)
}

// pub fn old_move_player(
//     query: Single<(&Player, &mut Transform)>,
//     input: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
// ) {
//     let (player, mut transform) = query.into_inner();

//     let mut rotation_factor: f32 = 0.0;
//     let mut movement_factor: f32 = 0.0;

//     if input.pressed(player.settings.accelerate) {
//         movement_factor += 1.0;
//     }

//     if input.pressed(player.settings.decelerate) {
//         movement_factor -= 1.0;
//     }

//     if movement_factor.abs() > 0.0 {
//         if input.pressed(player.settings.steer_right) {
//             rotation_factor -= 1.0;
//         }

//         if input.pressed(player.settings.steer_left) {
//             rotation_factor += 1.0;
//         }

//         transform.rotate_z(rotation_factor * player.handling * time.delta_secs());
//     }

//     let movement_direction = transform.rotation * Vec3::Y;

//     let movement_distance = movement_factor * player.speed * time.delta_secs();

//     let translation_delta = movement_direction * movement_distance;

//     transform.translation += translation_delta;
// }
