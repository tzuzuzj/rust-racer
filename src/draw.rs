use minifb::{KeyRepeat, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

pub struct Canvas {
    window: Window,
    dt: DrawTarget,
}

trait PathBuilderExtention {
    fn rect_rotatable(&mut self, x: f32, y: f32, width: f32, height: f32, orientation: f32);
    fn compute_start_of_rect(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        orientation: f32,
    ) -> (f32, f32);
}

impl PathBuilderExtention for PathBuilder {
    fn compute_start_of_rect(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        orientation: f32,
    ) -> (f32, f32) {
        let x_new = x - orientation.cos() * height / 2.0;
        let y_new = y - orientation.sin() * height / 2.0;

        let x_new = x_new - (orientation - std::f32::consts::PI / 2.0).cos() * width / 2.0;
        let y_new = y_new - (orientation - std::f32::consts::PI / 2.0).sin() * width / 2.0;

        (x_new, y_new)
    }

    fn rect_rotatable(&mut self, x: f32, y: f32, width: f32, height: f32, orientation: f32) {
        let (x_corrected, y_corrected) =
            PathBuilder::compute_start_of_rect(x, y, width, height, orientation);

        self.move_to(x_corrected, y_corrected);

        let x_new = x_corrected + orientation.cos() * height;
        let y_new = y_corrected + orientation.sin() * height;
        self.line_to(x_new, y_new);

        let x_new = x_new + (orientation - std::f32::consts::PI / 2.0).cos() * width;
        let y_new = y_new + (orientation - std::f32::consts::PI / 2.0).sin() * width;
        self.line_to(x_new, y_new);

        let x_new = x_corrected + (orientation - std::f32::consts::PI / 2.0).cos() * width;
        let y_new = y_corrected + (orientation - std::f32::consts::PI / 2.0).sin() * width;
        self.line_to(x_new, y_new);

        self.line_to(x_corrected, y_corrected);

        self.close();
    }
}

impl Canvas {
    pub fn default() -> Self {
        let window = creat_window();
        let dt = DrawTarget::new(WIDTH as i32, HEIGHT as i32);
        Self { window, dt }
    }

    pub fn clear(&mut self) {
        self.dt.clear(get_background_color());
    }

    pub fn draw_object(
        &mut self,
        (x, y, orientation): (f32, f32, f32),
        (width, height): (f32, f32),
    ) {
        let (x_pixel, y_pixel) = relative_position_to_pixel((x, y));

        let mut pb = PathBuilder::new();
        pb.rect_rotatable(x_pixel, y_pixel, width, height, orientation);
        let path = pb.finish();
        self.dt.fill(
            &path,
            &Source::Solid(get_player_color()),
            &DrawOptions::new(),
        );
    }

    pub fn update_window(&mut self) {
        self.window
            .update_with_buffer(self.dt.get_data(), WIDTH, HEIGHT)
            .unwrap();
    }

    pub fn get_keys_pressed(&mut self) -> Vec<minifb::Key> {
        self.window.get_keys_pressed(KeyRepeat::Yes)
    }
}

fn creat_window() -> Window {
    Window::new(
        "Raqote",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap()
}

fn get_background_color() -> SolidSource {
    SolidSource::from_unpremultiplied_argb(0xff, 0x30, 0x30, 0x40)
}

fn get_player_color() -> SolidSource {
    SolidSource::from_unpremultiplied_argb(0xff, 0, 0xa0, 0x20)
}

fn relative_position_to_pixel(position: (f32, f32)) -> (f32, f32) {
    (position.0 * WIDTH as f32, position.1 * HEIGHT as f32)
}