// https://adventofcode.com/2021/day/6

// Expects the number of iterations (80/256) as command line
// argument, reads the numbers for fishes from stdin.


use std::io::{self, BufRead};

fn main() {
    let iterations;
    match std::env::args().nth(1) {
        Some(x) => { iterations = x.parse::<i64>().unwrap() },
        // The division was invalid
        None    => {
            eprintln!("No number given!");
            std::process::exit(1);
        }
    }
    
    let stdin = io::stdin();

    // N fishes of the same age can be generalized to one count
    let mut generations: [i64; 9] = [ 0,0,0,0,0,0,0,0,0];
    
    let mut iterator = stdin.lock().lines();
    
    // First line holds the counters
    let line1 = iterator.next().unwrap().unwrap();
    let numbers: Vec<i64> = line1.split(',')
        .map(|s| s.parse::<i64>().unwrap() )
        .collect();

    for n in numbers {
        generations[n as usize] += 1;
    }

    // Each day, a 0 becomes a 6 and adds a new 8 to the end of the list,
    // while each other number decreases by 1 if it was present at the
    // start of the day.
    for _ in 0..iterations {
        let new_8s = generations[0];
        let new_6s = generations[0];        
        for i in 0..8 {
            generations[i] = generations[i+1];
        }
        generations[8] = new_8s;
        generations[6] += new_6s;
        println!("{:?}", generations );
    }
    
    println!("The number is {:?}", generations.iter().sum::<i64>() );

}
