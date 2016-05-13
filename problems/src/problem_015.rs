

pub fn problem_015() -> u64 {
    let n = 20;

    let mut num_paths: Vec<Vec<u64>> = vec![vec![0; (n+1)];(n+1)];

    num_paths[0] = vec![1; (n+1)];
    for i in 0..(n + 1) {
        num_paths[i][0] = 1;
    }

    for i in 1..(n + 1) {
        for j in 1..(n + 1) {
            let from_left = num_paths[i][j - 1];
            let from_above = num_paths[i - 1][j];
            num_paths[i][j] = from_left + from_above;
        }
    }
    num_paths[n][n]

}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_015() {
        let ans: u64 = problem_015();
        println!("Answer to Problem 15: {}", ans);
        assert!(ans == 137846528820)
    }

    #[bench]
    fn bench_problem_015(b: &mut Bencher) {
        b.iter(|| problem_015());
    }
}
