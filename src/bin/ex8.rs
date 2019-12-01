use std::fs::read_to_string;

pub fn main() {
    let f = read_to_string("ex8_input").expect("Input file missing");
    let grid = f
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).expect("Not a digit") as usize)
        .collect::<Vec<_>>();

    let mut max: usize = 0;
    for row in 0..grid.len() {
        if row + 13 < grid.len() {
            let row_prod = (row..row + 13)
                .into_iter()
                .map(|i| grid[i])
                .fold(1, |acc, i| acc * i);
            if row_prod > max {
                max = row_prod;
            }
        }
    }
    println!("{}", max);
}
