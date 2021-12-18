use advent_of_code_2021::read_file_lines_as;
use itertools::{max, Itertools};

type Expression = Vec<Token>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Token {
    Start,
    End,
    Value(u32),
}

impl Token {
    pub fn from_char(c: char) -> Option<Self> {
        if c == '[' {
            Some(Self::Start)
        } else if c == ']' {
            Some(Self::End)
        } else {
            c.to_digit(10).map(|d| Self::Value(d))
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "[")?,
            Self::End => write!(f, "],")?,
            Self::Value(v) => write!(f, "{},", v)?,
        }

        Ok(())
    }
}

fn parse_expression(line: &str) -> Expression {
    line.chars().filter_map(Token::from_char).collect()
}

fn print_expression(expression: &Expression) {
    for t in expression {
        print!("{}", t);
    }
    println!();
}

fn add_expressions(lhs: &Expression, rhs: &Expression) -> Expression {
    let l_len = lhs.len();
    let r_len = rhs.len();
    let mut result = Vec::with_capacity(l_len + r_len + 2);

    result.push(Token::Start);
    result.extend(lhs.iter());
    result.extend(rhs.iter());
    result.push(Token::End);

    reduce_expression(&mut result);

    result
}

fn reduce_expression(expression: &mut Expression) {
    let mut can_reduce = true;

    while can_reduce {
        let did_explode = try_explode_first_eligible_pair(expression);
        if !did_explode {
            let did_split = try_split_first_eligible_value(expression);
            if !did_split {
                can_reduce = false;
            }
        }
    }
}

fn try_explode_first_eligible_pair(expression: &mut Expression) -> bool {
    if let Some((position, x, y)) = find_pair_to_explode(&expression) {
        if let Some(value) = find_first_value_left_of_pair_start(expression, position) {
            *value += x;
        }
        if let Some(value) = find_first_value_right_of_pair_start(expression, position) {
            *value += y;
        }

        print_expression(&expression);
        *expression.get_mut(position - 1).unwrap() = Token::Value(0);
        print_expression(&expression);
        expression.drain(position..(position + 3));
        print_expression(&expression);

        true
    } else {
        false
    }
}

fn find_first_value_left_of_pair_start(expression: &mut Expression, i: usize) -> Option<&mut u32> {
    let range = &mut expression[..i];
    range.iter_mut().rev().find_map(|t| {
        if let Token::Value(v) = t {
            Some(v)
        } else {
            None
        }
    })
}

fn find_first_value_right_of_pair_start(expression: &mut Expression, i: usize) -> Option<&mut u32> {
    let range = &mut expression[(i + 2)..];
    range.iter_mut().find_map(|t| {
        if let Token::Value(v) = t {
            Some(v)
        } else {
            None
        }
    })
}

fn find_pair_to_explode(expression: &Expression) -> Option<(usize, u32, u32)> {
    let mut depth = 0;
    for (i, t) in expression.iter().enumerate() {
        match t {
            Token::Start => depth += 1,
            Token::End => depth -= 1,
            Token::Value(x) => {
                if depth == 5 {
                    if let Token::Value(y) = expression[i + 1] {
                        return Some((i, *x, y));
                    }
                }
            }
        }
    }
    None
}

fn try_split_first_eligible_value(expression: &mut Expression) -> bool {
    if let Some((position, value)) = find_value_to_split(&expression) {
        let (x, y) = calculate_split_values(value);

        *expression.get_mut(position).unwrap() = Token::End;
        expression.insert(position, Token::Value(y));
        expression.insert(position, Token::Value(x));
        expression.insert(position, Token::Start);

        true
    } else {
        false
    }
}

fn find_value_to_split(expression: &Expression) -> Option<(usize, u32)> {
    expression.iter().enumerate().find_map(|(i, t)| {
        if let Token::Value(v) = t {
            if *v >= 10 {
                Some((i, *v))
            } else {
                None
            }
        } else {
            None
        }
    })
}

fn calculate_split_values(v: u32) -> (u32, u32) {
    let half = (v as f32) / 2.0;
    (half.floor() as u32, half.ceil() as u32)
}

fn calculate_sum_of_expressions(mut expressions: Vec<Expression>) -> Expression {
    let first = expressions.remove(0);
    expressions
        .iter()
        .fold(first, |acc, next| add_expressions(&acc, next))
}

fn calculate_magnitude(expression: &Expression) -> u32 {
    let (magnitude, _) = calculate_magnitude_rec(0, &expression);
    magnitude
}

fn calculate_magnitude_rec(acc: u32, part: &[Token]) -> (u32, &[Token]) {
    if let Some((head, tail)) = part.split_first() {
        let mut m = 0;

        let (_x, head, tail) = if let Token::Value(v) = head {
            m += 3 * v;
            let (head, tail) = tail.split_first().unwrap();
            (*v, head, tail)
        } else {
            let (x, tail) = calculate_magnitude_rec(0, tail);
            if let Some((head, tail)) = tail.split_first() {
                m += 3 * x;
                (x, head, tail)
            } else {
                return (x, &[]);
            }
        };

        let (_y, _head, tail) = if let Token::Value(v) = head {
            m += 2 * v;
            let (head, tail) = tail.split_first().unwrap();
            (*v, head, tail)
        } else {
            let (y, tail) = calculate_magnitude_rec(0, tail);
            if let Some((head, tail)) = tail.split_first() {
                m += 2 * y;
                (y, head, tail)
            } else {
                return (y, &[]);
            }
        };

        (m, tail)
    } else {
        (acc, &[])
    }
}

fn find_homework_magnitude(expressions: Vec<Expression>) {
    let result = calculate_sum_of_expressions(expressions);
    print_expression(&result);
    let magnitude = calculate_magnitude(&result);
    println!("The magnitude is {}", magnitude);
}

fn find_largest_magnitude_from_sum(expressions: &Vec<Expression>) {
    let max_magnitude = expressions
        .iter()
        .cartesian_product(expressions.iter())
        .filter_map(|(e1, e2)| {
            if e1 != e2 {
                let result = add_expressions(e1, e2);
                let magnitude = calculate_magnitude(&result);
                Some(magnitude)
            } else {
                None
            }
        })
        .max()
        .unwrap();

    println!(
        "The largest magnitude value of all possible sums is {}",
        max_magnitude
    );
}

fn main() {
    let expressions = read_file_lines_as("input/day18.txt", parse_expression);
    // find_homework_magnitude(expressions.clone());
    find_largest_magnitude_from_sum(&expressions);
}
