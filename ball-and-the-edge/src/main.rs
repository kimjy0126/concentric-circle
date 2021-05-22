extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
use graphics::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

type Position = [f64; 2];

struct App {
    current_pos: Position,
    velocity: [f64; 2],
    radius: f64,
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Ball and the Edge", [1400, 1000])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut app = App {
        current_pos: [700.0, 500.0],
        velocity: [10.0, 5.0],
        radius: 10.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                let transform = c.transform.trans(0.0, 0.0);

                clear(color::WHITE, g);

                ellipse(color::BLACK,
                        rectangle::square(app.current_pos[0] - app.radius, app.current_pos[1] - app.radius, app.radius * 2.),
                        transform, g);
            });
        }

        if let Some(_u) = e.update_args() {
            app.current_pos[0] += app.velocity[0];
            app.current_pos[1] += app.velocity[1];

            /* collide with the edge */
            if app.current_pos[0] < 0.0 + app.radius {
                app.current_pos[0] = (0.0 + app.radius) + ((0.0 + app.radius) - app.current_pos[0]);
                app.velocity[0] *= -1.;
            }
            if app.current_pos[0] > 1400.0 - app.radius {
                app.current_pos[0] = (1400.0 - app.radius) - (app.current_pos[0] - (1400.0 - app.radius));
                app.velocity[0] *= -1.;
            }
            if app.current_pos[1] < app.radius {
                app.current_pos[1] = (0.0 + app.radius) + ((0.0 + app.radius) - app.current_pos[1]);
                app.velocity[1] *= -1.;
            }
            if app.current_pos[1] > 1000.0 - app.radius {
                app.current_pos[1] = (1000.0 - app.radius) - (app.current_pos[1] - (1000.0 - app.radius));
                app.velocity[1] *= -1.;
            }
        }
    }
}
