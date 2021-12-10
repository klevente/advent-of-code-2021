use advent_of_code_2021::read_file_lines;
use itertools::Itertools;
use phf::phf_map;

const ERROR_SCORES: phf::Map<char, u32> = phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
};

fn get_syntax_error_score(c: &char) -> u32 {
    *ERROR_SCORES.get(c).unwrap()
}

const COMPLETION_SCORE_MULTIPLIER: u64 = 5;
const COMPLETION_SCORES: phf::Map<char, u64> = phf_map! {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4,
};

fn get_completion_score(c: &char) -> u64 {
    *COMPLETION_SCORES.get(c).unwrap()
}

const PAIRS: phf::Map<char, char> = phf_map! {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
};

fn is_opener(c: &char) -> bool {
    PAIRS.contains_key(c)
}

fn map_opening_to_closing(c: &char) -> char {
    *PAIRS.get(c).unwrap()
}

fn check_line(s: &str) -> Result<Vec<char>, char> {
    let mut chunks = Vec::new();
    for c in s.chars() {
        if is_opener(&c) {
            chunks.push(map_opening_to_closing(&c));
        } else {
            let expected_char = chunks.pop().unwrap();
            if c != expected_char {
                return Err(c);
            }
        }
    }
    Ok(chunks)
}

fn calculate_syntax_error_score(lines: &Vec<String>) -> u32 {
    lines.iter().fold(0, |score, line| {
        if let Some(c) = check_line(line).err() {
            score + get_syntax_error_score(&c)
        } else {
            score
        }
    })
}

fn calc_completion_score_for(remaining: Vec<char>) -> u64 {
    remaining.iter().rev().fold(0, |score, c| {
        COMPLETION_SCORE_MULTIPLIER * score + get_completion_score(c)
    })
}

fn calculate_completion_score(lines: &Vec<String>) -> u64 {
    let mut sorted_scores_iter = lines
        .iter()
        .filter_map(|line| check_line(line).ok())
        .map(calc_completion_score_for)
        .sorted();

    sorted_scores_iter
        .nth(sorted_scores_iter.len() / 2)
        .unwrap()
}

fn main() {
    let lines = read_file_lines("input/day10.txt");

    let syntax_error_score = calculate_syntax_error_score(&lines);
    println!("The syntax error score is {}", syntax_error_score);

    let completion_score = calculate_completion_score(&lines);
    println!("The completion score is {}", completion_score);
}
