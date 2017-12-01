fn sum_equal_digits(digits: &Vec<u32>, step: usize) -> u32 {
    digits.iter().enumerate().filter_map(|(idx, x)| {
        match *x == digits[(idx + step) % digits.len()] {
            true => Some(x),
            false => None
        }
    }).sum()
}

fn part_1_solve(input_str: &str) -> u32 {
    let digits = input_str.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
    sum_equal_digits(&digits, 1)
}

fn part_2_solve(input_str: &str) -> u32 {
    let digits = input_str.chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
    sum_equal_digits(&digits, digits.len()/2)
}

fn main() {
    println!("Part 1: {}", part_1_solve(include_str!("../input/input.txt")));
    println!("Part 2: {}", part_2_solve(include_str!("../input/input.txt")));
}

#[test]
fn test1() {
    assert_eq!(part_1_solve("1122"), 3);
    assert_eq!(part_1_solve("1111"), 4);
    assert_eq!(part_1_solve("1234"), 0);
    assert_eq!(part_1_solve("91212129"), 9);
}

#[test]
fn test2() {
    assert_eq!(part_2_solve("1212"), 6);
    assert_eq!(part_2_solve("1221"), 0);
    assert_eq!(part_2_solve("123425"), 4);
    assert_eq!(part_2_solve("123123"), 12);
    assert_eq!(part_2_solve("12131415"), 4);
}