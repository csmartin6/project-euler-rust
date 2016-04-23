fn next_collatz(n: usize) -> usize{
    match n % 2{
        0 => n/2,
        _ => 3 * n + 1,
    }
}

pub fn problem_014() -> usize {
    let n = 1000000;
    let mut collatz: Vec<usize> = vec![0;(n+1)];

    let mut max_seq = 0;
    let mut arg_max = 0;
    collatz[1] = 1;

    for i in 2..(n+1) {
        let mut x = i ;
        while x != 1{
            if x < i {
                collatz[i] += collatz[x];
                if collatz[i] > max_seq {
                    max_seq = collatz[i];
                    arg_max = i; 
                }
                break; 
            } else {
                x = next_collatz(x);
                collatz[i] += 1;
            }
        }
    }
    arg_max
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_014() {
        let ans: usize = problem_014();
        println!("Answer to Problem 14: {}", ans);
        assert!(ans == 837799)
    }

    #[bench]
    fn bench_problem_014(b: &mut Bencher) {
        b.iter(|| problem_014());
    }
}
