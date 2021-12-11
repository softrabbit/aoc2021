// https://adventofcode.com/2021/day/11

// Flashing octopuses in a cellular automaton

use std::io::{self, BufRead};
use std::cmp;

fn main() {
    // Set up the input reading and read input
    let stdin = io::stdin();    
    // Peekable might be useful, depending on how we iterate.
    let input_iter = stdin.lock().lines().peekable();

    // Input is multiline, keep in vector if needed later
    let mut field = Vec::new();
    for line in input_iter {
        let tmp = line
            .unwrap()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        field.push(tmp);
    } 

    // Here we go...
    let mut flashcounter = 0;
    let mut part1 = 0;
    let part2;
    let mut i = 0;
    loop {
        for row in field.iter_mut() {
            *row = row.iter().map( |&x| x+1).collect::<Vec<i32>>();
        }

        let mut flashes = true;
        while flashes {
            flashes = false;
            for y in 0..field.len() {
                for x in 0..field[y].len() {
                    let element = &field[y][x];
                    // Then, any octopus with an energy level greater than 9 flashes and adds
                    // to the energy of its neighbors.
                    if element > &9 {
                        // Not sure this is the best way to iterate the neighbors,
                        // it gets a bit tricky when you need to consider x=0 or y=0
                        for row in (cmp::max(0,y as i32 -1) as usize)..(cmp::min(field.len(), y+2) as usize) {
                            for col in (cmp::max(0,x as i32 -1) as usize)..(cmp::min(field[y].len(), x+2) as usize) {
                                if x==col && y==row {
                                    // Mark this as flashed
                                    field[row][col] = -11;
                                    flashes = true;
                                    flashcounter += 1;
                                } else if field[row][col] != -1 {
                                    // Add 1 to surrounding elements
                                    field[row][col] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        for row in field.iter_mut() {
            // Finally, any octopus that flashed during this step has its energy level set to 0
            *row = row.iter().map( |&x| if x < 0 { 0 } else { x } ).collect::<Vec<i32>>();
        }

        // Increment and do checks
        i += 1;
        
        if i == 100 {
            part1 = flashcounter;
        }
        // Part 2, check for all 0, i.e. synchronization.
        let tmp = field.iter()
            .map(|row| row.iter()
                 .fold(0, |sum, i| sum + i))
            .fold(0, |sum,i| sum + i);
        if tmp == 0 {
            part2 = i;
            break;
        }
    }
    
    println!("Part 1: {:?}", part1 );
    println!("Part 2: {:?}", part2 );
}
