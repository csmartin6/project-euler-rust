use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader, Error};
use na::{DMatrix, Column, Transpose, DVector};


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


pub fn rotate(mat: DMatrix<u32>) -> DMatrix<u32> {
    let m = mat.nrows();
    let n = mat.ncols();
    let mut rotated_mat = DMatrix::from_elem(n, m, 0);

    for i in 0..n {
        for j in 0..m {
            rotated_mat[(i, j)] = mat[(m - j - 1, i)]
        }
    }
    rotated_mat
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

pub fn diagonal(mat: DMatrix<u32>, k: isize) -> DVector<u32> {

    let m = mat.nrows();
    let n = mat.ncols();

    let mut i: usize = 0;
    let mut j: usize = 0;

    if k >= 0 {
        j = k as usize;
    } else {
        i = -k as usize;
    }

    let mut diag: Vec<u32> = vec![];

    while i < m && j < n {
        diag.push(mat[(i, j)]);
        i += 1;
        j += 1;
    }

    DVector::from_slice(diag.len(), &diag)
}

pub fn max_diagonal_product(mat: DMatrix<u32>, length: usize) -> u32 {
    let mut max: u32 = 0;
    let m = mat.nrows() as isize;
    let n = mat.ncols() as isize;

    for k in -m..n {
        let diag = diagonal(mat.clone(), k as isize);
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
    let rot_diag_max = max_diagonal_product(rotate(mat.clone()), length);

    let maxes = vec![col_max, row_max, diag_max, rot_diag_max];

    *maxes.iter().max().unwrap()
}


#[cfg(test)]
mod test {
    use super::*;
    use test::Bencher;
    use na::{DMatrix, Row, Column};

    #[test]
    fn test_matrix_rotation() {
        let elems = vec![1, 2, 3, 4, 5, 6];
        let mat = DMatrix::from_row_vector(3, 2, &elems[..]);
        let rot_mat = rotate(mat.clone());
        assert_eq!(rot_mat.column(0).at, vec![5, 6]);
        assert_eq!(rot_mat.column(1).at, vec![3, 4]);
        assert_eq!(rot_mat.column(2).at, vec![1, 2]);
        assert_eq!(rot_mat.row(0).at, vec![5, 3, 1]);
        assert_eq!(rot_mat.row(1).at, vec![6, 4, 2]);
    }

    #[test]
    fn test_diagonal() {
        let elems = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mat = DMatrix::from_row_vector(3, 3, &elems[..]);
        assert_eq!(diagonal(mat.clone(), 2).at, vec![3]);
        assert_eq!(diagonal(mat.clone(), 1).at, vec![2, 6]);
        assert_eq!(diagonal(mat.clone(), 0).at, vec![1, 5, 9]);
        assert_eq!(diagonal(mat.clone(), -1).at, vec![4, 8]);
        assert_eq!(diagonal(mat.clone(), -2).at, vec![7]);
    }


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
