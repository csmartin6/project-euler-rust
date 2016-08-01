use utils;

pub fn problem_030() -> usize {

    let mut numbers: Vec<u32> = vec![];
    for num in 2..250000 {
        let digits = utils::as_digit_array(num);
        let sum_of_fifth_powers = digits.iter().fold(0, |a, &b| a + b.pow(5));

        if sum_of_fifth_powers == num as u32 {
            numbers.push(num as u32)
        }
    }
    numbers.iter().fold(0, |a, &b| a + b as usize)
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_030() {
        let ans: usize = problem_030();
        println!("Answer to Problem 30: {}", ans);
        assert!(ans == 443839)
    }

    #[bench]
    fn bench_problem_030(b: &mut Bencher) {
        b.iter(|| problem_030());
    }

}
