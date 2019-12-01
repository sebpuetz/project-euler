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

fn is_prime(n: usize) -> bool {
    if n % 2 == 0 {
        return false
    }
    let mut j = 3;
    while j <= (n as f32).sqrt() as usize {
        if n % j == 0 {
            return false
        }
        j += 2;
    }
    true
}