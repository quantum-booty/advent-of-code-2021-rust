use std::collections::HashMap;

fn main() {
    let test_input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    assert_eq!(solution(test_input), 4512);

    let input = include_str!("../inputs.txt");
    println!("{}", solution(input));
    assert_eq!(solution(input), 29440);
}

fn solution(input: &str) -> i32 {
    let (nums_str, boards) = input.split_once("\n\n").unwrap();
    let nums: Vec<i32> = nums_str
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut positions = parse_positions(boards);

    let row_count = HashMap::<(usize, usize), i32>::new();
    let col_count = HashMap::<(usize, usize), i32>::new();

    let board_width = boards
        .split("\n")
        .next()
        .unwrap()
        .split_whitespace()
        .count();

    let winning_pos =
        find_first_winning_position(nums, &mut positions, row_count, col_count, board_width)
            .unwrap();
    calculate_score(positions, winning_pos)
}

fn parse_positions(boards: &str) -> Vec<Position> {
    let positions: Vec<Position> = boards
        .split("\n\n")
        .enumerate()
        .map(|(board_id, rows)| {
            rows.split('\n')
                .enumerate()
                .map(move |(row_id, row)| {
                    row.split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .enumerate()
                        .map(move |(col_id, number)| Position {
                            board_id,
                            row_id,
                            col_id,
                            number,
                            marked: false,
                        })
                })
                .flatten()
        })
        .flatten()
        .collect();
    positions
}

fn calculate_score(positions: Vec<Position>, winning_pos: Position) -> i32 {
    positions.iter().fold(0, |sum, p| {
        if (p.board_id == winning_pos.board_id) & (!p.marked) {
            return sum + p.number;
        }
        return sum;
    }) * winning_pos.number
}

fn find_first_winning_position(
    nums: Vec<i32>,
    positions: &mut Vec<Position>,
    mut row_count: HashMap<(usize, usize), i32>,
    mut col_count: HashMap<(usize, usize), i32>,
    board_width: usize,
) -> Option<Position> {
    for num in nums {
        for p in &mut *positions {
            if (num == p.number) & (!p.marked) {
                p.marked = true;
                let count = row_count.entry((p.board_id, p.row_id)).or_insert(0);
                *count += 1;
                if *count as usize == board_width {
                    return Some(*p);
                }
                let count = col_count.entry((p.board_id, p.col_id)).or_insert(0);
                *count += 1;
                if *count as usize == board_width {
                    return Some(*p);
                }
            }
        }
    }
    None
}

#[derive(Clone, Copy, Debug)]
struct Position {
    board_id: usize,
    row_id: usize,
    col_id: usize,
    number: i32,
    marked: bool,
}
