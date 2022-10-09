fn main() {
    let test_input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
    assert_eq!(solution(test_input), 288957);
    println!("{}", solution(include_str!("../inputs.txt")));
}

fn solution(input: &str) -> usize {
    let mut scores: Vec<usize> = input
        .lines()
        .filter_map(|line| {
            let mut cs = Vec::<char>::new();
            for c in line.chars() {
                // check if c opens
                let is_open = (c == '(') || (c == '{') || (c == '[') || (c == '<');
                if is_open {
                    cs.push(c);
                    continue;
                }

                // check if c closes the previous char
                if let Some(last) = cs.last() {
                    let is_close = match c {
                        ')' => *last == '(',
                        '}' => *last == '{',
                        ']' => *last == '[',
                        '>' => *last == '<',
                        _ => unreachable!(),
                    };
                    if is_close {
                        cs.pop();
                    } else {
                        return None;
                    };
                };
            }
            Some(cs)
        })
        .map(|cs| {
            cs.iter()
                .rev()
                .map(|c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                })
                .fold(0, |acc, score| acc * 5 + score)
        })
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}
