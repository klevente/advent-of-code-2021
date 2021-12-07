use advent_of_code_2021::read_file_to_string;

fn sum_of_first_n_integers(n: u32) -> u32 {
    // (1..=n).sum()
    // iteration version takes too long, so use closed-form version instead
    n * (n + 1) / 2
}

fn distance(x: u32, y: u32) -> u32 {
    (x as i32 - y as i32).abs() as u32
}

fn solve_using_formula(
    positions: &Vec<u32>,
    formula: fn(pos: u32, acc: u32, next: u32) -> u32,
) -> u32 {
    let max_pos = *positions.iter().max().unwrap();

    (0..=max_pos).fold(u32::MAX, |min_fuel_all_crabs, pos| {
        let fuel_all_crabs = positions
            .iter()
            .fold(0, |acc, next| formula(pos, acc, *next));

        min_fuel_all_crabs.min(fuel_all_crabs)
    })
}

fn solve_using_linear_consumption(positions: &Vec<u32>) {
    let min_fuel = solve_using_formula(&positions, |pos, acc, next| {
        let crab_fuel = distance(pos, next);
        acc + crab_fuel
    });

    println!(
        "The minimum required fuel assuming linear consumption is {}",
        min_fuel
    );
}

fn solve_using_increasing_consumption(positions: &Vec<u32>) {
    let min_fuel = solve_using_formula(&positions, |pos, acc, next| {
        let distance = distance(pos, next);
        let crab_fuel = sum_of_first_n_integers(distance);
        acc + crab_fuel
    });

    println!(
        "The minimum required fuel assuming increasing consumption is {}",
        min_fuel
    );
}

fn parse_file(path: &str) -> Vec<u32> {
    read_file_to_string(path)
        .split(',')
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn main() {
    let positions = parse_file("input/day7.txt");
    solve_using_linear_consumption(&positions);
    solve_using_increasing_consumption(&positions);
}
