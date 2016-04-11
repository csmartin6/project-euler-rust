use utils;

pub fn problem_003() -> u64 {
    let n: u64 = 600851475143;
    *utils::prime_factors(n).iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_003() {
        let ans: u64 = problem_003();
        println!("Answer to Problem 3: {}", ans);
        assert!(ans == 6857)
    }

    #[bench]
    fn bench_problem_003(b: &mut Bencher) {
        b.iter(|| problem_003());
    }
}