use std::collections::{HashMap, HashSet};

fn main() {
    let test_input = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
    assert_eq!(solution(test_input), 61229);

    let input = include_str!("../inputs.txt");
    println!("{}", solution(input));
}

fn solution(input: &str) -> i32 {
    input.trim().lines().map(|line| solution_line(line)).sum()
}

fn solution_line(line: &str) -> i32 {
    let mut pattern_map = HashMap::<i32, HashSet<char>>::new();

    let idx = line.find(" | ").unwrap();
    let output_patterns: Vec<&str> = line[idx + 2..].split_whitespace().collect();
    let input_patterns: Vec<&str> = line[..idx].split_whitespace().collect();

    input_patterns.iter().for_each(|pattern| {
        let char_set = pattern_to_set(pattern);
        match pattern.len() {
            2 => pattern_map.insert(1, char_set),
            4 => pattern_map.insert(4, char_set),
            3 => pattern_map.insert(7, char_set),
            7 => pattern_map.insert(8, char_set),
            _ => None,
        };
    });
    // 1, 4, 7, 8, 9, 3

    input_patterns
        .iter()
        .filter(|pattern| pattern.len() == 6)
        .for_each(|pattern| {
            let pattern_set = pattern_to_set(pattern);
            let four_diff_pattern: HashSet<char> = pattern_map
                .get(&4)
                .unwrap()
                .difference(&pattern_set)
                .copied()
                .collect();
            if four_diff_pattern.len() == 0 {
                pattern_map.insert(9, pattern_set);
            } else if four_diff_pattern
                .difference(pattern_map.get(&1).unwrap())
                .count()
                == 0
            {
                pattern_map.insert(6, pattern_set);
            } else {
                pattern_map.insert(0, pattern_set);
            }
        });

    input_patterns
        .iter()
        .filter(|pattern| pattern.len() == 5)
        .for_each(|pattern| {
            let pattern_set = pattern_to_set(pattern);
            if pattern_set.is_superset(pattern_map.get(&1).unwrap()) {
                pattern_map.insert(3, pattern_set);
            } else if pattern_map
                .get(&9)
                .unwrap()
                .difference(&pattern_set)
                .copied()
                .collect::<HashSet<char>>()
                .difference(pattern_map.get(&7).unwrap())
                .count()
                == 0
            {
                pattern_map.insert(5, pattern_set);
            } else {
                pattern_map.insert(2, pattern_set);
            }
        });

    let result: Vec<i32> = output_patterns
        .iter()
        .map(|pattern| {
            let pattern_set = pattern_to_set(&pattern);
            for (k, v) in pattern_map.iter() {
                if *v == pattern_set {
                    return k;
                }
            }
            unreachable!()
        })
        .copied()
        .collect();

    vec_to_int(result)
}

fn vec_to_int(result: Vec<i32>) -> i32 {
    result.iter().fold(0, |acc, elem| acc * 10 + elem)
}

fn pattern_to_set(pattern: &str) -> HashSet<char> {
    pattern.chars().collect::<HashSet<_>>()
}

