use sequence_trie::SequenceTrie;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;
use utils;


pub fn problem_043() -> u64 {

    let mut tries = vec![];
    let primes = vec![2,3,5,7,11,13,17];
    for &p in primes.iter(){
        let mut trie: SequenceTrie<u32, u32> = SequenceTrie::new();
        let nums = (1..(1000/p)).map(|n| n * p).filter(|n| format!("{:03}",n).chars().unique().count() == 3);
        for n in nums{
            let c: Vec<u32> = format!("{:03}",n).chars().map(|c| c as u32 - 48).collect();
            trie.insert(&c[..],n);
        }                               
        tries.push(trie);
    }

    let (first, rest) = tries.split_first().unwrap();

    let mut candidates: HashSet<Vec<u32>> = HashSet::from_iter(first.keys().map(|k| k.iter().map(|&a| *a).collect()));

    for ref trie in rest.iter() {
        let mut new_candidates: HashSet<Vec<u32>> = HashSet::new();
        for ref c in candidates.iter() {
            let (_, last_two) = c.split_at(c.len()-2);

            match trie.get_node(last_two) {
                Some(tr) => for k in tr.keys(){
                    let mut candidate: Vec<u32> = c.to_owned().clone();
                    candidate.extend(k);

                    let unique_digits: HashSet<u32> = HashSet::from_iter(candidate.clone().into_iter());
                    if unique_digits.len() == candidate.len(){
                        new_candidates.insert(candidate.clone());
                    }
                    },
                None => {},
            }
        }
        candidates = new_candidates;

    }

    let mut results: Vec<u64> = vec![];

    for ref digit_arr in candidates.iter(){
        for i in 1..10 {
            let z = i as u32;
            if !digit_arr.contains(&z){
                let mut num = vec![z];
                num.extend(digit_arr.clone());
                results.push(utils::from_digit_array(num));
                break;
            }
        }
    }
    

    results.iter().fold(0, |a, &b| a + b)
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_043() {
        let ans: u64 = problem_043();
        println!("Answer to Problem 43: {}", ans);
        assert!(ans == 16695334890)
    }

    #[bench]
    fn bench_problem_043(b: &mut Bencher) {
        b.iter(|| problem_043());
    }
}

