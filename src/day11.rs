use advent_of_code_2021::{parse_2d_number_grid, read_file_to_string};
use array2d::Array2D;

struct OctopusGrid {
    tiles: Array2D<u8>,
}

impl OctopusGrid {
    pub fn parse(s: &str) -> Self {
        Self {
            tiles: parse_2d_number_grid(s),
        }
    }

    fn step(&mut self) {
        for tile in self.tiles.elements_row_major_iter_mut() {
            *tile += 1;
        }

        dbg!(&self.tiles);
    }
}

fn main() {
    let input = read_file_to_string("input/day11.txt");

    let mut grid = OctopusGrid::parse(&input);

    grid.step();
}
