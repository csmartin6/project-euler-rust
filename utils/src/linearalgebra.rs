use na::{DMatrix, DVector};
use num::Zero;

pub trait Diagonal<T> {
    fn diag(&self, k: isize) -> DVector<T>;
}

pub trait Rotate<T> {
    // rotate pi/2 in the clockwise direction

    // return rotated matrix
    fn rotated(&self) -> DMatrix<T>;

    // rotate in place
    fn rotate(&mut self);
}

impl<T: Clone> Diagonal<T> for DMatrix<T> {
    fn diag(&self, k: isize) -> DVector<T> {

        let m = self.nrows();
        let n = self.ncols();

        let mut i: usize = 0;
        let mut j: usize = 0;

        if k >= 0 {
            j = k as usize;
        } else {
            i = -k as usize;
        }

        let mut diag: Vec<T> = vec![];

        while i < m && j < n {
            diag.push(self[(i, j)].clone());
            i += 1;
            j += 1;
        }

        DVector::from_slice(diag.len(), &diag)

    }
}

impl<T: Copy + Clone + Zero> Rotate<T> for DMatrix<T> {
    fn rotated(&self) -> DMatrix<T> {
        let m = self.nrows();
        let n = self.ncols();
        let mut rotated_mat = DMatrix::new_zeros(n, m);

        for i in 0..n {
            for j in 0..m {
                rotated_mat[(i, j)] = self[(m - j - 1, i)]
            }
        }
        rotated_mat
    }

    fn rotate(&mut self) {
        let m = self.nrows();
        let n = self.ncols();
        let mut rotated_mat = DMatrix::new_zeros(n, m);

        for i in 0..n {
            for j in 0..m {
                rotated_mat[(i, j)] = self[(m - j - 1, i)]
            }
        }
        *self = rotated_mat
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use na::{DMatrix, Column, Row};
    #[test]
    fn test_rotated() {
        let elems = vec![1, 2, 3, 4, 5, 6];
        let mat = DMatrix::from_row_vector(3, 2, &elems[..]);
        let rot_mat = mat.rotated();
        assert_eq!(rot_mat.column(0).at, vec![5, 6]);
        assert_eq!(rot_mat.column(1).at, vec![3, 4]);
        assert_eq!(rot_mat.column(2).at, vec![1, 2]);
        assert_eq!(rot_mat.row(0).at, vec![5, 3, 1]);
        assert_eq!(rot_mat.row(1).at, vec![6, 4, 2]);
    }

    #[test]
    fn test_rotate() {
        let elems = vec![1, 2, 3, 4, 5, 6];
        let mut mat = DMatrix::from_row_vector(3, 2, &elems[..]);
        mat.rotate();
        assert_eq!(mat.column(0).at, vec![5, 6]);
        assert_eq!(mat.column(1).at, vec![3, 4]);
        assert_eq!(mat.column(2).at, vec![1, 2]);
        assert_eq!(mat.row(0).at, vec![5, 3, 1]);
        assert_eq!(mat.row(1).at, vec![6, 4, 2]);
    }


    #[test]
    fn test_diagonal() {
        let elems = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mat = DMatrix::from_row_vector(3, 3, &elems[..]);
        assert_eq!(mat.diag(2).at, vec![3]);
        assert_eq!(mat.diag(1).at, vec![2, 6]);
        assert_eq!(mat.diag(0).at, vec![1, 5, 9]);
        assert_eq!(mat.diag(-1).at, vec![4, 8]);
        assert_eq!(mat.diag(-2).at, vec![7]);
    }
}
