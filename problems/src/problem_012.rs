use utils;

pub fn problem_012() -> u64{
	let n = 500;
    let mut num = 1;
    let mut i = 1;
    while utils::count_divisors(num) < n {
        i += 1;
        num += i;
    }
    num
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;
    
    #[test]
    fn test_problem_012() {
        let ans: u64 = problem_012();
        println!("Answer to Problem 10: {}", ans);
        assert!(ans == 76576500)
    }

    #[bench]
    fn bench_problem_012(b: &mut Bencher) {
        b.iter(|| problem_012());
    }    
}