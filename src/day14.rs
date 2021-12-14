use advent_of_code_2021::read_file_lines_extract_first;
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools, MinMaxResult,
};
use sscanf::scanf;
use std::{collections::HashMap, hash::Hash, str::FromStr};

struct RuleSlow {
    pattern_1: char,
    pattern_2: char,
    insert: char,
}

impl RuleSlow {
    pub fn is_applicable(&self, c1: char, c2: char) -> Option<char> {
        if c1 == self.pattern_1 && c2 == self.pattern_2 {
            Some(self.insert)
        } else {
            None
        }
    }
}

impl FromStr for RuleSlow {
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

impl std::fmt::Display for RuleSlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} -> {}",
            &self.pattern_1, &self.pattern_2, &self.insert
        )
    }
}

fn increment_map_entry<K: Eq + Hash, V: std::ops::AddAssign + Default>(
    map: &mut HashMap<K, V>,
    k: K,
    v: V,
) {
    *map.entry(k).or_insert(V::default()) += v;
}

fn char_frequencies_of(s: &str) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut acc, c| {
        increment_map_entry(&mut acc, c, 1);
        acc
    })
}

#[derive(Clone)]
struct PolymerSlow {
    elements: String,
}

impl PolymerSlow {
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

    pub fn apply_rules_n_times(self, rules: &[RuleSlow], n: usize) -> Self {
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

    pub fn apply_rules(mut self, rules: &[RuleSlow]) -> Self {
        let chars_to_insert = self.collect_chars_to_insert(rules);

        for (i, c) in chars_to_insert {
            self.elements.insert(i, c);
        }

        self
    }

    fn collect_chars_to_insert(&self, rules: &[RuleSlow]) -> Vec<(usize, char)> {
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

    fn is_any_rule_applicable(c1: char, c2: char, rules: &[RuleSlow]) -> Option<char> {
        rules
            .iter()
            .fold_while(None, |_, r| {
                r.is_applicable(c1, c2)
                    .map_or(Continue(None), |x| Done(Some(x)))
            })
            .into_inner()
    }
}

impl std::fmt::Display for PolymerSlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.elements)
    }
}

fn calculate_most_common_minus_least_common_elements_after_10_steps_slow(
    polymer: String,
    rules: &[String],
) {
    let polymer = PolymerSlow::new(polymer);
    let rules = rules
        .iter()
        .map(|s| RuleSlow::from_str(s).unwrap())
        .collect::<Vec<_>>();

    let result = polymer.apply_rules_n_times(&rules, 10);
    let value = result.most_common_element_minus_least_common_element();
    println!(
        "The quantity of the most common element minus the least common element after 10 steps is {}",
        value
    );
}

#[derive(Debug)]
struct RuleFast {
    p1: char,
    p2: char,
    new: char,
}

impl RuleFast {
    pub fn pattern(&self) -> (char, char) {
        (self.p1, self.p2)
    }

    pub fn apply_rule(&self) -> ((char, char), (char, char)) {
        ((self.p1, self.new), (self.new, self.p2))
    }
}

impl FromStr for RuleFast {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2, new) = scanf!(s, "{}{} -> {}", char, char, char).ok_or("Invalid format")?;

        Ok(Self { p1, p2, new })
    }
}

#[derive(Debug)]
struct PolymerFast {
    active_rules: HashMap<(char, char), usize>,
    element_frequencies: HashMap<char, usize>,
}

impl PolymerFast {
    pub fn parse(s: &str) -> Self {
        let active_rules =
            s.chars()
                .tuple_windows::<(_, _)>()
                .fold(HashMap::new(), |mut acc, next| {
                    increment_map_entry(&mut acc, next, 1);
                    acc
                });

        let element_frequencies = char_frequencies_of(s);

        Self {
            active_rules,
            element_frequencies,
        }
    }

    pub fn most_common_element_minus_least_common_element(&self) -> usize {
        let minmax = self.element_frequencies.values().minmax();
        match minmax {
            MinMaxResult::MinMax(min, max) => max - min,
            _ => 0,
        }
    }

    pub fn apply_rules_n_times(&mut self, rules: &[RuleFast], n: usize) {
        for step in 1..=n {
            println!("Step {}...", step);
            self.apply_rules(rules);
            println!("Step {} complete.", step);
        }
    }

    fn apply_rules(&mut self, rules: &[RuleFast]) {
        let result = self
            .active_rules
            .iter()
            .fold(HashMap::new(), |mut acc, (&pat, &i)| {
                if let Some(rule) = rules.iter().find(|r| r.pattern() == pat) {
                    let (res1, res2) = rule.apply_rule();
                    increment_map_entry(&mut acc, res1, i);
                    increment_map_entry(&mut acc, res2, i);
                    increment_map_entry(&mut self.element_frequencies, rule.new, i);
                }
                acc
            });

        self.active_rules = result;
    }
}

fn calculate_most_common_minus_least_common_elements_after_40_steps_fast(
    polymer: &str,
    rules: &[String],
) {
    let mut polymer = PolymerFast::parse(polymer);
    let rules = rules
        .iter()
        .map(|s| RuleFast::from_str(s).unwrap())
        .collect::<Vec<_>>();

    polymer.apply_rules_n_times(&rules, 40);

    let value = polymer.most_common_element_minus_least_common_element();
    println!(
        "The quantity of the most common element minus the least common element after 40 steps is {}",
        value
    );
}

fn main() {
    let (polymer, rules) = read_file_lines_extract_first("input/day14.txt");
    calculate_most_common_minus_least_common_elements_after_10_steps_slow(polymer.clone(), &rules);
    calculate_most_common_minus_least_common_elements_after_40_steps_fast(&polymer, &rules);
}
