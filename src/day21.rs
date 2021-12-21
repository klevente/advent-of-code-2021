use advent_of_code_2021::read_file_to_string;
use itertools::Itertools;
use sscanf::scanf;
use std::collections::HashMap;
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

struct QuantumGame {
    // p1: player 1, p2: player 2
    // s: score, p: position
    //               p1s p1p p2s p2p
    states: HashMap<(u8, u8, u8, u8), u64>,
    player_1_won: u64,
    player_2_won: u64,
}

impl QuantumGame {
    pub fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        let player_1_pos = Player::from_str(lines.next().unwrap()).unwrap().position;
        let player_2_pos = Player::from_str(lines.next().unwrap()).unwrap().position;

        let mut states = Self::generate_empty_state_map();
        states.insert((0, player_1_pos, 0, player_2_pos), 1);

        QuantumGame {
            states,
            player_1_won: 0,
            player_2_won: 0,
        }
    }

    pub fn play(&mut self) -> u64 {
        let mut is_player_1_turn = true;
        while self.is_not_finished() {
            self.step(is_player_1_turn);
            is_player_1_turn = !is_player_1_turn;
        }

        self.player_1_won.max(self.player_2_won)
    }

    fn is_not_finished(&self) -> bool {
        self.states.iter().any(|(_, v)| *v > 0)
    }

    fn step(&mut self, is_player_1_turn: bool) {
        let mut new_states = Self::generate_empty_state_map();

        let rolls = Self::generate_dice_rolls();

        for ((p1s, p1p, p2s, p2p), n) in &self.states {
            if *n == 0 {
                continue;
            }
            for roll in &rolls {
                if is_player_1_turn {
                    let (new_pos, new_score) =
                        Self::calculate_new_position_and_score(p1p, p1s, roll);

                    *new_states
                        .get_mut(&(new_score, new_pos, *p2s, *p2p))
                        .unwrap() += n;
                } else {
                    let (new_pos, new_score) =
                        Self::calculate_new_position_and_score(p2p, p2s, roll);

                    *new_states
                        .get_mut(&(*p1s, *p1p, new_score, new_pos))
                        .unwrap() += n;
                }
            }
        }

        let mut num_of_wins = 0;
        for ((p1s, _p1p, p2s, _p2p), n) in &mut new_states {
            if is_player_1_turn {
                if *p1s == 21 {
                    num_of_wins += *n;
                    *n = 0;
                }
            } else {
                if *p2s == 21 {
                    num_of_wins += *n;
                    *n = 0;
                }
            }
        }

        if is_player_1_turn {
            self.player_1_won += num_of_wins;
        } else {
            self.player_2_won += num_of_wins;
        }

        self.states = new_states;
    }

    fn calculate_new_position_and_score(position: &u8, score: &u8, roll: &u8) -> (u8, u8) {
        let new_position = (position + roll - 1) % 10 + 1;
        let new_score = (score + new_position).min(21);

        (new_position, new_score)
    }

    fn generate_dice_rolls() -> Vec<u8> {
        let range = 1u8..=3;
        range
            .clone()
            .cartesian_product(range.clone())
            .cartesian_product(range)
            .map(|((x, y), z)| x + y + z)
            .sorted()
            .collect::<Vec<_>>()
    }

    fn generate_empty_state_map() -> HashMap<(u8, u8, u8, u8), u64> {
        let player_1_state = (0..=21).into_iter().cartesian_product(1..=10);
        let player_2_state = player_1_state.clone();
        let state_iter = player_1_state
            .cartesian_product(player_2_state)
            .map(|((p1s, p1p), (p2s, p2p))| ((p1s, p1p, p2s, p2p), 0));

        HashMap::from_iter(state_iter)
    }
}

fn main() {
    let input = read_file_to_string("input/day21.txt");

    let mut game = Game::parse(&input);
    let score_of_losing_player_times_num_of_dice_rolls = game.play();
    println!("The product of the losing player's score and number of dice rolls in the practice game is {}", score_of_losing_player_times_num_of_dice_rolls);

    let mut quantum_game = QuantumGame::parse(&input);
    let num_of_wins_for_player_who_wins_more = quantum_game.play();
    println!(
        "The number of wins for the player who wins more in the quantum game is {}",
        num_of_wins_for_player_who_wins_more
    );
}
