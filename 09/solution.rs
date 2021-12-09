// https://adventofcode.com/2021/day/9

// 1: Find low points and calculate their sum.
// 2: Find basins around the low points.

use std::io::{self, BufRead};

// A basin is all locations that eventually flow downward to a single low point
// i.e. the point and all the higher points around it (9 excluded). Recursion FTW.
fn find_basin(row: usize, col: usize, map: &Vec<Vec<char>>, basin: Vec<(usize,usize)>) -> Vec<(usize,usize)> {
    let mut tmp = basin;
    tmp.push( (row,col) );

    // Build up a list of coordinates to check
    let mut coords = Vec::new();
    if row > 0  { coords.push( (row-1, col) ); }
    if row < map.len()-1  { coords.push( (row+1, col) ); }
    if col > 0  { coords.push( (row, col-1) ); }
    if col < map[0].len()-1  { coords.push( (row, col+1) ); }

    for (r,c) in coords {
        let mut i = tmp.iter();
        if i.find(|(y,x)| *y == r && c == *x ).is_none()
            && &map[r][c] < &'9'
            && &map[r][c] >= &map[row][col] {
                tmp = find_basin(r,c, &map, tmp);
        }
    }
    
    return tmp;
}

fn main() {
    // Set up the input reading and read input
    let stdin = io::stdin();    
    // Peekable might be useful, depending on how we iterate.
    let input_iter = stdin.lock().lines().peekable();
    
    // Input is multiline, keep in vector if needed later
    let mut input = Vec::new();
    for line in input_iter {
        input.push(line.unwrap());
    } 

    // Logic goes here
    let mut data = Vec::new();
    for line in input {
        // Nope, can't just say "data.push(line.chars)" because lifetime...
        let mut tmp = Vec::new();
        for c in line.chars() {
            tmp.push(c);            
        }
        data.push(tmp);
    }

    let mut part1 = 0;
    let mut basins = Vec::new();
    // Now, assume all lines were of equal length...
    for (line_no, line) in data.iter().enumerate() {
        for (idx, n) in line.iter().enumerate() {
            let mut low = true; // Assume we have a low point
            if idx > 0 && line[idx-1] <= *n { low = false } // left
            if idx < line.len()-1 && line[idx+1] <= *n { low = false } // right

            if line_no > 0 && data[line_no-1][idx] <= *n { low = false } // up
            if line_no < data.len()-1 && data[line_no+1][idx] <= *n { low = false } // down

            if low {
                // The risk level of a low point is 1 plus its height
                part1 += n.to_digit(10).unwrap() + 1;

                // Part 2: find out the basin of the low point
                let dummy = Vec::<(usize,usize)>::new();
                basins.push(find_basin(line_no, idx, &data, dummy));
            }
        }
    }
    // Find the three largest basins and multiply their sizes together
    let mut basin_sizes = basins.iter()
        .map(|b| b.len() as i32).collect::<Vec<i32>>();
    basin_sizes.sort_by(|a, b| b.cmp(a)); // .sort().rev() won't work
    let part2 = basin_sizes[0..3].iter().fold(1, |sum, x| sum * x);
    
    println!("Part 1 answer is {:?}", part1);
    println!("Part 2 answer is {:?}", part2);
}
