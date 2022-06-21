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
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|num| num.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let low_points = find_low_points(&grid, &width, &height);
    low_points
        .iter()
        .map(|(x, y)| fill_basin(&grid, x, y))
        .map(|basin| basin.len() as i32)
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn fill_basin(grid: &Vec<Vec<i32>>, x: &i32, y: &i32) -> HashSet<(i32, i32)> {
    let mut basin = HashSet::<(i32, i32)>::new();
    basin.insert((*x, *y));
    fill_basin_rec(grid, &mut basin, x, y);
    plot_basin(grid, &basin);
    basin
}

fn plot_basin(grid: &Vec<Vec<i32>>, basin: &HashSet<(i32, i32)>) {
    let basin_str: String = grid
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, char)| {
                    if basin.contains(&(i as i32, j as i32)) {
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

fn fill_basin_rec(grid: &Vec<Vec<i32>>, basin: &mut HashSet<(i32, i32)>, x: &i32, y: &i32) {
    // keep a set of traversed points
    // from a point, try traverse in the neighbour coordinates
    // if a point is in the set of traversed points
    // else if a point is higher than previous point, add to set of traversed points
    // and recurse from the new point
    if grid[*x as usize][*y as usize] == 9 {
        return;
    }
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    for (i, j) in get_neighbour_coords(x, y, &width, &height) {
        if basin.contains(&(i, j)) {
            continue;
        }
        if (grid[i as usize][j as usize] > grid[*x as usize][*y as usize])
            && (grid[i as usize][j as usize] != 9)
        {
            basin.insert((i, j));
            fill_basin_rec(grid, basin, &i, &j);
        };
    }
}

fn find_low_points(grid: &Vec<Vec<i32>>, width: &i32, height: &i32) -> Vec<(i32, i32)> {
    (0..*height)
        .cartesian_product(0..*width)
        .filter(|(x, y)| {
            get_neighbour_coords(x, y, &width, &height)
                .iter()
                .all(|(i, j)| grid[*x as usize][*y as usize] < grid[*i as usize][*j as usize])
        })
        .collect()
}

fn get_neighbour_coords(x: &i32, y: &i32, width: &i32, height: &i32) -> Vec<(i32, i32)> {
    let delta: Vec<i32> = vec![-1, 0, 1];
    delta
        .iter()
        .cartesian_product(delta.iter())
        .filter(|(dx, dy)| ((dx.abs() == 1) ^ (dy.abs() == 1)))
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|(i, j)| (0 <= *i) && (*i < *height) && (0 <= *j) && (*j < *width))
        .collect()
}
