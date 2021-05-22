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
type Vector2d = [f64; 2];

struct App {
    lclick: bool,
    hold_pos: Position,
    current_pos: Position,
    mouse_pos: Position,
    velocity: Vector2d,
    radius: f64,
    arrow: Vector2d,
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Drag and Shoot", [1400, 1000])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut app = App {
        lclick: false,
        hold_pos: [700.0, 500.0],
        current_pos: [700.0, 500.0],
        mouse_pos: [0.0, 0.0],
        velocity: [15.0, 8.0],
        radius: 10.0,
        arrow: [0.0, 0.0],
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

                let arrow_size = (app.arrow[0].powi(2) + app.arrow[1].powi(2)).sqrt();

                let mut start_of_arrow: Position = [0.0, 0.0];
                start_of_arrow[0] = app.current_pos[0] + 30. * app.arrow[0] / arrow_size;
                start_of_arrow[1] = app.current_pos[1] + 30. * app.arrow[1] / arrow_size;
                let mut end_of_arrow: Position = [0.0, 0.0];
                end_of_arrow = start_of_arrow + app.arrow;
                end_of_arrow[0] = start_of_arrow[0] + app.arrow[0];
                end_of_arrow[1] = start_of_arrow[1] + app.arrow[1];

                let mut left_of_triangle: Position = [0.0, 0.0];
                left_of_triangle[0] = end_of_arrow[0] - 20. * app.arrow[0] / arrow_size - (20. * app.arrow[1] / arrow_size);
                left_of_triangle[1] = end_of_arrow[1] - 20. * app.arrow[1] / arrow_size + (20. * app.arrow[0] / arrow_size);
                let mut right_of_triangle: Position = [0.0, 0.0];
                right_of_triangle[0] = end_of_arrow[0] - 20. * app.arrow[0] / arrow_size + (20. * app.arrow[1] / arrow_size);
                right_of_triangle[1] = end_of_arrow[1] - 20. * app.arrow[1] / arrow_size - (20. * app.arrow[0] / arrow_size);

                let triangle = [left_of_triangle, right_of_triangle, end_of_arrow];
                polygon(color::RED,
                        &triangle,
                        transform, g);

                end_of_arrow[0] -= 20. * app.arrow[0] / arrow_size;
                end_of_arrow[1] -= 20. * app.arrow[1] / arrow_size;
                line_from_to(color::RED,
                             10.0,
                             start_of_arrow,
                             end_of_arrow,
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

        if let Some(b) = e.press_args() {
            if let Button::Mouse(MouseButton::Left) = b {
                app.lclick = true;
                app.hold_pos = app.mouse_pos;
                app.velocity[0] = 0.0;
                app.velocity[1] = 0.0;
            }
        }

        if let Some(b) = e.release_args() {
            if let Button::Mouse(MouseButton::Left) = b {
                if app.lclick {
                    /* drop and shoot time!! */
                    app.velocity[0] = app.arrow[0] / 25.;
                    app.velocity[1] = app.arrow[1] / 25.;
                }

                app.lclick = false;
                app.arrow = [0.0, 0.0];
            }
        }

        if let Some(m) = e.mouse_cursor_args() {
            if app.lclick {
                /* dragging */
                app.arrow[0] = app.hold_pos[0] - m[0];
                app.arrow[1] = app.hold_pos[1] - m[1];
            }
            app.mouse_pos = m;
        }
    }
}
