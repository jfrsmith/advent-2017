use std::collections::HashMap;
use std::cmp;

fn main() {
    let result = run_instructions(include_str!("../input/input.txt"));
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}

fn run_instructions(instr: &str) -> (i32, i32) {
    let mut max_val = 0;
    let mut registers : HashMap<&str, i32> = HashMap::new();

    for instruction in instr.lines() {
        let split_instr = instruction.split_whitespace().collect::<Vec<&str>>();

        let adjust_val = if split_instr[1] == "dec" { 
            -1 * split_instr[2].parse::<i32>().unwrap()
        } else { 
            split_instr[2].parse::<i32>().unwrap() 
        };

        let mut adjust = {
            let comp_reg = registers.entry(split_instr[4]).or_insert(0);
            let comp_val = split_instr[6].parse::<i32>().unwrap();
            
            match split_instr[5] {
                ">" => *comp_reg > comp_val,
                "<" => *comp_reg < comp_val,
                ">=" => *comp_reg >= comp_val,
                "<=" => *comp_reg <= comp_val,
                "!=" => *comp_reg != comp_val,
                "==" => *comp_reg == comp_val,
                _ => false
            }
        };

        if adjust {
            *registers.entry(split_instr[0]).or_insert(0) += adjust_val;
        }

        max_val = cmp::max(max_val, *registers.values().max().unwrap());
    }

    (*registers.values().max().unwrap(), max_val)
}

#[test]
fn test_1() {
    let input_str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    assert_eq!(run_instructions(input_str).0, 1);
}

#[test]
fn test_2() {
    let input_str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    assert_eq!(run_instructions(input_str).1, 10);
}