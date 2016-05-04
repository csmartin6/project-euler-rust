use utils;

pub fn problem_025() -> usize {
	let num_digits  = 1000;
	let mut fib = vec![vec![1],vec![1]];
	let mut i = 1;
	while fib[i].len() < num_digits {
	    let next = utils::add_digit_array(&fib[i],&fib[i - 1]);
	    fib.push(next);
        i += 1;
	}
    return i+1;
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_025() {
        let ans: usize = problem_025();
        println!("Answer to Problem 25: {}", ans);
        assert!(ans == 4782)
    }

    #[bench]
    fn bench_problem_025(b: &mut Bencher) {
        b.iter(|| problem_025());
    }
    
}
