fn solution_a(inputs: &str) -> u32 {
    let lines = inputs.lines().collect::<Vec<&str>>();
    let width = lines.first().unwrap().len();
    let count = lines.len();

    let gamma = lines
        .iter()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .fold(vec![0; width], |count, bits| {
            count
                .into_iter()
                .enumerate()
                // in rust << and >> take presedence before &
                .map(|(i, n)| n + ((bits & 1 << i) >> i))
                .collect()
        })
        .into_iter()
        .enumerate()
        // use (b >= count / 2) as u32 to get most common bit
        // then shift it to the ith position using << i
        .map(|(i, b)| ((b >= count / 2) as u32) << i)
        .sum::<u32>();

    gamma * (!gamma & ((1 << width) - 1))
}

fn solution_b(inputs: &str) -> u32 {
    let lines = inputs.lines().collect::<Vec<&str>>();
    let width = lines.first().unwrap().len();

    let nums = lines
        .iter()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let oxy = (0..width)
        .rev()
        .scan(nums.clone(), |oxy, i| {
            if oxy.len() == 1 {
                return Some(oxy.clone());
            }
            let one_count = oxy
                .iter()
                .fold(0, |count, bits| count + ((bits & 1 << i) >> i));
            if one_count >= ((oxy.len() + 1) / 2) {
                oxy.retain(|bits| ((bits & 1 << i) >> i) == 1);
                Some(oxy.clone())
            } else {
                oxy.retain(|bits| ((bits & 1 << i) >> i) == 0);
                Some(oxy.clone())
            }
        })
        .last()
        .unwrap();

    let co2 = (0..width)
        .rev()
        .scan(nums.clone(), |oxy, i| {
            if oxy.len() == 1 {
                return Some(oxy.clone());
            }
            let one_count = oxy
                .iter()
                .fold(0, |count, bits| count + ((bits & 1 << i) >> i));
            if one_count >= ((oxy.len() + 1) / 2) {
                oxy.retain(|bits| ((bits & 1 << i) >> i) == 0);
                Some(oxy.clone())
            } else {
                oxy.retain(|bits| ((bits & 1 << i) >> i) == 1);
                Some(oxy.clone())
            }
        })
        .last()
        .unwrap();

    (oxy.first().unwrap() * co2.first().unwrap()).try_into().unwrap()
}

//

fn main() {
    let test_inputs = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    assert_eq!(solution_a(test_inputs), 198);
    assert_eq!(solution_b(test_inputs), 230);

    let inputs = include_str!("../inputs.txt");
    println!("{}", solution_a(&inputs));
    println!("{}", solution_b(&inputs));
}
