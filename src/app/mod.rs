mod state;
use self::state::*;

use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };
use rayon::prelude::*;

const GRID_SIZE: usize = 200;
const CELL_SIZE: f64 = 5f64;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct App {
    gl: GlGraphics,
    state: State,
}

impl App {
    pub fn new(opengl: OpenGL) -> App {
        App {
            gl: GlGraphics::new(opengl),
            state: State::new(GRID_SIZE),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let squares: Vec<([f64; 4], [f32; 4])> = self.state.cells.par_iter()
            .enumerate()
            .map(|(i, &cell)| {
                let color = if cell { WHITE } else { BLACK };
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

    pub fn update(&mut self, args: &UpdateArgs) {
        let new_cells = self.state.cells.par_iter()
            .enumerate()
            .map(|(i, cell)| {
                let living_neighbors = self.state.get_num_live_neighbors(i);
                if !cell {
                    living_neighbors == 3
                } else {
                    living_neighbors == 2 || living_neighbors == 3
                }
            }).collect();

        self.state.cells = new_cells;
    }
}