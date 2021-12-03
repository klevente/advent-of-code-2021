use advent_of_code_2021::read_file_lines_as;

fn calculate_most_common_bits_in_all_indices(reports: &Vec<Vec<u8>>) -> Vec<u8> {
    let report_len = reports.first().unwrap().len();
    let mut num_of_ones = vec![0u32; report_len];

    for report in reports {
        for (i, digit) in report.iter().enumerate() {
            num_of_ones[i] += *digit as u32;
        }
    }

    let half = reports.len() / 2;
    let bits = num_of_ones
        .iter()
        .map(|n| most_common_bit(*n, half))
        .collect::<Vec<u8>>();

    bits
}

fn most_common_bit(n_ones: u32, half: usize) -> u8 {
    if n_ones == half as u32 {
        1 // if both digits are equally common, return 1
    } else if n_ones > half as u32 {
        1u8
    } else {
        0u8
    }
}

fn least_common_bit(n_ones: u32, half: usize) -> u8 {
    invert(most_common_bit(n_ones, half))
}

fn bits_to_u32(bits: &Vec<u8>) -> u32 {
    bits.iter()
        .fold(0u32, |acc, next| (acc << 1) | *next as u32)
}

fn invert(bit: u8) -> u8 {
    // if bit == 1u8 { 0u8 } else { 1u8 }
    !bit & 1u8
}

fn invert_bits(bits: &Vec<u8>) -> Vec<u8> {
    bits.iter().map(|bit| invert(*bit)).collect()
}

fn power_consumption(gamma: u32, epsilon: u32) -> u32 {
    gamma * epsilon
}

fn calculate_power_consumption(reports: &Vec<Vec<u8>>) -> u32 {
    let most_common_bits = calculate_most_common_bits_in_all_indices(&reports);
    let least_common_bits = invert_bits(&most_common_bits);

    let gamma = bits_to_u32(&most_common_bits);
    let epsilon = bits_to_u32(&least_common_bits);

    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
    power_consumption(gamma, epsilon)
}

fn get_num_of_ones_at_index(values: &Vec<Vec<u8>>, i: usize) -> u32 {
    values.iter().fold(0u32, |acc, next| acc + next[i] as u32)
}

fn calculate_most_common_bit_at_index(values: &Vec<Vec<u8>>, i: usize) -> u8 {
    let num_of_ones = get_num_of_ones_at_index(&values, i);
    most_common_bit(num_of_ones, values.len() / 2)
}

fn calculate_least_common_bit_at_index(values: &Vec<Vec<u8>>, i: usize) -> u8 {
    let num_of_ones = get_num_of_ones_at_index(&values, i);
    least_common_bit(num_of_ones, values.len() / 2)
}

fn filter_on_bits_until_one_left(
    values: &Vec<Vec<u8>>,
    pred: fn(&Vec<Vec<u8>>, usize) -> u8,
) -> u32 {
    let mut values = values.clone();
    let mut current_idx = 0usize;
    while values.len() > 1 {
        let desired_bit = pred(&values, current_idx);
        values = values
            .iter()
            .filter(|bits| bits[current_idx] == desired_bit)
            .cloned()
            .collect();
        current_idx += 1;
    }
    let result = values.first().unwrap();
    bits_to_u32(result)
}

fn calculate_o2_generator_rating(reports: &Vec<Vec<u8>>) -> u32 {
    filter_on_bits_until_one_left(&reports, calculate_most_common_bit_at_index)
}

fn calculate_co2_scrubber_rating(reports: &Vec<Vec<u8>>) -> u32 {
    filter_on_bits_until_one_left(&reports, calculate_least_common_bit_at_index)
}

fn life_support_rating(o2_generator_rating: u32, co2_scrubber_rating: u32) -> u32 {
    o2_generator_rating * co2_scrubber_rating
}

fn calculate_life_support_rating(reports: &Vec<Vec<u8>>) -> u32 {
    let o2_generator_rating = calculate_o2_generator_rating(&reports);
    let co2_scrubber_rating = calculate_co2_scrubber_rating(&reports);

    println!(
        "O2 generator rating: {}, CO2 scrubber rating: {}",
        o2_generator_rating, co2_scrubber_rating
    );
    life_support_rating(o2_generator_rating, co2_scrubber_rating)
}

fn main() {
    let reports = read_file_lines_as("input/day3.txt", |s| {
        s.chars()
            .map(|digit| digit.to_digit(2).unwrap() as u8)
            .collect::<Vec<u8>>()
    });

    let power_consumption = calculate_power_consumption(&reports);
    println!("Power consumption is: {}", power_consumption);

    let life_support_rating = calculate_life_support_rating(&reports);
    println!("Life support rating is: {}", life_support_rating);
}
