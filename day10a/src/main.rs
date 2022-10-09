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
    assert_eq!(solution(test_input), 26397);
    println!("{}", solution(include_str!("../inputs.txt")));
}

fn solution(input: &str) -> usize {
    input
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
                        return Some(c);
                    };
                };
            }
            None
        })
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum()
}
