use utils;

pub fn problem_020() -> u32 {
    let n = 100;
    let mut fact: Vec<u32> = vec![1];

    for i in 1..(n + 1) {
        fact = utils::scalar_multiply_digit_array(&fact[..], i);
    }
    fact.iter().fold(0, |a, &b| a + b)
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_020() {
        let ans: u32 = problem_020();
        println!("Answer to Problem 20: {}", ans);
        assert!(ans == 648)
    }

    #[bench]
    fn bench_problem_020(b: &mut Bencher) {
        b.iter(|| problem_020());
    }


}
