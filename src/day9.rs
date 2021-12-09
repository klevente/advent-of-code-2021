use advent_of_code_2021::{print_2d_array, read_file_to_string};
use array2d::Array2D;

#[derive(Debug)]
struct HeightMap {
    tiles: Array2D<u8>,
}

impl HeightMap {
    pub fn parse(s: &str) -> Self {
        let elements = &*s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|d| d.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            tiles: Array2D::from_rows(elements),
        }
    }

    pub fn find_local_minima_values(&self) -> Vec<u8> {
        let mut res = Vec::new();
        for (y, row) in self.tiles.rows_iter().enumerate() {
            for (x, column) in row.enumerate() {
                if self.is_local_minimum(x, y) {
                    res.push(*column);
                    println!("Found local minimum: ({}, {}) -> {}", y, x, column);
                }
            }
        }
        res
    }

    pub fn calculate_risk_value(&self) -> u32 {
        self.find_local_minima_values()
            .iter()
            .map(|x| (x + 1) as u32)
            .sum()
    }

    pub fn print_map(&self) {
        print_2d_array(&self.tiles);
    }

    fn is_local_minimum(&self, x: usize, y: usize) -> bool {
        let height = self.tiles.get(y, x).unwrap();

        let mut neighbours = Vec::with_capacity(4);

        if y != 0 {
            neighbours.push(*self.tiles.get(y - 1, x).unwrap());
        }

        if x != 0 {
            neighbours.push(*self.tiles.get(y, x - 1).unwrap());
        }

        if (y + 1) != self.tiles.num_rows() {
            neighbours.push(*self.tiles.get(y + 1, x).unwrap());
        }

        if (x + 1) != self.tiles.num_columns() {
            neighbours.push(*self.tiles.get(y, x + 1).unwrap());
        }

        neighbours.iter().all(|neighbour| height < neighbour)
    }
}

fn main() {
    let input = read_file_to_string("input/day9.txt");
    let height_map = HeightMap::parse(&input);

    height_map.print_map();

    let risk_value = height_map.calculate_risk_value();
    println!("The risk value is {}", risk_value);
}
