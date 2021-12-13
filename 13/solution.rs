// https://adventofcode.com/2021/day/13

// Dots on a paper, and folding it... recycled a lot from day 5

use std::io::{self, BufRead};

// Returns the numbers from a string like '0,1'
fn get_numbers(in_str: &str) -> [i32; 2] {
    let num_parts: Vec<&str> = in_str.split(',').collect();
    let mut numbers: [i32; 2] = [0,0];
    numbers[0] = num_parts[0].parse::<i32>().unwrap();
    numbers[1] = num_parts[1].parse::<i32>().unwrap();

    numbers
}

fn main() {
    let stdin = io::stdin();
    let mut input = Vec::new();
    
    for line in stdin.lock().lines() {
        input.push(line.unwrap());
    }
    
    let mut map = vec![vec![false; 1]; 1]; // 1x1, this will grow later
    let mut input_iter = input.iter();
    let mut line = input_iter.next().unwrap();
    while line != "" {
        let coords = get_numbers(&line);
        let x = coords[0] as usize;
        let y = coords[1] as usize;

        // Map growing logic...
        let mut w = map[0].len();
        let h = map.len();
        if x >= w {
            // Resize every row
            for row in map.iter_mut() {
                row.resize(x+1,false);
            }
            w = x+1;            
        }
        if y >= h {
            // Add rows at end
            map.resize(y+1, vec![false; w]);
        }

        map[y][x] = true;

        line = input_iter.next().unwrap();
    }
    // Here we should have read an empty line, so we can go on with folding instructions
    // i.e. "fold along x=7"
    let mut line = input_iter.next();
    let mut first = true;
    while line.is_some() {
        let num = line.unwrap().split("=").last().unwrap().parse::<usize>().unwrap();
        if line.unwrap().starts_with("fold along x=") {
            for l in 0..map.len() {
                map[l] = map[l][..num].iter()
                    .zip(map[l][num+1..].iter().rev() )
                    .map( |(x,y)| *x || *y)
                    .collect::<Vec<bool>>();
            }            
        } else if line.unwrap().starts_with("fold along y=") {
            // Simpler folding this way, let's hope it's always along the middle...
            let mut y = num-1;
            for l in num+1..map.len() {
                // Overlapping dots will be combined
                map[y] = map[y].iter()
                    .zip(map[l].iter())
                    .map(|(x,y)| *x || *y)
                    .collect::<Vec<bool>>();
                // println!("{:?}", map[y]);
                y = (y as i32 - 1) as usize;
            }
            map.truncate(num);
        }
        if first {
            // Part 1 is concerned with the first fold...
            first = false;
            let part1: i32 = map.iter().flatten()
                .map( |b| if *b { 1 } else { 0 })
                .sum();
            println!("Part 1: {:?}", part1);
        }
        line = input_iter.next();
    }
    for l in map {
        println!("{:?}",l.iter().map(|b| if *b {'#'} else {' '}).collect::<String>());
    }

    // println!("The number is {:?}", n);
}
