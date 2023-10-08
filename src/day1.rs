use std::fs;

pub fn solve(n: usize) -> u32 {
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
