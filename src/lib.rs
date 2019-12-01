pub fn is_prime(n: usize) -> bool {
    if n % 2 == 0 && n != 2 {
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