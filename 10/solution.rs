// https://adventofcode.com/2021/day/10

// Syntax check parentheses and bracketseses

use std::io::{self, BufRead};

fn main() {
    // Set up the input reading and read input
    let stdin = io::stdin();    
    // Peekable might be useful, depending on how we iterate.
    let mut input_iter = stdin.lock().lines().peekable();
    
    // Input is multiline, keep in vector if needed later
    let mut input = Vec::new();
    for line in input_iter {
        input.push(line.unwrap());
    } 

    
    // Logic goes here
    let mut result = 0;
    let mut stack: Vec<char> = Vec::new();
    let mut incompletes = Vec::new();
    let brackets = [('(', ')', 3, 1),
                    ('[', ']', 57, 2),
                    ('{', '}',1197, 3),
                    ('<', '>', 25137, 4)];
                    
    
    for (index,line) in input.iter().enumerate() {
        let mut error = 0;
        for c in line.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                stack.push(c);
            } else {
                let x = stack.pop().unwrap();
                for (open, close, score, score2) in brackets {
                // Points for first illegal character on line
                    if c == close && x != open {
                        error = if error == 0 {score} else {error}; } 
                }
            }            
        }
        result += error;
        if error == 0 {
            // Keep incomplete lines for part 2
            incompletes.push(line);
        }
    }

    // Part 2, complete the lines
    let mut score_table = Vec::new();
    for line in incompletes {
        stack = Vec::new();
        let mut completion = Vec::new();
        let mut score: i64 = 0;
        // Starting from end...
        for c in line.chars().rev() {
            let mut ok = false;
            for (open, close, score, score2) in brackets {
                if c == close {
                    stack.push(c);
                    ok = true;
                } else {
                    if c == open && stack.last().is_some() && stack.last().unwrap() == &close {
                        stack.pop();
                        ok = true;
                    }
                }
            }
            // This would be an opening bracket with no closer
            if !ok {
                for (open, close, sc, score2) in brackets {
                    if open == c {
                        completion.push(close);
                        score *= 5;
                        score += score2;
                    }
                }
            }
            
        }
        score_table.push(score);
        println!("{:?} {:?}", completion.iter().collect::<String>(), score);
    }
    score_table.sort();
    let part2 = score_table[ (score_table.len() as f32 / 2.0).floor() as usize];
    println!("Part 1: {:?}", result);
    println!("Part 2: {:?}", part2);
}
