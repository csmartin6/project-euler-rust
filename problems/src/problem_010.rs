use utils;

pub fn problem_010() -> u64 {
    let n = 2000000;
    let primes = utils::prime_sieve(n);
    primes.iter().fold(0,|a, &b| a + b as u64)
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;
    
    #[test]
    fn test_problem_010() {
        let ans: u64 = problem_010();
        println!("Answer to Problem 10: {}", ans);
        assert!(ans == 142913828922)
    }

    #[bench]
    fn bench_problem_010(b: &mut Bencher) {
        b.iter(|| problem_010());
    }    
}