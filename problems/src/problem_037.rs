use std::collections::HashSet;
use utils;

pub fn problem_037() -> u32 {
    let n = 1000000;
    let mut truncateable_primes = vec![];
    let mut primes = HashSet::new();
    for p in utils::prime_sieve(n) {
        let prime = p.to_string();
        if !(prime.contains('0') | prime.contains('4') | prime.contains('6') |
             prime.contains('8')) {
            primes.insert(p as u32);
        }
    }

    for &p in primes.iter() {
        let prime = p.to_string();
        let mut truncateable = true;
        for i in 1..prime.len() {
            let left = prime[i..].parse::<u32>().unwrap();
            if !primes.contains(&left) {
                truncateable = false;
                break;
            }
            let right = prime[0..i].parse::<u32>().unwrap();
            if !primes.contains(&right) {
                truncateable = false;
                break;
            }
        }
        if truncateable & (p > 10) {
            truncateable_primes.push(p);
        }
    }
    truncateable_primes.iter().fold(0, |a, &b| a + b)
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_037() {
        let ans: u32 = problem_037();
        println!("Answer to Problem 37: {}", ans);
        assert!(ans == 748317)
    }

    #[bench]
    fn bench_problem_037(b: &mut Bencher) {
        b.iter(|| problem_037());
    }
}
