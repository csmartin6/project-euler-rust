use std::collections::HashSet;
use std::iter::FromIterator;
use std::u32;
use std::cmp;

pub fn problem_044() -> u32 {

    let pentagonal: HashSet<u32> = HashSet::from_iter((1..10000).map(|n| n * (3 * n - 1) / 2 as u32));
    let mut min_diff = u32::MAX;


    for i in 2..10000 {
        let pent_i: u32 = i * (3 * i - 1) / 2;
        let mut min_j: u32 = 1;
        if pent_i > min_diff {
            min_j = (-0.5/3.0  + (0.25 - 6.0 * ((pent_i - min_diff) as f64).sqrt())/3.) as u32;
            min_j = cmp::max(1,min_j);
        }

        for j in min_j..i {
            let pent_j: u32 = j * (3 * j - 1) / 2;
            let diff: u32 = pent_i - pent_j;
            let sum: u32= pent_i + pent_j;

            if pentagonal.contains(&diff) & 
                pentagonal.contains(&sum) 
            {
                if diff < min_diff {
                    min_diff = diff;
                }
            }

        }
    }
    min_diff
}



#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_044() {
        let ans: u32 = problem_044();
        println!("Answer to Problem 44: {}", ans);
        assert!(ans == 5482660)
    }


    #[bench]
    fn bench_problem_044(b: &mut Bencher) {
        b.iter(|| problem_044());
    }
}



