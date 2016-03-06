use std::hash::Hash;
use std::collections::HashMap;

pub fn smallest_prime_factor(n: u64) -> u64 {
    if n % 2 == 0 {
        return 2;
    }

    let mut i: u64 = 3;
    while i <= ((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return i;
        }
        i += 2;
    }
    n
}

pub fn prime_factors(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![1];
    }
    let mut factors = vec![];

    let mut m: u64 = n;
    while m > 1 {
        let next_factor = smallest_prime_factor(m);
        factors.push(next_factor);
        m /= next_factor;
    }
    factors
}

pub fn to_digit_array(num: u64) -> Vec<u32> {
    let mut n: u64 = num;
    let mut digits: Vec<u32> = vec![];
    while n > 1 {
        digits.push((n % 10) as u32);
        n /= 10;
    }
    digits.reverse();
    digits
}

pub fn counts<T>(list: &[T]) -> HashMap<T,u64> where T: Eq + Hash + Copy{
    let mut counter: HashMap<T,u64> = HashMap::new();

    for &el in list.iter() {
        let new_count = counter.get(&el).unwrap_or(&0) + 1;
        counter.insert(el.to_owned(),new_count);
    }
    counter
}

pub fn is_palindrome(num: u64) -> bool {
    let digits = to_digit_array(num);
    let n = digits.len();
    for i in 0 .. n/2 {
        if digits[i] != digits[n-i-1]{
            return false;
        }
    }
    true
}


