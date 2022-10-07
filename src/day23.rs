use crate::Amphipod::{A, B, C, D};
use advent_of_code_2021::{read_file_lines, vec_to_array};
use sscanf::scanf;

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl std::fmt::Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.to_char();
        write!(f, "{}", c)
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

    pub fn to_char(&self) -> char {
        match self {
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
        }
    }
}

fn option_amphipod_to_char(pod: &Option<Amphipod>) -> char {
    pod.map_or('.', |p| p.to_char())
}

#[derive(Debug, Clone)]
struct Room {
    home_for: Amphipod,
    position: usize,
    room_size: usize,
    spots: Vec<Amphipod>,
}

impl Room {
    pub fn new(home_for: Amphipod, position: usize, initial_data: Vec<Amphipod>) -> Self {
        Self {
            home_for,
            position,
            room_size: initial_data.len(),
            spots: initial_data,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.spots.len() == self.room_size && self.spots.iter().all(|&p| p == self.home_for)
    }

    pub fn is_empty(&self) -> bool {
        self.spots.len() == 0
    }

    pub fn can_accept(&self, amphipod: &Amphipod) -> bool {
        self.spots.len() < self.room_size && self.spots.iter().all(|a| a == amphipod)
    }

    pub fn try_push(&mut self, amphipod: Amphipod) -> Option<u32> {
        if self.home_for == amphipod && self.can_accept(&amphipod) {
            self.spots.push(amphipod);
            Some((self.room_size - self.spots.len() + 1) as u32 * amphipod.get_energy_value())
        } else {
            None
        }
    }

    pub fn top(&self) -> Option<Amphipod> {
        self.spots.last().copied()
    }

    pub fn try_pop(&mut self) -> Option<(u32, Amphipod)> {
        if self.spots.iter().all(|&a| a == self.home_for) {
            return None;
        }
        let popped = self.spots.pop();
        let new_size = self.spots.len();
        popped.map(|a| ((self.room_size - new_size) as u32 * a.get_energy_value(), a))
    }
}

fn distance_between(from: usize, to: usize) -> u32 {
    (if to > from { to - from } else { from - to }) as u32
}

#[derive(Debug, Clone)]
struct Burrow {
    hallway: [Option<Amphipod>; 11],
    rooms: [Room; 4],
}

impl Burrow {
    pub fn new(input: &Vec<String>) -> Self {
        let first_room_row = 2;
        let second_roow_row = first_room_row + 1;
        let num_of_room_rows = input.len() - 3;
        let last_room_row = first_room_row + num_of_room_rows;
        let room_rows_excluding_first = &input[second_roow_row..last_room_row];

        let mut vec_a: Vec<Amphipod> = Vec::with_capacity(num_of_room_rows);
        let mut vec_b: Vec<Amphipod> = Vec::with_capacity(num_of_room_rows);
        let mut vec_c: Vec<Amphipod> = Vec::with_capacity(num_of_room_rows);
        let mut vec_d: Vec<Amphipod> = Vec::with_capacity(num_of_room_rows);

        for r in room_rows_excluding_first.iter().rev() {
            let (a, b, c, d) = scanf!(r, "  #{}#{}#{}#{}#", char, char, char, char).unwrap();
            vec_a.push(a.into());
            vec_b.push(b.into());
            vec_c.push(c.into());
            vec_d.push(d.into());
        }

        let (a, b, c, d) = scanf!(
            input[first_room_row],
            "###{}#{}#{}#{}###",
            char,
            char,
            char,
            char
        )
        .unwrap();

        vec_a.push(a.into());
        vec_b.push(b.into());
        vec_c.push(c.into());
        vec_d.push(d.into());

        Self {
            hallway: [None; 11],
            rooms: [
                Room::new(A, 2, vec_a),
                Room::new(B, 4, vec_b),
                Room::new(C, 6, vec_c),
                Room::new(D, 8, vec_d),
            ],
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("#############");
        let hallway = self
            .hallway
            .iter()
            .map(|s| option_amphipod_to_char(&s))
            .collect::<String>();

        println!("#{hallway}#");

        let map_row_to_chars = |row_idx: usize| -> [char; 4] {
            vec_to_array(
                self.rooms
                    .iter()
                    .map(|r| option_amphipod_to_char(&r.spots.get(row_idx).copied()))
                    .collect(),
            )
        };

        let room_sizes = self.rooms[0].room_size;

        let top_chars = map_row_to_chars(room_sizes - 1);
        println!(
            "###{}#{}#{}#{}###",
            top_chars[0], top_chars[1], top_chars[2], top_chars[3],
        );
        for i in (0..(room_sizes - 1)).rev() {
            let chars = map_row_to_chars(i);
            println!("  #{}#{}#{}#{}#", chars[0], chars[1], chars[2], chars[3],);
        }

        println!("  #########");
        println!();
    }

    pub fn solve(&mut self) -> Option<u32> {
        let mut cost = 0u32;
        loop {
            let mut greedy = self
                .try_move_an_amphipod_in_hallway_to_respective_room()
                .unwrap_or(0);
            greedy += self
                .try_move_an_amphipod_in_a_wrong_room_to_respective_room()
                .unwrap_or(0);

            if greedy == 0 {
                break;
            }

            cost += greedy;
        }

        if self.is_solved() {
            return Some(cost);
        }

        let mut best = u32::MAX;
        for room_idx in 0..4 {
            if self.rooms[room_idx].is_empty() {
                continue;
            }

            for space_idx in 0..11 {
                let mut copy = self.clone();
                if let Some(move_cost) =
                    copy.try_move_an_amphipod_from_a_wrong_room_to_hallway(room_idx, space_idx)
                {
                    let rec_result = copy.solve();
                    if rec_result.is_none() {
                        continue;
                    }
                    best = best.min(move_cost + rec_result.unwrap());
                }
            }
        }

        if best == u32::MAX {
            return None;
        }

        return Some(best + cost);
    }

    fn is_solved(&self) -> bool {
        self.rooms.iter().all(|r| r.is_finished())
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

        self.hallway[i..j].iter().filter(|s| s.is_some()).count() <= 1
    }

    fn is_path_free_between_inclusive(&self, from: usize, to: usize) -> bool {
        let i = from.min(to);
        let j = from.max(to);

        self.hallway[i..=j].iter().all(|s| s.is_none())
    }

    fn try_move_an_amphipod_in_hallway_to_respective_room(&mut self) -> Option<u32> {
        let hallway_len = self.hallway.len();

        for i in 0..hallway_len {
            if let Some(pod) = self.hallway[i] {
                let room_idx = Self::get_room_idx_for_amphipod(&pod);
                if self.is_path_free_between(i, self.rooms[room_idx].position) {
                    if let Some(entry_cost) = self.rooms[room_idx].try_push(pod) {
                        self.hallway[i] = None;
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

    fn try_move_an_amphipod_in_a_wrong_room_to_respective_room(&mut self) -> Option<u32> {
        for start_room_idx in 0usize..4 {
            let starting_pos = self.rooms[start_room_idx].position;

            if let Some(pod) = self.rooms[start_room_idx].top() {
                let target_room_idx = Self::get_room_idx_for_amphipod(&pod);
                let target_pos = self.rooms[target_room_idx].position;
                if start_room_idx != target_room_idx
                    && self.is_path_free_between_inclusive(starting_pos, target_pos)
                {
                    if let Some(entry_cost) = self.rooms[target_room_idx].try_push(pod) {
                        let (exit_cost, _) = self.rooms[start_room_idx].try_pop().unwrap();
                        let move_cost =
                            distance_between(starting_pos, target_pos) * pod.get_energy_value();

                        let total_cost = exit_cost + move_cost + entry_cost;
                        return Some(total_cost);
                    }
                }
            }
        }
        None
    }

    fn try_move_an_amphipod_from_a_wrong_room_to_hallway(
        &mut self,
        room_idx: usize,
        space_idx: usize,
    ) -> Option<u32> {
        let room = &self.rooms[room_idx];
        let starting_pos = room.position;

        if room.is_empty() {
            return None;
        }

        if space_idx == 2 || space_idx == 4 || space_idx == 6 || space_idx == 8 {
            return None;
        }

        if !self.is_path_free_between_inclusive(starting_pos, space_idx) {
            return None;
        }

        let room = &mut self.rooms[room_idx];

        if let Some((exit_cost, pod)) = room.try_pop() {
            self.hallway[space_idx] = Some(pod);
            let move_cost = distance_between(starting_pos, space_idx) * pod.get_energy_value();
            let total_cost = exit_cost + move_cost;
            return Some(total_cost);
        }
        None
    }
}

fn solve_first(input: &Vec<String>) {
    let min_total_energy = Burrow::new(input).solve().unwrap();
    println!(
        "The minimum energy required for ampipods to organize in the original input is {}",
        min_total_energy
    );
}

fn solve_second(input: &Vec<String>) {
    let mut input = input.clone();
    input.insert(3, "  #D#C#B#A#".to_string());
    input.insert(4, "  #D#B#A#C#".to_string());
    let min_total_energy = Burrow::new(&input).solve().unwrap();
    println!(
        "The minimum energy required for ampipods to organize in the extended input is {}",
        min_total_energy
    );
}

fn main() {
    let input = read_file_lines("input/day23.txt");
    solve_first(&input);
    solve_second(&input);
}
