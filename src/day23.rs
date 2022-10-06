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

impl Amphipod {
    pub fn get_energy_value(&self) -> u32 {
        match self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }
}

#[derive(Debug)]
struct Room {
    position: usize,
    bottom: Option<Amphipod>,
    top: Option<Amphipod>,
}

impl Room {
    pub fn new(position: usize, bottom: Amphipod, top: Amphipod) -> Self {
        Self {
            position,
            bottom: Some(bottom),
            top: Some(top),
        }
    }

    pub fn is_full(&self) -> bool {
        self.bottom.is_some() && self.top.is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.bottom.is_none() && self.top.is_none()
    }

    pub fn try_push(&mut self, amphipod: Amphipod) -> Option<u32> {
        if self.bottom.is_none() {
            self.bottom = Some(amphipod);
            return Some(2 * amphipod.get_energy_value());
        }

        if self.top.is_none() {
            self.top = Some(amphipod);
            return Some(1 * amphipod.get_energy_value());
        }

        None
    }

    pub fn try_pop(&mut self) -> Option<(u32, Amphipod)> {
        if let Some(top) = self.top {
            let energy = 1 * top.get_energy_value();
            return Some((energy, top));
        }

        if let Some(bottom) = self.bottom {
            let energy = 2 * bottom.get_energy_value();
            return Some((energy, bottom));
        }

        None
    }
}

fn distance_between(from: usize, to: usize) -> u32 {
    (if to > from { to - from } else { from - to }) as u32
}

#[derive(Debug)]
struct Burrow {
    hallway: [Option<Amphipod>; 11],
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
            hallway: [None; 11],
            rooms: [
                Room::new(2, b_a.into(), t_a.into()),
                Room::new(4, b_b.into(), t_b.into()),
                Room::new(6, b_c.into(), t_c.into()),
                Room::new(8, b_d.into(), t_d.into()),
            ],
        }
    }

    pub fn solve(&mut self) -> u32 {
        1
    }

    fn get_room_idx_for_amphipod(amphipod: &Amphipod) -> usize {
        match amphipod {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }

    fn is_path_free_between(&self, from: usize, to: usize) -> bool {
        let i = from.min(to);
        let j = from.max(to);

        self.hallway[i..j].iter().all(|s| s.is_none())
    }

    fn try_move_an_amphipod_in_hallway_to_respective_room(&mut self) -> Option<u32> {
        for (i, space) in self.hallway.iter().enumerate() {
            if let Some(pod) = space {
                let room_idx = Self::get_room_idx_for_amphipod(pod);
                if self.is_path_free_between(i, self.rooms[room_idx].position) {
                    if let Some(entry_cost) = self.rooms[room_idx].try_push(*pod) {
                        let move_cost = distance_between(i, self.rooms[room_idx].position)
                            * pod.get_energy_value();
                        let total_cost = move_cost + entry_cost;
                        return Some(total_cost);
                    }
                }
            }
        }
        None
    }

    fn try_move_an_amphipod_in_wrong_rooms_to_respective_room(&mut self) -> Option<u32> {
        for i in 0usize..4 {
            let starting_pos = self.rooms[i].position;
            if let Some((exit_cost, pod)) = self.rooms[i].try_pop() {
                let target_pos = self.rooms[Self::get_room_idx_for_amphipod(&pod)].position;
                if self.is_path_free_between(starting_pos, target_pos) {
                    let target_room = &mut self.rooms[Self::get_room_idx_for_amphipod(&pod)];
                    if let Some(entry_cost) = target_room.try_push(pod) {
                        let move_cost = distance_between(starting_pos, target_room.position)
                            * pod.get_energy_value();
                        let total_cost = exit_cost + move_cost + entry_cost;
                        return Some(total_cost);
                    }
                }
            }
        }
        None
    }
}

fn main() {
    let input = read_file_lines("input/day23.txt");

    let mut burrow = Burrow::new(&input);

    println!("{:?}", burrow);

    let result = burrow.solve();
    println!("{result}");
}
