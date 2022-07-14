use itertools::Itertools;

fn main() {
    let input = include_str!("inputs.txt");
    let (points_str, fold_instructions_str) = input.split_once("\n\n").unwrap();
    let fold_instructions: Vec<_> = fold_instructions_str
        .trim()
        .lines()
        .map(|line| {
            let coord: usize = line.trim()[13..].parse().unwrap();
            match line.trim().chars().nth(11).unwrap() {
                'y' => FoldInstruction::Y(coord),
                'x' => FoldInstruction::X(coord),
                _ => unreachable!(),
            }
        })
        .collect();
    let mut points: Vec<_> = points_str
        .trim()
        .lines()
        .map(|line| line.trim().split_once(",").unwrap())
        .map(|(x, y)| Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
        .collect();

    plot(&points);

    for fold_instruction in fold_instructions {
        points = points
            .iter()
            .filter(|point| match fold_instruction {
                FoldInstruction::X(axis) => point.x != axis,
                FoldInstruction::Y(axis) => point.y != axis,
            })
            .map(|point| point.fold(&fold_instruction))
            .unique()
            .collect();

        plot(&points);
        println!("{}", points.len());
    }
}

fn plot(points: &Vec<Point>) {
    let width = points.iter().map(|p| p.x).max().unwrap() + 1;
    let height = points.iter().map(|p| p.y).max().unwrap() + 1;
    let line = vec!["."; width];
    let mut lines = vec![line; height];
    for p in points {
        lines[p.y][p.x] = "#";
    }
    let yaya = lines.iter().map(|line| line.join("")).join("\n");
    println!("{}", yaya);
    println!("");
}

#[derive(Debug)]
enum FoldInstruction {
    Y(usize),
    X(usize),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn fold(&self, fold_instruction: &FoldInstruction) -> Point {
        match fold_instruction {
            FoldInstruction::X(axis) => {
                if self.x > *axis {
                    Self {
                        x: Self::_fold(self.x, *axis),
                        y: self.y,
                    }
                } else {
                    self.clone()
                }
            }
            FoldInstruction::Y(axis) => {
                if self.y > *axis {
                    Self {
                        x: self.x,
                        y: Self::_fold(self.y, *axis),
                    }
                } else {
                    self.clone()
                }
            }
        }
    }

    fn _fold(coord: usize, axis: usize) -> usize {
        axis - axis.abs_diff(coord)
    }
}
