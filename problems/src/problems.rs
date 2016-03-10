use utils;
use std::collections::HashMap;


pub fn problem_001() -> usize {
    let mut s: usize = 0;

    for x in 0..1000{
        if x % 3 == 0 || x % 5 ==0{
            s += x;
        }
    }
    s
}

pub fn problem_002() -> usize {
    let n = 4000000;

    let mut fib : Vec<usize> = vec![1,1];

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

pub fn problem_003() -> usize {
    let n: usize = 600851475143;
    *utils::prime_factors(n).iter().max().unwrap()
}




pub fn problem_004() -> usize {

	let mut max_palindrome: usize = 0;
	for (x, y) in iproduct!(900..1000, 900..1000) {
    	if  x*y > max_palindrome && utils::is_palindrome(x*y) {
    		max_palindrome = x*y;
    	}
    }
    max_palindrome
}

pub fn problem_005() -> usize {
	let n = 21;

	let mut cumulative_prime_factors: HashMap<usize,usize> = HashMap::new();

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

pub fn problem_006() -> usize {
    let n = 100;
    let sum_of_squares: usize = (1..n+1).map(|x| x*x).fold(0, |sum, x| sum + x);
    let square_of_sum: usize = (1..n+1).fold(0, |sum, x| sum + x).pow(2);
    square_of_sum - sum_of_squares
}

pub fn problem_007() -> usize {
    let mut primecount: usize = 0;
    let mut n: usize = 1;

    while primecount < 10001 {
        n += 1;
        if utils::is_prime(n){
            primecount += 1;
        }
       
    }
    n
}

pub fn problem_007_sieve() -> usize {
    let n = 10001;
    let max_candidate = 2 * n * (n as f64).ln() as usize;

    let primes = utils::prime_sieve(max_candidate);
    primes[10000]
}




