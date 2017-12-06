use std::collections::HashSet;

fn main() {
    let mut banks_vec : Vec<u32> = include_str!("../input/input.txt").split_whitespace()
                                                                    .filter_map(|x| {
                                                                        x.parse().ok()
                                                                    })
                                                                    .collect();
    println!("Part 1: {}", num_cycles(&mut banks_vec));
    println!("Part 2: {}", num_cycles(&mut banks_vec))
}

fn redistribute(banks: &mut Vec<u32>) {
    let mut max_val = *banks.iter().max().unwrap();
    let mut curr_idx = banks.iter().position(|&x| x == max_val).unwrap();

    banks[curr_idx] = 0;
    while max_val > 0 {
        curr_idx = (curr_idx + 1) % banks.len();
        banks[curr_idx] += 1;
        max_val -= 1;    
    }
}

fn hash(banks: &Vec<u32>) -> String {
    banks.iter().fold(String::new(), |acc, val| {
        format!("{}{}", acc, val.to_string())
    })
}

fn num_cycles(mut banks_vec: &mut Vec<u32>) -> u32 {
    let mut banks = HashSet::new();

    banks.insert(hash(&banks_vec));

    let mut seen = false;
    let mut cycles = 0;

    while !seen {
        cycles += 1;        
        redistribute(&mut banks_vec);
        seen = !banks.insert(hash(&banks_vec));
    }

    cycles
}

#[test]
fn test() {
    let input_str = "0  2   7   0";
    let mut banks_vec : Vec<u32> = input_str.split_whitespace()
                                            .filter_map(|x| x.parse().ok())
                                            .collect();
    
    assert_eq!(num_cycles(&mut banks_vec), 5);
    assert_eq!(num_cycles(&mut banks_vec), 4);
}