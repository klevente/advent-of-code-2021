use advent_of_code_2021::read_file_to_string;
use sscanf::scanf;
use std::str::FromStr;

struct Player {
    id: u8,
    position: u8,
    score: u32,
}

impl FromStr for Player {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, position) =
            scanf!(s, "Player {} starting position: {}", u8, u8).ok_or("Invalid format")?;
        Ok(Self {
            id,
            position,
            score: 0,
        })
    }
}

impl Player {
    pub fn roll_and_step(&mut self, dice: &mut Dice) -> bool {
        let amount = dice.roll_three_times();
        println!("Player {} rolled {}", self.id, amount);
        self.step(amount);
        println!(
            "Player {} moves to space {} for a total score of {}",
            self.id, self.position, self.score
        );
        self.has_won()
    }

    fn has_won(&self) -> bool {
        self.score >= 1000
    }

    fn step(&mut self, amount: u32) {
        let new_position = ((self.position as u32 - 1 + amount) % 10 + 1) as u8;
        self.position = new_position;
        self.increase_score(new_position);
    }

    fn increase_score(&mut self, value: u8) {
        self.score += value as u32;
    }
}

struct Dice {
    next: u8,
    counter: u32,
}

impl Dice {
    pub fn new() -> Self {
        Self {
            next: 1,
            counter: 0,
        }
    }

    pub fn roll(&mut self) -> u32 {
        self.counter += 1;

        let next = self.next;
        self.next += 1;
        if self.next > 100 {
            self.next = 1;
        }
        next as u32
    }

    pub fn roll_three_times(&mut self) -> u32 {
        // (0..2).fold(0, |acc, _| acc + self.roll())
        let first = self.roll();
        let second = self.roll();
        let third = self.roll();
        let sum = first + second + third;
        println!("Rolled {}+{}+{}={}", first, second, third, sum);
        sum
    }
}

struct Game {
    player_1: Player,
    player_2: Player,
    dice: Dice,
}

impl Game {
    pub fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let player_1 = Player::from_str(lines.next().unwrap()).unwrap();
        let player_2 = Player::from_str(lines.next().unwrap()).unwrap();

        Self {
            player_1,
            player_2,
            dice: Dice::new(),
        }
    }

    pub fn play(&mut self) -> u32 {
        loop {
            let has_player_1_won = self.player_1.roll_and_step(&mut self.dice);
            if has_player_1_won {
                return self.calculate_required_value(&self.player_2);
            }
            let has_player_2_won = self.player_2.roll_and_step(&mut self.dice);
            if has_player_2_won {
                return self.calculate_required_value(&self.player_1);
            }
        }
    }

    fn calculate_required_value(&self, losing_player: &Player) -> u32 {
        println!(
            "Score of losing player: {}, dice counter: {}",
            losing_player.score, self.dice.counter
        );
        losing_player.score * self.dice.counter
    }
}

fn main() {
    let input = read_file_to_string("input/day21.txt");

    let mut game = Game::parse(&input);
    let result = game.play();

    dbg!(result);
}
