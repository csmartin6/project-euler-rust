use std::collections::HashSet;
use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, Error};
use std::iter::FromIterator;

pub fn read_words() -> Result<Vec<String>,Error> {
    let f = try!(File::open("../data/p042_words.txt"));
    let reader = BufReader::new(f);

    let mut words: Vec<String> = vec![];

    for line in reader.lines() {
        let line = try!(line);
        let line = line.replace("\"","");
        words = line.split(",").map(|x| x.to_string()).collect();
    }
    Ok(words)
}

fn is_triangular(word: &str, triangular: &HashSet<u32>) -> bool{
    let ascii_offset = 64; // 'A' => 65
    let word_score =  word.chars().map(|c| (c as u8 - ascii_offset)).fold(0, |a,b| a + b as u32); 
    triangular.contains(&word_score)
}




pub fn problem_042() -> usize {
    let triangular = HashSet::from_iter((0..100).map(|n| n*(n+1)/2));
    let words = read_words().unwrap();
    words.iter().filter(|w| is_triangular(w, &triangular)).count()
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_042() {
        let ans: usize = problem_042();
        println!("Answer to Problem 42: {}", ans);
        assert!(ans == 162)
    }

    #[bench]
    fn bench_problem_042(b: &mut Bencher) {
        b.iter(|| problem_042());
    }
}

