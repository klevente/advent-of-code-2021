use advent_of_code_2021::read_file_lines_as;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(format!("Could not parse '{}' as Direction", s)),
        }
    }
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    amount: u32,
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(' ').collect();
        if tokens.len() != 2 {
            return Err(format!("Could not split '{}' into 2 pieces", s));
        }
        let dir = Direction::from_str(tokens[0])?;
        let amount = tokens[1]
            .parse::<u32>()
            .map_err(|_| format!("Could not parse '{}' as amount", tokens[1]))?;
        Ok(Command { dir, amount })
    }
}

impl Command {
    fn from_str_unwrapped(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

#[derive(Debug)]
struct Submarine {
    position: u32,
    depth: u32,
    aim: u32,
}

impl Display for Submarine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(position: {}, depth: {})", self.position, self.depth)
    }
}

impl Submarine {
    fn new() -> Self {
        Self {
            position: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn solution(&self) -> u32 {
        self.position * self.depth
    }

    fn execute(&mut self, command: &Command) {
        use Direction::*;
        let Command { dir, amount } = command;

        // first assignemnt
        /*match dir {
            Forward => self.position += amount,
            Down => self.depth += amount,
            Up => self.depth -= amount,
        }*/

        // second assignment
        match dir {
            Forward => {
                self.position += amount;
                self.depth += amount * self.aim
            }
            Down => self.aim += amount,
            Up => self.aim -= amount,
        }
    }

    fn execute_commands(&mut self, commands: &Vec<Command>) {
        for command in commands {
            self.execute(command);
        }
    }
}

fn main() {
    let commands = read_file_lines_as("input/day2.txt", Command::from_str_unwrapped);

    let mut submarine = Submarine::new();
    submarine.execute_commands(&commands);

    println!("Final coordinates are: {}", submarine);
    println!("Solution is: {}", submarine.solution());
}
