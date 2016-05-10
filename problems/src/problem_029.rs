use std::collections::HashSet;
use utils;

pub fn prime_factors_counts(n: u64, max_factor: usize) -> Vec<u32>{
    let prime_factors = utils::prime_factors(n);

    let mut factor_counts: Vec<u32> = vec![0;max_factor];
    for factor in prime_factors{
        factor_counts[factor as usize] += 1;
    }
    factor_counts
}



pub fn problem_029() -> usize {
    let max_factor = 100;
    let mut unique_terms = HashSet::new();

    for a in 2..101 {
        let factor_counts = prime_factors_counts(a, max_factor);
        for b in 2..101 {
            let adj_factor_counts: Vec<u32> = factor_counts.clone().iter().map(|&c| c*b).collect();
            unique_terms.insert(adj_factor_counts);
        }
    }
    unique_terms.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_029() {
        let ans: usize = problem_029();
        println!("Answer to Problem 29: {}", ans);
        assert!(ans == 9183)
    }

    #[bench]
    fn bench_problem_029(b: &mut Bencher) {
        b.iter(|| problem_029());
    }

}


