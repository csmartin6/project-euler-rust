extern crate nalgebra as na;
extern crate num;


use std::hash::Hash;
use std::collections::HashMap;
use std::cmp::max;

pub mod linearalgebra;


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
            let mut i = p * p;
            while i <= n {
                candidate_primes[i] = false;
                i += p;
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

pub fn find_divisors(n: u64)-> Vec<u64> {
    let mut factors: Vec<u64> = vec![1];

    for x in 2..((n as f64).sqrt() as u64 + 1){
        if n % x == 0{
            factors.push(x);
        }
    }
    let mut complementary_factors: Vec<u64> = vec![];
    for x in factors.iter().rev() {
        complementary_factors.push(n/x);
    }
    // If n is a perfect square acoid adding the square root twice
    if factors.last() == complementary_factors.first() {
        factors.pop();
    }
    factors.extend(complementary_factors);
    factors
}

pub fn proper_divisors(n: u64)-> Vec<u64> {

    let mut divisors: Vec<u64> = find_divisors(n);
    divisors.pop();
    divisors
}

pub fn count_divisors(n: u64) -> u64 {
    let factor_counts = counts(&prime_factors(n));

    factor_counts.values().fold(1, |a, &b| a * (b as u64 + 1))
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

pub fn is_palindrome(num: u64) -> bool {
    let digits = to_digit_array(num);
    let n = digits.len();
    for i in 0..n / 2 {
        if digits[i] != digits[n - i - 1] {
            return false;
        }
    }
    true
}

pub fn to_digit_array(num: u64) -> Vec<u32> {
    let mut n: u64 = num;
    let mut digits: Vec<u32> = vec![];
    while n >= 1 {
        digits.push((n % 10) as u32);
        n /= 10;
    }
    digits.reverse();
    digits
}

pub fn to_digit_array_base(num: u64, base: u64) -> Vec<u32> {
    let mut n: u64 = num;
    let mut digits: Vec<u32> = vec![];
    while n >= 1 {
        digits.push((n % base) as u32);
        n /= base;
    }
    digits.reverse();
    digits
}

pub fn from_digit_array(digits: Vec<u32>) -> u64 {
    let base: u64 = 10;
    digits.iter()
          .rev()
          .enumerate()
          .map(|(pos, &d)| d as u64 * base.pow(pos as u32))
          .fold(0, |a, b| a + b as u64)
}

pub fn from_digit_array_base(digits: Vec<u32>, base: u64) -> u64 {
    digits.iter()
          .rev()
          .enumerate()
          .map(|(pos, &d)| d as u64 * base.pow(pos as u32))
          .fold(0, |a, b| a + b as u64)
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

pub fn scalar_multiply_digit_array(array: &[u32], multiple: u32) -> Vec<u32> {
    let base = 10;
    let mut carry: u32 = 0;
    let mut result: Vec<u32> = vec![];

    for digit in array.iter().rev() {
        let x = digit * multiple + carry;
        result.push(x % base);
        carry = x / base;
    }

    while carry >= 1 {
        result.push(carry % base);
        carry = carry / base;

    }

    result.into_iter().rev().collect()
}

pub fn multiply_digit_array(array_a: &[u32], array_b: &[u32]) -> Vec<u32> {

    let mut result: Vec<u32> = vec![];

    for (index, &digit) in array_b.iter().rev().enumerate() {
        let mut to_add = scalar_multiply_digit_array(array_a, digit);
        to_add.extend(vec![0;index]);
        if index == 0 {
            result = scalar_multiply_digit_array(array_a, digit);

        } else {
            result = add_digit_array(&result[..], &to_add[..]);
        }
    }
    result
}

pub fn number_to_words(num: u32) -> String {
    match num {
        1000...999999 => {
            format!("{} thousand{}",
                    number_to_words(num / 1000),
                    match num % 1000 {
                        n if n > 0 => format!(" {}", number_to_words(n)),
                        _ => "".to_string(),
                    })
        }
        100...999 => {
            format!("{} hundred{}",
                    number_to_words(num / 100),
                    match num % 100 {
                        n if n > 0 => format!(" and {}", number_to_words(n)),
                        _ => "".to_string(),
                    })
        }
        90 => "ninety".to_string(),
        80 => "eighty".to_string(),
        70 => "seventy".to_string(),
        60 => "sixty".to_string(),
        50 => "fifty".to_string(),
        40 => "forty".to_string(),
        30 => "thirty".to_string(),
        21...99 => {
            format!("{}-{}",
                    number_to_words(num - num % 10),
                    number_to_words(num % 10))
        } 
        20 => "twenty".to_string(),
        19 => "nineteen".to_string(),
        18 => "eighteen".to_string(),
        17 => "seventeen".to_string(),
        16 => "sixteen".to_string(),
        15 => "fifteen".to_string(),
        14 => "fourteen".to_string(),
        13 => "thirteen".to_string(),
        12 => "twelve".to_string(),
        11 => "eleven".to_string(),
        10 => "ten".to_string(),
        9 => "nine".to_string(),
        8 => "eight".to_string(),
        7 => "seven".to_string(),
        6 => "six".to_string(),
        5 => "five".to_string(),
        4 => "four".to_string(),
        3 => "three".to_string(),
        2 => "two".to_string(),
        1 => "one".to_string(),
        _ => panic!("Something has gone horribly wrong at number: {}", num),
    }
}

pub fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n-1)
    }
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
        assert_eq!(prime_factors(4), vec![2, 2]);
        assert_eq!(prime_factors(23), vec![23]);
        assert_eq!(prime_factors(18), vec![2, 3, 3]);

    }

    #[test]
    fn test_count_divisors() {
        assert!(count_divisors(28) == 6);
        assert!(count_divisors(7) == 2)
    }
    #[test]
    fn test_find_divisors() {
        assert_eq!(find_divisors(28),vec![1,2,4,7,14,28]);
        assert_eq!(find_divisors(7),vec![1,7]);
        assert_eq!(find_divisors(4),vec![1,2,4]);
    }

    #[test]
    fn test_proper_divisors() {
        assert_eq!(proper_divisors(28),vec![1,2,4,7,14]);
        assert_eq!(proper_divisors(7),vec![1]);
        assert_eq!(proper_divisors(4),vec![1,2]);
    }

    #[test]
    fn test_to_digit_array() {
        assert_eq!(to_digit_array(325), vec![3, 2, 5]);
        assert_eq!(to_digit_array(505), vec![5, 0, 5]);
    }

    fn test_to_digit_array_base() {
        assert_eq!(to_digit_array_base(50,7), vec![1, 0,  1]);
        assert_eq!(to_digit_array_base(505,7), vec![1,3, 2, 1]);
    }


    #[test]
    fn test_from_digit_array() {
        assert_eq!(from_digit_array(vec![3, 2, 5]), 325);
        assert_eq!(from_digit_array(vec![5, 0, 5]), 505);
    }

        #[test]
    fn test_from_digit_array_base() {
        assert_eq!(from_digit_array_base(vec![1,0,1],7), 50);
        assert_eq!(from_digit_array_base(vec![1, 3, 2,1],7), 505);
    }

    #[test]
    fn test_counts() {

        let test_array = [2, 2, 4, 5, 6, 2, 4];
        let mut test_counts = HashMap::new();
        test_counts.insert(2, 3);
        test_counts.insert(4, 2);
        test_counts.insert(5, 1);
        test_counts.insert(6, 1);

        assert_eq!(counts(&test_array), test_counts);
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


        let array_a = vec![4, 5, 1];
        let array_b = vec![3, 1, 6, 1, 3];
        assert_eq!(add_digit_array(&array_a[..], &array_b[..]), [3, 2, 0, 6, 4]);
    }

    #[test]
    fn test_scalar_multiply_digit_array() {
        let array = vec![5, 1];
        let multiple = 3;
        assert_eq!(scalar_multiply_digit_array(&array[..], multiple), [1, 5, 3]);

        let array = vec![5, 9];
        let multiple = 3;
        assert_eq!(scalar_multiply_digit_array(&array[..], multiple), [1, 7, 7]);

        let array = vec![5, 9];
        let multiple = 13;
        assert_eq!(scalar_multiply_digit_array(&array[..], multiple), [7, 6, 7]);
    }

    #[test]
    fn test_multiply_digit_array() {
        let array_a = vec![5, 1];
        let array_b = vec![1, 3];
        assert_eq!(multiply_digit_array(&array_a[..], &array_b[..]), [6, 6, 3]);

        let array_a = vec![5, 9];
        let array_b = vec![1, 3];
        assert_eq!(multiply_digit_array(&array_a[..], &array_b[..]), [7, 6, 7]);

        let array_a = vec![4, 5, 1];
        let array_b = vec![6, 1, 3];
        assert_eq!(multiply_digit_array(&array_a[..], &array_b[..]),
                   [2, 7, 6, 4, 6, 3]);

        let array_a = vec![4, 5, 1];
        let array_b = vec![1, 6, 1, 3];
        assert_eq!(multiply_digit_array(&array_a[..], &array_b[..]),
                   [7, 2, 7, 4, 6, 3]);


        let array_a = vec![4, 5, 1];
        let array_b = vec![3, 1, 6, 1, 3];
        assert_eq!(multiply_digit_array(&array_a[..], &array_b[..]),
                   [1, 4, 2, 5, 7, 4, 6, 3]);
    }

    #[test]
    fn test_number_to_words() {
        assert_eq!(number_to_words(85), "eighty-five");
        assert_eq!(number_to_words(172), "one hundred and seventy-two");
        assert_eq!(number_to_words(2750),
                   "two thousand seven hundred and fifty");
        assert_eq!(number_to_words(1000), "one thousand");
    }
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0),1);
        assert_eq!(factorial(1),1);
        assert_eq!(factorial(5),120);
    }

}
