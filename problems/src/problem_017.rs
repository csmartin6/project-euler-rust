use utils;

pub fn problem_017() -> u32 {
    let n = 1001;

    (1..n)
        .map(|i| utils::number_to_words(i).replace(" ", "").replace("-", ""))
        .fold(0, |a, b| a + b.len() as u32)
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_017() {
        let ans: u32 = problem_017();
        println!("Answer to Problem 17: {}", ans);
        assert!(ans == 21124)
    }

    #[bench]
    fn bench_problem_017(b: &mut Bencher) {
        b.iter(|| problem_017());
    }


}
