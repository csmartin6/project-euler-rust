pub fn problem_009() -> u64 {
    let n = 1000;

    for i in 0..n {
        for j in i..((n - 1) / 2) {
            let k = n - i - j;
            if i * i + j * j == k * k {
                return i * j * k;
            }
        }
    }
    0
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;
    
    #[test]
    fn test_problem_009() {
        let ans: u64 = problem_009();
        println!("Answer to Problem 9: {}", ans);
        assert!(ans == 31875000)
    }

    #[bench]
    fn bench_problem_009(b: &mut Bencher) {
        b.iter(|| problem_009());
    }    
}