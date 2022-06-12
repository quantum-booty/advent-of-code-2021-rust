fn main() {
    let test_input = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(solution(test_input), 37);

    let input = include_str!("../inputs.txt");
    println!("{}", solution(input));
}

fn solution(input: &str) -> i32 {
    let mut nums: Vec<i32> = input
        .trim()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    let m = median(&mut nums);
    nums.iter().map(|n| (n - m).abs()).sum()
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}
