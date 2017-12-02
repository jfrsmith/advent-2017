extern crate itertools;

use itertools::Itertools;
use itertools::MinMaxResult::{MinMax};

fn part_1_solve(input_str: &str) -> u32 {
    input_str.lines().map(|line| {
        line.split_whitespace().map(|x| {
            x.parse::<u32>().unwrap()
        })
    }).map(|digits| {
        match digits.minmax() {
            MinMax(min, max) => max - min,
            _ => 0
        }
    }).sum()
}

fn part_2_solve(input_str: &str) -> u32 {
    input_str.lines().map(|line| {
        line.split_whitespace().map(|x| {
            x.parse::<u32>().unwrap()
        })
    }).map(|digits| {
        digits.combinations(2).find(|ref x| {
            x[0] % x[1] == 0 || x[1] % x[0] == 0
        }).map_or(0, |found| {
            if found[0] > found[1] { found[0] / found[1] } else { found[1] / found[0] }
        })
    }).sum()
}

fn main() {
    println!("Part 1: {}", part_1_solve(include_str!("../input/input.txt")));
    println!("Part 2: {}", part_2_solve(include_str!("../input/input.txt")));
}

#[test]
fn test_1() {
    let input_str = "5 1 9 5
7 5 3
2 4 6 8";
    assert_eq!(part_1_solve(input_str), 18);
}

#[test]
fn test_2() {
    let input_str = "5 9 2 8
9 4 7 3
3 8 6 5";

    assert_eq!(part_2_solve(input_str), 9);
}