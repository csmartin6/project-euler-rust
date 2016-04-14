use utils;

pub fn problem_004() -> usize {

    let mut max_palindrome: usize = 0;
    for (x, y) in iproduct!(900..1000, 900..1000) {
        if x * y > max_palindrome && utils::is_palindrome(x * y) {
            max_palindrome = x * y;
        }
    }
    max_palindrome
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_004() {
        let ans: usize = problem_004();
        println!("Answer to Problem 4: {}", ans);
        assert!(ans == 906609)
    }

    #[bench]
    fn bench_problem_004(b: &mut Bencher) {
        b.iter(|| problem_004());
    }
}
