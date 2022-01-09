use advent_of_code_2021::read_file_lines;
use std::time::Instant;

struct Alu {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Alu {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        }
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.z = 0;
        self.w = 0;
    }

    pub fn run(&mut self, instructions: &[String], mut input: &[i64]) -> (i64, i64, i64, i64) {
        for instruction in instructions {
            /*println!("{}", instruction);
            println!("x: {}, y: {}, z: {}, w: {}", self.x, self.y, self.z, self.w);
            println!("-----------------------------------");*/
            let (op_code, args) = instruction.split_once(' ').unwrap();
            match op_code {
                "inp" => {
                    let reg = self.register(args);
                    let (value, rest) = input.split_first().unwrap();
                    *reg = *value;
                    input = rest;
                }
                op_code => {
                    let (reg, value) = args.split_once(' ').unwrap();
                    let value = self.value(value);
                    let reg = self.register(reg);
                    match op_code {
                        "add" => *reg += value,
                        "mul" => *reg *= value,
                        "div" => *reg /= value,
                        "mod" => *reg %= value,
                        "eql" => *reg = (*reg == value).into(),
                        _ => unreachable!(),
                    }
                }
            }
        }

        (self.x, self.y, self.z, self.w)
    }

    fn register(&mut self, reg: &str) -> &mut i64 {
        match reg {
            "x" => &mut self.x,
            "y" => &mut self.y,
            "z" => &mut self.z,
            "w" => &mut self.w,
            _ => unreachable!(),
        }
    }

    fn value(&self, value: &str) -> i64 {
        match value {
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            "w" => self.w,
            s => s.parse().unwrap(),
        }
    }
}

fn print_result(regs: &(i64, i64, i64, i64)) {
    let (x, y, z, w) = regs;
    println!("===================================");
    println!("x: {}, y: {}, z: {}, w: {}", x, y, z, w);
}

fn main() {
    let instructions = read_file_lines("input/day24.txt");

    let mut alu = Alu::new();

    let input = vec![5; 14];
    let start = Instant::now();
    let result = alu.run(&instructions, &input);
    let elapsed = start.elapsed();
    print_result(&result);
}
