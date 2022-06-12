#![feature(drain_filter)]

use std::collections::HashMap;

const WIDTH: usize = 5;

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

    assert_eq!(solution(test_input), 1924);

    let input = include_str!("../inputs.txt");
    println!("{}", solution(input));
}

fn solution(input: &str) -> i32 {
    let (nums_str, boards_str) = input.split_once("\n\n").unwrap();
    let nums: Vec<i32> = nums_str
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Vec<Position>> = boards_str
        .split("\n\n")
        .enumerate()
        .map(|(board_id, board_str)| {
            board_str
                .split('\n')
                .enumerate()
                .flat_map(|(row_id, row)| {
                    row.split_whitespace()
                        .enumerate()
                        .map(move |(col_id, num_str)| {
                            let number = num_str.parse().unwrap();
                            Position {
                                board_id,
                                row_id,
                                col_id,
                                number,
                                marked: false,
                            }
                        })
                })
                .collect()
        })
        .collect();

    let mut row_count = HashMap::<(usize, usize), i32>::new();
    let mut col_count = HashMap::<(usize, usize), i32>::new();

    let (board, num) = nums
        .iter()
        // filter by winning number
        .filter_map(|num| {
            // return boards if it wins
            let winning_board = boards
                .drain_filter(|board| {
                    // find if a board wins
                    for p in board {
                        if p.number == *num {
                            p.marked = true;
                            let count = row_count.entry((p.board_id, p.row_id)).or_insert(0);
                            *count += 1;
                            if *count as usize == WIDTH {
                                return true;
                            }
                            let count = col_count.entry((p.board_id, p.col_id)).or_insert(0);
                            *count += 1;
                            if *count as usize == WIDTH {
                                return true;
                            }
                        }
                    }
                    false
                })
                .last();
            if winning_board.is_some() {
                Some((winning_board.unwrap(), num))
            } else {
                None
            }
        })
        .last()
        .unwrap();

    let score = board.iter().fold(0, |sum, p| {
        if !p.marked {
            return sum + p.number;
        }
        return sum;
    }) * num;

    score
}

#[derive(Clone, Copy, Debug)]
struct Position {
    board_id: usize,
    row_id: usize,
    col_id: usize,
    number: i32,
    marked: bool,
}
