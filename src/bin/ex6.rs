pub fn main() {
    let sum = (1..=100).sum::<usize>();
    let squared_sums = sum * sum;
    let summed_squares = (1..=100).into_iter().map(|i| i * i).sum::<usize>();
    println!("{}", squared_sums - summed_squares);
}