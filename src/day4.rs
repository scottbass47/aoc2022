use std::{fs, num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Range {
    lower: isize,
    upper: isize,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }
    fn overlaps(&self, other: &Range) -> bool {
        (self.lower >= other.lower && self.lower <= other.upper)
            || (self.upper >= other.lower && self.upper <= other.upper)
            || (other.lower >= self.lower && other.lower <= self.upper)
            || (other.upper >= self.lower && other.upper <= self.upper)
    }
}

#[derive(Debug)]
struct ParseRangeError;

impl From<ParseIntError> for ParseRangeError {
    fn from(_: ParseIntError) -> Self {
        ParseRangeError
    }
}

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(ParseRangeError);
        }

        let lower = parts[0].parse::<isize>()?;
        let upper = parts[1].parse::<isize>()?;

        Ok(Range { lower, upper })
    }
}

fn solve<F>(filter: F) -> usize
where
    F: Fn(&Range, &Range) -> bool,
{
    fs::read_to_string("data/day4_input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let range1: Range = parts.next().unwrap().parse().unwrap();
            let range2: Range = parts.next().unwrap().parse().unwrap();
            (range1, range2)
        })
        .filter(|(r1, r2)| filter(r1, r2))
        .count()
}

#[test]
fn solve_part1() {
    let total = solve(|r1, r2| r1.contains(r2) || r2.contains(r1));
    println!("Solution: {}", total)
}

#[test]
fn solve_part2() {
    let total = solve(|r1, r2| r1.overlaps(r2));
    println!("Solution: {}", total)
}

#[test]
fn overlaps_none() {
    let one = Range { lower: 0, upper: 1 };
    let two = Range { lower: 2, upper: 3 };
    assert!(!one.overlaps(&two))
}

#[test]
fn overlaps_lower() {
    let one = Range {
        lower: 0,
        upper: 10,
    };
    let two = Range {
        lower: 5,
        upper: 11,
    };
    assert!(one.overlaps(&two))
}

#[test]
fn overlaps_upper() {
    let one = Range {
        lower: 0,
        upper: 10,
    };
    let two = Range {
        lower: -10,
        upper: 1,
    };
    assert!(one.overlaps(&two))
}

#[test]
fn overlaps_contains() {
    let one = Range {
        lower: 0,
        upper: 10,
    };
    let two = Range { lower: 5, upper: 7 };
    assert!(one.overlaps(&two))
}

#[test]
fn overlaps_contains2() {
    let one = Range { lower: 5, upper: 7 };
    let two = Range {
        lower: 0,
        upper: 10,
    };
    assert!(one.overlaps(&two))
}
