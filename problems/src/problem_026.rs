use num::Integer;

pub fn multiplicative_order(a: u64, n: u64) -> u32 {
    if (a.gcd(&n) != 1) | (n <= 1){
        return 0;
    }

    let mut k = 0;
    let mut a_k = a;
    while a_k % n != 1 {
        k += 1;
        a_k = (a_k % n) * (a % n);
    }
    k
}


pub fn problem_026() -> usize {
    let n: usize = 1000;
    let repetend_lengths: Vec<u32> = (0..n).map(|p| multiplicative_order(10, p as u64)).collect();
    (0..n).max_by_key(|&x| repetend_lengths[x]).unwrap()
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
