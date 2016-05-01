use std::collections::HashSet;
use utils;
use itertools::Itertools;

pub fn problem_023() -> u64 {
    let n: u64 = 28124;
    
    let abundant_numbers: Vec<u64> = (1..n).filter(|&x| utils::proper_divisors(x).iter().fold(0,|a,&b| a+b) > x).collect();
    let mut sum =  HashSet::new();

    for num_a in abundant_numbers.clone().into_iter(){
    	for num_b in abundant_numbers.clone().into_iter(){
    		if num_a+num_b < n {
    			sum.insert(num_a+num_b);
    		}
    	}
    }
    let sum_abundant_numbers: u64 = sum.iter().fold(0,|a, &b| a + b);
    
    (n*(n-1)/2 as u64) - sum_abundant_numbers
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_023() {
        let ans: u64 = problem_023();
        println!("Answer to Problem 23: {}", ans);
        assert!(ans == 4179871)
    }

    #[bench]
    fn bench_problem_023(b: &mut Bencher) {
        b.iter(|| problem_023());
    }
    
}




