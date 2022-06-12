#![feature(slice_group_by)]

use std::collections::HashMap;

fn main() {
    let test_input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    let inputs = include_str!("../inputs.txt");
    assert_eq!(solution(test_input), 5);
    println!("{}", solution(inputs));
}

fn solution(input: &str) -> usize {
    let lines: Vec<Line> = input
        .lines()
        .filter_map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            let (x_from, y_from) = from.split_once(',').unwrap();
            let (x_to, y_to) = to.split_once(',').unwrap();
            let x_from = x_from.parse().unwrap();
            let x_to = x_to.parse().unwrap();
            let y_from = y_from.parse().unwrap();
            let y_to = y_to.parse().unwrap();

            if (x_from != x_to) & (y_from != y_to) {
                return None;
            }

            let line = Line {
                from: Point {
                    x: x_from,
                    y: y_from,
                },
                to: Point { x: x_to, y: y_to },
            };
            Some(line)
        })
        .collect();

    let mut cover_map = HashMap::<Point, usize>::new();
    for line in lines {
        for point in line.interpolate() {
            let count = cover_map.entry(point).or_insert(0);
            *count += 1;
        }
    }
    cover_map.iter().filter(|(_, &v)| v >= 2).count()
}

struct Line {
    from: Point,
    to: Point,
}

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Line {
    fn interpolate(&self) -> Vec<Point> {
        if self.from.x == self.to.x {
            let from_y = std::cmp::min(self.from.y, self.to.y);
            let to_y = std::cmp::max(self.from.y, self.to.y);
            return (from_y..to_y + 1)
                .map(|y| Point { x: self.from.x, y })
                .collect();
        };

        if self.from.y == self.to.y {
            let from_x = std::cmp::min(self.from.x, self.to.x);
            let to_x = std::cmp::max(self.from.x, self.to.x);
            return (from_x..to_x + 1)
                .map(|x| Point { x, y: self.from.y })
                .collect();
        };

        vec![Point {
            x: self.from.x,
            y: self.from.y,
        }]
    }
}
