use advent_of_code_2021::{parse_2d_number_grid, read_file_to_string};
use array2d::Array2D;
use std::time::Instant;

type Coords = (usize, usize);

struct Map {
    tiles: Array2D<u8>,
    end: Coords,
}

impl Map {
    pub fn parse(s: &str) -> Self {
        let tiles = parse_2d_number_grid(s);
        let end = (tiles.num_columns() - 1, tiles.num_rows() - 1);
        Self { tiles, end }
    }

    pub fn parse_whole(s: &str) -> Self {
        let small = parse_2d_number_grid(s);
        let small_width = small.num_columns();
        let small_height = small.num_rows();
        let width = small_width * 5;
        let height = small_height * 5;
        let mut tiles = Array2D::filled_with(0u8, height, width);

        for (row, column) in tiles.indices_row_major() {
            let t_r = row / small_height;
            let t_c = column / small_width;
            let r = row % small_height;
            let c = column % small_width;

            let value = small.get(r, c).unwrap();
            let mut value_modified = (*value as usize + t_r + t_c) as u8;
            if value_modified > 9 {
                value_modified -= 9;
            }

            tiles.set(row, column, value_modified).unwrap();
        }

        let end = (tiles.num_columns() - 1, tiles.num_rows() - 1);
        Self { tiles, end }
    }

    pub fn calculate_lowest_risk_value(&self) -> usize {
        let mut risk_values_in_tiles =
            Array2D::filled_with(usize::MAX, self.tiles.num_rows(), self.tiles.num_columns());
        risk_values_in_tiles.set(0, 0, 0).unwrap();
        risk_values_in_tiles
            .set(1, 0, *self.tiles.get(1, 0).unwrap() as usize)
            .unwrap();
        risk_values_in_tiles
            .set(0, 1, *self.tiles.get(0, 1).unwrap() as usize)
            .unwrap();

        let mut correction_happened = true;
        while correction_happened {
            correction_happened = false;
            correction_happened |= self.step(&mut risk_values_in_tiles);
        }

        let (m_y, m_x) = self.end;
        *risk_values_in_tiles.get(m_y, m_x).unwrap()
    }

    fn step(&self, risk_values: &mut Array2D<usize>) -> bool {
        let mut correction_happened = false;
        for (row, column) in self.tiles.indices_row_major() {
            let risk_value = *risk_values.get(row, column).unwrap();

            let neighbours = self.get_neighbours((column, row));
            for (n_x, n_y) in &neighbours {
                let neighbour_risk_value = *risk_values.get(*n_y, *n_x).unwrap();

                let potential_risk = risk_value + *self.tiles.get(*n_y, *n_x).unwrap() as usize;
                if potential_risk < neighbour_risk_value {
                    risk_values.set(*n_y, *n_x, potential_risk).unwrap();
                    correction_happened = true;
                }
            }
        }

        correction_happened
    }

    fn get_neighbours(&self, tile: Coords) -> Vec<Coords> {
        let (x, y) = tile;
        let (max_x, max_y) = self.end;
        let mut neighbours = Vec::with_capacity(4);
        if x != 0 {
            neighbours.push((x - 1, y));
        }
        if y != 0 {
            neighbours.push((x, y - 1));
        }
        if x != max_x {
            neighbours.push((x + 1, y));
        }
        if y != max_y {
            neighbours.push((x, y + 1));
        }

        neighbours
    }
}

fn calculate_original_risk_value(s: &str) {
    let map = Map::parse(&s);
    let start = Instant::now();
    let minimum_risk_value_original = map.calculate_lowest_risk_value();
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!(
        "The minimum risk value of the original map is {}",
        minimum_risk_value_original
    );
}

fn calculate_whole_risk_value(s: &str) {
    let map = Map::parse_whole(&s);
    let start = Instant::now();
    let minimum_risk_value_original = map.calculate_lowest_risk_value();
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!(
        "The minimum risk value of the whole map is {}",
        minimum_risk_value_original
    );
}

fn main() {
    let input = read_file_to_string("input/day15.txt");

    calculate_original_risk_value(&input);
    calculate_whole_risk_value(&input);
}
