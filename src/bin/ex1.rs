pub fn main() {
    println!(
        "{}",
        (0..1000).into_iter().filter(|&i| i % 3 == 0 || i % 5 == 0).sum::<usize>()
    );
}
