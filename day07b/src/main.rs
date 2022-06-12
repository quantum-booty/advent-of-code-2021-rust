fn main() {
    let test_input = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(solution(test_input), 168);

    let input = include_str!("../inputs.txt");
    println!("{}", solution(input));
}

fn solution(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    let m = mean(&nums);
    (m..)
        .map(|m| {
            nums.iter()
                .map(|num| {
                    let n = (num - m).abs() + 1;
                    n * (n - 1)
                })
                .sum::<i32>()
                / 2
        })
        .take(2)
        .min()
        .unwrap()
}

fn mean(numbers: &[i32]) -> i32 {
    numbers.iter().sum::<i32>() / numbers.len() as i32
}
