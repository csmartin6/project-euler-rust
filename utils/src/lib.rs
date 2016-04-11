#![feature(step_by)]

use std::hash::Hash;
use std::collections::HashMap;

pub fn is_prime(n: usize) -> bool {
    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut i: usize = 3;
    while i <= (n as f64).sqrt() as usize {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

pub fn prime_sieve(n: usize) -> Vec<usize> {
    let mut candidate_primes: Vec<bool> = vec!(true; n);
    let mut p = 2;

    candidate_primes[0] = false;
    candidate_primes[1] = false;

    while p <= (n as f64).sqrt() as usize {
        if candidate_primes[p] {
            for i in (p * p..n).step_by(p) {
                candidate_primes[i] = false;
            }
        }
        p += 1;
    }

    let mut primes: Vec<usize> = vec![];

    for (p, &isprime) in candidate_primes.iter().enumerate() {
        if isprime {
            primes.push(p)
        }
    }
    primes
}

pub fn nth_prime(n: usize) -> usize {
    let max_candidate: usize = 2 * n * (n as f64).ln() as usize;
    let primes = prime_sieve(max_candidate);
    primes[n - 1]
}



pub fn smallest_prime_factor(n: usize) -> usize {
    if n % 2 == 0 {
        return 2;
    }

    let mut i: usize = 3;
    while i <= ((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return i;
        }
        i += 2;
    }
    n
}

pub fn prime_factors(n: usize) -> Vec<usize> {
    if n == 1 {
        return vec![1];
    }
    let mut factors = vec![];

    let mut m: usize = n;
    while m > 1 {
        let next_factor = smallest_prime_factor(m);
        factors.push(next_factor);
        m /= next_factor;
    }
    factors
}

pub fn to_digit_array(num: usize) -> Vec<usize> {
    let mut n: usize = num;
    let mut digits: Vec<usize> = vec![];
    while n > 1 {
        digits.push((n % 10) as usize);
        n /= 10;
    }
    digits.reverse();
    digits
}

pub fn counts<T>(list: &[T]) -> HashMap<T, usize>
    where T: Eq + Hash + Copy
{
    let mut counter: HashMap<T, usize> = HashMap::new();

    for &el in list.iter() {
        let new_count = counter.get(&el).unwrap_or(&0) + 1;
        counter.insert(el.to_owned(), new_count);
    }
    counter
}

pub fn is_palindrome(num: usize) -> bool {
    let digits = to_digit_array(num);
    let n = digits.len();
    for i in 0..n / 2 {
        if digits[i] != digits[n - i - 1] {
            return false;
        }
    }
    true
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
