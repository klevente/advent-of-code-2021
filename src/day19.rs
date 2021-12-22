use advent_of_code_2021::read_file_to_string;
use itertools::Itertools;
use phf::phf_map;
use sscanf::scanf;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

const ORIENTATIONS: phf::Map<u8, fn(&Vec3) -> Vec3> = phf_map! {
    0u8  => |v| Vec3::new( v.x(),  v.y(),  v.z()),
    1u8  => |v| Vec3::new( v.x(), -v.z(),  v.y()),
    2u8  => |v| Vec3::new( v.x(), -v.y(), -v.z()),
    3u8  => |v| Vec3::new( v.x(),  v.z(), -v.y()),
    4u8  => |v| Vec3::new(-v.y(),  v.x(),  v.z()),
    5u8  => |v| Vec3::new( v.z(),  v.x(),  v.y()),
    6u8  => |v| Vec3::new( v.y(),  v.x(), -v.z()),
    7u8  => |v| Vec3::new(-v.z(),  v.x(), -v.y()),
    8u8  => |v| Vec3::new(-v.x(), -v.y(),  v.z()),
    9u8  => |v| Vec3::new(-v.x(), -v.z(), -v.y()),
    10u8 => |v| Vec3::new(-v.x(),  v.y(), -v.z()),
    11u8 => |v| Vec3::new(-v.x(),  v.z(),  v.y()),
    12u8 => |v| Vec3::new( v.y(), -v.x(),  v.z()),
    13u8 => |v| Vec3::new( v.z(), -v.x(), -v.y()),
    14u8 => |v| Vec3::new(-v.y(), -v.x(), -v.z()),
    15u8 => |v| Vec3::new(-v.z(), -v.x(),  v.y()),
    16u8 => |v| Vec3::new(-v.z(),  v.y(),  v.x()),
    17u8 => |v| Vec3::new( v.y(),  v.z(),  v.x()),
    18u8 => |v| Vec3::new( v.z(), -v.y(),  v.x()),
    19u8 => |v| Vec3::new(-v.y(), -v.z(),  v.x()),
    20u8 => |v| Vec3::new(-v.z(), -v.y(), -v.x()),
    21u8 => |v| Vec3::new(-v.y(),  v.z(), -v.x()),
    22u8 => |v| Vec3::new( v.z(),  v.y(), -v.x()),
    23u8 => |v| Vec3::new( v.y(), -v.z(), -v.x()),
};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Vec3 {
    data: [i32; 3],
}

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0)
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

    pub fn rotate(&self, orientation_id: u8) -> Vec3 {
        ORIENTATIONS.get(&orientation_id).unwrap()(self)
    }

    pub fn manhattan_distance(&self, rhs: &Vec3) -> i32 {
        (self.x() - rhs.x()).abs() + (self.y() - rhs.y()).abs() + (self.z() - rhs.z()).abs()
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

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x(), self.y(), self.z())
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    id: usize,
    beacons: HashSet<Vec3>,
}

impl Scanner {
    pub fn parse(s: &str) -> Self {
        let (head, tail) = s.split_once("\r\n").unwrap();
        let id = scanf!(head, "--- scanner {} ---", usize).unwrap();
        let beacons = tail.lines().map(|l| Vec3::from_str(l).unwrap()).collect();

        Self { id, beacons }
    }

    pub fn find_overlapping_points_with(
        &self,
        known_beacons: &HashSet<Vec3>,
    ) -> Option<(HashSet<Vec3>, Vec3)> {
        for known_beacon in known_beacons {
            for orientation_id in 0..24 {
                let rotated_beacons = self
                    .beacons
                    .iter()
                    .map(|v| v.rotate(orientation_id))
                    .collect::<Vec<_>>();

                for rotated_beacon in rotated_beacons.iter() {
                    let possible_translation = known_beacon - &rotated_beacon;

                    let translated_beacons = rotated_beacons
                        .iter()
                        .map(|v| v + &possible_translation)
                        .collect::<HashSet<_>>();

                    let num_of_matching_beacons =
                        known_beacons.intersection(&translated_beacons).count();

                    if num_of_matching_beacons >= 12 {
                        return Some((translated_beacons, possible_translation));
                    }
                }
            }
        }
        None
    }
}

fn discover_all_beacons(scanners: Vec<Scanner>) -> (HashSet<Vec3>, HashSet<Vec3>) {
    let mut remaining = VecDeque::from(scanners);
    let scanner_0 = remaining.pop_front().unwrap();
    let mut found_beacons: HashSet<Vec3> = HashSet::from_iter(scanner_0.beacons.into_iter());
    let mut beacon_positions: HashSet<Vec3> = HashSet::new();
    beacon_positions.insert(Vec3::zero());

    while !remaining.is_empty() {
        println!("{} unhandled scanners remain", remaining.len());
        let candidate = remaining.pop_front().unwrap();
        if let Some((new_beacons, scanner_position)) =
            candidate.find_overlapping_points_with(&found_beacons)
        {
            println!("Found overlap with scanner {}", &candidate.id);
            found_beacons.extend(new_beacons.iter());
            beacon_positions.insert(scanner_position);
        } else {
            remaining.push_back(candidate);
        }
    }

    (found_beacons, beacon_positions)
}

fn map_ocean_trench(scanners: Vec<Scanner>) {
    let (all_beacons, scanner_positions) = discover_all_beacons(scanners);
    let num_of_all_beacons = all_beacons.len();
    println!(
        "The number of all beacons in the trench is {}",
        num_of_all_beacons
    );

    let max_distance_between_scanners = scanner_positions
        .iter()
        .cartesian_product(scanner_positions.iter())
        .filter_map(|(p1, p2)| {
            if p1 != p2 {
                Some(p1.manhattan_distance(p2))
            } else {
                None
            }
        })
        .max()
        .unwrap();

    println!(
        "The maximum Manhattan distance between any 2 scanners is {}",
        max_distance_between_scanners
    );
}

fn main() {
    let input = read_file_to_string("input/day19.txt");
    let scanners = input
        .split("\r\n\r\n")
        .map(Scanner::parse)
        .collect::<Vec<_>>();

    map_ocean_trench(scanners.clone());
}
