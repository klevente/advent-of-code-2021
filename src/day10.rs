use advent_of_code_2021::read_file_lines;
use phf::phf_map;

const SCORES: phf::Map<char, u32> = phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137
};

fn get_score(c: &char) -> u32 {
    *SCORES.get(c).unwrap()
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

fn check_line(s: &str) -> Result<(), char> {
    let mut chunks = Vec::new();
    for c in s.chars() {
        if is_opener(&c) {
            chunks.push(c);
        } else {
            let current_chunk = chunks.pop().unwrap();
            let expected_char = map_opening_to_closing(&current_chunk);
            if c != expected_char {
                return Err(c);
            }
        }
    }
    Ok(())
}

fn calculate_syntax_error_score(lines: &Vec<String>) -> u32 {
    let mut score = 0;
    for line in lines.iter() {
        check_line(&line).unwrap_or_else(|e| score += get_score(&e));
    }
    score
}

fn main() {
    let lines = read_file_lines("input/day10.txt");

    let syntax_error_score = calculate_syntax_error_score(&lines);
    println!("The syntax error score is {}", syntax_error_score);
}
