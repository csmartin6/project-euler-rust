use utils;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn problem_049() -> String {
    let primes: Vec<usize> = utils::prime_sieve(10000).into_iter().filter(|&p| p > 1000).collect();
    let mut primes_by_sorted_digits: HashMap<Vec<u32>,Vec<u32>> = HashMap::new();
    for &p in primes.iter(){
        let mut s: Vec<u32> = utils::as_digit_array(p as u64);
        s.sort();
        primes_by_sorted_digits.entry(s).or_insert(vec![]).push(p as u32);
    }

    for (_digits, ref primes) in primes_by_sorted_digits.iter() {
        if primes.len() >= 3 {
            let primes_set: HashSet<&u32> = HashSet::from_iter(primes.iter());
            let primes_copy = primes.to_owned();
            for (i,&a) in primes_copy.iter().enumerate() {
                let rest = primes_copy.to_owned().split_off(i+1);
                for &b in rest.iter(){
                    let c = 2*b -a;
                    if primes_set.contains(&c) & (c!=8147) {
                        let seq: Vec<String> = vec![a.to_string(),b.to_string(),c.to_string()];
                        return seq.concat();
                    }
                }
            }
        }
    }


    "error".to_string()
}   



#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_049() {
        let ans: String = problem_049();
        println!("Answer to Problem 49: {}", ans);
        assert!(&ans == "296962999629")
    }


    #[bench]
    fn bench_problem_049(b: &mut Bencher) {
        b.iter(|| problem_049());
    }
}
