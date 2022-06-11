use std::fs;

struct Ship {
    horizontal_pos: i32,
    depth: i32,
}

impl Ship {
    fn move_ship(&mut self, command: &str, distance: i32) {
        match command {
            "forward" => self.horizontal_pos += distance,
            "up" => self.depth -= distance,
            "down" => self.depth += distance,
            _ => unreachable!(),
        }
    }
}

fn solution_a_1(inputs: &str) -> i32 {
    let mut ship = Ship {
        horizontal_pos: 0,
        depth: 0,
    };
    inputs
        .lines()
        .map(|line| {
            let l = line.split_once(' ').unwrap();
            (l.0, l.1.parse::<i32>().unwrap())
        })
        .for_each(|(command, distance)| ship.move_ship(command, distance));
    ship.horizontal_pos * ship.depth
}

fn main() {
    let test_inputs = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    assert_eq!(solution_a_1(test_inputs), 150);

    let inputs = fs::read_to_string("inputs.txt").unwrap();
    println!("{}", solution_a_1(&inputs));
}
