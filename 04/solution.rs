// https://adventofcode.com/2021/day/4

// First line, bingo numbers. After that, bingo board.
// Find winning card and sum unmarked numbers.

// Part 1: find card that is first to win
// Part 2: find card that is last to win

use std::io::{self, BufRead};

// Check for bingo in 5x5 board
fn check_bingo(board: [(i32, bool); 25]) -> bool {
    let mut bingo = false;
    // Horizontal
    for offset in (0..25).step_by(5) {
        bingo = bingo || ( board[offset..offset+5].into_iter()
                   .filter(|(_i, b)| *b)
                   .collect::<Vec<&(i32,bool)>>()
                   .len() == 5);
    }
    // Vertical
    for offset in 0..5 {
        let mut tmp = true;
        for location in (offset..offset+25).step_by(5) {
            tmp = tmp && (board[location].1);
        }
        bingo = bingo || tmp;
    }
    return bingo;
}

// Mark any number n with true
fn mark_number(n: i32, board: &mut [(i32, bool); 25]) {
    *board = board.map(|(i,b)| (i, if i==n {true} else {b} ) );
}

fn sum_unmarked(board: [(i32, bool); 25]) -> i32 {
    //let sum: i32 = board.iter().sum();
    return  board.iter()
        .map(|(i,b)| if *b {0} else {*i})
        //.filter(|i| *i > &0)
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

    let mut boards = Vec::<[(i32, bool); 25]>::new();    

    while iterator.peek().is_some() {
        // Consume blank line
        let _dummy = iterator.next();
        let mut board: [(i32, bool); 25] =  [(0, false); 25];
        for offset in (0..25).step_by(5) {
            let row: Vec<i32> = iterator.next().unwrap().unwrap()
                .split(' ').map(|s| s.trim() )
                .filter(|s| !s.is_empty() )
                .map(|s| s.parse::<i32>().unwrap() )
                .collect();
            for (idx, n) in row.iter().enumerate() {
                board[offset + idx] = (*n, false);
            }
        }
        boards.push(board);
    }
    
    // Iterate through list of numbers, mark and check boards...

    let mut win;
    let mut skip_boards = Vec::new();
        
    for n in &numbers {
        // How obvious: this doesn't work, but an explicit index does...
        // for (idx, b) in boards.iter().enumerate() {
        for idx in 0..boards.len() {
            mark_number(*n, &mut boards[idx]);
            
            if !skip_boards.contains(&idx) && check_bingo(boards[idx]) {
                skip_boards.push(idx);
                win = n * sum_unmarked(boards[idx]);
                // Print out all wins, which will satisfy both part 1 and 2.
                println!("Win, score {:?} in board {:?} after {:?}",win, idx,n);
            }
        }
    }
}
