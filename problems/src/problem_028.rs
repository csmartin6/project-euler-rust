
pub fn problem_028() -> u64 {
    let n = 1001;
    let mut s = 1;
    for a in (3 ..(n + 1)).step_by(2){
        let corners = (((a - 2) * (a - 2)+a-1)..(a*a+1)).step_by(a - 1);
        s += corners.fold(0,|a,b| a+b);
    }
    s
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_028() {
        let ans: u64= problem_028();
        println!("Answer to Problem 28: {}", ans);
        assert!(ans == 669171001)
    }

    #[bench]
    fn bench_problem_028(b: &mut Bencher) {
        b.iter(|| problem_028());
    }

}


