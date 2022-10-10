use itertools::Itertools;
use std::collections::HashMap;

pub fn solution_b(input: &str, steps: u8) -> usize {
    let mut lines = input.lines();
    let polymer = lines.next().unwrap().chars().collect_vec();
    lines.next();

    let insertion_rules: HashMap<(char, char), char> = lines
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(a, b)| {
            let pair: (char, char) = a.chars().tuple_windows().next().unwrap();
            (pair, b.chars().next().unwrap())
        })
        .collect();

    let mut pairs: HashMap<(char, char), _> = polymer.iter().copied().tuple_windows().counts();
    let mut counts: HashMap<char, _> = polymer.iter().copied().counts();

    for _ in 0..steps {
        let keys = pairs.keys().cloned().collect_vec();
        let mut new_pairs = HashMap::<(char, char), _>::new();
        for pair in keys.iter() {
            let middle = insertion_rules[pair];
            let (left, right) = pair;

            let pair_count = pairs.get_mut(pair).unwrap();
            let old_pair_count = *pair_count;
            *pair_count = 0;

            counts
                .entry(middle)
                .and_modify(|count| *count += old_pair_count)
                .or_insert(old_pair_count);
            new_pairs
                .entry((*left, middle))
                .and_modify(|count| *count += old_pair_count)
                .or_insert(old_pair_count);
            new_pairs
                .entry((middle, *right))
                .and_modify(|count| *count += old_pair_count)
                .or_insert(old_pair_count);
        }
        pairs.extend(new_pairs);
    }
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

