use num::Integer;
use num::bigint::ToBigInt;

pub fn multiplicative_order(a: u64, n: u64) -> u32 {
    if a.gcd(&n) != 1 {
        return 0;
    }


    let max_k = 10000000;
    let mut k = 0;
    let mut ak = ToBigInt::to_bigint(&a).unwrap();
    let a_big = ToBigInt::to_bigint(&a).unwrap();
    let n_big = ToBigInt::to_bigint(&n).unwrap();
    let one = ToBigInt::to_bigint(&1).unwrap();
    while ak.mod_floor(&n_big) != one  {
        k += 1;
        ak = ak*a_big.clone();
            panic!("k greater than {}",max_k);
        }
    }
    k
}

pub fn problem_026() -> usize {
    let n: usize = 1000;
    let repetend_lengths: Vec<u32> = (2..n).map(|p| multiplicative_order(10, p as u64)).collect();
    (0..(n-2)).max_by_key(|&x| repetend_lengths[x]).unwrap() + 2
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_026() {
        let ans: usize = problem_026();
        println!("Answer to Problem 26: {}", ans);
        assert!(ans == 983)
    }

    #[bench]
    fn bench_problem_026(b: &mut Bencher) {
        b.iter(|| problem_026());
    }
    
}
