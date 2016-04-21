use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, Error};
use utils;

fn read_numbers_as_digit_array() -> Result<Vec<Vec<u32>>, Error> {
    let f = try!(File::open("../data/problem_013_input.txt"));
    let reader = BufReader::new(f);

    let mut nums = vec![vec![]];

    for line in reader.lines() {
        let line = try!(line);
        let row: Vec<u32> = line.chars().map(|d| d.to_string().parse::<u32>().unwrap()).collect();
        nums.push(row);
    }

    Ok(nums)
}

pub fn problem_013() -> u64 {
    let nums = read_numbers_as_digit_array().unwrap();

    let initial_sum = vec![0];
    let sum_digit_array = nums.iter().fold(initial_sum,|acc,ref b| utils::add_digit_array(&acc, b));
    let first_10_digits: Vec<u32> = sum_digit_array[0..10].to_vec();
    utils::from_digit_array(first_10_digits)
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;


    #[test]
    fn test_problem_013() {
        let ans: u64 = problem_013();
        println!("Answer to Problem 13: {}", ans);
        assert!(ans == 5537376230)
    }



    #[bench]
    fn bench_problem_013(b: &mut Bencher) {
        b.iter(|| problem_013());
    }
}
