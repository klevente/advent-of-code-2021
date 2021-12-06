use advent_of_code_2021::read_file_to_string;
use std::time::{Duration, Instant};

const NUM_OF_STATES: usize = 8;
const TIMER_RESET_VALUE: usize = 6;

#[derive(Clone)]
struct LanternFish {
    timer: u8,
}

impl LanternFish {
    pub fn parse(s: &str) -> Self {
        let timer = s.parse().unwrap();
        Self { timer }
    }

    pub fn tick(&mut self) -> Option<LanternFish> {
        if self.timer == 0 {
            self.timer = TIMER_RESET_VALUE as u8;
            Some(Self::spawn_new())
        } else {
            self.timer -= 1;
            None
        }
    }

    fn spawn_new() -> LanternFish {
        Self { timer: 8 }
    }
}

fn parse_fish(raw: &str) -> Vec<LanternFish> {
    raw.split(',').map(LanternFish::parse).collect()
}

fn simulate_one_day_slow(fish: &mut Vec<LanternFish>) {
    let mut new_fish = Vec::new();

    for f in &mut *fish {
        if let Some(new_f) = f.tick() {
            new_fish.push(new_f);
        }
    }

    fish.append(&mut new_fish);
}

fn simulate_slow(mut fish: Vec<LanternFish>, num_of_days: u32) -> u64 {
    let start = Instant::now();
    for day in 0..num_of_days {
        println!("Day {}", day);
        simulate_one_day_slow(&mut fish);
        println!("End of day {}, fish: {}", day, fish.len());
    }
    let duration = start.elapsed();

    println!("Simulation took: {:?}", duration);

    fish.len() as u64
}

#[derive(Debug)]
struct FishSchool {
    num_of_fish_by_timer: Vec<u64>,
}

impl FishSchool {
    pub fn parse(s: &str) -> Self {
        let mut num_of_fish_by_timer = vec![0; NUM_OF_STATES + 1];

        s.split(',')
            .map(|d| d.parse::<usize>().unwrap())
            .for_each(|i| num_of_fish_by_timer[i] += 1);

        Self {
            num_of_fish_by_timer,
        }
    }

    pub fn simulate(&mut self, num_of_days: u64) -> u64 {
        let start = Instant::now();
        for day in 0..num_of_days {
            println!("Day {}", day);
            self.tick();
            println!("End of day {}, fish: {}", day, self.num_of_all_fish());
        }
        let duration = start.elapsed();
        println!("Simulation took: {:?}", duration);
        self.num_of_all_fish()
    }

    fn tick(&mut self) {
        let num_to_spawn = self.num_of_fish_by_timer[0];
        self.num_of_fish_by_timer.rotate_left(1);
        self.num_of_fish_by_timer[TIMER_RESET_VALUE] += num_to_spawn;
    }

    fn num_of_all_fish(&self) -> u64 {
        self.num_of_fish_by_timer.iter().sum()
    }
}

fn slow_way(raw: &str) {
    let fish = parse_fish(&raw);
    let num_of_fish_after_80_days = simulate_slow(fish.clone(), 80);
    println!(
        "The number of lanternfish after 80 days is {}",
        num_of_fish_after_80_days
    );

    /*let num_of_fish_after_256_days = simulate_slow(fish, 256);
    println!(
        "The number of lanternfish after 256 days is {}",
        num_of_fish_after_256_days
    );*/
}

fn fast_way(raw: &str) {
    let mut school1 = FishSchool::parse(&raw);
    let num_of_fish_after_80_days = school1.simulate(80);
    println!(
        "The number of lanternfish after 80 days is {}",
        num_of_fish_after_80_days
    );
    let mut school2 = FishSchool::parse(&raw);
    let num_of_fish_after_256_days = school2.simulate(256);
    println!(
        "The number of lanternfish after 256 days is {}",
        num_of_fish_after_256_days
    );
}

fn main() {
    let raw = read_file_to_string("input/day6.txt");
    slow_way(&raw);
    fast_way(&raw);
}
