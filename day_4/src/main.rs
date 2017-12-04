#![feature(test)]

extern crate test;
extern crate rayon;

use rayon::prelude::*;

fn main() {
    println!("Part 1: {}", get_num_passphrases(include_str!("../input/input.txt"), false));
    println!("Part 2: {}", get_num_passphrases(include_str!("../input/input.txt"), true));
}

fn get_num_passphrases(passphrases: &str, with_anagrams: bool) -> usize {
    passphrases.par_lines().filter(|passphrase| {
        match with_anagrams {
            true => is_valid_passphrase_with_anagram(passphrase),
            false => is_valid_passphrase(passphrase)
        }
    }).count()
}

fn is_valid_passphrase(passphrase: &str) -> bool {
    let mut passphrase_vec : Vec<&str> = passphrase.split_whitespace().collect();
    let num_vals = passphrase_vec.len();
    
    passphrase_vec.sort();
    passphrase_vec.dedup();

    num_vals == passphrase_vec.len()
}

fn is_valid_passphrase_with_anagram(passphrase: &str) -> bool {
    let passphrase_vec : Vec<&str> = passphrase.split_whitespace().collect();
    let num_vals = passphrase_vec.len();
    
    let mut sorted_passphrase_vec = passphrase_vec.into_iter().map(|x| {
        let mut chars : Vec<char> = x.chars().collect();
        chars.sort();
        chars.into_iter().collect::<String>()
    }).collect::<Vec<String>>();

    sorted_passphrase_vec.sort();
    sorted_passphrase_vec.dedup();

    num_vals == sorted_passphrase_vec.len()
}

#[test]
fn test_1() {
    assert_eq!(is_valid_passphrase("aa bb cc dd ee"), true);
    assert_eq!(is_valid_passphrase("aa bb cc dd aa"), false);
    assert_eq!(is_valid_passphrase("aa bb cc dd aaa"), true);
}

#[test]
fn test_2() {
    assert_eq!(is_valid_passphrase_with_anagram("abcde fghij"), true);
    assert_eq!(is_valid_passphrase_with_anagram("abcde xyz ecdab"), false);
    assert_eq!(is_valid_passphrase_with_anagram("a ab abc abd abf abj"), true);
    assert_eq!(is_valid_passphrase_with_anagram("iiii oiii ooii oooi oooo"), true);
    assert_eq!(is_valid_passphrase_with_anagram("oiii ioii iioi iiio"), false);
}

#[bench]
fn bench_part_1(b: &mut test::Bencher) {
    let input_str = include_str!("../input/input.txt");
    b.iter(|| {
        get_num_passphrases(input_str, false);
    });
}

#[bench]
fn bench_part_2(b: &mut test::Bencher) {
    let input_str = include_str!("../input/input.txt");
    b.iter(|| {
        get_num_passphrases(input_str, true);
    });
}