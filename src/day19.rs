use advent_of_code_2021::{read_file_lines, read_file_to_string};
use itertools::Itertools;
use sscanf::scanf;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum TransformStep {
    //       swap   swap   negate
    Rotate90(usize, usize, usize),
    //        negate negate
    Rotate180(usize, usize),
    Translate(Vec3),
}

type Transform = Vec<TransformStep>;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Vec3 {
    data: [i32; 3],
}

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn x(&self) -> i32 {
        self.data[0]
    }

    pub fn y(&self) -> i32 {
        self.data[1]
    }

    pub fn z(&self) -> i32 {
        self.data[2]
    }

    pub fn x_mut(&mut self) -> &mut i32 {
        self.data.get_mut(0).unwrap()
    }

    pub fn y_mut(&mut self) -> &mut i32 {
        self.data.get_mut(1).unwrap()
    }

    pub fn z_mut(&mut self) -> &mut i32 {
        self.data.get_mut(2).unwrap()
    }

    pub fn calculate_rotation_transform(&self, from: &Vec3) -> Transform {
        let mut transform = Vec::new();
        let self_abs = self.abs();
        let mut working_value = from.clone();

        // if x coordinates do not match, swap them so they do
        if self_abs.x() != working_value.abs().x() {
            let swap_this = 0;
            let to_that = if self_abs.x() == working_value.abs().y() {
                1
            } else {
                2
            };

            let negate = if self.get(0) == working_value.get(to_that) {
                to_that
            } else {
                0
            };

            let swap = TransformStep::Rotate90(swap_this, to_that, negate);
            working_value.apply_transform_step(&swap);
            transform.push(swap);
        }

        // if y coordinates do not match, swap them so they do
        if self_abs.y() != working_value.abs().y() {
            let swap_this = 1;
            let to_that = 2;

            let negate = if self.get(1) == working_value.get(to_that) {
                to_that
            } else {
                1
            };

            let swap = TransformStep::Rotate90(swap_this, to_that, negate);
            working_value.apply_transform_step(&swap);
            transform.push(swap);
        }

        // now all coordinates are in good positions, but 2 might need negating
        let mut values_to_negate = Vec::new();
        if working_value.x() != self.x() {
            values_to_negate.push(0);
        }
        if working_value.y() != self.y() {
            values_to_negate.push(1);
        }
        if working_value.z() != self.z() {
            values_to_negate.push(2);
        }

        if !values_to_negate.is_empty() {
            if values_to_negate.len() != 2 {
                panic!(
                    "Values to negate is not 2 length, it is instead {}",
                    values_to_negate.len()
                );
            }
            let negate = TransformStep::Rotate180(
                *values_to_negate.first().unwrap(),
                *values_to_negate.last().unwrap(),
            );
            working_value.apply_transform_step(&negate);
            transform.push(negate);
        }

        transform
    }

    pub fn apply_transform_step(&mut self, step: &TransformStep) {
        match step {
            TransformStep::Rotate90(swap_this, to_that, negate) => {
                self.data.swap(*swap_this, *to_that);
                *self.data.get_mut(*negate).unwrap() *= -1;
            }
            TransformStep::Rotate180(negate_1, negate_2) => {
                *self.data.get_mut(*negate_1).unwrap() *= -1;
                *self.data.get_mut(*negate_2).unwrap() *= -1;
            }
            TransformStep::Translate(v) => {
                *self += v;
            }
        }
    }

    pub fn transform_applied(&self, step: &TransformStep) -> Self {
        let mut result = self.clone();
        result.apply_transform_step(step);

        result
    }

    pub fn transform(&self, transform: &Transform) -> Self {
        transform
            .iter()
            .fold(self.clone(), |acc, next| acc.transform_applied(next))
    }

    pub fn abs(&self) -> Self {
        Self::new(self.x().abs(), self.y().abs(), self.z().abs())
    }

    pub fn are_all_elements_different_abs(&self) -> bool {
        let abs = self.abs();
        abs.x() != abs.y() && abs.y() != abs.z() && abs.x() != abs.z()
    }

    pub fn contains_same_elements_abs_as(&self, rhs: &Vec3) -> bool {
        let lhs_abs = self.abs();
        let rhs_abs = rhs.abs();

        let lhs_set: HashSet<i32> = HashSet::from_iter(lhs_abs.data.iter().cloned());
        let rhs_set: HashSet<i32> = HashSet::from_iter(rhs_abs.data.iter().cloned());

        lhs_set == rhs_set
    }

    fn get(&self, i: usize) -> i32 {
        self.data[i]
    }
}

impl FromStr for Vec3 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = scanf!(s, "{},{},{}", i32, i32, i32).ok_or("Invalid format".to_string())?;

        Ok(Self { data: [x, y, z] })
    }
}

impl<'a, 'b> std::ops::Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &'b Vec3) -> Vec3 {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl<'a, 'b> std::ops::AddAssign<&'b Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &'b Vec3) {
        *self.x_mut() += rhs.x();
        *self.y_mut() += rhs.y();
        *self.z_mut() += rhs.z();
    }
}

impl<'a, 'b> std::ops::Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &'b Vec3) -> Vec3 {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

#[derive(Debug, Copy, Clone)]
struct Vec3WithEnds {
    v: Vec3,
    start: Vec3,
    end: Vec3,
}

impl Vec3WithEnds {
    pub fn new(v: Vec3, start: Vec3, end: Vec3) -> Self {
        Self { v, start, end }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Scanner {
    id: usize,
    beacons: Vec<Vec3>,
}

impl Scanner {
    pub fn parse(s: &str) -> Self {
        let (head, tail) = s.split_once("\r\n").unwrap();
        let id = scanf!(head, "--- scanner {} ---", usize).unwrap();
        let beacons = tail
            .lines()
            .map(|l| Vec3::from_str(l).unwrap())
            .collect::<Vec<_>>();

        Self { id, beacons }
    }

    fn create_vectors(&self, starting_from: usize) -> Vec<Vec3WithEnds> {
        let starting_point = self.beacons.get(starting_from).unwrap();

        self.beacons
            .iter()
            .filter_map(|p| {
                if p != starting_point {
                    let vector = p - starting_point;
                    Some(Vec3WithEnds::new(vector, *starting_point, *p))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_overlapping_area(
        &self,
        other: &Scanner,
    ) -> Option<Vec<(Vec3WithEnds, Vec3WithEnds)>> {
        for i in 0..self.beacons.len() {
            let self_vectors = self.create_vectors(i);

            for j in 0..other.beacons.len() {
                let other_vectors = other.create_vectors(j);
                let product = self_vectors
                    .clone()
                    .into_iter()
                    .cartesian_product(other_vectors.into_iter());

                let result = product
                    .filter(|(v1, v2)| v1.v.contains_same_elements_abs_as(&v2.v))
                    .collect::<Vec<_>>();

                if result.len() >= 11 {
                    return Some(result);
                }
            }
        }

        None
    }

    pub fn apply_transform_to_beacons(&self, transform: &Transform) -> Vec<Vec3> {
        self.beacons
            .iter()
            .map(|b| b.transform(transform))
            .collect()
    }
}

fn find_transform(pairs: &Vec<(Vec3WithEnds, Vec3WithEnds)>) -> Transform {
    let (to, from) = pairs
        .iter()
        .find_map(|(v1, v2)| {
            if v1.v.are_all_elements_different_abs() && v2.v.are_all_elements_different_abs() {
                Some((v1.v, v2.v))
            } else {
                None
            }
        })
        .unwrap();

    let mut transform = to.calculate_rotation_transform(&from);
    let (a, b) = pairs.first().map(|(x, y)| (x.start, y.start)).unwrap();

    let b_transformed = b.transform(&transform);

    let translate_by = &a - &b_transformed;
    println!("REQUIRED TRANSLATION: {:?}", &translate_by);
    transform.push(TransformStep::Translate(translate_by));

    transform
}

fn calculate_transform_between_scanners(to: &Scanner, from: &Scanner) -> Option<Transform> {
    to.get_overlapping_area(from)
        .map(|area| find_transform(&area))
}

fn discover_all_beacons(scanners: &Vec<Scanner>) {
    let mut beacons: HashSet<Vec3> = HashSet::new();
    let mut transforms: HashMap<&Scanner, Transform> = HashMap::new();
    let mut done = Vec::new();
    done.push(scanners.first().unwrap().id);
    let mut check_index = 0;
    while done.len() < scanners.len() {
        let current_id = done.get(check_index).unwrap();
        let current = scanners.iter().find(|s| s.id == *current_id).unwrap();
        for scanner in scanners {
            if done.contains(&scanner.id) {
                continue;
            }
            let transform = calculate_transform_between_scanners(current, scanner);
            if let Some(transform) = transform {
                let current_to_0 = transforms.get(current).unwrap_or(&Vec::new()).clone();
                let scanner_transform = transforms.entry(scanner).or_insert(Vec::new());
                scanner_transform.extend(current_to_0.iter());
                scanner_transform.extend(transform.iter());
                done.push(scanner.id);
            }
        }
        check_index += 1;
    }

    let empty_transform = Vec::new();
    for scanner_id in done {
        let scanner = scanners.iter().find(|s| s.id == scanner_id).unwrap();
        let transform = transforms.get(scanner).unwrap_or(&empty_transform);
        let scanner_beacons = scanner.apply_transform_to_beacons(transform);
        for b in scanner_beacons.into_iter() {
            beacons.insert(b);
        }
    }

    let num_of_beacons = beacons.len();

    println!("The number of beacons is {}", num_of_beacons);
}

fn main() {
    let input = read_file_to_string("input/day19.txt");
    let scanners = input
        .split("\r\n\r\n")
        .map(Scanner::parse)
        .collect::<Vec<_>>();

    discover_all_beacons(&scanners);
}
