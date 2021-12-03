use advent_of_code_2021::read_file_lines_as;

fn calculate_most_common_bits(reports: &Vec<Vec<char>>) -> Result<Vec<u8>, String> {
    let report_len = reports
        .first()
        .ok_or("There are no reports inside the input".to_string())?
        .len();
    let mut num_of_ones = vec![0u32; report_len];

    for report in reports {
        for (i, digit) in report.iter().enumerate() {
            num_of_ones[i] += digit.to_digit(2).ok_or(format!(
                "The input contained an invalid character: {}",
                digit
            ))?;
        }
    }

    let half = reports.len() / 2;
    let bits = num_of_ones
        .iter()
        .map(|n| most_common_bit(n, half))
        .collect::<Vec<u8>>();

    Ok(bits)
}

fn most_common_bit(n_ones: &u32, half: usize) -> u8 {
    if *n_ones as usize > half {
        1u8
    } else {
        0u8
    }
}

fn calculate_value(bits: &Vec<u8>) -> u32 {
    bits.iter()
        .fold(0u32, |acc, next| (acc << 1) | *next as u32)
}

fn invert_bits(bits: &Vec<u8>) -> Vec<u8> {
    bits.iter()
        // .map(|bit| if *bit == 1u8 { 0u8 } else { 1u8 })
        .map(|bit| !bit & 1u8)
        .collect()
}

fn power_consumption(gamma: u32, epsilon: u32) -> u32 {
    gamma * epsilon
}

fn calculate_power_consumption(reports: &Vec<Vec<char>>) -> u32 {
    let most_common_bits = calculate_most_common_bits(&reports).unwrap();
    let least_common_bits = invert_bits(&most_common_bits);

    let gamma = calculate_value(&most_common_bits);
    let epsilon = calculate_value(&least_common_bits);

    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
    power_consumption(gamma, epsilon)
}

fn main() {
    let reports = read_file_lines_as("input/day3.txt", |s| s.chars().collect::<Vec<char>>());
    let power_consumption = calculate_power_consumption(&reports);
    println!("Power consumption is: {}", power_consumption);
}
