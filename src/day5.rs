use advent_of_code_2021::read_file_lines;
use array2d::Array2D;
use sscanf::scanf;

#[derive(Debug)]
enum LineType {
    Straight,
    Diagonal,
}

#[derive(Debug)]
struct Line {
    from_x: u32,
    from_y: u32,
    to_x: u32,
    to_y: u32,
    line_type: LineType,
}

impl Line {
    fn scan_str(s: &str) -> (u32, u32, u32, u32) {
        scanf!(s, "{},{} -> {},{}", u32, u32, u32, u32).unwrap()
    }

    fn construct(coords: (u32, u32, u32, u32), line_type: LineType) -> Self {
        let (from_x, from_y, to_x, to_y) = coords;
        Self {
            from_x,
            from_y,
            to_x,
            to_y,
            line_type,
        }
    }

    fn is_straight((from_x, from_y, to_x, to_y): (u32, u32, u32, u32)) -> bool {
        from_x == to_x || from_y == to_y
    }

    pub fn parse(s: &String) -> Line {
        use LineType::*;
        let coords = Self::scan_str(s);
        let line_type = if Self::is_straight(coords) {
            Straight
        } else {
            Diagonal
        };
        Self::construct(coords, line_type)
    }

    pub fn parse_straight(s: &String) -> Option<Line> {
        let coords = Self::scan_str(s);

        if Self::is_straight(coords) {
            Some(Self::construct(coords, LineType::Straight))
        } else {
            None
        }
    }

    pub fn get_highest_x_y(&self) -> (u32, u32) {
        (self.from_x.max(self.to_x), self.from_y.max(self.to_y))
    }

    pub fn get_line_positions(&self) -> Vec<(u32, u32)> {
        use LineType::*;
        match self.line_type {
            Straight => self.get_straight_line_positions(),
            Diagonal => self.get_diagonal_line_positions(),
        }
    }

    fn get_straight_line_positions(&self) -> Vec<(u32, u32)> {
        let start_x = self.from_x.min(self.to_x);
        let end_x = self.from_x.max(self.to_x);
        let start_y = self.from_y.min(self.to_y);
        let end_y = self.from_y.max(self.to_y);

        let mut v = Vec::new();
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                v.push((x, y));
            }
        }
        v
    }

    fn get_diagonal_line_positions(&self) -> Vec<(u32, u32)> {
        let mut v = Vec::new();

        let (start_x, start_y, end_x, end_y) = if self.from_x <= self.to_x {
            (self.from_x, self.from_y, self.to_x, self.to_y)
        } else {
            (self.to_x, self.to_y, self.from_x, self.from_y)
        };

        let dx = end_x - start_x;
        let y_dir = (end_y as i32 - start_y as i32) / (dx) as i32;

        for p in 0..=dx {
            v.push((start_x + p, (start_y as i32 + (p as i32 * y_dir)) as u32))
        }
        v
    }
}

fn get_map_size(lines: &Vec<Line>) -> (u32, u32) {
    let (highest_x, highest_y) = lines.iter().fold((0, 0), |(c_x, c_y), l| {
        let (l_x, l_y) = l.get_highest_x_y();
        (c_x.max(l_x), c_y.max(l_y))
    });

    (highest_x + 1, highest_y + 1)
}

struct Map {
    tiles: Array2D<u32>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            tiles: Array2D::filled_with(0, height, width),
        }
    }

    pub fn add_line(&mut self, line: &Line) {
        let positions = line.get_line_positions();
        for (x, y) in &positions {
            let tile = self.tiles.get_mut(*y as usize, *x as usize).unwrap();
            *tile += 1;
        }
    }

    /*pub fn print_map(&self) {
        println!("Current state of the map:");
        for row in self.tiles.rows_iter() {
            for column in row.into_iter() {
                print!("{}", column);
            }
            println!();
        }
    }*/

    pub fn sum_of_dangerous_areas(&self) -> u32 {
        self.tiles
            .elements_row_major_iter()
            .filter(|&&n| n > 1u32)
            .count() as u32
    }
}

fn calculate_dangerous_areas(lines: &Vec<Line>) -> u32 {
    let (width, height) = get_map_size(&lines);

    let mut map = Map::new(width as usize, height as usize);

    for line in lines {
        map.add_line(line);
    }

    map.sum_of_dangerous_areas()
}

fn calculate_dangerous_areas_only_straight(lines: &Vec<String>) {
    let lines = lines
        .iter()
        .filter_map(Line::parse_straight)
        .collect::<Vec<_>>();

    let result = calculate_dangerous_areas(&lines);

    println!(
        "The sum of dangerous areas when only dealing with straight lines is: {}",
        result
    );
}

fn calculate_dangerous_areas_all(lines: &Vec<String>) {
    let lines = lines.iter().map(Line::parse).collect::<Vec<_>>();

    let result = calculate_dangerous_areas(&lines);

    println!(
        "The sum of dangerous areas when dealing with all lines is: {}",
        result
    );
}

fn main() {
    let lines = read_file_lines("input/day5.txt");
    calculate_dangerous_areas_only_straight(&lines);
    calculate_dangerous_areas_all(&lines);
}
