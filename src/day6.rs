use std::{collections::HashSet, fs};

fn solve(n: usize) -> usize {
    let input = fs::read_to_string("data/day6_input.txt").unwrap();
    input
        .as_bytes()
        .windows(n)
        .take_while(|w| HashSet::<&u8>::from_iter(w.into_iter()).len() < n)
        .count()
        + n
}

#[test]
fn solve_part1() {
    println!("Solution: {}", solve(4))
}

#[test]
fn solve_part2() {
    println!("Solution: {}", solve(14))
}
