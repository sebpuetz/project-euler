pub fn main() {
    let mut i = 1;
    let mut j = 1;
    let mut sum = 0;
    while i < 4000000 {
        let tmp = i;
        i += j;
        j = tmp;
        if i % 2 == 0 {
            sum += i;
        }
    }
    println!("{}", sum);
}