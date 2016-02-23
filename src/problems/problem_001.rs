
use test::Bencher;

fn problem_001() -> i64 {
    let mut s: i64 = 0;

    for x in 0..1000{
        if x % 3 == 0 || x % 5 ==0{
            s += x;
        }
    }
    s
}

#[test]
fn test_problem_001() {
    let ans: i64 = problem_001();
    println!("Answer to Problem 1: {}",ans);
    assert!(ans == 233168)
}


#[bench]
fn bench_problem_001(b: &mut Bencher) {
        b.iter(|| problem_001());
}



