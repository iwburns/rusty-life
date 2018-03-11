use rand::Rng;
use rand;

pub struct State {
    pub side_len: usize,
    pub cells: Vec<bool>,
}

impl State {
    pub fn new(side_len: usize) -> State {
        let mut rng = rand::thread_rng();

        let mut cells = Vec::with_capacity(side_len * side_len);
        for i in 0..cells.capacity() {
            cells.push(rng.gen());
        }

        State {
            side_len,
            cells,
        }
    }

    pub fn get_num_live_neighbors(&self, index: usize) -> u32 {
        let neighbor_indices = self.get_neighbor_indices(index);
        let mut living = 0;
        for &i in neighbor_indices.iter() {
            if self.cells[i] {
                living += 1;
            }
        }
        living
    }

    fn get_neighbor_indices(&self, index: usize) -> [usize; 8] {
        let i = index as isize;
        let side_len = self.side_len as isize;
        let total_cells = side_len * side_len;

        let up = modulo(i - side_len, total_cells);
        let down = modulo(i + side_len, total_cells);
        let left = modulo(i - 1, side_len) + (i / side_len) * side_len;
        let right = modulo(i + 1, side_len) + (i / side_len) * side_len;
        let up_left = modulo(left - side_len, total_cells);
        let up_right = modulo(right - side_len, total_cells);
        let down_left = modulo(left + side_len, total_cells);
        let down_right = modulo(right + side_len, total_cells);

        [
            up_left as usize,
            up as usize,
            up_right as usize,
            left as usize,
            right as usize,
            down_left as usize,
            down as usize,
            down_right as usize,
        ]
    }
}

fn modulo(value: isize, range_max: isize) -> isize {
    let rem = value % range_max;
    if rem.is_negative() {
        return range_max + rem;
    }
    rem
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_modulo() {
        let m = modulo(2, 4);
        assert_eq!(m, 2);

        let m = modulo(5, 4);
        assert_eq!(m, 1);

        let m = modulo(10, 4);
        assert_eq!(m, 2);

        let m = modulo(-1, 4);
        assert_eq!(m, 3);
    }

    #[test]
    fn get_neighbor_indices_simple() {
        let state = State::new(4);

        let neighbors = state.get_neighbor_indices(5);

        assert_eq!(neighbors[0], 0);
        assert_eq!(neighbors[1], 1);
        assert_eq!(neighbors[2], 2);
        assert_eq!(neighbors[3], 4);
        assert_eq!(neighbors[4], 6);
        assert_eq!(neighbors[5], 8);
        assert_eq!(neighbors[6], 9);
        assert_eq!(neighbors[7], 10);
    }

    #[test]
    fn get_neighbor_indices_complex() {
        let state = State::new(4);

        let neighbors = state.get_neighbor_indices(0);

        assert_eq!(neighbors[0], 15);
        assert_eq!(neighbors[1], 12);
        assert_eq!(neighbors[2], 13);
        assert_eq!(neighbors[3], 3);
        assert_eq!(neighbors[4], 1);
        assert_eq!(neighbors[5], 7);
        assert_eq!(neighbors[6], 4);
        assert_eq!(neighbors[7], 5);
    }
}
