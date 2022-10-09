use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let test_input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    let test_answer = solution(test_input);
    println!("{}", test_answer);

    let input = include_str!("inputs.txt");
    let answer = solution(input);
    println!("{}", answer);
}

fn solution(input: &str) -> usize {
    let mut polymer = input.lines().next().unwrap().to_owned();
    let mut lines = input.lines();
    lines.next();
    lines.next();
    let insertion_rules: HashMap<String, char> = lines
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(a, b)| (a.to_owned(), b.chars().next().unwrap()))
        .collect();
    for _ in 0..10 {
        polymer = polymer
            .chars()
            .into_iter()
            .tuple_windows()
            .map(|(a, b)| {
                let pair = format!("{a}{b}");
                let new = insertion_rules.get(&pair).copied();
                (a, b, new)
            })
            .fold(String::new(), |mut acc, (a, b, between)| {
                acc.pop();
                acc.push(a);
                acc.push(between.unwrap_or_default());
                acc.push(b);
                acc
            });
    }
    let counts = polymer.chars().counts();
    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    max - min
}
