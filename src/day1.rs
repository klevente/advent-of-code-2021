use advent_of_code_2021::read_file_lines_as;
use itertools::Itertools;

fn main() {
    // let measurements = read_file("input/day1.txt");
    let measurements = read_file_lines_as("input/day1.txt", |l| l.parse::<u32>().unwrap());

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

fn find_num_of_increases(measurements: &Vec<u32>) -> u32 {
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

fn find_num_of_increases_sliding_window(measurements: &Vec<u32>) -> u32 {
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

    fn sum_windows((e1, e2, e3): (&u32, &u32, &u32)) -> u32 {
        e1 + e2 + e3
    }

    let w1 = measurements[0..measurements.len() - 1]
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(sum_windows);
    let w2 = measurements
        .iter()
        .skip(1)
        .tuple_windows::<(_, _, _)>()
        .map(sum_windows);

    w1.zip(w2).fold(
        0,
        |acc, (m_old, m_new)| if m_new > m_old { acc + 1 } else { acc },
    )
}
