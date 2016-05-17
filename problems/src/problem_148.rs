use utils;
use std::u64;

pub fn pascal_triangle_mod_7(n: u64) -> u64 {

    let n_base_7 = utils::as_digit_array_base(n, 7);
    let m = n_base_7.len() as u32;
    let mut count = 0;

    let base_triangle: u64 = 28;
    let mut sources = 1;
    for (i, &d) in n_base_7.iter().enumerate() {

        if d > 0 {
            let num_i_triangles = sources * ((d * (d + 1) / 2) as u64);
            let triangle_i = base_triangle.pow((m - (i as u32) - 1) as u32);
            count += num_i_triangles * triangle_i;
            sources = sources * (d + 1) as u64;
        };


    }
    count
}

pub fn problem_148() -> u64 {
    let n: u64 = 1000000000;
    pascal_triangle_mod_7(n)
}

#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_problem_148() {
        let ans: u64 = problem_148();
        println!("Answer to Problem 148: {}", ans);
        assert!(ans == 2129970655314432)
    }

    #[test]
    fn test_problem_148_pascal_triangle_mod_7() {
        assert_eq!(pascal_triangle_mod_7(7), 28);
        assert_eq!(pascal_triangle_mod_7(8), 30);
        assert_eq!(pascal_triangle_mod_7(9), 34);
        assert_eq!(pascal_triangle_mod_7(100), 2361);
        assert_eq!(pascal_triangle_mod_7(1000), 118335);
        assert_eq!(pascal_triangle_mod_7(7500), 3753960);
        assert_eq!(pascal_triangle_mod_7(1000000000), 2129970655314432);
    }


    #[bench]
    fn bench_problem_148(b: &mut Bencher) {
        b.iter(|| problem_148());
    }

}
