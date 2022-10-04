use advent_of_code_2021::read_file_to_string;
use sscanf::scanf;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn sort(a: i64, b: i64) -> (i64, i64) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

#[derive(Clone)]
struct Step {
    from: (i64, i64, i64),
    to: (i64, i64, i64),
    on: bool,
}

impl Step {
    pub fn get_range_clamped(
        &self,
    ) -> (
        RangeInclusive<i64>,
        RangeInclusive<i64>,
        RangeInclusive<i64>,
    ) {
        let x = Self::clamp_range(self.from.0, self.to.0);
        let y = Self::clamp_range(self.from.1, self.to.1);
        let z = Self::clamp_range(self.from.2, self.to.2);

        (x, y, z)
    }

    fn clamp_range(from: i64, to: i64) -> RangeInclusive<i64> {
        let from = from.max(-50);
        let to = to.min(50);
        from..=to
    }

    pub fn intersect(&self, rhs: &Step) -> Option<Step> {
        let min_x = self.from.0.max(rhs.from.0);
        let max_x = self.to.0.min(rhs.to.0);

        let min_y = self.from.1.max(rhs.from.1);
        let max_y = self.to.1.min(rhs.to.1);

        let min_z = self.from.2.max(rhs.from.2);
        let max_z = self.to.2.min(rhs.to.2);

        if min_x > max_x || min_y > max_y || min_z > max_z {
            return None;
        }

        Some(Step {
            from: (min_x, min_y, min_z),
            to: (max_x, max_y, max_z),
            on: !self.on,
        })
    }

    pub fn calculate_volume(&self) -> i64 {
        let volume =
            (self.to.0 - self.from.0 + 1) *
                (self.to.1 - self.from.1 + 1) *
                (self.to.2 - self.from.2 + 1);

        if self.on {
            volume
        } else {
            -1 * volume
        }
    }
}

impl FromStr for Step {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (state, x1, x2, y1, y2, z1, z2) = scanf!(
            s,
            "{} x={}..{},y={}..{},z={}..{}",
            String,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64
        )
            .ok_or("Invalid format")?;

        let on = if state == "on" { true } else { false };
        let (from_x, to_x) = sort(x1, x2);
        let (from_y, to_y) = sort(y1, y2);
        let (from_z, to_z) = sort(z1, z2);

        Ok(Self {
            on,
            from: (from_x, from_y, from_z),
            to: (to_x, to_y, to_z),
        })
    }
}

struct RebootSequence {
    sequence: Vec<Step>,
}

impl RebootSequence {
    pub fn parse(s: &str) -> Self {
        let sequence = s.lines().map(|l| Step::from_str(l).unwrap()).collect();

        Self { sequence }
    }

    pub fn reboot_initialization(&self) {
        let mut cubes_on: HashSet<(i64, i64, i64)> = HashSet::new();
        for step in &self.sequence {
            let (s_x, s_y, s_z) = step.get_range_clamped();

            for x in s_x.clone() {
                for y in s_y.clone() {
                    for z in s_z.clone() {
                        if step.on {
                            cubes_on.insert((x, y, z));
                        } else {
                            cubes_on.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
        println!("Number of cubes that are on: {}", cubes_on.len());
    }

    pub fn reboot_full(&self) {
        let mut all_steps: Vec<Step> = Vec::new();
        for step in &self.sequence {
            let mut merge = Vec::new();

            if step.on {
                merge.push(step.clone());
            }

            for s in &all_steps {
                if let Some(intersection) = s.intersect(step) {
                    merge.push(intersection);
                }
            }

            all_steps.append(&mut merge);
        }

        let result: i64 = all_steps.iter().map(Step::calculate_volume).sum();

        println!("Number of cubes that are on: {}", result);
    }
}

fn main() {
    let input = read_file_to_string("input/day22.txt");
    let sequence = RebootSequence::parse(&input);

    sequence.reboot_initialization();
    sequence.reboot_full();
}
