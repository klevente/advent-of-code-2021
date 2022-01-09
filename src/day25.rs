use advent_of_code_2021::read_file_to_string;
use array2d::Array2D;

#[derive(Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    East,
    South,
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '>' => Self::East,
            'v' => Self::South,
            _ => unreachable!(),
        }
    }
}

struct OceanFloor {
    tiles: Array2D<Tile>,
}

impl OceanFloor {
    pub fn parse(s: &str) -> Self {
        let tiles = &*s
            .lines()
            .map(|l| l.chars().map(|c| Tile::from_char(c)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            tiles: Array2D::from_rows(tiles).unwrap(),
        }
    }

    pub fn step_until_stuck(&mut self) -> usize {
        let mut num_of_steps = 1;
        while !self.step() {
            num_of_steps += 1;
        }
        num_of_steps
    }

    fn step(&mut self) -> bool {
        let (east_herd, south_herd) = self.get_east_and_south_herds();

        let movable_east_herd = east_herd
            .into_iter()
            .filter(|&(x, y)| self.is_east_neighbour_free(x, y))
            .collect::<Vec<_>>();

        for &(x, y) in &movable_east_herd {
            self.move_east(x, y);
        }

        let movable_south_herd = south_herd
            .into_iter()
            .filter(|&(x, y)| self.is_south_neighbour_free(x, y))
            .collect::<Vec<_>>();

        for &(x, y) in &movable_south_herd {
            self.move_south(x, y);
        }

        movable_east_herd.is_empty() && movable_south_herd.is_empty()
    }

    fn get_east_and_south_herds(&self) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let mut east_herd = Vec::new();
        let mut south_herd = Vec::new();
        for (y, x) in self.tiles.indices_row_major() {
            let tile = self.tiles.get(y, x).unwrap();
            match tile {
                Tile::East => east_herd.push((x, y)),
                Tile::South => south_herd.push((x, y)),
                _ => {}
            }
        }

        (east_herd, south_herd)
    }

    fn move_east(&mut self, x: usize, y: usize) {
        let (n_x, n_y) = self.east_neighbour_pos(x, y);
        self.tiles.set(y, x, Tile::Empty).unwrap();
        self.tiles.set(n_y, n_x, Tile::East).unwrap();
    }

    fn is_east_neighbour_free(&self, x: usize, y: usize) -> bool {
        *self.east_neighbour(x, y) == Tile::Empty
    }

    fn east_neighbour(&self, x: usize, y: usize) -> &Tile {
        let (n_x, n_y) = self.east_neighbour_pos(x, y);
        self.tiles.get(n_y, n_x).unwrap()
    }

    fn east_neighbour_pos(&self, x: usize, y: usize) -> (usize, usize) {
        let n_x = if x == self.tiles.num_columns() - 1 {
            0
        } else {
            x + 1
        };
        (n_x, y)
    }

    fn move_south(&mut self, x: usize, y: usize) {
        let (n_x, n_y) = self.south_neighbour_pos(x, y);
        self.tiles.set(y, x, Tile::Empty).unwrap();
        self.tiles.set(n_y, n_x, Tile::South).unwrap();
    }

    fn is_south_neighbour_free(&self, x: usize, y: usize) -> bool {
        *self.south_neighbour(x, y) == Tile::Empty
    }

    fn south_neighbour(&self, x: usize, y: usize) -> &Tile {
        let (n_x, n_y) = self.south_neighbour_pos(x, y);
        self.tiles.get(n_y, n_x).unwrap()
    }

    fn south_neighbour_pos(&self, x: usize, y: usize) -> (usize, usize) {
        let n_y = if y == self.tiles.num_rows() - 1 {
            0
        } else {
            y + 1
        };
        (x, n_y)
    }
}

fn main() {
    let input = read_file_to_string("input/day25.txt");

    let mut ocean_floor = OceanFloor::parse(&input);

    let result = ocean_floor.step_until_stuck();
    println!(
        "The first step on which no sea cucumbers move is step {}",
        result
    );
}
