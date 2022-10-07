use crate::Amphipod::{A, B, C, D};
use advent_of_code_2021::read_file_lines;
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

#[derive(Debug, Clone)]
struct Room {
    home_for: Amphipod,
    position: usize,
    bottom: Option<Amphipod>,
    top: Option<Amphipod>,
}

impl Room {
    pub fn new(home_for: Amphipod, position: usize, bottom: Amphipod, top: Amphipod) -> Self {
        Self {
            home_for,
            position,
            bottom: Some(bottom),
            top: Some(top),
        }
    }

    pub fn is_finished(&self) -> bool {
        // could've used `is_some_and` but it's unstable
        if let Some(bottom) = self.bottom {
            if let Some(top) = self.top {
                return bottom == self.home_for && top == self.home_for;
            }
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        self.bottom.is_none() && self.top.is_none()
    }

    pub fn can_accept(&self) -> bool {
        self.bottom.is_none() || self.top.is_none()
    }

    pub fn try_push(&mut self, amphipod: Amphipod) -> Option<u32> {
        if self.home_for != amphipod {
            return None;
        }
        if self.bottom.is_none() {
            self.bottom = Some(amphipod);
            return Some(2 * amphipod.get_energy_value());
        }

        if self.top.is_none() {
            if self.bottom.unwrap() != self.home_for {
                return None;
            }
            self.top = Some(amphipod);
            return Some(1 * amphipod.get_energy_value());
        }

        None
    }

    pub fn top(&self) -> Option<Amphipod> {
        if let Some(top) = self.top {
            if self.home_for == top {
                return None;
            }
            return self.top;
        } else if let Some(bottom) = self.bottom {
            if self.home_for == bottom {
                return None;
            }
            return self.bottom;
        } else {
            None
        }
    }

    pub fn try_pop(&mut self) -> Option<(u32, Amphipod)> {
        if let Some(top) = self.top {
            if self.home_for == top {
                return None;
            }
            let energy = 1 * top.get_energy_value();
            self.top = None;
            return Some((energy, top));
        }

        if let Some(bottom) = self.bottom {
            if self.home_for == bottom {
                return None;
            }
            let energy = 2 * bottom.get_energy_value();
            self.bottom = None;
            return Some((energy, bottom));
        }

        None
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
        let top_row = input.get(2).unwrap();
        let bottom_row = input.get(3).unwrap();

        let (t_a, t_b, t_c, t_d) =
            scanf!(top_row, "###{}#{}#{}#{}###", char, char, char, char).unwrap();

        let (b_a, b_b, b_c, b_d) =
            scanf!(bottom_row, "  #{}#{}#{}#{}#", char, char, char, char).unwrap();

        Self {
            hallway: [None; 11],
            rooms: [
                Room::new(A, 2, b_a.into(), t_a.into()),
                Room::new(B, 4, b_b.into(), t_b.into()),
                Room::new(C, 6, b_c.into(), t_c.into()),
                Room::new(D, 8, b_d.into(), t_d.into()),
            ],
        }
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

        self.hallway[i..j].iter().all(|s| s.is_none())
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
        for i in 0usize..4 {
            let starting_pos = self.rooms[i].position;

            if let Some(pod) = self.rooms[i].top() {
                let target_room_idx = Self::get_room_idx_for_amphipod(&pod);
                let target_pos = self.rooms[target_room_idx].position;
                if self.is_path_free_between_inclusive(starting_pos, target_pos)
                    && self.rooms[target_room_idx].can_accept()
                {
                    if let Some(entry_cost) = self.rooms[target_room_idx].try_push(pod) {
                        let (exit_cost, _) = self.rooms[i].try_pop().unwrap();
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

        if space_idx == starting_pos {
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

fn main() {
    let input = read_file_lines("input/day23.txt");

    let mut burrow = Burrow::new(&input);

    println!("{:?}", burrow);

    let result = burrow.solve();
    println!("{:?}", result);
}
