
// https://adventofcode.com/2021/day/7

// Reads from stdin and prints a number.
// rustc --cfg part2 for the second case.

use std::io::{self, BufRead};

// In part 1, the cost of fuel is 1 for each step
#[cfg(not(part2))]
fn costof(n: i64) -> i64 {
    n
}
// In part 2, costs are calculated in an exponential way.
// Basically, sum all numbers up to n. Recursion is stupid, too.
#[cfg(part2)]
fn costof(n: i64) -> i64 {
    // NB. Expecting a non-negative integer!
    return if n == 0 { 0 } else { n + costof(n -1) };
}


fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // First line holds the positions
    let line1 = iterator.next().unwrap().unwrap();
    let numbers: Vec<i64> = line1.split(',')
        .map(|s| s.parse::<i64>().unwrap() )
        .collect();

    // Align the elements to the same X.
    // I think it's pretty obvious that the 
    // potential spots for aligning
    // should be between max and min of the array?
    let mut min = numbers[0];
    let mut max = numbers[0];
    for n in &numbers {
        min = if n < &min {*n} else {min};
        max = if n > &max {*n} else {max};
    }

    // Brute forcing it
    let mut min_cost = i64::MAX;
    let mut min_idx = 0;
    for n in min..=max {        
        let cost = numbers.iter().map(|i| costof((n-i).abs()) ).sum();
        if cost < min_cost {
            min_cost = cost;
            min_idx = n;
        }
    }


    println!("Minimum cost {:?} at {:?}", min_cost, min_idx);
}