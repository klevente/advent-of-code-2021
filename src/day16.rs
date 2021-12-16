use advent_of_code_2021::read_file_to_string;
use phf::phf_map;
use std::convert::identity;

const HEX_TO_BIN: phf::Map<char, &'static str> = phf_map! {
    '0' => "0000",
    '1' => "0001",
    '2' => "0010",
    '3' => "0011",
    '4' => "0100",
    '5' => "0101",
    '6' => "0110",
    '7' => "0111",
    '8' => "1000",
    '9' => "1001",
    'A' => "1010",
    'B' => "1011",
    'C' => "1100",
    'D' => "1101",
    'E' => "1110",
    'F' => "1111",
};

fn parse_input_as_binary_str(path: &str) -> String {
    let input = read_file_to_string(path);

    input
        .chars()
        .map(|c| HEX_TO_BIN.get(&c).unwrap().to_string())
        .collect()
}

fn binary_str_to_usize(s: impl AsRef<str>) -> usize {
    usize::from_str_radix(s.as_ref(), 2).unwrap()
}

fn binary_str_to_bool(s: impl AsRef<str>) -> bool {
    match s.as_ref() {
        "1" => true,
        "0" => false,
        _ => panic!("Cannot convert '{}' to bool", s.as_ref()),
    }
}

fn split_and_map_head<'a, T, F: Fn(&'a str) -> T>(str: &'a str, n: usize, f: F) -> (T, &'a str) {
    let (head, tail) = str.split_at(n);
    (f(head), tail)
}

fn split_off_n_digits(str: &str, n: usize) -> (&str, &str) {
    split_and_map_head(str, n, identity)
}

fn parse_and_remove_n_digits(str: &str, n: usize) -> (usize, &str) {
    split_and_map_head(str, n, binary_str_to_usize)
}

fn parse_and_remove_flag(str: &str) -> (bool, &str) {
    split_and_map_head(str, 1, binary_str_to_bool)
}

fn process_packet(packet: &str) -> (usize, &str) {
    let (version, packet) = parse_and_remove_n_digits(packet, 3);
    let (type_id, packet) = parse_and_remove_n_digits(packet, 3);

    if type_id == 4 {
        (version, process_literal(packet))
    } else {
        let (sum, p) = process_operator(packet);
        (sum + version, p)
    }
}

fn process_literal(packet: &str) -> &str {
    let (mut group, mut packet) = split_off_n_digits(packet, 5);
    let mut value = String::new();

    loop {
        let (has_more, literal_part) = parse_and_remove_flag(group);
        value += literal_part;
        if !has_more {
            break;
        }
        let (g, p) = split_off_n_digits(packet, 5);
        group = g;
        packet = p;
    }

    dbg!(&value);
    let numeric_value = binary_str_to_usize(&value);
    dbg!(numeric_value);
    packet
}

fn process_operator(packet: &str) -> (usize, &str) {
    let (is_length_type_num_of_sub_packets, packet) = parse_and_remove_flag(packet);

    if is_length_type_num_of_sub_packets {
        let (num_of_sub_packets, packet) = parse_and_remove_n_digits(packet, 11);
        process_sub_packets_num(packet, num_of_sub_packets)
    } else {
        let (length, packet) = parse_and_remove_n_digits(packet, 15);
        process_sub_packets_with_total_length(packet, length)
    }
}

fn process_sub_packets_with_total_length(packet: &str, length: usize) -> (usize, &str) {
    let original_length = packet.len();
    let mut new_length = original_length;
    let mut sum_of_versions = 0;
    let mut packet = packet;
    println!("Processing sub packets of size {}", length);
    while original_length - length != new_length {
        println!("Processing sub packet");
        let (sum, p) = process_packet(packet);
        packet = p;
        sum_of_versions += sum;
        new_length = packet.len();
    }

    (sum_of_versions, packet)
}

fn process_sub_packets_num(packet: &str, n: usize) -> (usize, &str) {
    let mut sum_of_versions = 0;
    let mut packet = packet;
    println!("Processing {} sub packets", n);
    for i in 0..n {
        println!("Processing sub packet {}", i);
        let (sum, p) = process_packet(packet);
        packet = p;
        sum_of_versions += sum;
        println!("Processed sub packet {}", i);
    }

    (sum_of_versions, packet)
}

fn main() {
    let packet_str = parse_input_as_binary_str("input/day16.txt");
    let (sum_of_versions, _) = process_packet(&packet_str);
    println!("The sum of all version numbers is {}", sum_of_versions);
}
