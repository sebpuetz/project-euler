use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let f = File::open("ex11_input").expect("Input file missing");
    let mut grid = Vec::new();
    for line in BufReader::new(f).lines() {
        let line = line.expect("Can't read line");
        let row = line.split_whitespace()
            .map(|c| c.parse::<usize>().expect("Not a digit"))
            .collect::<Vec<_>>();
        grid.push(row);
    }


    let mut max: usize = 0;
    let n_cols = grid[0].len();
    for row in 0..grid.len() {
        for col in 0..n_cols {
            if row + 4 < grid.len() {
                let row_prod = (row..row + 4)
                    .into_iter()
                    .map(|i| grid[i][col])
                    .fold(1, |acc, i| acc * i);
                if row_prod > max {
                    max = row_prod;
                }
            }
            if col + 4 < n_cols {
                let col_prod = (col..col + 4)
                    .into_iter()
                    .map(|i| grid[row][i])
                    .fold(1, |acc, i| acc * i);
                if col_prod > max {
                    max = col_prod;
                }
            }
            if col + 4 < n_cols && row + 4 < grid.len() {
                let col_prod = (0..4)
                    .into_iter()
                    .map(|i| grid[row+i][col+i])
                    .fold(1, |acc, i| acc * i);
                if col_prod > max {
                    max = col_prod;
                }
            }
            if col + 4 < n_cols && row >= 4 &&  row + 4 < grid.len() {
                let col_prod = (0..4)
                    .into_iter()
                    .map(|i| grid[row-i][col+i])
                    .fold(1, |acc, i| acc * i);
                if col_prod > max {
                    max = col_prod;
                }
            }
        }
    }
    println!("{}", max);
}
