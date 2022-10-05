extern crate core;

use crate::Amphipod::{A, B, C, D};
use advent_of_code_2021::read_file_lines;
use sscanf::scanf;

#[derive(Debug, Copy, Clone)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl From<char> for Amphipod {
    fn from(value: char) -> Self {
        match value {
            'A' => A,
            'B' => B,
            'C' => C,
            'D' => D,
            _ => panic!("Invalid Amphipod"),
        }
    }
}

#[derive(Debug)]
struct Room {
    home_for: Amphipod,
    bottom: Option<Amphipod>,
    top: Option<Amphipod>,
}

impl Room {
    pub fn new(home_for: Amphipod, bottom: Amphipod, top: Amphipod) -> Self {
        Self {
            home_for,
            bottom: Some(bottom),
            top: Some(top),
        }
    }
}

#[derive(Debug)]
struct Hallway {
    spaces: [Option<Amphipod>; 11],
}

impl Hallway {
    pub fn new() -> Self {
        Self { spaces: [None; 11] }
    }
}

#[derive(Debug)]
struct Burrow {
    hallway: Hallway,
    rooms: [Room; 4],
}

impl Burrow {
    pub fn new(input: &Vec<String>) -> Self {
        let top_row = input.get(2).unwrap();
        let bottom_row = input.get(3).unwrap();

        let (t_a, t_b, t_c, t_d) =
            scanf!(top_row, "###{}#{}#{}#{}###", char, char, char, char).unwrap();

        let (b_a, b_b, b_c, b_d) =
            scanf!(bottom_row, "  #{}#{}#{}#{}#", char, char, char, char).unwrap();

        Self {
            hallway: Hallway::new(),
            rooms: [
                Room::new(A, b_a.into(), t_a.into()),
                Room::new(B, b_b.into(), t_b.into()),
                Room::new(C, b_c.into(), t_c.into()),
                Room::new(D, b_d.into(), t_d.into()),
            ],
        }
    }
}

fn main() {
    let input = read_file_lines("input/day23.txt");

    let burrow = Burrow::new(&input);

    println!("{:?}", burrow);
}
