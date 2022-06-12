fn solution(input: &str, days: usize) -> usize {
    let mut fish_counts = vec![0; 9];
    for f in input.split(',').map(|n| n.parse::<usize>().unwrap()) {
        fish_counts[f] += 1;
    }

    (0..days)
        .for_each(|_| {
            let zeroes = fish_counts[0];
            fish_counts.rotate_left(1);
            fish_counts[6] += zeroes;
        });

    fish_counts.iter().sum()
}

fn main() {
    let test_input = "3,4,3,1,2";
    assert_eq!(solution(test_input, 80), 5934);
    assert_eq!(solution(test_input, 256), 26984457539);
    let input = include_str!("../inputs.txt").trim();
    println!("{}", solution(input, 80));
    println!("{}", solution(input, 256));
}
