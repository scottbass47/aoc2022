use std::{cmp::Ordering, collections::HashSet, fs};

trait CharPriority {
    fn priority(&self) -> usize;
}

impl CharPriority for char {
    fn priority(&self) -> usize {
        // A-Z
        if matches!(self.cmp(&'a'), Ordering::Less) {
            *self as usize - 'A' as usize + 27
        } else {
            *self as usize - 'a' as usize + 1
        }
    }
}

trait TriplesExt<I: Iterator> {
    fn triples(self) -> Triples<I>;
}

impl<I: Iterator> TriplesExt<I> for I {
    fn triples(self) -> Triples<I> {
        Triples { iter: self }
    }
}

struct Triples<I: Iterator> {
    iter: I,
}

impl<I: Iterator> Triples<I> {
    fn new(iter: I) -> Self {
        Triples { iter }
    }
}

impl<I: Iterator> Iterator for Triples<I> {
    type Item = (I::Item, I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let one = self.iter.next()?;
        let two = self.iter.next()?;
        let three = self.iter.next()?;
        Some((one, two, three))
    }
}

fn solve_part1() -> usize {
    fs::read_to_string("data/day3_input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let half = line.len() / 2;
            let first = &line[..half];
            let second = &line[half..];

            let first_chars: HashSet<char> = first.chars().collect();

            second
                .chars()
                .find(|c| first_chars.contains(c))
                .unwrap()
                .priority()
        })
        .sum()
}

fn solve_part2() -> usize {
    fs::read_to_string("data/day3_input.txt")
        .unwrap()
        .lines()
        .triples()
        .map(|(one, two, three)| {
            let one_chars: HashSet<_> = one.chars().collect();
            let two_chars: HashSet<_> = two.chars().collect();
            let three_chars: HashSet<_> = three.chars().collect();

            one_chars
                .intersection(&two_chars)
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&three_chars)
                .next()
                .unwrap()
                .priority()
        })
        .sum()
}

#[test]
fn run_it() {
    println!("Sum: {}", solve_part1())
}

#[test]
fn solve_2() {
    println!("Sum: {}", solve_part2())
}

#[test]
fn lowercase_a() {
    assert_eq!(1, 'a'.priority())
}

#[test]
fn lowercase_z() {
    assert_eq!(26, 'z'.priority())
}

#[test]
fn uppercase_a() {
    assert_eq!(27, 'A'.priority())
}
