use utils;

pub fn problem_007() -> usize {
    let n = 10001;
    let max_candidate = 2 * n * (n as f64).ln() as usize;

    let primes = utils::prime_sieve(max_candidate);
    primes[n - 1]
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

	#[test]
	fn test_problem_007() {
	    let ans: usize = problem_007();
	    println!("Answer to Problem 7: {}", ans);
	    assert!(ans == 104743)
	}

	#[bench]
	fn bench_problem_007(b: &mut Bencher) {
	    b.iter(|| problem_007());
	}	
}