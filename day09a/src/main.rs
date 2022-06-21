use itertools::Itertools;
fn main() {
    let test_input = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";
    assert_eq!(solution(test_input), 15);
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

    let low_points: Vec<i32> = (0..height)
        .cartesian_product(0..width)
        .filter(|(x, y)| {
            get_neighbour_coords(x, y, &width, &height)
                .iter()
                .all(|(i, j)| grid[*x as usize][*y as usize] < grid[*i as usize][*j as usize])
        })
        .map(|(x, y)| grid[x as usize][y as usize])
        .collect();
    low_points.iter().map(|p| p + 1).sum()
}

fn get_neighbour_coords(x: &i32, y: &i32, width: &i32, height: &i32) -> Vec<(i32, i32)> {
    let delta: Vec<i32> = vec![-1, 0, 1];
    delta
        .iter()
        .cartesian_product(delta.iter())
        .filter(|(dx, dy)| (**dx != dy.abs()))
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|(i, j)| (0 <= *i) & (*i < *height) & (0 <= *j) & (*j < *width))
        .collect()
}
