use std::collections::HashSet;

pub fn problem_039() -> usize {
    // generate triples using brute force
    let mut triples = HashSet::new();
    let mut counter = vec![0;1000];
    for a in 2..333 {
        for b in a..500 {
            let sum_of_squares: usize = a * a + b * b;
            let c = (sum_of_squares as f64).sqrt() as usize;
            if (a + b + c < 1000) & (c * c == sum_of_squares) {
                counter[a + b + c] += 1;
                triples.insert((a, b, c));
            }
        }
    }

    counter.iter()
           .enumerate()
           .fold((0, 0),
                 |(max_idx, max_count), (i, &count)| 
                 if count > max_count {
                     (i, count)
                 } else {
                     (max_idx, max_count)
                 })
           .0
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_039() {
        let ans: usize = problem_039();
        println!("Answer to Problem 39: {}", ans);
        assert!(ans == 840)
    }

    #[bench]
    fn bench_problem_039(b: &mut Bencher) {
        b.iter(|| problem_039());
    }
}
