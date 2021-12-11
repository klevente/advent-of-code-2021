use advent_of_code_2021::parse_2d_number_grid;
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
}

fn main() {}
