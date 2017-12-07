extern crate indextree;

use std::collections::HashMap;
//use indextree::Arena;

fn main() {
    println!("Part 1: {}", find_root_name(include_str!("../input/input.txt")));
    //println!("Part 2: {}", get_weight_val(include_str!("../input/input.txt")));
}

fn find_root_name(programs: &str) -> &str {
    let mut program_counter = HashMap::new();

    for program in programs.lines() {
        let has_children = program.contains("->");
        for child_program in program.split_whitespace()
                                        .enumerate()
                                        .filter_map(|(i, split)| {
                                            if i == 0 || (i > 2 && has_children) {
                                                Some(split.trim_right_matches(","))
                                            } else {
                                                None
                                            }
                                        }) {
            let program_count = program_counter.entry(child_program).or_insert(0);
            *program_count += 1;
        }
    }
    
    program_counter.iter().filter_map(|(key,val)| if *val == 1 { Some(key) } else { None }).nth(0).unwrap()
}

/*struct Program {
    weight: i32,
    children: Vec<&str>
}

fn get_weight_val(programs: &str) -> u32 {
    let mut program_weights = HashMap::new();

    for program in programs.lines() {
        let split = program.split_whitespace().collect::<Vec<&str>>();
        program_weights.insert(split[0], Program {
            weight: split[1].trim_matches(|c| c == '(' || c == ')').parse::<u32>().unwrap(),
            children: match program.contains("->") {
                true => (2..),
                false => vec!()
            }
        });
    }

    let tree = &mut Arena::new();
    tree.new_node(find_root_name(programs));


    0
}*/

#[test]
fn test_1() {
    let input_str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    assert_eq!(find_root_name(input_str), "tknk");
}

/*#[test]
fn test_2() {
    let input_str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    assert_eq!(get_weight_val(input_str), 60);
}*/