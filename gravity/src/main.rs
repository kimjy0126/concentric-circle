extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use graphics::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

type Position = [f64; 2];
type Vector2d = [f64; 2];

struct Ball {
    pos: Position,
    velocity: Vector2d,
    radius: f64,
    is_dropping: bool,
}

struct App {
    ball: Ball,
    g: f64,
    speed: f64,
    epsilon: f64,   // Yeah, I know this looks silly...
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Gravity", [1400, 700])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let epsilon = 0.000001;
    let ball = Ball {
        pos: [700.0, 200.0 + epsilon],
        velocity: [0.0, 0.0],
        radius: 20.0,
        is_dropping: true,
    };

    let mut app = App {
        ball,
        g: 9.8,
        speed: 2.0,
        epsilon,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                let transform = c.transform.trans(0.0, 0.0);

                clear(color::WHITE, g);

                ellipse(color::RED,
                        rectangle::square(app.ball.pos[0] - app.ball.radius, app.ball.pos[1] - app.ball.radius, app.ball.radius * 2.),
                        transform, g);
            });
        }

        if let Some(_u) = e.update_args() {
            app.ball.velocity[1] = (2. * app.g * (app.ball.pos[1] - 200.0)).sqrt() / 20.;
            if !app.ball.is_dropping {
                app.ball.velocity[1] *= -1.
            }
            app.ball.pos[0] += app.ball.velocity[0];
            app.ball.pos[1] += app.ball.velocity[1];

            if app.ball.pos[1] > 700.0 {
                app.ball.pos[1] = 1400.0 - app.ball.pos[1];
                app.ball.is_dropping = false;
            }
            if app.ball.pos[1] <= 200.0 + app.epsilon {
                app.ball.pos[1] = (200.0 + app.epsilon) * 2. - app.ball.pos[1];
                app.ball.is_dropping = true;
            }
        }

        if let Some(b) = e.press_args() {
            if let Button::Keyboard(Key::Left) = b { 
                app.ball.velocity[0] -= app.speed;
            }
            if let Button::Keyboard(Key::Right) = b {
                app.ball.velocity[0] += app.speed;
            }
        }

        if let Some(b) = e.release_args() {
            if let Button::Keyboard(Key::Left) = b {
                app.ball.velocity[0] += app.speed;
            }
            if let Button::Keyboard(Key::Right) = b {
                app.ball.velocity[0] -= app.speed;
            }
        }
    }
}
