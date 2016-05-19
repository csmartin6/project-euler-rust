
fn is_palindrome(s: &String) -> bool {

    for (a,b) in s.chars().zip(s.chars().rev()){
        if a != b {
            return false;
        }
    }
    true
}


pub fn problem_036() -> u32 {
    let n = 1000000;
    let mut palindromes: Vec<u32> = vec![];

    for i in 0..n {
        let base_10 = format!("{}",i);
        let base_2 = format!("{:b}", i);
        if is_palindrome(&base_10) & is_palindrome(&base_2){
            palindromes.push(i as u32);
        }

    }

    palindromes.iter().fold(0,|a,&b| a + b)
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_036() {
        let ans: u32 = problem_036();
        println!("Answer to Problem 36: {}", ans);
        assert!(ans == 872187)
    }

    #[bench]
    fn bench_problem_036(b: &mut Bencher) {
        b.iter(|| problem_036());
    }
}


