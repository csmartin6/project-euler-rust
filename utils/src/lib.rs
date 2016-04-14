#![feature(step_by)]

use std::hash::Hash;
use std::collections::HashMap;
use std::cmp::max;

pub fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut i: u64 = 3;
    while i <= (n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

pub fn prime_sieve(n: usize) -> Vec<usize> {
    let mut candidate_primes: Vec<bool> = vec!(true; n+1);
    let mut p = 2;

    candidate_primes[0] = false;
    candidate_primes[1] = false;

    while p <= (n as f64).sqrt() as usize {
        if candidate_primes[p] {
            for i in ((p * p)..(n + 1)).step_by(p) {
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
    let log_n = max((n as f64).ln().ceil() as usize, 1);
    let max_candidates: usize = 2 * n * log_n;
    let primes = prime_sieve(max_candidates);
    primes[n - 1]
}



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

pub fn count_divisors(n: u64) -> u64 {
    let factor_counts = counts(&prime_factors(n));

    factor_counts.values().fold(1, |a, &b| a * (b as u64 + 1))
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

pub fn add_digit_array(array_a: &[u32], array_b: &[u32]) -> Vec<u32> {

    let base = 10;
    let mut iter_a = array_a.iter().rev();
    let mut iter_b = array_b.iter().rev();
    let mut carry = 0;
    let mut result: Vec<u32> = vec![];

    loop {
        let s = match (iter_a.next(), iter_b.next()) {
            (Some(a), Some(b)) => carry + a + b,
            (Some(a), None) => carry + a,
            (None, Some(b)) => carry + b,
            (None, None) => {
                break;
            }
        };

        let rem = s % base;
        carry = match s {
            x if x >= base => 1,
            _ => 0,
        };

        result.push(rem);

    }
    if carry == 1 {
        result.push(carry);
    }

    result.into_iter().rev().collect()
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(28));
        assert!(is_prime(7));
    }

    #[test]
    fn test_prime_sieve() {
        assert_eq!(prime_sieve(9), vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_nth_prime() {
        assert!(nth_prime(1) == 2);
        assert!(nth_prime(6) == 13);
    }

    #[test]
    fn test_smallest_prime_factor() {
        assert!(smallest_prime_factor(18) == 2);
        assert!(smallest_prime_factor(9) == 3);
        assert!(smallest_prime_factor(23) == 23);
    }

        #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(4) ,vec!(2,2));
        assert_eq!(prime_factors(23) ,vec!(23));
        assert_eq!(prime_factors(18) ,vec!(2,3,3));

    }
    #[test]
    fn test_count_divisors() {
        assert!(count_divisors(28) == 6);
        assert!(count_divisors(7) == 2)
    }

    #[test]
    fn test_to_digit_array() {
        assert_eq!(to_digit_array(325),vec![3,2,5]);
        assert_eq!(to_digit_array(505),vec![5,0,5]);
    }

    #[test]
    fn test_counts() {

        let test_array = [2,2,4,5,6,2,4];
        let mut test_counts = HashMap::new();
        test_counts.insert(2,3);
        test_counts.insert(4,2);
        test_counts.insert(5,1);
        test_counts.insert(6,1);

        assert_eq!(counts(&test_array),test_counts);
    }


    #[test]
    fn test_add_digit_array() {
        let array_a = vec![5, 1];
        let array_b = vec![1, 3];
        assert_eq!(add_digit_array(&array_a[..], &array_b[..]), [6, 4]);

        let array_a = vec![5, 9];
        let array_b = vec![1, 3];
        assert_eq!(add_digit_array(&array_a[..], &array_b[..]), [7, 2]);

        let array_a = vec![4, 5, 1];
        let array_b = vec![6, 1, 3];
        assert_eq!(add_digit_array(&array_a[..], &array_b[..]), [1, 0, 6, 4]);

        let array_a = vec![4, 5, 1];
        let array_b = vec![1, 6, 1, 3];
        assert_eq!(add_digit_array(&array_a[..], &array_b[..]), [2, 0, 6, 4]);
    }

}
