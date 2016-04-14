use utils;

pub fn problem_016() -> u32 {
    let n = 1000;
    let mut digits: Vec<u32> = vec![1];
    for _ in 0..n {
    	digits = utils::add_digit_array(&digits[..], &digits[..])
    }

    digits.iter().fold(0,|a,&b| a + b)
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_016() {
        let ans: u32 = problem_016();
        println!("Answer to Problem 16: {}", ans);
        assert!(ans == 1366)
    }

    #[bench]
    fn bench_problem_016(b: &mut Bencher) {
        b.iter(|| problem_016());
    }
}


