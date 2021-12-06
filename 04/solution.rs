// https://adventofcode.com/2021/day/4

// First line, bingo numbers. After that, bingo cards.
// Find winning card and sum unmarked numbers.

use std::io::{self, BufRead};

// Check for bingo in 5x5 board
fn check_bingo(board: [i32; 25]) -> bool {
    let mut bingo = false;
    // Horizontal
    for offset in (0..25).step_by(5) {
        bingo = bingo || ( board[offset..offset+5].into_iter()
                   .filter(|i| i < &&0)
                   .collect::<Vec<&i32>>()
                   .len() == 5);
    }
    // Vertical
    for offset in 0..5 {
        let mut tmp = true;
        for row in (0..25).step_by(5) {
            tmp = tmp && (board[offset + row] < 0);
        }
        bingo = bingo || tmp;
    }


    return bingo;
}

// Using negative values for marked numbers
fn mark_number(n: i32, board: &mut [i32; 25]) {
    *board = board.map(|i| if i==n { i*-1 } else { i });
}

fn sum_unmarked(board: [i32; 25]) -> i32 {
    //let sum: i32 = board.iter().sum();
    return  board.iter()
        .filter(|i| *i > &0)
        // .collect::<Vec<&i32>>()
        .sum();
}

fn main() {
    let stdin = io::stdin();
    // let mut input = Vec::new();

    
    let mut iterator = stdin.lock().lines().peekable();
    
    // First line holds the bingo numbers
    let line1 = iterator.next().unwrap().unwrap();
    let numbers: Vec<i32> = line1.split(',')
        .map(|s| s.parse::<i32>().unwrap() )
        .collect();

    // Build list of boards    
    // 2D arrays are cool, but maybe this will do...

    let mut boards = Vec::<[i32; 25]>::new();    

    while iterator.peek().is_some() {
        // Consume blank line
        let _dummy = iterator.next();
        let mut board: [i32; 25] =  [0; 25];
        for offset in (0..25).step_by(5) {
            let row: Vec<i32> = iterator.next().unwrap().unwrap()
                .split(' ').map(|s| s.trim() )
                .filter(|s| !s.is_empty() )
                .map(|s| s.parse::<i32>().unwrap() )
                .collect();
            for (idx, n) in row.iter().enumerate() {
                board[offset + idx] = *n;
            }
        }
        boards.push(board);
    }
    
    // Iterate through list of numbers, mark and check boards...
    //let mut n: i32;
    let mut win = -1;

    'outer_loop:
    for n in &numbers {
        // How obvious: this doesn't work, but an explicit index does...
        // for (idx, b) in boards.iter().enumerate() {
        for idx in 0..boards.len() {
            mark_number(*n, &mut boards[idx]);
            
            if check_bingo(boards[idx]) {
                win = n * sum_unmarked(boards[idx]);
                break 'outer_loop; // a GOTO by any other name...
            }
        }
    }
    
    println!("{:?}", boards);
    
    println!("The number is {:?}", win);
}
