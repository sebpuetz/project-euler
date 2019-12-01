pub fn main() {
    let mut i = 2520;
    while !is_divisible_to_20(i) {
        i += 1;
    }
    println!("{}", i);
}

fn is_divisible_to_20(num: usize) -> bool {
    for i in 11..=20 {
        if num % i != 0 {
            return false
        }
    }
    return true
}