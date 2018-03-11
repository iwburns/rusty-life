use std::ops::Index;

use rand::Rng;
use rand;

use super::GRID_SIZE;

pub struct State {
    pub cell_count: usize,
    pub cells: [[bool; GRID_SIZE]; GRID_SIZE],
}

impl State {
    pub fn new() -> State {

        let mut rng = rand::thread_rng();

        let mut cells = [[true; GRID_SIZE]; GRID_SIZE];
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                cells[i][j] = rng.gen();
            }
        }

        State {
            cell_count: GRID_SIZE * GRID_SIZE,
            cells,
        }
    }
}

impl Index<usize> for State {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= GRID_SIZE * GRID_SIZE {
            panic!("Index out of bounds when trying to access State's cells.")
        }

        let div = index / GRID_SIZE;
        let rem = index % GRID_SIZE;

        &self.cells[div][rem]
    }
}