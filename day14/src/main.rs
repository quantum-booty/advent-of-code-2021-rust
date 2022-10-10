mod solution_a;
mod solution_b;

use solution_a::solution_a;
use solution_b::solution_b;

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

    let test_answer = solution_a(test_input, 10);
    println!("{}", test_answer);

    let input = include_str!("inputs.txt");
    let answer = solution_a(input, 10);
    println!("{}", answer);

    let test_answer = solution_b(test_input, 40);
    println!("{}", test_answer);

    let input = include_str!("inputs.txt");
    let answer = solution_b(input, 40);
    println!("{}", answer);
}
