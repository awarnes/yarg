use bevy::math::Vec2;

#[derive(Copy, Clone, Debug)]
pub struct Vehicle {
    // Car State
    pub heading: f32, // angle car is pointed at (radians)
    // pub position: Vec2,           // metres in world coords
    pub velocity: Vec2,           // m/s in world coords
    pub local_velocity: Vec2,     // m/s in local car coords (x is forward y is sideways)
    pub acceleration: Vec2,       // acceleration in world coords
    pub local_acceleration: Vec2, // accleration in local car coords
    pub absolute_velocity: f32,   // absolute velocity m/s
    pub yaw_rate: f32,            // angular velocity in radians
    pub steer: f32,               // amount of steering input (-1.0..1.0)
    pub steer_angle: f32,         // actual front wheel steer angle (-maxSteer..maxSteer)

    // Calculated from config
    pub inertia: f32,                 // will be = mass
    pub wheel_base: f32,              // set from axle to CG lengths
    pub axel_weight_ratio_front: f32, // % car weight on the front axle
    pub axel_weight_ratio_rear: f32,  // % car weight on the rear axle

    // Configuration Magic Constants
    pub gravity: f32, // m/s^2 TODO: Move to global config, use gravity scale instead
    pub mass: f32,    // kg
    // pub inertia_scale: f32, // Multiply by mass for inertia
    // pub half_width: f32, // Centre to side of chassis (metres)
    // pub center_to_front: f32, // Centre of gravity to front of chassis (metres)
    // pub center_to_rear: f32, // Centre of gravity to rear of chassis
    pub center_to_front_axle: f32, // Centre gravity to front axle
    pub center_to_rear_axle: f32,  // Centre gravity to rear axle
    pub center_height: f32,        // Centre gravity height
    // pub wheel_radius: f32, // Includes tire (also represents height of axle)
    // pub wheel_width: f32, // Used for render only
    pub tire_grip: f32, // How much grip tires have
    pub lock_grip: f32, // % of grip available when wheel is locked
    pub engine_force: f32,
    pub brake_force: f32,
    pub ebrake_force: f32,
    pub weight_transfer: f32, // How much weight is transferred during acceleration/braking
    pub max_steer: f32,       // Maximum steering angle in radians
    pub corner_stiffness_front: f32,
    pub corner_stiffness_rear: f32,
    pub air_resistance: f32, // air resistance (* vel)
    pub roll_resistance: f32,
}

impl Vehicle {
    pub fn default() -> Vehicle {
        let brake_force = 12000.0;
        let mass = 10.0;
        let inertia_scale = 1.0;

        let center_to_front_axle = 1.25;
        let center_to_rear_axle = 1.25;
        let wheel_base = center_to_front_axle + center_to_rear_axle;

        Vehicle {
            heading: 0.0, // angle car is pointed at (radians)
            // position: Vec2::ZERO,           // metres in world coords
            velocity: Vec2::ZERO,           // m/s in world coords
            local_velocity: Vec2::ZERO,     // m/s in local car coords (x is forward y is sideways)
            acceleration: Vec2::ZERO,       // acceleration in world coords
            local_acceleration: Vec2::ZERO, // accleration in local car coords
            absolute_velocity: 0.0,         // absolute velocity m/s
            yaw_rate: 0.0,                  // angular velocity in radians
            steer: 0.0,                     // amount of steering input (-1.0..1.0)
            steer_angle: 0.0,               // actual front wheel steer angle (-maxSteer..maxSteer)

            // Calculated from config
            inertia: mass * inertia_scale, // will be = mass
            wheel_base,                    // set from axle to CG lengths
            axel_weight_ratio_front: center_to_rear_axle / wheel_base, // % car weight on the front axle
            axel_weight_ratio_rear: center_to_front_axle / wheel_base, // % car weight on the rear axle

            gravity: 9.81,
            mass,
            // inertia_scale,        // Multiply by mass for inertia
            // half_width: 0.8,      // Centre to side of chassis (metres)
            // center_to_front: 2.0, // Centre of gravity to front of chassis (metres)
            // center_to_rear: 2.0,  // Centre of gravity to rear of chassis
            center_to_front_axle, // Centre gravity to front axle
            center_to_rear_axle,  // Centre gravity to rear axle
            center_height: 0.55,  // Centre gravity height
            // wheel_radius: 0.3,    // Includes tire (also represents height of axle)
            // wheel_width: 0.2,     // Used for render only
            tire_grip: 2.0, // How much grip tires have
            lock_grip: 0.7, // % of grip available when wheel is locked
            engine_force: 8000.0,
            brake_force,
            ebrake_force: brake_force / 2.5,
            weight_transfer: 0.2, // How much weight is transferred during acceleration/braking
            max_steer: 0.6,       // Maximum steering angle in radians
            corner_stiffness_front: 5.0,
            corner_stiffness_rear: 5.2,
            air_resistance: 2.5, // air resistance (* vel)
            roll_resistance: 8.0,
        }
    }
}
