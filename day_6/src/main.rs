use std::collections::HashSet;

fn main() {
    let banks_vec : Vec<u32> = include_str!("../input/input.txt").split_whitespace()
                                                                    .filter_map(|x| {
                                                                        x.parse().ok()
                                                                    })
                                                                    .collect();
    println!("Part 1: {}", num_cycles(banks_vec).0);

    let banks_vec_2 : Vec<u32> = include_str!("../input/input.txt").split_whitespace()
                                                                    .filter_map(|x| {
                                                                        x.parse().ok()
                                                                    })
                                                                    .collect();
    println!("Part 2: {}", num_cycles(num_cycles(banks_vec_2).1).0)
}

fn redistribute(banks: &Vec<u32>) -> Vec<u32> {
    let mut max_val = *banks.iter().max().unwrap();
    let mut curr_idx = banks.iter().position(|&x| x == max_val).unwrap();

    let mut next_banks = banks.clone();
    
    next_banks[curr_idx] = 0;
    while max_val > 0 {
        curr_idx = (curr_idx + 1) % next_banks.len();
        next_banks[curr_idx] += 1;
        max_val -= 1;    
    }

    next_banks
}

fn hash(banks: &Vec<u32>) -> String {
    banks.iter().fold(String::new(), |acc, val| {
        format!("{}{}", acc, val.to_string())
    })
}

fn num_cycles(mut banks_vec: Vec<u32>) -> (u32, Vec<u32>) {
    let mut banks = HashSet::new();

    banks.insert(hash(&banks_vec));

    let mut seen = false;
    let mut cycles = 0;

    while !seen {
        cycles += 1;        
        banks_vec = redistribute(&banks_vec);
        seen = !banks.insert(hash(&banks_vec));
    }

    (cycles, banks_vec.clone())
}

#[test]
fn test_1() {
    let input_str = "0  2   7   0";
    let banks_vec : Vec<u32> = input_str.split_whitespace().filter_map(|x| x.parse().ok()).collect();
    
    assert_eq!(num_cycles(banks_vec).0, 5);
}

#[test]
fn test_2() {
    let input_str = "0  2   7   0";
    let banks_vec : Vec<u32> = input_str.split_whitespace().filter_map(|x| x.parse().ok()).collect();

    assert_eq!(num_cycles(num_cycles(banks_vec).1).0, 4);
}