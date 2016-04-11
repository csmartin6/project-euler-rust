pub fn problem_006() -> usize {
    let n = 100;
    let sum_of_squares: usize = (1..n + 1).map(|x| x * x).fold(0, |sum, x| sum + x);
    let square_of_sum: usize = (1..n + 1).fold(0, |sum, x| sum + x).pow(2);
    square_of_sum - sum_of_squares
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_006() {
        let ans: usize = problem_006();
        println!("Answer to Problem 6: {}", ans);
        assert!(ans == 25164150)
    }

    #[bench]
    fn bench_problem_006(b: &mut Bencher) {
        b.iter(|| problem_006());
    }
}
