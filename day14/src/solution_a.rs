use itertools::Itertools;
use std::collections::HashMap;

pub fn solution_a(input: &str, steps: u8) -> usize {
    let mut polymer = input.lines().next().unwrap().to_owned();
    let mut lines = input.lines();
    lines.next();
    lines.next();
    let insertion_rules: HashMap<String, char> = lines
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(a, b)| (a.to_owned(), b.chars().next().unwrap()))
        .collect();
    for _ in 0..steps {
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
