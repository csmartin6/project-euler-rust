use std::collections::{HashMap,HashSet};
use utils;

pub fn problem_021() -> u64 {
    let n = 10001;
    let mut proper_divisor_sums = HashMap::new();
    let mut amicable_numbers = HashSet::new();

    for num in 0..n{
        let proper_divisor_sum = utils::proper_divisors(num).iter().fold(0,|a,&b| a + b);
        proper_divisor_sums.insert(num,proper_divisor_sum);

        let mut c_divisor_sum = 0;
        if proper_divisor_sums.contains_key(&proper_divisor_sum){
            c_divisor_sum = *proper_divisor_sums.get(&proper_divisor_sum).unwrap();
        }else{
            c_divisor_sum = utils::proper_divisors(proper_divisor_sum).iter().fold(0,|a,&b| a + b);
            proper_divisor_sums.insert(proper_divisor_sum, c_divisor_sum);
        }


        if (num == c_divisor_sum) && num != proper_divisor_sum {
            amicable_numbers.insert(num);
            amicable_numbers.insert(proper_divisor_sum);
        }

    }


    return amicable_numbers.iter().fold(0,|a,&b| a + b)
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_021() {
        let ans: u64 = problem_021();
        println!("Answer to Problem 21: {}", ans);
        assert!(ans == 31626)
    }

    #[bench]
    fn bench_problem_021(b: &mut Bencher) {
        b.iter(|| problem_021());
    }


}
