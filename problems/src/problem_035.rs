use utils;
use std::collections::HashSet;

pub fn problem_035() -> usize {
    let n = 1000000;

    let mut primes = HashSet::new();
    for &p in utils::prime_sieve(n).iter(){
        primes.insert(p);
    }
    let mut circular_primes = HashSet::new();
    
    for &prime in primes.iter(){
        let digits = utils::as_digit_array(prime as u64);
        let mut prime_count = 0;
        for i in 0..digits.len() {
            let mut test_prime_digits = digits[i..].to_vec();
            test_prime_digits.extend(digits[0..i].to_vec());
            let test_prime: usize = utils::from_digit_array(test_prime_digits) as usize;
            if !(primes.contains(&test_prime)){
                break
            } else{
                prime_count += 1;
            }
        }
        if prime_count == digits.len(){
            circular_primes.insert(prime);
        }
    }
    circular_primes.len()
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_035() {
        let ans: usize = problem_035();
        println!("Answer to Problem 35: {}", ans);
        assert!(ans == 55)
    }

    #[bench]
    fn bench_problem_035(b: &mut Bencher) {
        b.iter(|| problem_035());
    }
}


