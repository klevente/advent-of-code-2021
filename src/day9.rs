use advent_of_code_2021::{print_2d_array, read_file_to_string, vec_to_array};
use array2d::Array2D;
use itertools::Itertools;
use std::collections::HashSet;

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

    pub fn find_local_minima_values(&self) -> Vec<(usize, usize, u8)> {
        let mut res = Vec::new();
        for (y, row) in self.tiles.rows_iter().enumerate() {
            for (x, column) in row.enumerate() {
                if self.is_local_minimum(x, y) {
                    res.push((x, y, *column));
                    println!("Found local minimum: ({}, {}) -> {}", y, x, column);
                }
            }
        }
        res
    }

    pub fn calculate_risk_value(&self) -> u32 {
        self.find_local_minima_values()
            .iter()
            .map(|(_, _, x)| (x + 1) as u32)
            .sum()
    }

    pub fn print_map(&self) {
        print_2d_array(&self.tiles);
    }

    pub fn get_basin_sizes(&self) -> Vec<u32> {
        let local_minima = self.find_local_minima_values();

        local_minima
            .iter()
            .map(|&(x, y, _)| self.get_basin_size(x, y))
            .collect()
    }

    pub fn get_largest_3_basin_sizes(&self) -> [u32; 3] {
        vec_to_array(
            self.get_basin_sizes()
                .iter()
                .sorted_by(|a, b| b.cmp(a))
                .take(3)
                .map(|d| *d)
                .collect::<Vec<_>>(),
        )
    }

    pub fn calculate_product_of_three_largest_basin_sizes(&self) -> u32 {
        self.get_largest_3_basin_sizes()
            .iter()
            .fold(1, |acc, next| acc * next)
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

    fn get_basin_size(&self, x: usize, y: usize) -> u32 {
        let mut positions = HashSet::new();
        self.get_basin_size_rec(x as i32, y as i32, &mut positions);
        positions.len() as u32
    }

    fn get_basin_size_rec(&self, x: i32, y: i32, acc: &mut HashSet<(usize, usize)>) {
        if x < 0
            || x >= self.tiles.num_columns() as i32
            || y < 0
            || y >= self.tiles.num_rows() as i32
        {
            return;
        }

        if self.is_top(x, y) {
            return;
        }

        if acc.contains(&(x as usize, y as usize)) {
            return;
        }

        acc.insert((x as usize, y as usize));

        self.get_basin_size_rec(x - 1, y, acc);
        self.get_basin_size_rec(x, y - 1, acc);
        self.get_basin_size_rec(x + 1, y, acc);
        self.get_basin_size_rec(x, y + 1, acc);
    }

    fn is_top(&self, x: i32, y: i32) -> bool {
        *self.tiles.get(y as usize, x as usize).unwrap() == 9
    }
}

fn main() {
    let input = read_file_to_string("input/day9.txt");
    let height_map = HeightMap::parse(&input);

    height_map.print_map();

    let risk_value = height_map.calculate_risk_value();
    println!("The risk value is {}", risk_value);

    let result = height_map.calculate_product_of_three_largest_basin_sizes();
    println!("The product of the 3 largest basin sizes is {}", result);
}
