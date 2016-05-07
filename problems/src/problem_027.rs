use utils;
use std::collections::HashSet;

fn consecutive_quadratic_primes(a: i64, b: i64, primes: &HashSet<u64>) -> u64{
    let mut n = 0;

    let mut prime_candidate: i64 = n * n + a * n + b;

    while (prime_candidate > 1) &primes.contains(&(prime_candidate as u64)){
        n += 1;
        prime_candidate = n * n + a * n + b;
    }
    n as u64
}

pub fn problem_027() -> i64 {
    let n = 100000;
    let max_a: i64 = 1000;
    let max_b = 1000;
    
    let mut primes = HashSet::new();
    for &n in utils::prime_sieve(n).iter(){
        primes.insert(n as u64);
    }

    // b must be prime and positive
    let possible_b = utils::prime_sieve(max_b);

    let mut max_consecutive_primes = 0;
    let mut pair = (0, 0);
    for &b in possible_b.iter() {
        for a in (-1*max_a + 1)..max_a {
            let n = consecutive_quadratic_primes(a, b as i64, &primes);
            if n > max_consecutive_primes {
                max_consecutive_primes = n;
                pair = (a as i64, b as i64);
            }
        }
    }
    pair.0 * pair.1
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_027() {
        let ans: i64= problem_027();
        println!("Answer to Problem 27: {}", ans);
        assert!(ans == -59231)
    }

    #[bench]
    fn bench_problem_027(b: &mut Bencher) {
        b.iter(|| problem_027());
    }

}


