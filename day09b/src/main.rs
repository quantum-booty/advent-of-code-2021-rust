use std::collections::HashSet;

use itertools::Itertools;
fn main() {
    let test_input = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";
    // solution(test_input);
    assert_eq!(solution(test_input), 1134);
    println!("{}", solution(include_str!("../inputs.txt")));
}

fn solution(input: &str) -> i32 {
    let height_map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|num| num.to_digit(10).unwrap()).collect())
        .collect();
    let grid = Grid { height_map };
    let low_points = grid.find_low_points();
    low_points
        .iter()
        .map(|p| fill_basin(&grid, *p))
        .map(|basin| basin.len() as i32)
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn fill_basin(grid: &Grid, point: Point) -> HashSet<Point> {
    let mut basin = HashSet::<Point>::new();
    basin.insert(point);
    fill_basin_rec(grid, &mut basin, point);
    // plot_basin(grid, &basin);
    basin
}

fn plot_basin(grid: &Grid, basin: &HashSet<Point>) {
    let basin_str: String = grid
        .height_map
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, char)| {
                    if basin.contains(&Point {
                        x: i as i32,
                        y: j as i32,
                    }) {
                        String::from("*")
                    } else {
                        char.to_string()
                    }
                })
                .collect::<String>()
        })
        .join("\n");
    println!("{}", basin_str);
}

fn fill_basin_rec(grid: &Grid, basin: &mut HashSet<Point>, point: Point) {
    // keep a set of traversed points
    // from a point, try traverse in the neighbour coordinates
    // if a point is in the set of traversed points
    // else if a point is higher than previous point, add to set of traversed points
    // and recurse from the new point
    if grid.get_height(&point) == 9 {
        return;
    }
    for p in grid.get_neighbour_points(&point) {
        if basin.contains(&p) {
            continue;
        }
        if (grid.get_height(&p) > grid.get_height(&point)) && (grid.get_height(&p) != 9) {
            basin.insert(p);
            fill_basin_rec(grid, basin, p);
        };
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Grid {
    height_map: Vec<Vec<u32>>,
}

impl Grid {
    fn grid_height(&self) -> i32 {
        self.height_map.len() as i32
    }

    fn grid_width(&self) -> i32 {
        self.height_map[0].len() as i32
    }

    fn get_neighbour_points(&self, point: &Point) -> Vec<Point> {
        let delta: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (1, 0), (-1, 0)];
        delta
            .iter()
            .map(|(dx, dy)| Point {
                x: point.x + dx,
                y: point.y + dy,
            })
            .filter(|p| {
                (0 <= p.x) && (p.x < self.grid_height()) && (0 <= p.y) && (p.y < self.grid_width())
            })
            .collect()
    }

    fn get_height(&self, point: &Point) -> u32 {
        self.height_map[point.x as usize][point.y as usize]
    }

    fn find_low_points(&self) -> Vec<Point> {
        (0..self.grid_height())
            .cartesian_product(0..self.grid_width())
            .map(|(x, y)| Point { x, y })
            .filter(|p| {
                self.get_neighbour_points(p)
                    .iter()
                    .all(|neighbour| self.get_height(p) < self.get_height(neighbour))
            })
            .collect()
    }
}
