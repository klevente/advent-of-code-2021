use advent_of_code_2021::read_file_lines_extract_first;
use itertools::FoldWhile::{Continue, Done};
use itertools::{Itertools, MinMaxResult};
use sscanf::scanf;
use std::collections::HashMap;
use std::str::FromStr;

struct Rule {
    pattern_1: char,
    pattern_2: char,
    insert: char,
}

impl Rule {
    pub fn is_applicable(&self, c1: char, c2: char) -> Option<char> {
        if c1 == self.pattern_1 && c2 == self.pattern_2 {
            Some(self.insert)
        } else {
            None
        }
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern_1, pattern_2, insert) =
            scanf!(s, "{}{} -> {}", char, char, char).ok_or("Invalid pattern")?;
        Ok(Self {
            pattern_1,
            pattern_2,
            insert,
        })
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} -> {}",
            &self.pattern_1, &self.pattern_2, &self.insert
        )
    }
}

fn char_frequencies_of(s: &str) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

#[derive(Clone)]
struct Polymer {
    elements: String,
}

impl Polymer {
    pub fn new(elements: String) -> Self {
        Self { elements }
    }

    pub fn most_common_element_minus_least_common_element(&self) -> usize {
        let frequencies = char_frequencies_of(&self.elements);
        let minmax = frequencies.values().minmax();
        match minmax {
            MinMaxResult::MinMax(min, max) => max - min,
            _ => 0,
        }
    }

    pub fn apply_rules_n_times(self, rules: &[Rule], n: usize) -> Self {
        println!("Original:");
        println!("{}", self);
        let mut this = self;
        for step in 1..=n {
            println!("Step {}:", step);
            this = this.apply_rules(rules);
            println!("{}", this);
        }

        this
    }

    pub fn apply_rules(mut self, rules: &[Rule]) -> Self {
        let chars_to_insert = self.collect_chars_to_insert(rules);

        for (i, c) in chars_to_insert {
            self.elements.insert(i, c);
        }

        self
    }

    fn collect_chars_to_insert(&self, rules: &[Rule]) -> Vec<(usize, char)> {
        self.elements.char_indices().tuple_windows::<(_, _)>().fold(
            Vec::new(),
            |mut acc, ((_i1, c1), (i2, c2))| {
                if let Some(char_to_insert) = Self::is_any_rule_applicable(c1, c2, rules) {
                    let offset = acc.len();
                    acc.push((i2 + offset, char_to_insert));
                }
                acc
            },
        )
    }

    fn is_any_rule_applicable(c1: char, c2: char, rules: &[Rule]) -> Option<char> {
        rules
            .iter()
            .fold_while(None, |_, r| {
                r.is_applicable(c1, c2)
                    .map_or(Continue(None), |x| Done(Some(x)))
            })
            .into_inner()
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

fn calculate_most_common_minus_least_common_after_10_steps(polymer: Polymer, rules: &[Rule]) {
    let result = polymer.apply_rules_n_times(&rules, 10);
    let value = result.most_common_element_minus_least_common_element();
    println!(
        "The quantity of the most common element minus the least common element after 10 steps is {}",
        value
    );
}

fn calculate_most_common_minus_least_common_after_40_steps(polymer: Polymer, rules: &[Rule]) {
    let result = polymer.apply_rules_n_times(&rules, 40);
    let value = result.most_common_element_minus_least_common_element();
    println!(
        "The quantity of the most common element minus the least common element after 40 steps is {}",
        value
    );
}

fn main() {
    let (polymer, rules) = parse_file("input/day14.txt");

    calculate_most_common_minus_least_common_after_10_steps(polymer.clone(), &rules);
    calculate_most_common_minus_least_common_after_40_steps(polymer, &rules);
}
