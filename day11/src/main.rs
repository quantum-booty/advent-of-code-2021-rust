use itertools::Itertools;
use std::fmt;

fn main() {
    let test_input = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
        ";
    let input = "\
4585612331
5863566433
6714418611
1746467322
6161775644
6581631662
1247161817
8312615113
6751466142
1161847732
";
    assert_eq!(solution_a(test_input), 1656);
    println!("{}", solution_a(input));

    assert_eq!(solution_b(test_input), 195);
    println!("{}", solution_b(input));
}

fn solution_a(input: &str) -> u32 {
    let mut grid = Grid::new(input);
    let mut flashes = 0;
    for _ in 0..100 {
        step(&mut grid, &mut flashes);
    }
    flashes
}

fn solution_b(input: &str) -> u32 {
    let mut grid = Grid::new(input);
    let mut flashes = 0;
    for i in 1.. {
        step(&mut grid, &mut flashes);
        let sum: u32 = grid.grid.iter().map(|line| line.iter().sum::<u32>()).sum();
        if sum == 0 {
            return i;
        }
    }
    0
}

fn step(grid: &mut Grid, flashes: &mut u32) {
    increment(grid);
    flash(grid, flashes);
}

fn increment(grid: &mut Grid) {
    // increase energy level of each points by 1
    for x in 0..grid.get_width() {
        for y in 0..grid.get_height() {
            *grid.get_val_mut(&Point { x, y }) += 1
        }
    }
}

fn flash(grid: &mut Grid, flashes: &mut u32) {
    // for each point in the grid,
    // if its value is 0, do nothing
    // else increment its value by 1
    // and if higher than 9, set it self to 0
    // and iter on its neighbours
    fn flash_rec(grid: &mut Grid, point: &Point, flashes: &mut u32) {
        let val = grid.get_val_mut(&point);
        if *val > 9 {
            *flashes += 1;
            *val = 0;
            for n in grid.get_neighbour_points(point) {
                let v = grid.get_val_mut(&n);
                if *v > 0 {
                    *v += 1;
                    flash_rec(grid, &n, flashes);
                }
            }
        }
    }

    (0..grid.get_width())
        .cartesian_product(0..grid.get_height())
        .map(|(x, y)| Point { x, y })
        .for_each(|p| {
            if grid.get_val(&p) > 9 {
                flash_rec(grid, &p, flashes)
            }
        })
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

struct Grid {
    grid: Vec<Vec<u32>>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|line| line.iter().map(|c| c.to_string()).collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = input
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        Grid { grid }
    }

    fn get_height(&self) -> i32 {
        self.grid.len() as i32
    }

    fn get_width(&self) -> i32 {
        self.grid[0].len() as i32
    }

    fn get_neighbour_points(&self, point: &Point) -> Vec<Point> {
        let delta: Vec<(i32, i32)> = vec![
            (0, -1),
            (0, 1),
            (1, 0),
            (-1, 0),
            (-1, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
        ];
        delta
            .iter()
            .map(|(dx, dy)| Point {
                x: point.x + dx,
                y: point.y + dy,
            })
            .filter(|p| {
                (0 <= p.x) && (p.x < self.get_height()) && (0 <= p.y) && (p.y < self.get_width())
            })
            .collect()
    }

    fn get_val(&self, point: &Point) -> u32 {
        self.grid[point.x as usize][point.y as usize]
    }

    fn get_val_mut(&mut self, point: &Point) -> &mut u32 {
        &mut self.grid[point.x as usize][point.y as usize]
    }
}
