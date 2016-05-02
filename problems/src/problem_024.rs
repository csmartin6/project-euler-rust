use utils;

pub fn find_nth_ordering(n: u64, digits: Vec<u32>, seq_length: usize) -> Vec<u32>{
    let num_orderings = utils::factorial(seq_length as u64 - 1);
    if seq_length == 1 {
        return vec![digits[n as usize]];
    }
    
    for (index, &digit) in digits.iter().enumerate(){
        if (index as u64 + 1) * num_orderings > n {
            
            let mut remaining_digits: Vec<u32> = digits.clone();
            remaining_digits.remove(index);

            let mut ordering: Vec<u32>=  vec![digit];
            ordering.extend(find_nth_ordering(n - ((index as u64) * num_orderings), remaining_digits.clone(), seq_length - 1));
            return ordering;
        }
    }
    panic!("Something has gone wrong");
}
    

pub fn problem_024() -> u64 {
    let d: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let seq_length = d.len();
    let nth: u64 = 999999;

    return utils::from_digit_array(find_nth_ordering(nth,d,seq_length));
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_024() {
        let ans: u64 = problem_024();
        println!("Answer to Problem 24: {}", ans);
        assert!(ans == 2783915460)
    }

    #[bench]
    fn bench_problem_024(b: &mut Bencher) {
        b.iter(|| problem_024());
    }
    
}



