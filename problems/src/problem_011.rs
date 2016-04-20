use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, Error};
use na::{DMatrix, Column, Transpose};
use utils::linearalgebra::{Diagonal, Rotate};

fn read_grid_as_matrix() -> Result<DMatrix<u32>, Error> {
    let f = try!(File::open("../data/problem_011_input.txt"));
    let reader = BufReader::new(f);

    let mut grid = vec![];
    let mut nrows = 0;
    let mut ncols = 0;

    for line in reader.lines() {
        let line = try!(line);
        let row: Vec<u32> = line.split(' ').map(|x| x.parse::<u32>().unwrap()).collect();
        ncols = row.len();
        grid.extend(row);
        nrows += 1;

    }

    Ok(DMatrix::from_col_vector(nrows, ncols, &grid[..]))
}

fn max_product(arr: &[u32], length: usize) -> u32 {
    let mut max: u32 = 0;
    if arr.len() < length {
        return 0;
    }
    for i in 0..(arr.len() - length + 1) {
        let product = arr[i..(i + length)].iter().fold(1, |a, &b| a * b);
        if product > max {
            max = product;
        }
    }
    max
}

fn max_column_product(mat: DMatrix<u32>, length: usize) -> u32 {
    let mut max: u32 = 0;
    for j in 0..mat.ncols() {
        let max_col = max_product(&mat.column(j)[..], length);
        if max_col > max {
            max = max_col
        }
    }
    max
}

pub fn max_diagonal_product(mat: DMatrix<u32>, length: usize) -> u32 {
    let mut max: u32 = 0;
    let m = mat.nrows() as isize;
    let n = mat.ncols() as isize;

    for k in -m..n {
        let diag = mat.diag(k as isize);
        let max_product_in_diag = max_product(&diag.at[..], length);
        if max_product_in_diag > max {
            max = max_product_in_diag;
        }
    }
    max
}



pub fn problem_011() -> u32 {
    let length = 4;
    let mat = read_grid_as_matrix().unwrap();

    let col_max = max_column_product(mat.clone(), length);
    let row_max = max_column_product(mat.clone().transpose(), length);
    let diag_max = max_diagonal_product(mat.clone(), length);
    let rot_diag_max = max_diagonal_product(mat.rotated(), length);

    let maxes = vec![col_max, row_max, diag_max, rot_diag_max];

    *maxes.iter().max().unwrap()
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;


    #[test]
    fn test_problem_011() {
        let ans: u32 = problem_011();
        println!("Answer to Problem 11: {}", ans);
        assert!(ans == 70600674)
    }



    #[bench]
    fn bench_problem_011(b: &mut Bencher) {
        b.iter(|| problem_011());
    }
}
