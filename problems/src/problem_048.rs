use utils;


pub fn problem_048() -> u64 {
    let n = 1000;

    let mut last_ten_digits = vec![0];

    for i in 1..(n+1){
        let digits = utils::as_digit_array(i);
        let product = utils::power_digit_array_truncated(&digits, i as u32, 10);
        last_ten_digits = utils::add_digit_array(&last_ten_digits, &product);
        
        if last_ten_digits.len() > 10{
            let excess = last_ten_digits.len() - 10;
            last_ten_digits = last_ten_digits.split_off(excess);
        }
        
    }
    utils::from_digit_array(last_ten_digits)

}   



#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_048() {
        let ans: u64 = problem_048();
        println!("Answer to Problem 48: {}", ans);
        assert!(ans == 9110846700)
    }


    #[bench]
    fn bench_problem_048(b: &mut Bencher) {
        b.iter(|| problem_048());
    }
}
