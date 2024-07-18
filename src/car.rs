use std::cmp::max_by;

const MAX_FORWARD_SPEED: f32 = 0.005;
const MAX_BACKWARD_SPEED: f32 = 0.003;

const MIN_FORWARD_SPEED: f32 = 0.001;
const MIN_BACKWARD_SPEED: f32 = 0.001;

const FORWARD_ACCELERATION: f32 = 0.0002;
const BACKWARD_ACCELERATION: f32 = 0.0002;

const STEERING_SENSITIVITY: f32 = 3.0 * std::f32::consts::PI;

#[derive(Debug)]
pub struct Car {
    x: f32,
    y: f32,
    speed: f32,
    orientation: f32,
}

impl Car {
    pub fn default() -> Self {
        Self {
            x: 0.5,
            y: 0.5,
            speed: 0.0,
            orientation: 0.0,
        }
    }

    pub fn forward_accelerate(&mut self) {
        let speed = self.speed + FORWARD_ACCELERATION;
        self.speed = if speed > MAX_FORWARD_SPEED {
            MAX_BACKWARD_SPEED
        } else if speed > self.speed && self.speed < 0.0 && speed.abs() < MIN_FORWARD_SPEED {
            0.0
        } else {
            speed
        };
    }

    pub fn backward_acceleration(&mut self) {
        let speed = self.speed - BACKWARD_ACCELERATION;

        self.speed = if speed < -MAX_BACKWARD_SPEED {
            -MAX_BACKWARD_SPEED
        } else if speed < self.speed && self.speed > 0.0 && speed.abs() < MIN_BACKWARD_SPEED {
            0.0
        } else {
            speed
        };
    }

    pub fn update_car_position(&mut self) {
        self.y += self.orientation.sin() * self.speed;
        self.x += self.orientation.cos() * self.speed;
    }

    pub fn steer_left(&mut self) {
        self.orientation -= STEERING_SENSITIVITY * self.speed;
    }

    pub fn steer_right(&mut self) {
        self.orientation += STEERING_SENSITIVITY * self.speed;
    }

    pub fn get_position(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.orientation)
    }
}
