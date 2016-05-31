use utils;
use std::collections::HashSet;
use std::iter::FromIterator;


pub fn problem_046() -> usize {
    let n = 100;
    let squares: Vec<usize> = (0..n).map(|i| i * i).collect();
    let primes: Vec<usize> = utils::prime_sieve(n * n);
    let prime_set: HashSet<&usize> = HashSet::from_iter(primes.iter());

    let mut prime_plus_square: HashSet<usize> = HashSet::new();
    for s in squares.iter() {
        for p in primes.iter() {
            prime_plus_square.insert(2 * s + p);
        }
    }

    let mut i = 3;
    while i < n * n {
        if !prime_set.contains(&i) & !prime_plus_square.contains(&i) {
            return i;
        }
        i += 2;
    }

    0
}



#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_046() {
        let ans: usize = problem_046();
        println!("Answer to Problem 46: {}", ans);
        assert!(ans == 5777)
    }


    #[bench]
    fn bench_problem_046(b: &mut Bencher) {
        b.iter(|| problem_046());
    }
}
