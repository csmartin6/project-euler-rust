pub fn next_row(row: &Vec<u32>) -> Vec<u32>{
    let mut prev_elem = 0;
    let mut next_row: Vec<u32> = vec![];
    for &elem in row.iter(){
        next_row.push((prev_elem + elem) % 7);
        prev_elem = elem;
    }
    next_row.push(1);
    next_row
}


pub fn problem_148() -> usize {
	let n: u32 = 1000000000;
    let mut count = 1;
    let mut curr_row = vec![1];
    for i in 2..(n+1) {
        let next_row = next_row(&curr_row);
        //println!("next_row {:?}",next_row);
        curr_row = next_row.to_owned();
        count += next_row.iter().filter(|x| **x > 0).count();
        match i {
            7 => println!("row: {}, count non divisible by 7: {}",i,count),
            14 => println!("row: {}, count non divisible by 7: {}",i,count),
            49 => println!("row: {}, count non divisible by 7: {}",i,count),
            98 => println!("row: {}, count non divisible by 7: {}",i,count),
            343 => println!("row: {}, count non divisible by 7: {}",i,count),
            686 => println!("row: {}, count non divisible by 7: {}",i,count),
            2401 => println!("row: {}, count non divisible by 7: {}",i,count),
            4802 => println!("row: {}, count non divisible by 7: {}",i,count),
            16807 => println!("row: {}, count non divisible by 7: {}",i,count),
            33614 => println!("row: {}, count non divisible by 7: {}",i,count),
            _ => {},
        } 
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_148() {
        let ans: usize = problem_148();
        println!("Answer to Problem 148: {}", ans);
        assert!(ans == 2361)
    }

    #[bench]
    fn bench_problem_148(b: &mut Bencher) {
        b.iter(|| problem_148());
    }
    
}
