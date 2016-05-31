use utils;
use std::collections::HashSet;
use std::iter::FromIterator;


pub fn problem_047() -> u64 {
    let k = 4;
    let mut consecutive: Vec<u64> = vec![];
    for n in 2..1000000 {
        let prime_factors = utils::prime_factors(n);
        let distinct_prime_factors: HashSet<&u64> = HashSet::from_iter(prime_factors.iter());
        if distinct_prime_factors.len() == k {
            consecutive.push(n);
            if consecutive.len() == k {
                return consecutive[0];
            }
        } else {
            consecutive = vec![];
        }
    }
    0
}



#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_047() {
        let ans: u64 = problem_047();
        println!("Answer to Problem 47: {}", ans);
        assert!(ans == 134043)
    }


    #[bench]
    fn bench_problem_047(b: &mut Bencher) {
        b.iter(|| problem_047());
    }
}
