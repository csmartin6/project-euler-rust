use std::collections::HashSet;
use utils;

pub fn problem_032() -> u32 {
    // 2 possible multipulcation setups
    // 2 * 3 * 4
    // 1 * 4 * 4

    // # 2 * 3

    let mut pandigital = HashSet::new();
    let zero: u32 = 0;

    for a in 12..99{
        let mut a_digits:  HashSet<u32> = HashSet::new();
        for &d in utils::to_digit_array(a).iter(){
            a_digits.insert(d);
        }
        if a_digits.len() == 2 {
            for b in 123..988{
                let mut digits: HashSet<u32> = HashSet::new();
                for &d in a_digits.iter() {
                    digits.insert(d);
                }
                for &d in utils::to_digit_array(b).iter(){
                    digits.insert(d);
                }
                if digits.len() == 5{
                    let m = a * b;
                    for &d in utils::to_digit_array(m).iter() {
                        digits.insert(d);
                    }
                    if !(digits.contains(&zero)) & (digits.len() == 9) & (m < 10000){
                        pandigital.insert(m as u32);
                    }
                }

            }
        }
    }

    for a in 1..10{
        let mut a_digits: HashSet<u32> = HashSet::new();
        for &d in utils::to_digit_array(a).iter(){
            a_digits.insert(d);
        }
        for b in 1234..9877{
            let mut digits: HashSet<u32> = HashSet::new();
            for &d in a_digits.iter() {
                digits.insert(d);
            }
            for d in utils::to_digit_array(b) {
                digits.insert(d);
            }
            if digits.len() == 5{
                let m = a * b;
                for d in utils::to_digit_array(m) {
                    digits.insert(d);
                }
                if !(digits.contains(&zero)) & (digits.len() == 9) & (m < 10000){
                    pandigital.insert(m as u32);
                }
            }
        }
    }
    pandigital.iter().fold(0,|a,&b| a + b)
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_032() {
        let ans: u32 = problem_032();
        println!("Answer to Problem 32: {}", ans);
        assert!(ans == 45228)
    }

    #[bench]
    fn bench_problem_032(b: &mut Bencher) {
        b.iter(|| problem_032());
    }

}


