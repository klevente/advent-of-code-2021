use advent_of_code_2021::read_file_to_string;
use array2d::Array2D;
use sscanf::scanf;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Up,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Direction::Left),
            'y' => Ok(Direction::Up),
            _ => Err("Invalid direction specifier".to_string()),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct Fold {
    along: usize,
    direction: Direction,
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, along) =
            scanf!(s, "fold along {}={}", char, usize).ok_or("Invalid fold format".to_string())?;

        Ok(Self {
            along,
            direction: dir.try_into()?,
        })
    }
}

impl std::fmt::Display for Fold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let axis = match self.direction {
            Direction::Up => "row",
            Direction::Left => "column",
        };
        write!(f, "{} along {} {}", self.direction, axis, self.along)
    }
}

struct Paper {
    tiles: Array2D<bool>,
}

impl Paper {
    pub fn parse(dots_str: &[&str]) -> Self {
        let dots = dots_str
            .iter()
            .filter_map(|l| {
                l.split_once(',').map(|(x_str, y_str)| {
                    (
                        x_str.parse::<usize>().unwrap(),
                        (y_str.parse::<usize>().unwrap()),
                    )
                })
            })
            .collect::<Vec<_>>();

        let (width, height) = Self::get_paper_size(&dots);

        let mut tiles = Array2D::filled_with(false, height, width);
        for (x, y) in dots {
            tiles.set(y, x, true).unwrap();
        }

        Self { tiles }
    }

    pub fn count_num_of_dots(&self) -> u32 {
        self.tiles.elements_row_major_iter().filter(|&&b| b).count() as u32
    }

    pub fn fold_by(self, fold: &Fold) -> Self {
        match fold.direction {
            Direction::Up => self.fold_vertically_up(fold),
            Direction::Left => self.fold_horizontally_left(fold),
        }
    }

    fn fold_vertically_up(self, fold: &Fold) -> Self {
        let num_rows = self.tiles.num_rows() - (fold.along + 1);
        let num_columns = self.tiles.num_columns();

        let iter = self
            .tiles
            .elements_row_major_iter()
            .map(|b| *b)
            .take(num_rows * num_columns);

        let mut tiles: Array2D<bool> =
            Array2D::from_iter_row_major(iter, num_rows, num_columns).unwrap();

        for ((row, column), v) in self
            .tiles
            .enumerate_row_major()
            .skip((num_rows + 1) * num_columns)
        {
            let row_to_update = 2 * num_rows - row;
            let field_to_update = tiles.get_mut(row_to_update, column).unwrap();
            *field_to_update |= v;
        }

        Self { tiles }
    }

    fn fold_horizontally_left(self, fold: &Fold) -> Self {
        let num_rows = self.tiles.num_rows();
        let num_columns = self.tiles.num_columns() - (fold.along + 1);

        let iter = self
            .tiles
            .elements_column_major_iter()
            .map(|b| *b)
            .take(num_rows * num_columns);

        let mut tiles = Array2D::from_iter_column_major(iter, num_rows, num_columns).unwrap();

        for ((row, column), v) in self
            .tiles
            .enumerate_column_major()
            .skip(num_rows * (num_columns + 1))
        {
            let column_to_update = 2 * num_columns - column;
            let field_to_update = tiles.get_mut(row, column_to_update).unwrap();
            *field_to_update |= v;
        }

        Self { tiles }
    }

    fn get_paper_size(dots: &Vec<(usize, usize)>) -> (usize, usize) {
        let (highest_x, highest_y) = dots.iter().fold((0, 0), |(c_x, c_y), (d_x, d_y)| {
            (c_x.max(*d_x), c_y.max(*d_y))
        });

        (highest_x + 1, highest_y + 1)
    }
}

impl std::fmt::Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.tiles.num_columns();
        let height = self.tiles.num_rows();
        writeln!(f, "The paper is {}x{}:", width, height)?;
        for row in self.tiles.rows_iter() {
            for column in row.into_iter() {
                let c = if *column { '#' } else { '.' };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn calculate_number_of_dots_after_one_fold(dots_str: &[&str], fold: &Fold) -> u32 {
    let mut paper = Paper::parse(dots_str);
    paper = paper.fold_by(fold);

    paper.count_num_of_dots()
}

fn fold_paper_according_to_instructions(dots_str: &[&str], folds: &[Fold]) {
    let mut paper = Paper::parse(dots_str);

    println!("Initial paper state:");
    println!("{}", &paper);

    for fold in folds {
        paper = paper.fold_by(fold);
        println!("Paper after folding:");
        println!("{}", &paper);
    }
}

fn main() {
    let input = read_file_to_string("input/day13.txt");

    let (dots_str, folds_str): (Vec<_>, Vec<_>) = input.lines().partition(|l| !l.contains("fold"));

    let folds = folds_str
        .iter()
        .map(|l| Fold::from_str(l).unwrap())
        .collect::<Vec<_>>();

    let num_of_dots_after_one_fold =
        calculate_number_of_dots_after_one_fold(&dots_str, folds.first().unwrap());

    println!(
        "The number of dots after one fold is {}",
        num_of_dots_after_one_fold
    );

    fold_paper_according_to_instructions(&dots_str, &folds);
}
