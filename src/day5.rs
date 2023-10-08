use std::{
    fs,
    iter::{Skip, StepBy},
    str::{Chars, FromStr},
};

#[derive(Debug)]
struct Board {
    stacks: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Board {
    fn apply_move(&mut self, board_move: &Move, move_together: bool) {
        if move_together {
            let mut substack = Vec::new();
            for _ in 0..board_move.amount {
                substack.insert(0, self.stacks[board_move.from].pop().unwrap());
            }
            self.stacks[board_move.to].append(&mut substack)
        } else {
            for _ in 0..board_move.amount {
                let item = self.stacks[board_move.from].pop().unwrap();
                self.stacks[board_move.to].push(item)
            }
        }
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iters: Vec<_> = s.lines().map(parse_board_line).collect();
        iters.reverse();
        let mut stacks: Vec<_> = iters.remove(0).map(|_| Vec::<char>::new()).collect();

        iters.into_iter().for_each(|line| {
            line.enumerate().for_each(|(i, c)| {
                if !c.is_whitespace() {
                    stacks[i].push(c);
                }
            })
        });

        Ok(Board { stacks })
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        let amount: usize = parts[1].parse().unwrap();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();

        Ok(Move {
            amount,
            from: from - 1,
            to: to - 1,
        })
    }
}

fn parse_board_line(line: &str) -> StepBy<Skip<Chars>> {
    line.chars().skip(1).step_by(4)
}

fn solve(move_together: bool) -> String {
    let input = fs::read_to_string("data/day5_input.txt").unwrap();
    let mut puzzle_parts = input.split("\n\n");

    let configuration = puzzle_parts.next().unwrap();
    let moves = puzzle_parts.next().unwrap();

    let mut board: Board = configuration.parse().unwrap();

    moves
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .for_each(|m| board.apply_move(&m, move_together));

    board
        .stacks
        .into_iter()
        .map(|stack| stack.last().cloned())
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .collect()
}

#[test]
fn solve_part1() {
    println!("Solution: {}", solve(false));
}

#[test]
fn solve_part2() {
    println!("Solution: {}", solve(true));
}
