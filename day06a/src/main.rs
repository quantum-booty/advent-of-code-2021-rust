fn solution(input: &str) -> usize {
    let initial_state: Vec<_> = input
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let final_count = (0..80)
        .scan(initial_state, |fishes, _| {
            let mut new_fishes = Vec::new();
            for f in fishes.iter_mut() {
                match f {
                    0 => {
                        *f = 6;
                        new_fishes.push(8);
                    }
                    x => *x -= 1,
                }
            }
            fishes.append(&mut new_fishes);
            Some(fishes.clone())
        })
        .last()
        .unwrap()
        .len();

    final_count
}

fn main() {
    let test_input = "3,4,3,1,2";
    assert_eq!(solution(test_input), 5934);
    let input = include_str!("../inputs.txt").trim();
    println!("{}", solution(input));
}
