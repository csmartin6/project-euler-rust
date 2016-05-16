use utils;
use num::Integer;


pub fn problem_033() -> u32 {
    let mut digit_cancelling_fractions: Vec<(u32,u32)>= vec![];
    for i in 10..100 {
        for j in (i+1)..100 {
            let gcd = i.gcd(&j);
            let value = (i/gcd as u32, j/gcd as u32);
            let numerator = utils::to_digit_array(i as u64);
            let denominator = utils::to_digit_array(j as u64);
            for (i,&d) in numerator.iter().enumerate(){
                if (d != 0) & denominator.contains(&d) {
                    let mut new_numerator = numerator.clone();
                    new_numerator.remove(i);
                    let mut new_denominator = denominator.clone();
                    if new_denominator[0] == d {
                        new_denominator.remove(0);
                    } else {
                        new_denominator.remove(1);
                    }
                    
                    if new_denominator[0] != 0 {
                        let new_gcd = new_numerator[0].gcd(&new_denominator[0]);
                        let new_value = (new_numerator[0]/new_gcd , new_denominator[0]/new_gcd);
                            
                        if (new_value.0 == value.0) & (new_value.1 == value.1) {
                            digit_cancelling_fractions.push(value);
                        }
                    }
                    // println!("i: {}, j: {}",i,j);
                }
            }
        }
    }
    //println!("digit_cancelling_fractions: {:?}",digit_cancelling_fractions);
    
    let prod = digit_cancelling_fractions.iter().fold((1,1),|a,b| (a.0*b.0,a.1*b.1));
    prod.1 / (prod.0).gcd(&prod.1)
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_033() {
        let ans: u32 = problem_033();
        println!("Answer to Problem 33: {}", ans);
        assert!(ans == 100)
    }

    #[bench]
    fn bench_problem_033(b: &mut Bencher) {
        b.iter(|| problem_033());
    }
}


