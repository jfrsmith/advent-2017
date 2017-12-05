#![feature(test)]

extern crate test;

fn main() {
    println!("Part 1: {}", get_exit_distance(include_str!("../input/input.txt"), None));
    println!("Part 2: {}", get_exit_distance(include_str!("../input/input.txt"), Some(3)));
}

fn get_exit_distance(jump_offsets: &str, decrease_diff: Option<i32>) -> u32 {
    let mut offsets_vec = jump_offsets.lines().map(|x| {
        x.parse::<i32>().unwrap()
    }).collect::<Vec<i32>>();

    let mut pos : i32 = 0;
    let mut jumps : u32 = 0;
    while pos >= 0 && (pos as usize) < offsets_vec.len() {
        let offset = offsets_vec[pos as usize];
        let new_pos = pos + offset;

        offsets_vec[pos as usize] = match decrease_diff {
            Some(x) => if offset >= x {
                offset - 1
            } else {
                offset + 1
            },
            None => offset + 1
        };
        
        pos = new_pos;
        jumps += 1;
    }

    jumps
}

#[test]
fn test_1() {
    let input_str = "0
3
0
1
-3";

    assert_eq!(get_exit_distance(input_str, None), 5);
}

#[test]
fn test_2() {
    let input_str = "0
3
0
1
-3";

    assert_eq!(get_exit_distance(input_str, Some(3)), 10);
}

#[bench]
fn bench_part_1(b: &mut test::Bencher) {
    let input_str = include_str!("../input/input.txt");

    b.iter(|| {
        get_exit_distance(input_str, None);
    });
}

#[bench]
fn bench_part_2(b: &mut test::Bencher) {
     let input_str = include_str!("../input/input.txt");

    b.iter(|| {
        get_exit_distance(input_str, Some(3));
    });
}