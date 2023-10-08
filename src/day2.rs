use std::{fs, str::FromStr};

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum GameResult {
    Win,
    Loss,
    Tie,
}

impl Hand {
    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn play_game(&self, other: &Hand) -> GameResult {
        match (self, other) {
            (Hand::Rock, Hand::Rock) => GameResult::Tie,
            (Hand::Rock, Hand::Paper) => GameResult::Loss,
            (Hand::Rock, Hand::Scissors) => GameResult::Win,
            (Hand::Paper, Hand::Rock) => GameResult::Win,
            (Hand::Paper, Hand::Paper) => GameResult::Tie,
            (Hand::Paper, Hand::Scissors) => GameResult::Loss,
            (Hand::Scissors, Hand::Rock) => GameResult::Loss,
            (Hand::Scissors, Hand::Paper) => GameResult::Win,
            (Hand::Scissors, Hand::Scissors) => GameResult::Tie,
        }
    }
}

struct Game {
    opp_hand: Hand,
    my_hand: Hand,
}

impl Game {
    fn my_outcome(&self) -> GameResult {
        self.my_hand.play_game(&self.opp_hand)
    }

    fn score(&self) -> usize {
        self.my_outcome().score() + self.my_hand.score()
    }
}

impl GameResult {
    fn score(&self) -> usize {
        match self {
            GameResult::Win => 6,
            GameResult::Loss => 0,
            GameResult::Tie => 3,
        }
    }
}

impl FromStr for GameResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Loss),
            "Y" => Ok(GameResult::Tie),
            "Z" => Ok(GameResult::Win),
            _ => Err(format!(
                "Invalid game result: {}. Must be either X, Y, or Z.",
                s
            )),
        }
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let hand1 = split
            .next()
            .ok_or("Missing first hand".to_owned())
            .and_then(|s| s.parse::<Hand>())?;
        let hand2 = split
            .next()
            .ok_or("Missing second hand".to_owned())
            .and_then(|s| s.parse::<Hand>())?;

        Ok(Game {
            opp_hand: hand1,
            my_hand: hand2,
        })
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(format!("Invalid hand: {}", s)),
        }
    }
}

fn calculate_my_hand(opp_hand: Hand, game_result: &GameResult) -> Hand {
    if matches!(game_result, GameResult::Tie) {
        return opp_hand;
    }

    match (opp_hand, game_result) {
        (Hand::Rock, GameResult::Win) => Hand::Paper,
        (Hand::Rock, GameResult::Loss) => Hand::Scissors,
        (Hand::Paper, GameResult::Win) => Hand::Scissors,
        (Hand::Paper, GameResult::Loss) => Hand::Rock,
        (Hand::Scissors, GameResult::Win) => Hand::Rock,
        (Hand::Scissors, GameResult::Loss) => Hand::Paper,
        _ => panic!("Unreachable"),
    }
}

pub fn solve_part1() -> usize {
    fs::read_to_string("data/day2_input.txt")
        .unwrap()
        .lines()
        .map(|game| game.parse::<Game>().unwrap().score())
        .sum()
}

pub fn solve_part2() -> usize {
    fs::read_to_string("data/day2_input.txt")
        .unwrap()
        .lines()
        .map(|game| {
            let parts: Vec<_> = game.split(' ').collect();
            let opp_hand: Hand = parts[0].parse().unwrap();
            let game_result: GameResult = parts[1].parse().unwrap();
            let my_hand = calculate_my_hand(opp_hand, &game_result);

            my_hand.score() + game_result.score()
        })
        .sum()
}

#[test]
fn run_it() {
    let score = solve_part2();
    println!("Final Score: {}", score)
}
