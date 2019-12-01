use project_euler::is_prime;

pub fn main() {
    let mut n = 14;
    let mut primes = vec![2,3,5,7,11,13];
    while primes.len() < 10001 {
        if is_prime(n) {
            primes.push(n);
        }
        n +=1;
    }
    println!("{}", primes[10000]);
}