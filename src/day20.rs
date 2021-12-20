use advent_of_code_2021::read_file_lines_extract_first;
use array2d::Array2D;

struct EnhancementAlgorithm {
    enhancements: Vec<bool>,
}

impl EnhancementAlgorithm {
    pub fn parse(s: &str) -> Self {
        let enhancements = s
            .chars()
            .map(|c| if c == '#' { true } else { false })
            .collect();

        Self { enhancements }
    }

    pub fn get(&self, index: usize) -> bool {
        *self.enhancements.get(index).unwrap()
    }
}

struct Image {
    pixels: Array2D<bool>,
}

impl Image {
    pub fn parse(lines: &[String]) -> Self {
        let height = lines.len() + 2;
        let width = lines.first().unwrap().len() + 2;

        let mut pixels = Array2D::filled_with(false, height, width);

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x + 1;
                let y = y + 1;
                if c == '#' {
                    pixels.set(y, x, true).unwrap();
                }
            }
        }

        Self { pixels }
    }

    pub fn count_lit_pixels(&self) -> usize {
        self.pixels
            .elements_row_major_iter()
            .filter(|&p| *p)
            .count()
    }

    pub fn enhance_twice(&self, algorithm: &EnhancementAlgorithm) -> Self {
        let once = self.enhance(10, 10, &algorithm);
        let twice = once.enhance(0, 0, &algorithm);
        let cropped = twice.crop(3);
        cropped
    }

    fn crop(&self, n: usize) -> Self {
        let mut pixels = Array2D::filled_with(
            false,
            self.pixels.num_rows() - 2 * n,
            self.pixels.num_columns() - 2 * n,
        );

        for (y, x) in pixels.indices_row_major() {
            let cropped_x = x + n;
            let cropped_y = y + n;

            let value = *self.pixels.get(cropped_y, cropped_x).unwrap();
            pixels.set(y, x, value).unwrap();
        }

        Self { pixels }
    }

    pub fn enhance(&self, widen: usize, heighten: usize, algorithm: &EnhancementAlgorithm) -> Self {
        let mut pixels = Array2D::filled_with(
            false,
            self.pixels.num_rows() + heighten,
            self.pixels.num_columns() + widen,
        );

        for (y, x) in pixels.indices_row_major() {
            let input_x = x as isize - (heighten / 2) as isize;
            let input_y = y as isize - (heighten / 2) as isize;
            let index = self.calculate_enhancement_index(input_x, input_y);
            let pixel = algorithm.get(index);
            pixels.set(y, x, pixel).unwrap();
        }

        Self { pixels }
    }

    fn calculate_enhancement_index(&self, x: isize, y: isize) -> usize {
        self.get_scan_area(x, y)
            .iter()
            .fold(0, |acc, next| (acc << 1) | *next as usize)
    }

    fn get_scan_area(&self, x: isize, y: isize) -> Vec<u8> {
        let area = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        area.iter()
            .map(|(x, y)| {
                if *x >= 0 && *y >= 0 {
                    if *self.pixels.get(*y as usize, *x as usize).unwrap_or(&false) {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .collect()
    }

    pub fn print(&self) {
        println!(
            "Image is {}x{}",
            self.pixels.num_rows(),
            self.pixels.num_columns()
        );
        for row in self.pixels.rows_iter() {
            for column in row.into_iter() {
                let c = if *column { '#' } else { '.' };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

fn count_lit_pixels_after_2_enhancements(algorithm: &EnhancementAlgorithm, starting_image: &Image) {
    let result = starting_image.enhance_twice(&algorithm);
    result.print();

    let num_of_lit_pixels = result.count_lit_pixels();
    println!(
        "The number of lit pixels after 2 enhancements is {}",
        num_of_lit_pixels
    );
}

fn count_lit_pixels_after_50_enhancements(
    algorithm: &EnhancementAlgorithm,
    starting_image: &Image,
) {
    let result = (0..24)
        .into_iter()
        .fold(starting_image.enhance_twice(&algorithm), |acc, _| {
            acc.enhance_twice(&algorithm)
        });

    result.print();

    let num_of_lit_pixels = result.count_lit_pixels();
    println!(
        "The number of lit pixels after 50 enhancements is {}",
        num_of_lit_pixels
    );
}

fn main() {
    let (enhancement_rules, image) = read_file_lines_extract_first("input/day20.txt");

    let algorithm = EnhancementAlgorithm::parse(&enhancement_rules);
    let image = Image::parse(&image);

    count_lit_pixels_after_2_enhancements(&algorithm, &image);
    count_lit_pixels_after_50_enhancements(&algorithm, &image);
}
