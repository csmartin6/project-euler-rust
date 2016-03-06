use problems;
use test::Bencher;

#[bench]
fn bench_problem_001(b: &mut Bencher) {
        b.iter(|| problems::problem_001());
}

#[bench]
fn bench_problem_002(b: &mut Bencher) {
        b.iter(|| problems::problem_002());
}

#[bench]
fn bench_problem_003(b: &mut Bencher) {
        b.iter(|| problems::problem_003());
}

#[bench]
fn bench_problem_004(b: &mut Bencher) {
        b.iter(|| problems::problem_004());
}

#[bench]
fn bench_problem_005(b: &mut Bencher) {
        b.iter(|| problems::problem_005());
}
