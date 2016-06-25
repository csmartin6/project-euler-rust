use utils;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn problem_050() -> usize {
    let n = 1000000;
    let primes = utils::prime_sieve(n);
    let prime_set: HashSet<&usize> = HashSet::from_iter(primes.iter());

    let mut max_sum = 0;
    let mut max_seq_len = 0;
    for i in 0..primes.len() {
        let mut seq_sum: usize = primes[i..(i + max_seq_len)].iter().fold(0, |a, &b| a + b);

        if seq_sum > n {
            break;
        }

        let mut seq_len = max_seq_len;
        for (j, &q) in primes[(i + max_seq_len)..].iter().enumerate() {
            seq_sum += q;

            if seq_sum > n {
                if seq_len > max_seq_len {
                    max_seq_len = seq_len;
                }
                break;
            }

            if prime_set.contains(&seq_sum) {
                seq_len = max_seq_len + j;
                max_sum = seq_sum;
            }
        }
    }
    max_sum
}



#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_050() {
        let ans: usize = problem_050();
        println!("Answer to Problem 50: {}", ans);
        assert!(ans == 997651);
    }


    #[bench]
    fn bench_problem_050(b: &mut Bencher) {
        b.iter(|| problem_050());
    }
}
