use advent_of_code_2021::read_file_lines;
use array2d::Array2D;

#[derive(Clone, Copy, Debug)]
struct Field {
    value: u8,
    drawn: bool,
}

impl Field {
    fn new(value: u8) -> Self {
        Self {
            value,
            drawn: false,
        }
    }

    fn draw(self) -> Self {
        Self {
            drawn: true,
            ..self
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    board: Array2D<Field>,
    won: bool,
}

impl Board {
    pub fn parse(lines: &[String]) -> Self {
        let nums_iter = lines
            .iter()
            .map(|line| split_str_to_u8(line))
            .flatten()
            .map(Field::new);

        let board = Array2D::from_iter_row_major(nums_iter, 5, 5);

        Self { board, won: false }
    }

    pub fn draw(&mut self, num: u8) -> Option<u32> {
        if self.won {
            return None;
        }
        let position = self.find_drawn_value(num);
        if let Some((x, y, f)) = position {
            let f = f.draw(); // required so the immutable borrow is released here before calling `set` which borrows mutably
            self.board.set(y, x, f).unwrap();

            if self.is_win() {
                self.won = true;
                return Some(self.score(num));
            }
        }

        None
    }

    fn find_drawn_value(&self, num: u8) -> Option<(usize, usize, &Field)> {
        for (y, row) in self.board.rows_iter().enumerate() {
            for (x, column) in row.enumerate() {
                if column.value == num {
                    return Some((x, y, column));
                }
            }
        }
        None
    }

    fn is_win(&self) -> bool {
        self.board.rows_iter().any(|mut r| r.all(|f| f.drawn))
            || self.board.columns_iter().any(|mut c| c.all(|f| f.drawn))
    }

    fn score(&self, last_num: u8) -> u32 {
        let sum_unmarked = self.sum_unmarked();
        println!(
            "Sum of unmarked fields: {}, last number drawn: {}",
            sum_unmarked, last_num
        );
        self.sum_unmarked() * last_num as u32
    }

    fn sum_unmarked(&self) -> u32 {
        self.board.elements_row_major_iter().fold(0, |acc, field| {
            if !field.drawn {
                acc + field.value as u32
            } else {
                acc
            }
        })
    }
}

fn str_to_u8(num: &str) -> u8 {
    num.parse().expect(&*format!("The input was: {}", num))
}

fn split_str_to_u8(line: &str) -> Vec<u8> {
    line.split_whitespace().map(str_to_u8).collect()
}

fn parse_draws(line: &str) -> Vec<u8> {
    line.split(',').map(str_to_u8).collect()
}

fn find_score_of_first_winning_board(
    mut boards: Vec<Board>,
    nums: &Vec<u8>,
) -> Result<u32, String> {
    for n in nums {
        for board in &mut boards {
            let outcome = board.draw(*n);
            if let Some(score) = outcome {
                return Ok(score);
            }
        }
    }

    Err("No boards won after drawing all numbers.".to_string())
}

fn find_score_of_last_winning_board(mut boards: Vec<Board>, nums: &Vec<u8>) -> Result<u32, String> {
    let mut final_score: Option<u32> = None;
    for n in nums {
        for board in &mut boards {
            let outcome = board.draw(*n);
            if let Some(score) = outcome {
                final_score.replace(score);
            }
        }
    }
    final_score.ok_or("No boards won after drawing all numbers.".to_string())
}

fn read_and_transform_input_lines(path: &str) -> (String, Vec<String>) {
    let mut whole = read_file_lines(path)
        .iter()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.to_owned())
            }
        })
        .collect::<Vec<_>>();
    let draws = whole.remove(0);

    (draws, whole)
}

fn parse_file(path: &str) -> (Vec<u8>, Vec<Board>) {
    let (draws, boards_raw) = read_and_transform_input_lines(path);
    let draws = parse_draws(&draws);
    let boards = boards_raw
        .chunks_exact(5)
        .map(Board::parse)
        .collect::<Vec<_>>();

    (draws, boards)
}

fn main() {
    let (draws, boards) = parse_file("input/day4.txt");

    let score_first = find_score_of_first_winning_board(boards.clone(), &draws).unwrap();
    println!("The score of the first winning board is {}", score_first);
    let score_last = find_score_of_last_winning_board(boards.clone(), &draws).unwrap();
    println!("The score of the last winning board is {}", score_last);
}
