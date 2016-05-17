use utils;

pub fn problem_034() -> u32 {
    let mut digit_factorials = vec![];
    let factorials: Vec<u32> = (0..10).map(|x| utils::factorial(x as u64) as u32).collect();
    let max: u32 = 7*factorials[9];
    for x in 10..max {
        let digits = utils::as_digit_array(x as u64);
        let sum_factorial = digits.iter().fold(0, |a,b| a + factorials[*b as usize]);
        if sum_factorial == x {
            digit_factorials.push(x)
        }
    }
    digit_factorials.iter().fold(0,|a,b| a + b)
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_034() {
        let ans: u32 = problem_034();
        println!("Answer to Problem 34: {}", ans);
        assert!(ans == 40730)
    }

    #[bench]
    fn bench_problem_034(b: &mut Bencher) {
        b.iter(|| problem_034());
    }
}


