use std::fs;

fn solution_a_1(inputs: &str) -> i32 {
    let result = inputs
        .trim()
        .split('\n')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .fold((0, 0), |(acc, prev_element), x| {
            if x > prev_element {
                (acc + 1, x)
            } else {
                (acc, x)
            }
        });
    result.0 - 1
}

fn solution_a_2(inputs: &str) -> i32 {
    inputs
        .trim()
        .split('\n')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|w| if w[1] > w[0] { 1 } else { 0 })
        .sum()
}

fn solution_b_1(inputs: &str) -> i32 {
    // (a + b + c < b + c + d )== (a < d)
    inputs
        .trim()
        .split('\n')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(4)
        .map(|w| if w[3] > w[0] { 1 } else { 0 })
        .sum()
}

fn main() {
    let test_inputs = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263";
    assert_eq!(solution_a_1(test_inputs), 7);
    assert_eq!(solution_b_1(test_inputs), 5);

    let inputs = fs::read_to_string("inputs.txt").unwrap();
    println!("{}", solution_a_1(&inputs));
    println!("{}", solution_a_2(&inputs));
    println!("{}", solution_b_1(&inputs));
}
