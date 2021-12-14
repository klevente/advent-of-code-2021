use advent_of_code_2021::read_file_lines_extract_first;
use sscanf::scanf;
use std::str::FromStr;

struct Rule {
    pattern: String,
    insert: char,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern, insert) = scanf!(s, "{} -> {}", String, char).ok_or("Invalid pattern")?;

        Ok(Self { pattern, insert })
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", &self.pattern, &self.insert)
    }
}

struct Polymer {
    elements: String,
}

impl Polymer {
    pub fn new(elements: String) -> Self {
        Self { elements }
    }
}

impl std::fmt::Display for Polymer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.elements)
    }
}

fn parse_file(path: &str) -> (Polymer, Vec<Rule>) {
    let (polymer, rules) = read_file_lines_extract_first(path);

    let polymer = Polymer::new(polymer);
    let rules = rules
        .iter()
        .map(|s| Rule::from_str(s).unwrap())
        .collect::<Vec<_>>();

    (polymer, rules)
}

fn main() {
    let a = 1;
}
