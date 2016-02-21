extern crate piston_window;
extern crate graphics;

use piston_window::*;

fn sin_x(x: f64) -> f64 {
    (x * 3.1415 / 180.0).sin() * 150.0 + 240.0
}

fn main() {
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    let window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
                                   .exit_on_esc(true)
                                   .build()
                                   .unwrap_or_else(|e| panic!("Failed:, {}", e));
    for e in window {
        e.draw_2d(|c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            g.clear_stencil(0);
            let mut x = 0.0;
            while x < 680.0 {
                Rectangle::new(RED).draw([x, sin_x(x), 1.0, 1.0], &c.draw_state, c.transform, g);
                x += 1.0;
            }
        });
    }
}
