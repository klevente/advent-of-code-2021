use advent_of_code_2021::read_file_to_string;
use sscanf::scanf;
use std::ops::RangeInclusive;

fn sum_of_first_n_integers(n: i32) -> i32 {
    n * (n + 1) / 2
}

struct TargetArea {
    range_x: RangeInclusive<i32>,
    range_y: RangeInclusive<i32>,
}

impl TargetArea {
    pub fn parse(s: &str) -> Self {
        let (start_x, end_x, start_y, end_y) =
            scanf!(s, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32).unwrap();

        Self {
            range_x: start_x..=end_x,
            range_y: start_y..=end_y,
        }
    }

    pub fn contains_probe(&self, probe: &Probe) -> bool {
        self.contains_point(&probe.x, &probe.y)
    }

    pub fn has_probe_overshot(&self, probe: &Probe) -> bool {
        let p_x = &probe.x;
        let p_y = &probe.y;
        let max_x = self.range_x.end();
        let min_y = self.range_y.start();

        p_x > max_x || p_y < min_y
    }

    fn contains_point(&self, x: &i32, y: &i32) -> bool {
        self.range_x.contains(x) && self.range_y.contains(y)
    }
}

#[derive(Debug)]
struct Probe {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Probe {
    pub fn new(initial_vx: i32, initial_vy: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            vx: initial_vx,
            vy: initial_vy,
        }
    }

    pub fn step(&mut self) -> (i32, i32) {
        self.x += self.vx;
        self.y += self.vy;

        self.vx += if self.vx > 0 {
            -1
        } else if self.vx < 0 {
            1
        } else {
            0
        };
        self.vy -= 1;

        (self.x, self.y)
    }
}

struct Trench {
    target_area: TargetArea,
}

impl Trench {
    pub fn new(target_area: TargetArea) -> Self {
        Self { target_area }
    }

    pub fn calculate_maximum_y_for_all_throws(&self) -> i32 {
        self.run_simulation()
            .iter()
            .filter_map(|&r| r.map(|(v, _, _)| v))
            .max()
            .unwrap()
    }

    pub fn count_number_of_valid_throws(&self) -> usize {
        self.run_simulation()
            .iter()
            .filter_map(|&r| r)
            .inspect(|(_, vx, vy)| println!("{},{}", vx, vy))
            .count()
    }

    fn run_simulation(&self) -> Vec<Option<(i32, i32, i32)>> {
        let mut results = Vec::new();
        for probe in self.generate_probes_with_plausible_starting_velocities() {
            let vx = probe.vx;
            let vy = probe.vy;
            println!("Throwing probe: ({},{})", &probe.vx, &probe.vy);
            let result = self.throw_probe(probe);
            println!("Result of throw: {:?}", &result);
            results.push((result.map(|r| (r, vx, vy))));
        }

        results
    }

    fn generate_probes_with_plausible_starting_velocities(&self) -> Vec<Probe> {
        let mut probes = Vec::new();

        for vx in 0..=*self.target_area.range_x.end() {
            if sum_of_first_n_integers(vx) < *self.target_area.range_x.start() {
                continue;
            }
            for vy in *self.target_area.range_y.start()..512 {
                probes.push(Probe::new(vx, vy));
            }
        }

        probes
    }

    fn throw_probe(&self, mut probe: Probe) -> Option<i32> {
        let mut max_y = i32::MIN;
        loop {
            dbg!(&probe);
            if self.target_area.contains_probe(&probe) {
                return Some(max_y);
            }
            if self.target_area.has_probe_overshot(&probe) {
                return None;
            }
            let (_x, y) = probe.step();
            max_y = max_y.max(y);
        }
    }
}

fn main() {
    let input = read_file_to_string("input/day17.txt");

    let target_area = TargetArea::parse(&input);
    let trench = Trench::new(target_area);

    let maximum_y = trench.calculate_maximum_y_for_all_throws();
    println!("The maximum y value of all throws is {}", maximum_y);

    let num_of_valid_throws = trench.count_number_of_valid_throws();
    println!("The number of valid throws is {}", num_of_valid_throws);
}
