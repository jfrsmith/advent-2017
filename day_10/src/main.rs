#![feature(inclusive_range_syntax)]

extern crate hex;

fn main() {
    let start_values = (0..=255).zip(0..=255).collect::<Vec<(u8,u8)>>();
    let input = include_str!("../input/input.txt").split(',').filter_map(|x| x.parse::<u8>().ok()).collect::<Vec<u8>>();
    
    let output = transform(&start_values, &input, 0, 0).0;
    println!("Part 1: {}", (output[0].1 as i32) * (output[1].1 as i32));

    let part_2_input = include_str!("../input/input.txt").chars().collect::<Vec<char>>();
    println!("Part 2: {}", hex::encode(dense_hash(&sparse_hash(&start_values, &to_ascii(&part_2_input)))));
}

fn to_ascii(input: &[char]) -> Vec<u8> {
    let suffix : Vec<u8> = vec!(17,31,73,47,23);
    input.iter().map(|x| *x as u8).chain(suffix.into_iter()).collect()
}

fn sparse_hash(list: &[(u8, u8)], input: &[u8]) -> Vec<u8> {
    let mut transform_vec = list.to_vec();
    let mut pos = 0;
    let mut skip = 0;

    for _ in 0..64 {
        (transform_vec, pos, skip) = transform(&transform_vec, input, pos, skip);
    }
    
    transform_vec.iter().map(|x|x.1).collect::<Vec<u8>>()
}

fn dense_hash(sparse_hash: &[u8]) -> Vec<u8> {
    unimplemented!();
}

fn transform(list: &[(u8,u8)], input: &[u8], start_pos: usize, start_skip: usize) -> (Vec<(u8,u8)>, usize, usize) {
    let mut mod_list = list.to_vec();
    let mut position = start_pos;
    let mut skip_size = start_skip;

    for length in input {
        let range = list.iter()
                        .cycle()
                        .skip(position)
                        .take(*length as usize)
                        .map(|&val| val.0);

        let values = mod_list.iter()
                                .cycle()
                                .skip(position)
                                .take(*length as usize)
                                .map(|&val| val.1)
                                .collect::<Vec<u8>>();

        for (index, new_value) in range.zip(values.iter().rev()) {
            mod_list[index as usize].1 = *new_value;
        }

        let skip = *length as usize + skip_size;
        position = (position + skip) % list.len();
        skip_size += 1;
    }

    (
        mod_list.clone(),
        position,
        skip_size
    )
}

#[test]
fn test_1() {
    let input = vec!(3,4,1,5);
    let start_values = (0..5).zip(0..5).collect::<Vec<(u8,u8)>>();
    let output = transform(&start_values, &input, 0, 0).0;

    assert_eq!(output[0].1*output[1].1, 12);
}

#[test]
fn test_ascii_input() {
    assert_eq!(to_ascii(&"1,2,3".chars().collect::<Vec<char>>()), vec!(49,44,50,44,51,17,31,73,47,23));
}

#[test]
fn test_dense_hash() {
    assert_eq!(dense_hash(&vec!(65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22)), vec!(64));
}

#[test]
fn test_hashes() {
    let start_values = (0..=255).zip(0..=255).collect::<Vec<(u8,u8)>>();

    let mut input = "".chars().collect::<Vec<char>>();
    let mut hex_str = hex::encode(dense_hash(&sparse_hash(&start_values, &to_ascii(&input))));   
    assert_eq!(hex_str, "a2582a3a0e66e6e86e3812dcb672a272");

    input = "AoC 2017".chars().collect::<Vec<char>>();
    hex_str = hex::encode(dense_hash(&sparse_hash(&start_values, &to_ascii(&input))));
    assert_eq!(hex_str, "33efeb34ea91902bb2f59c9920caa6cd");

    input = "1,2,3".chars().collect::<Vec<char>>();
    hex_str = hex::encode(dense_hash(&sparse_hash(&start_values, &to_ascii(&input))));
    assert_eq!(hex_str, "3efbe78a8d82f29979031a4aa0b16a9d");

    input = "1,2,4".chars().collect::<Vec<char>>();
    hex_str = hex::encode(dense_hash(&sparse_hash(&start_values, &to_ascii(&input))));
    assert_eq!(hex_str, "63960835bcdc130f0b66d7ff4f6a5a8e"); 
}