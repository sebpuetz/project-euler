pub fn main() {
    let mut palindromes = Vec::new();
    for i in (100..1000).into_iter().rev() {
        for j in (100..1000).into_iter().rev() {
            let num = i*j;
            if is_palindrome(num) {
                palindromes.push(num)
            }
        }
    }
    let mut max = 0;
    for v in palindromes {
        if v > max {
            max = v;
        }
    }
    println!("{}", max);
}

fn is_palindrome(num: usize) -> bool {
    let s_num = num.to_string();
    for i in 0..s_num.len()/2 {
        if s_num[i..i+1] != s_num[s_num.len() - i - 1..s_num.len() - i] {
            return false
        }
    }
    true
}