use problems;

#[test]
fn test_problem_001() {
    let ans: usize = problems::problem_001();
    println!("Answer to Problem 1: {}",ans);
    assert!(ans == 233168)
}

#[test]
fn test_problem_002() {
    let ans: usize = problems::problem_002();
    println!("Answer to Problem 2: {}",ans);
    assert!(ans == 4613732)
}


#[test]
fn test_problem_003() {
    let ans: usize = problems::problem_003();
    println!("Answer to Problem 3: {}",ans);
    assert!(ans == 6857)
}

#[test]
fn test_problem_004() {
    let ans: usize = problems::problem_004();
    println!("Answer to Problem 4: {}",ans);
    assert!(ans == 906609)
}

#[test]
fn test_problem_005() {
    let ans: usize = problems::problem_005();
    println!("Answer to Problem 5: {}",ans);
    assert!(ans == 232792560)
}

#[test]
fn test_problem_006() {
    let ans: usize = problems::problem_006();
    println!("Answer to Problem 6: {}",ans);
    assert!(ans == 25164150)
}

#[test]
fn test_problem_007() {
    let ans: usize = problems::problem_007();
    println!("Answer to Problem 7: {}",ans);
    assert!(ans == 104743)
}

#[test]
fn test_problem_007_sieve() {
    let ans: usize = problems::problem_007_sieve();
    println!("Answer to Problem 7: {}",ans);
    assert!(ans == 104743)
}