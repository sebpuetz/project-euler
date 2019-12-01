use project_euler::is_prime;

pub fn main() {
    println!("{}", (2..2_000_000).filter(|i| is_prime(*i)).sum::<usize>())
}
