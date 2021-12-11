use advent_of_code_2021::{
    parse_2d_number_grid, print_2d_array, print_u8_2d_array_with_delim, read_file_to_string,
};
use array2d::Array2D;

#[derive(Clone)]
struct OctopusGrid {
    tiles: Array2D<u8>,
    num_of_flashes: u32,
}

impl OctopusGrid {
    pub fn parse(s: &str) -> Self {
        Self {
            tiles: parse_2d_number_grid(s),
            num_of_flashes: 0,
        }
    }

    pub fn simulate_until_all_flash(&mut self) -> u32 {
        let mut step = 1;
        loop {
            self.step();
            print_2d_array(&self.tiles);
            if self.did_every_octopus_flash() {
                return step;
            }
            step += 1;
        }
    }

    pub fn simulate(&mut self, num_of_steps: u32) -> u32 {
        println!("Original state:");
        print_2d_array(&self.tiles);
        for step in 1..=num_of_steps {
            self.step();
            println!("Step {}:", step);
            print_2d_array(&self.tiles);
        }

        self.num_of_flashes
    }

    pub fn step(&mut self) {
        self.increment_energy_levels();
        self.flash_tiles_in_positions(self.gather_tiles_to_be_flashed());
    }

    fn increment_energy_levels(&mut self) {
        for tile in self.tiles.elements_row_major_iter_mut() {
            *tile += 1;
        }
    }

    fn flash_tiles_in_positions(&mut self, tiles_to_flash: Vec<(usize, usize)>) {
        print_u8_2d_array_with_delim(&self.tiles);
        if tiles_to_flash.is_empty() {
            return;
        }

        for (x, y) in tiles_to_flash {
            self.flash(x, y);
        }

        self.flash_tiles_in_positions(self.gather_tiles_to_be_flashed());
    }

    fn gather_tiles_to_be_flashed(&self) -> Vec<(usize, usize)> {
        let mut tiles_to_flash = Vec::new();
        for (y, x) in self.tiles.indices_row_major() {
            if *self.tiles.get(y, x).unwrap() > 9 {
                tiles_to_flash.push((x, y));
            }
        }

        tiles_to_flash
    }

    fn flash(&mut self, x: usize, y: usize) {
        self.num_of_flashes += 1;

        self.tiles.set(y, x, 0).unwrap();

        let x = x as i32;
        let y = y as i32;
        let neighbour_positions = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        let neighbour_positions_filtered = neighbour_positions.iter().filter_map(|(x, y)| {
            if *x >= 0 && *y >= 0 {
                Some((*x as usize, *y as usize))
            } else {
                None
            }
        });

        for (x, y) in neighbour_positions_filtered {
            if let Some(element) = self.tiles.get_mut(y, x) {
                if *element > 0 {
                    *element += 1;
                }
            }
        }
    }

    fn did_every_octopus_flash(&self) -> bool {
        self.tiles.elements_row_major_iter().all(|&o| o == 0)
    }
}

fn main() {
    let input = read_file_to_string("input/day11.txt");

    let mut grid = OctopusGrid::parse(&input);

    let num_of_flashes = grid.clone().simulate(100);
    println!(
        "The number of flashes after 100 steps is {}",
        num_of_flashes
    );

    let step_when_all_octopuses_flash = grid.simulate_until_all_flash();
    println!(
        "The step when all octopuses flash is step {}",
        step_when_all_octopuses_flash
    );
}
