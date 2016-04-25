use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, Error};
use std::cmp;

pub fn read_traingle() -> Result<Vec<Vec<u32>>, Error> {
    let f = try!(File::open("../data/problem_018_input.txt"));
    let reader = BufReader::new(f);

    let mut nums = vec![];

    for line in reader.lines() {
        let line = try!(line);
        let row: Vec<u32> = line.split_whitespace().map(|d| d.parse::<u32>().unwrap()).collect();
        nums.push(row);
    }

    Ok(nums)
}


pub fn max_path(triangle: Vec<Vec<u32>>) -> u32 {

    let mut max_sum: Vec<Vec<u32>> = vec![];
    for (index, row) in triangle.iter().enumerate() {
        if index == 0 {
            max_sum.push(row.to_owned());
        } else {
            let mut row_max = vec![];
            for (pos, &value) in row.clone().iter().enumerate() {

                let left = match pos {
                    0 => 0,
                    _ => max_sum[index - 1][pos - 1],
                };
                let right = match pos {
                    x if x == index => 0,
                    _ => max_sum[index - 1][pos],
                };
                row_max.push(value + cmp::max(left, right));
            }
            max_sum.push(row_max)
        }
    }
    *max_sum.last().unwrap().iter().max().unwrap()
}

pub fn problem_018() -> u32 {
    let triangle = read_traingle().unwrap();
    max_path(triangle)
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_018() {
        let ans: u32 = problem_018();
        println!("Answer to Problem 18: {}", ans);
        assert!(ans == 1074)
    }

    #[bench]
    fn bench_problem_018(b: &mut Bencher) {
        b.iter(|| problem_018());
    }


}
