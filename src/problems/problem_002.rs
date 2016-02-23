use test::Bencher;

fn problem_002() -> i64 {
    let n = 4000000;

    let mut fib : Vec<i64> = vec![1,1];

    loop {
        let f_2 = fib[fib.len()-2];
        let f_1 = fib[fib.len()-1];
        if f_1 + f_2 > n {
            break;
        }
        fib.push(f_2 + f_1);

    }

    let mut sum = 0;
    for x in fib {
        if x % 2 == 0 {
            sum += x;
        }
            
    }
    return sum
}

#[test]
fn test_problem_002() {
    let ans: i64 = problem_002();
    println!("Answer to Problem 2: {}",ans);
    assert!(ans == 4613732)
}


#[bench]
fn bench_problem_002(b: &mut Bencher) {
        b.iter(|| problem_002());
}



