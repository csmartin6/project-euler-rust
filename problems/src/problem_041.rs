use utils;
use std::collections::HashSet;
use std::char;

fn is_pandigital(n: u64) -> bool{
    let s = n.to_string();
    let mut ns = HashSet::new();
    for d in s.chars(){
        if ns.contains(&d) {
            return false;
        }
        ns.insert(d);
    }

    for i in 1..(s.len()+1) {

        // 48 is the char offset for '0'
        let c = char::from_u32(i as u32 + 48).unwrap(); 
        if !ns.contains(&c){
            return false;
        }
    }
    true
}




pub fn problem_041() -> usize {
    
    // pandigital numbers with 9 or 8 digits cannot be prime
    // because the digits will sum to 45 or 36 respectively so 
    // they are divisible by 3

    let primes = utils::prime_sieve(7654322);
    for &p in primes.iter().rev(){
        if is_pandigital(p as u64){
            return p as usize 
        }
    }
    0
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_041() {
        let ans: usize = problem_041();
        println!("Answer to Problem 41: {}", ans);
        assert!(ans == 7652413)
    }

    #[bench]
    fn bench_problem_041(b: &mut Bencher) {
        b.iter(|| problem_041());
    }
}
