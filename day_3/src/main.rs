#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug)]
struct Cursor {
    position: (i32,i32),
    facing: Direction,

    forward_steps_per_turn: usize,
    forward_steps_till_turn: usize,
    total_turns: usize
}

impl Cursor {
    pub fn step(&mut self) {
        if self.forward_steps_till_turn > 0 {
            self.forward_steps_till_turn -= 1;
        } else {
            self.facing = match self.facing {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South
            };

            self.total_turns += 1;

            if self.total_turns % 2 == 0 {
                self.forward_steps_per_turn += 1;
            }
            
            self.forward_steps_till_turn = self.forward_steps_per_turn;
        }

        self.position = match self.facing {
            Direction::North    => (self.position.0, self.position.1+1),
            Direction::East     => (self.position.0+1, self.position.1),
            Direction::South    => (self.position.0, self.position.1-1),
            Direction::West     => (self.position.0-1, self.position.1)
        };
    }
}

fn run_spiral(max: usize) -> Cursor {
    let mut cursor = Cursor {
        position: (0,0),
        facing: Direction::East,
        forward_steps_per_turn: 0,
        forward_steps_till_turn: 1,
        total_turns: 0
    };

    for x in 1..max {
        cursor.step();
    }

    cursor
}

fn get_spiral_distance(max_val: usize) -> i32 {
    let end_cursor = run_spiral(max_val);
    end_cursor.position.0.abs() + end_cursor.position.1.abs()
}

fn main() {
    println!("Part 1: {}", get_spiral_distance(368078));
}

#[test]
fn test_1() {
    assert_eq!(get_spiral_distance(1), 0);
    assert_eq!(get_spiral_distance(12), 3);
    assert_eq!(get_spiral_distance(23), 2);
    assert_eq!(get_spiral_distance(1024), 31);
}