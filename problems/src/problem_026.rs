use num::Integer;
use num::bigint::BigInt;

pub fn multiplicative_order(a: u32, n: u32) -> u32:
    if a.gcd(&n) != 1{
        return 0;
    }
    let max_k = 1000000;
    let k = 0;
    let ak = a;
    while ak % n != 1 {
        k += 1;
        ak *= a;
        if k > max_k{
            panic!("k greater than {}",max_k));
        }
    }
    k
}

pub fn problem_026() -> usize {
    let n = 1000;
    let repetend_lengths = (2..n).map(|p| multiplicative_order(10, p)).collect();
    

    return np.argmax(repetend_lengths)+2
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
