use std::collections::HashSet;

pub fn problem_038() -> u32 {
    let zero = '0';
    for i in (9123..9876).rev() {
        let concat_product = format!("{}{}",i,2*i);
        let mut digits: HashSet<char> = HashSet::new();
        for c in concat_product.chars() {
            digits.insert(c);
        }
        if (digits.len() == 9) & !digits.contains(&zero) {
            return concat_product.parse::<u32>().unwrap();
        }
    }
    0
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_038() {
        let ans: u32 = problem_038();
        println!("Answer to Problem 38: {}", ans);
        assert!(ans == 932718654)
    }

    #[bench]
    fn bench_problem_038(b: &mut Bencher) {
        b.iter(|| problem_038());
    }
}
