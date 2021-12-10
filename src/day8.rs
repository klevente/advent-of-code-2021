use advent_of_code_2021::{read_file_lines_as, vec_to_array};
use itertools::Itertools;
use phf::phf_map;
use std::collections::HashMap;

// only `1` is 2-length, `4` is 4-length, `7` is 3-length and `8` is 7-length
const TRIVIAL_DIGITS: phf::Map<u32, u8> = phf_map! {
    2u32 => 1,
    4u32 => 4,
    3u32 => 7,
    7u32 => 8
};

#[derive(Debug)]
struct DisplayConfig {
    patterns: [String; 10],
    output: [String; 4],
}

impl DisplayConfig {
    pub fn parse(s: &str) -> Self {
        let (patterns_raw, output_raw) = s.split_once('|').unwrap();
        Self {
            patterns: split_then_sort_chars(patterns_raw),
            output: split_then_sort_chars(output_raw),
        }
    }

    pub fn decode_all(&self) -> [u8; 4] {
        let mappings = self.find_all_mappings();
        let result = self
            .apply_mapping_to_output(&mappings)
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .unwrap();
        vec_to_array(result)
    }

    pub fn calculate_decoded_output(&self) -> u32 {
        let result = self.decode_all();
        result.iter().fold(0, |acc, &next| acc * 10 + next as u32)
    }

    pub fn decode_trivial(&self) -> [Option<u8>; 4] {
        let mappings = self.find_trivial_mappings();
        vec_to_array(self.apply_mapping_to_output(&mappings))
    }

    pub fn count_trivial_digits_in_output(&self) -> u32 {
        let result = self.decode_trivial();
        result
            .iter()
            .fold(0, |acc, next| if next.is_some() { acc + 1 } else { acc })
    }

    fn find_trivial_mappings(&self) -> HashMap<&str, u8> {
        let mut mappings = HashMap::new();

        for p in &self.patterns {
            if let Some(&digit) = TRIVIAL_DIGITS.get(&(p.len() as u32)) {
                mappings.insert(p.as_str(), digit);
            }
        }
        mappings
    }

    fn find_all_mappings(&self) -> HashMap<&str, u8> {
        let mut mappings = self.find_trivial_mappings();

        let one = get_pattern_for(&mappings, 1).unwrap();

        // contains `0`, `6`, `9`
        let six_length_patterns = self.get_patterns_of_length(6);

        // only `6` does not contain all the segments of `1`
        let six = find_digit(&six_length_patterns, |p| !str_contains_chars(p, one));
        mappings.insert(six, 6);

        // contains `2`, `3`, `5`
        let five_length_patterns = self.get_patterns_of_length(5);

        // `6` contains all segments of `5`
        let five = find_digit(&five_length_patterns, |p| str_contains_chars(six, p));
        mappings.insert(five, 5);

        // only `3` contains all segments of `1`
        let three = find_digit(&five_length_patterns, |p| str_contains_chars(p, one));
        mappings.insert(three, 3);

        // as there are `3` 5-length digits, the remaining one must be `2`
        let two = find_digit(&five_length_patterns, |p| p != five && p != three);
        mappings.insert(two, 2);

        // only `9` contains all segments of `3`
        let nine = find_digit(&six_length_patterns, |p| str_contains_chars(p, three));
        mappings.insert(nine, 9);

        // as there are only `3` 6-length digits, the remaining one must be `0`
        let zero = find_digit(&six_length_patterns, |p| p != six && p != nine);
        mappings.insert(zero, 0);

        // dbg!(&mappings);

        mappings
    }

    fn apply_mapping_to_output(&self, mappings: &HashMap<&str, u8>) -> Vec<Option<u8>> {
        self.output
            .iter()
            .map(|s| mappings.get(s.as_str()).map(|d| *d))
            .collect()
    }

    fn get_patterns_of_length(&self, n: usize) -> Vec<&str> {
        let slice = &self.patterns;
        slice
            .iter()
            .filter_map(|p| if p.len() == n { Some(p.as_str()) } else { None })
            .collect()
    }
}

fn find_digit<'a, F: Fn(&str) -> bool>(patterns: &Vec<&'a str>, pred: F) -> &'a str {
    patterns.iter().find(|&&p| pred(p)).unwrap()
}

fn get_pattern_for<'a>(mappings: &HashMap<&'a str, u8>, digit: u8) -> Option<&'a str> {
    mappings
        .iter()
        .find_map(|(&key, &val)| if val == digit { Some(key) } else { None })
}

fn str_contains_chars(s: &str, candidates: &str) -> bool {
    candidates.chars().all(|c| s.contains(c))
}

fn split_then_sort_chars<const N: usize>(s: &str) -> [String; N] {
    split_whitespace_to_array(s).map(|segment| sort_chars_in_string(&segment))
}

fn split_whitespace_to_array<const N: usize>(s: &str) -> [String; N] {
    vec_to_array(s.split_whitespace().map(String::from).collect())
}

fn sort_chars_in_string(s: &str) -> String {
    s.chars().sorted().collect::<String>()
}

fn map_then_sum_configs(configs: &Vec<DisplayConfig>, mapper: fn(&DisplayConfig) -> u32) -> u32 {
    configs.iter().map(mapper).sum()
}

fn count_trivial_digits_in_configs(configs: &Vec<DisplayConfig>) {
    let num_of_trivial_digits =
        map_then_sum_configs(&configs, DisplayConfig::count_trivial_digits_in_output);
    println!("The count of trivial digits is {}", num_of_trivial_digits);
}

fn calculate_sum_of_output_values_in_configs(configs: &Vec<DisplayConfig>) {
    let sum_of_outputs = map_then_sum_configs(&configs, DisplayConfig::calculate_decoded_output);
    println!("The sum of all outputs is {}", sum_of_outputs);
}

fn main() {
    let configs = read_file_lines_as("input/day8.txt", DisplayConfig::parse);
    count_trivial_digits_in_configs(&configs);
    calculate_sum_of_output_values_in_configs(&configs);
}
