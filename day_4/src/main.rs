fn main() {
    println!("Part 1: {}", get_num_passphrases(include_str!("../input/input.txt")));
}

fn get_num_passphrases(passphrases: &str) -> usize {
    passphrases.lines().filter(|passphrase| {
        is_valid_passphrase(passphrase)
    }).collect::<Vec<&str>>().len()
}

fn is_valid_passphrase(passphrase: &str) -> bool {
    let mut passphrase_vec : Vec<&str> = passphrase.split_whitespace().collect();
    let num_vals = passphrase_vec.len();
    
    passphrase_vec.sort();
    passphrase_vec.dedup();

    num_vals == passphrase_vec.len()
}

#[test]
fn test_1() {
    assert_eq!(is_valid_passphrase("aa bb cc dd ee"), true);
    assert_eq!(is_valid_passphrase("aa bb cc dd aa"), false);
    assert_eq!(is_valid_passphrase("aa bb cc dd aaa"), true);
}