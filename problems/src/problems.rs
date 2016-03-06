use utils;
use std::collections::HashMap;

pub fn problem_001() -> i64 {
    let mut s: i64 = 0;

    for x in 0..1000{
        if x % 3 == 0 || x % 5 ==0{
            s += x;
        }
    }
    s
}

pub fn problem_002() -> i64 {
    let n = 4000000;

    let mut fib : Vec<i64> = vec![1,1];

    loop {
        let f_2 = fib[fib.len()-2];
        let f_1 = fib[fib.len()-1];
        if f_1 + f_2 > n {
            break;
        }
        fib.push(f_2 + f_1);

    }

    let mut sum = 0;
    for x in fib {
        if x % 2 == 0 {
            sum += x;
        }
            
    }
    return sum
}

pub fn problem_003() -> u64 {
    let n: u64 = 600851475143;
    *utils::prime_factors(n).iter().max().unwrap()
}




pub fn problem_004() -> u64 {

	let mut max_palindrome: u64 = 0;
	for (x, y) in iproduct!(900..1000, 900..1000) {
    	if  x*y > max_palindrome && utils::is_palindrome(x*y) {
    		max_palindrome = x*y;
    	}
    }
    max_palindrome
}

pub fn problem_005() -> u64 {
	let n = 21;

	let mut cumulative_prime_factors: HashMap<u64,u64> = HashMap::new();

	for i in 1..n {
		let factors = utils::prime_factors(i);
		let factor_count = utils::counts(&factors[..]);
		for (k,&v) in factor_count.iter(){
			if !cumulative_prime_factors.contains_key(k) || factor_count[k] > cumulative_prime_factors[k] {
				cumulative_prime_factors.insert(*k, v);
			}
		}
	}

	let mut product = 1;

	for (k,&v) in cumulative_prime_factors.iter(){
		product *= k.pow(v as u32);
	}

	product
}