use itertools::Itertools;
use std::io::Read;
use std::{fs::File, path::Path};

fn main() {
    let measurements = read_file("input/day1.txt");

    let num_of_increases = find_num_of_increases(&measurements);
    println!(
        "{} measurements are larger than the previous measurement.",
        num_of_increases
    );
    let num_of_increases_sliding_window = find_num_of_increases_sliding_window(&measurements);
    println!(
        "{} measurement windows are larger than the previous measurement window.",
        num_of_increases_sliding_window
    );
}

fn find_num_of_increases(measurements: &Vec<i32>) -> i32 {
    /*let mut num_of_increases = 0;
    for (i, m_new) in measurements.iter().enumerate().skip(1) {
        let m_old = &measurements[i - 1];
        if m_new > m_old {
            num_of_increases += 1;
        }
    }
    num_of_increases*/

    measurements.iter().tuple_windows::<(_, _)>().fold(
        0,
        |acc, (m_old, m_new)| {
            if m_new > m_old {
                acc + 1
            } else {
                acc
            }
        },
    )
}

fn find_num_of_increases_sliding_window(measurements: &Vec<i32>) -> i32 {
    /*let mut num_of_increases = 0;
    for (i, _) in measurements.iter().enumerate().skip(3) {
        let w_old = measurements[i - 3] + measurements[i - 2] + measurements[i - 1];
        let w_new = measurements[i - 2] + measurements[i - 1] + measurements[i];
        if w_new > w_old {
            num_of_increases += 1;
        }
    }
    num_of_increases*/

    // let without_last = &measurements[0..measurements.len() - 1];

    let w1 = measurements[0..measurements.len() - 1]
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|(e1, e2, e3)| e1 + e2 + e3);
    let w2 = measurements
        .iter()
        .skip(1)
        .tuple_windows::<(_, _, _)>()
        .map(|(e1, e2, e3)| e1 + e2 + e3);

    w1.zip(w2).fold(
        0,
        |acc, (m_old, m_new)| if m_new > m_old { acc + 1 } else { acc },
    )
}

fn read_file(path: impl AsRef<Path>) -> Vec<i32> {
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut v: Vec<i32> = Vec::new();
    for line in contents.lines() {
        v.push(line.parse::<i32>().unwrap());
    }
    v
}
