pub fn problem_019() -> u32 {

    let month_lengths: Vec<u32> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut sundays: u32 = 0;
    let mut day_of_week: u32 = 1;  // Jan 1 1901 was a Tuesday

    for year in 1901..2001 {
        for &month_length in month_lengths.iter(){
            if day_of_week == 6 {
                sundays += 1;
            }

            day_of_week = (day_of_week + month_length) % 7;

            // Account for Leap Years
            if  month_length == 28 && ((year % 4 == 0 && !(year % 100 == 0))|| year % 400 == 0){
                day_of_week = (day_of_week + 1) % 7;
            }
        }
    }
    sundays
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_019() {
        let ans: u32 = problem_019();
        println!("Answer to Problem 19: {}", ans);
        assert!(ans == 171)
    }

    #[bench]
    fn bench_problem_019(b: &mut Bencher) {
        b.iter(|| problem_019());
    }


}
