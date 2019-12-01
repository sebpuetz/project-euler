
pub fn main() {
    let mut n: usize = 600851475143;
    while n % 2 == 0 {
        println!("{} ", 2);
        n /= 2;
    }
    let mut i = 3;
    while i <= (n as f32).sqrt() as usize {
        while n % i == 0 {
            println!("{} ", i);
            n /= i;
        }
        i += 2;
    }
    if n > 2 {
        println!("{}", n);
    }
}