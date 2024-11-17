use crate::{
    car::Car,
    draw::Canvas,
    error::{Error, Result},
    load_map::Map,
};
use minifb::Key;
use std::{thread, time};

const PLAYER_WIDTH: f32 = 80.;
const PLAYER_HEIGHT: f32 = 120.;
const WAITING_TIME: time::Duration = time::Duration::from_millis(1);

pub fn game_loop(map_path: &str) -> Result<()> {
    let mut canvas = Canvas::default();
    let mut car = Car::default();
    let map = Map::load_map(map_path)?;

    loop {
        canvas.clear();

        // process input
        canvas.get_keys_pressed().iter().for_each(|key| match key {
            Key::W => {
                car.forward_accelerate();
            }
            Key::A => {
                car.steer_left();
            }
            Key::S => {
                car.backward_acceleration();
            }
            Key::D => {
                car.steer_right();
            }
            _ => (),
        });

        car.update_car_position();
        canvas.draw_map(&map);
        canvas.draw_car(car.get_position(), (PLAYER_WIDTH, PLAYER_HEIGHT));
        canvas.update_window();
        thread::sleep(WAITING_TIME);
    }
}
