extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod state;
use state::*;

const GRID_SIZE: usize = 200;
const CELL_SIZE: f64 = 2f64;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct App {
    gl: GlGraphics,
    state: State,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let squares: Vec<([f64; 4], [f32; 4])> = (0..self.state.cell_count).into_iter().map(|index| {
            let color = if self.state[index] { WHITE } else { BLACK };
            (rectangle::square(0.0, 0.0, CELL_SIZE), color)
        }).collect();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let mut x_offset = 0f64;
            let mut y_offset = 0f64;
            let mut cell_count = 0usize;

            for (square, color) in squares {
                let transform = c.transform.trans(x_offset, y_offset);
                rectangle(color, square, transform, gl);
                x_offset += CELL_SIZE;
                cell_count += 1;

                if cell_count == GRID_SIZE {
                    cell_count = 0;
                    x_offset = 0.0;
                    y_offset += CELL_SIZE;
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
//        for cell in self.state.cells.iter_mut() {
//            *cell = !*cell;
//        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "rusty-life",
        [200, 200]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        state: State::new(),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
