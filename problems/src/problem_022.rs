use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, Error};

pub fn read_names() -> Result<Vec<String>,Error> {
    let f = try!(File::open("../data/p022_names.txt"));
    let reader = BufReader::new(f);

    let mut names: Vec<String> = vec![];

    for line in reader.lines() {
        let line = try!(line);
        let line = line.replace("\"","");
        names = line.split(",").map(|x| x.to_string()).collect();
    }
    Ok(names)
}

pub fn problem_022() -> u32{
    
    let mut names = read_names().unwrap();
    names.sort();
    names.iter().enumerate()
        .map(|(i,name)| score(&name) * (i+1) as u32 )
        .fold(0, |a,b| a + b)
}

pub fn score(name: &String) -> u32{
    let letter_scores: Vec<u8> = name.chars().map(|c| (c as u8) - 64).collect();
    letter_scores.iter().fold(0, |a,&b| a + (b as u32))
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_022() {
        let ans: u32 = problem_022();
        println!("Answer to Problem 22: {}", ans);
        assert!(ans == 871198282)
    }

    #[bench]
    fn bench_problem_022(b: &mut Bencher) {
        b.iter(|| problem_022());
    }


}
