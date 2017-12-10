fn main() {
    println!("Part 1: {}", parse_stream(include_str!("../input/input.txt")).0);
    println!("Part 2: {}", parse_stream(include_str!("../input/input.txt")).2);
}

enum State {
    Idle,
    Group,
    Garbage,
    Ignore
}

fn parse_stream(stream: &str) -> (i32, i32, i32, State) {
    stream.chars().fold((0, 0, 0, State::Idle), |(score, multiplier, garbage, state), c| {
        match state {
            State::Idle => match c {
                '{' => (score, multiplier + 1, garbage, State::Group),
                '<' => (score, multiplier, garbage, State::Garbage),
                _ => (score, multiplier, garbage, state)
            },
            State::Group => match c {
                '}' => (score + multiplier, multiplier - 1, garbage, if multiplier > 1 { State::Group } else { State:: Idle }),
                '{' => (score, multiplier + 1, garbage, State::Group),
                '<' => (score, multiplier, garbage, State::Garbage),
                _ => (score, multiplier, garbage, state)
            },
            State::Garbage => match c {
                '>' => (score, multiplier, garbage, if multiplier > 0 { State::Group } else { State::Idle }),
                '!' => (score, multiplier, garbage, State::Ignore),
                _ => (score, multiplier, garbage + 1, state)
            },
            State::Ignore => {
                (score, multiplier, garbage, State::Garbage)
            }
        }     
    })
}

#[test]
fn part_1() {
    assert_eq!(parse_stream("{}").0, 1);
    assert_eq!(parse_stream("{{{}}}").0, 6);
    assert_eq!(parse_stream("{{},{}}").0, 5);
    assert_eq!(parse_stream("{{{},{},{{}}}}").0, 16);
    assert_eq!(parse_stream("{<a>,<a>,<a>,<a>}").0, 1);
    assert_eq!(parse_stream("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
    assert_eq!(parse_stream("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
    assert_eq!(parse_stream("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
}

#[test]
fn part_2() {
    assert_eq!(parse_stream("<>").2, 0);
    assert_eq!(parse_stream("<random characters>").2, 17);
    assert_eq!(parse_stream("<<<<>").2, 3);
    assert_eq!(parse_stream("<{!>}>").2, 2);
    assert_eq!(parse_stream("<!!>").2, 0);
    assert_eq!(parse_stream("<!!!>>").2, 0);
    assert_eq!(parse_stream("<{o\"i!a,<{i<a>").2, 10);
}