#![allow(dead_code)]
use std::fs;

fn solve_day1(n: usize) -> u32 {
    let mut elf_calories: Vec<u32> = fs::read_to_string("data/day1_input.txt")
        .unwrap()
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    elf_calories.sort_unstable_by(|a, b| b.cmp(a));

    elf_calories.into_iter().take(n).sum()
}

#[test]
fn test_solve_day1() {
    let top_elf = solve_day1(1);
    let top_three_elves = solve_day1(3);
    println!(
        "Solution: Top Elf = {}, Top 3 Elves = {}",
        top_elf, top_three_elves
    )
}
