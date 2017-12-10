use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", find_root_name(include_str!("../input/input.txt")));
    println!("Part 2: {}", get_weight_val(include_str!("../input/input.txt")));
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

type ProgramTree<'a> = HashMap<&'a str,(u32,Vec<&'a str>,Option<u32>)>;

fn build_tree(programs: &str) -> ProgramTree {
    programs.lines().map(|program| {
        let split = program.split_whitespace().collect::<Vec<&str>>();
        (
            split[0].clone(), 
            (
                split[1].trim_matches(|c| c == '(' || c == ')').parse::<u32>().unwrap(),
                split.iter().skip(3).map(|child| child.trim_right_matches(",")).collect::<Vec<&str>>(),
                None
            )
        )
    }).collect()
}

fn get_node_weight(node_name: &str, tree: &ProgramTree) -> u32 {
    let node = tree.get(node_name).unwrap();
    node.2.unwrap_or(node.1.iter().fold(node.0, |acc, child| acc + get_node_weight(child, tree)))
}

fn find_imbalance(root: &str, tree: &ProgramTree) -> Option<u32> {
    let node = tree.get(root).unwrap();
    let child_weights = node.1.iter().map(|child| (child.clone(), get_node_weight(child, tree))).collect::<Vec<(&str,u32)>>();
    
    let is_unbalanced = child_weights.len() > 0 && child_weights.iter().any(|&(_,w)| w != child_weights[0].1);
    
    if is_unbalanced {
        let mut unbalanced_child_name = "";
        let mut bad_weight = 0;
        
        for &(name,weight) in &child_weights {
            if child_weights.iter().filter(|&&(_,w)| w == weight).count() == 1 {
                unbalanced_child_name = name;
                bad_weight = weight;

                let child_imbalance = find_imbalance(&unbalanced_child_name, tree);
                if child_imbalance.is_some() {
                    return child_imbalance;
                }

                break;
            }
        }

        let good_weight = child_weights.iter().find(|&&(_,w)| w != bad_weight).unwrap().1;
        let diff = bad_weight as i32 - good_weight as i32;
        let unbalanced_child_weight = tree.get(unbalanced_child_name).unwrap().0;

        if diff > 0 {
            return Some(unbalanced_child_weight - diff as u32);
        } else {
            return Some(unbalanced_child_weight + diff.abs() as u32);
        }
    }

    None
}

fn get_weight_val(programs: &str) -> u32 {
    let tree = build_tree(programs);
    let weight_tree = tree.iter().map(|(key,val)| {
        (
            key.clone(),
            (
                val.0,
                val.1.clone(),
                Some(get_node_weight(key, &tree))
            )
        )
    }).collect::<ProgramTree>();
    
    find_imbalance(find_root_name(programs), &weight_tree).unwrap()
}

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

#[test]
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
}