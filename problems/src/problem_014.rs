use std::collections::HashMap;

fn next_collatz(n: u64) -> u64{
    match n % 2{
        0 => n/2,
        _ => 3 * n + 1,
    }
}

fn collatz_seq_length(num: u64, mut seq_lens: HashMap<u64,u64>) -> u64{
    let next_num = next_collatz(num);
    if seq_lens.contains_key(&next_num){
        return (1 + seq_lens.get(&next_num).unwrap()) as u64;
    } else {
        let seq_len = collatz_seq_length(next_num, seq_lens.clone());
        seq_lens.insert(next_num, seq_len);
        seq_len
    }


}

pub fn problem_014() -> u64 {
    let n = 1000000;
    let mut seq_len: HashMap<u64,u64> = HashMap::new();
    
    // insert powers of two
    for i in 0..20 {
        seq_len.insert((2 as u64).pow(i),(i+1) as u64);
    }
    let mut max_seq = 0;
    let mut arg_max = 0;
    for i in 2..(n+1) {
        let mut curr: u64 = i;
        let mut count: u64 = 0;
        while !seq_len.contains_key(&curr) {
            curr = next_collatz(curr);
            count += 1;
        }

        let seq_len_i = count + seq_len.get(&curr).unwrap();
        seq_len.insert(i,seq_len_i);
        if seq_len_i > max_seq {
            max_seq = seq_len_i;
            arg_max = i; 
        }
    }
    arg_max
}

pub fn problem_014_b() -> u64 {
    let n = 1000000;
    let mut seq_len: HashMap<u64,u64> = HashMap::new();
    
    // insert powers of two
    for i in 0..20 {
        seq_len.insert((2 as u64).pow(i),(i+1) as u64);
    }
    let mut max_seq = 0;
    let mut arg_max = 0;
    for i in 2..10000{//(n+1) {

        let seq_len_i = collatz_seq_length(i as u64, seq_len.clone());
        seq_len.insert(i,seq_len_i);
        if seq_len_i > max_seq {
            max_seq = seq_len_i;
            arg_max = i; 
        }
    }
    println!("seq_len : {:?}",seq_len);
    arg_max
}

pub fn problem_014_c() -> u64 {
    let n = 1000000;
    let mut collatz: Vec<usize> = vec![0;(n+1)];

    let mut max_seq = 0;
    let mut arg_max = 0;
    collatz[1] = 1;

    for i in 2..(n+1) {
        let mut x = i ;
        while x != 1{
            if x < i {
                collatz[i] += collatz[x];
                if collatz[i] > max_seq {
                    max_seq = collatz[i];
                    arg_max = i as u64; 
                }
                break; 
            } else {
                x = next_collatz(x as u64) as usize;
                collatz[i] += 1;
            }
        }
    }
    arg_max
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_014() {
        let ans: u64 = problem_014_c();
        println!("Answer to Problem 14: {}", ans);
        assert!(ans == 837799)
    }

    #[bench]
    fn bench_problem_014(b: &mut Bencher) {
        b.iter(|| problem_014());
    }
}
