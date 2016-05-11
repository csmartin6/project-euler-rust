
fn ways_to_make_change(total: u32, coins: &[u32]) -> u32 {
    if total == 0 {
        return 0
    }

    let mut result = 0;
    for (i, &c) in coins.iter().enumerate(){
        if c == total {
            result += 1;
        } else if c <= total {
            result += ways_to_make_change(total - c, &coins[i..]); 
        }
    }
    result
}

pub fn problem_031() -> u32 {
    let coins = vec![200, 100, 50, 20, 10, 5, 2, 1];
    let change = 200;
    ways_to_make_change(change, &coins)
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_031() {
        let ans: u32 = problem_031();
        println!("Answer to Problem 31: {}", ans);
        assert!(ans == 73682)
    }

    #[bench]
    fn bench_problem_031(b: &mut Bencher) {
        b.iter(|| problem_031());
    }

}


