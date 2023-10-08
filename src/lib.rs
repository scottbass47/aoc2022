#![allow(dead_code)]

mod day1;
mod day2;
mod day3;

#[test]
fn test_solve_day1() {
    let top_elf = day1::solve(1);
    let top_three_elves = day1::solve(3);
    println!(
        "Solution: Top Elf = {}, Top 3 Elves = {}",
        top_elf, top_three_elves
    )
}
