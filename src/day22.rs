use advent_of_code_2021::read_file_to_string;
use sscanf::scanf;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn sort(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

struct Step {
    from: (i32, i32, i32),
    to: (i32, i32, i32),
    on: bool,
}

impl Step {
    pub fn get_range(
        &self,
    ) -> (
        RangeInclusive<i32>,
        RangeInclusive<i32>,
        RangeInclusive<i32>,
    ) {
        let x = self.from.0..=self.to.0;
        let y = self.from.1..=self.to.1;
        let z = self.from.2..=self.to.2;

        (x, y, z)
    }

    pub fn get_range_clamped(
        &self,
    ) -> (
        RangeInclusive<i32>,
        RangeInclusive<i32>,
        RangeInclusive<i32>,
    ) {
        let x = Self::clamp_range(self.from.0, self.to.0);
        let y = Self::clamp_range(self.from.1, self.to.1);
        let z = Self::clamp_range(self.from.2, self.to.2);

        (x, y, z)
    }

    fn clamp_range(from: i32, to: i32) -> RangeInclusive<i32> {
        let from = from.max(-50);
        let to = to.min(50);
        from..=to
    }
}

impl FromStr for Step {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (state, x1, x2, y1, y2, z1, z2) = scanf!(
            s,
            "{} x={}..{},y={}..{},z={}..{}",
            String,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32
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

    pub fn reboot(&self) {
        let mut cubes_on: HashSet<(i32, i32, i32)> = HashSet::new();
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
            println!("Number of cubes that are on: {}", cubes_on.len());
        }
    }
}

fn main() {
    let input = read_file_to_string("input/day22.txt");
    let sequence = RebootSequence::parse(&input);

    sequence.reboot();
}
