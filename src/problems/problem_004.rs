use test::Bencher;
use utils;

use itertools::Itertools;

fn is_palindrome(num: u64) -> bool {
	let digits = utils::to_digit_array(num);
	let n = digits.len();
	for i in 0 .. n/2 {
		if digits[i] != digits[n-i-1]{
			return false;
		}
	}
	true
}


fn problem_004() -> u64 {

	let mut max_palindrome: u64 = 0;
	for (x, y) in iproduct!(900..1000, 900..1000) {
    	if  x*y > max_palindrome && is_palindrome(x*y) {
    		max_palindrome = x*y;
    	}
    }
    max_palindrome
}

#[test]
fn test_problem_004() {
    let ans: u64 = problem_004();
    println!("Answer to Problem 4: {}",ans);
    assert!(ans == 906609)
}


#[bench]
fn bench_problem_004(b: &mut Bencher) {
        b.iter(|| problem_004());
}
