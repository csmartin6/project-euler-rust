pub fn problem_040() -> u32 {
    let ten: u32 = 10;
    
    let nths = vec![1,10,100,1000,10000,100000,1000000];
    let mut digits = vec![];
    let mut prev_number_end = 0;
    let mut curr_magnitude = 1;
    let mut curr_base_number = 1;
    let mut remaining_numbers = 9;

    for n in nths {
        let mut digits_to_go = n - prev_number_end;

        if digits_to_go > curr_magnitude * remaining_numbers {
            digits_to_go -= curr_magnitude * remaining_numbers;
            curr_magnitude += 1;
            remaining_numbers = 9 * ten.pow(curr_magnitude-1);
            curr_base_number = ten.pow(curr_magnitude-1);
        }
        
        let numbers_past_base = digits_to_go / curr_magnitude;
        let digit_in_number: usize = (digits_to_go % curr_magnitude) as usize;
        remaining_numbers -= numbers_past_base+1;
        
        let nth_digit: u8 = match digit_in_number {
            0 => (curr_base_number + numbers_past_base -1).to_string().pop().unwrap() as u8 - ('0' as u8),
            _ => (curr_base_number + numbers_past_base).to_string().chars().nth(digit_in_number-1).unwrap() as u8 - ('0' as u8),
        };
        
        prev_number_end = n + (curr_magnitude-(digit_in_number as u32));
        digits.push(nth_digit);
    }
    digits.iter().fold(1,|a,&b| a * (b as u32)) 
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_040() {
        let ans: u32 = problem_040();
        println!("Answer to Problem 40: {}", ans);
        assert!(ans == 210)
    }

    #[bench]
    fn bench_problem_040(b: &mut Bencher) {
        b.iter(|| problem_040());
    }
}
