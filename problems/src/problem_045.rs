pub fn problem_045() -> u64 {
    let n:u64 = 100000000000;
    let mut triangular =  (1..n).map(|i| i * (i + 1)/2 );
    let mut pentagonal =  (1..n).map(|i| i * (3 * i - 1)/2 );
    let hexagonal =  (1..n).map(|i| i * (2 * i - 1));
   
    let mut p: u64 = 40756;
    let mut t: u64 = 40756;

    for h in hexagonal {
        while p < h {
            match pentagonal.next(){
                Some(x) => {p = x;},
                None => break,
            }
        }

        while t < h {
            match triangular.next(){
                Some(x) => {t = x;},
                None => break,
            }
        }

        if (p == h) & (t==h) {
            return h
        }
    }

    0
}



#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_045() {
        let ans: u64 = problem_045();
        println!("Answer to Problem 45: {}", ans);
        assert!(ans == 1533776805)
    }


    #[bench]
    fn bench_problem_045(b: &mut Bencher) {
        b.iter(|| problem_045());
    }
}



