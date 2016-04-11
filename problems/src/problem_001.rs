pub fn problem_001() -> usize {
    let mut s: usize = 0;

    for x in 0..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            s += x;
        }
    }
    s
}
#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;
	
	#[test]
	fn test_problem_001() {
	    let ans: usize = problem_001();
	    println!("Answer to Problem 1: {}", ans);
	    assert!(ans == 233168)
	}

	#[bench]
	fn bench_problem_001(b: &mut Bencher) {
	    b.iter(|| problem_001());
	}

}