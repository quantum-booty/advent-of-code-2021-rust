use std::fs;

struct Ship {
    horizontal_pos: i32,
    depth: i32,
    aim: i32,
}

impl Ship {
    fn move_ship(&mut self, command: &str, units: i32) {
        match command {
            "forward" => {
                self.horizontal_pos += units;
                self.depth += self.aim * units;
            }
            "up" => self.aim -= units,
            "down" => self.aim += units,
            _ => unreachable!(),
        }
    }
}

fn solution_b_1(inputs: &str) -> i32 {
    let mut ship = Ship {
        horizontal_pos: 0,
        depth: 0,
        aim: 0,
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
    assert_eq!(solution_b_1(test_inputs), 900);

    let inputs = fs::read_to_string("inputs.txt").unwrap();
    println!("{}", solution_b_1(&inputs));
}
