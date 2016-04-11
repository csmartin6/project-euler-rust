use utils;
use std::collections::HashMap;

pub fn problem_005() -> u64 {
    let n = 21;

    let mut cumulative_prime_factors: HashMap<u64, usize> = HashMap::new();

    for i in 1..n {
        let factors = utils::prime_factors(i);
        let factor_count = utils::counts(&factors[..]);
        for (k, &v) in factor_count.iter() {
            if !cumulative_prime_factors.contains_key(k) ||
               factor_count[k] > cumulative_prime_factors[k] {
                cumulative_prime_factors.insert(*k, v);
            }
        }
    }

    let mut product = 1;

    for (k, &v) in cumulative_prime_factors.iter() {
        product *= k.pow(v as u32);
    }

    product
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_005() {
        let ans: u64 = problem_005();
        println!("Answer to Problem 5: {}", ans);
        assert!(ans == 232792560)
    }

    #[bench]
    fn bench_problem_005(b: &mut Bencher) {
        b.iter(|| problem_005());
    }
}