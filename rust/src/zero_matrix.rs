/**
 * 1.8 Zero Matrix: Write an algorithm such that if an element in an MxN matrix is 0, its entire row and column are set to 0.
 */

use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Matrix {
    rows: Vec<Vec<i32>>,
    height: usize,
    width: usize,
}

impl Matrix {
    pub fn from(rows: Vec<Vec<i32>>) -> Self {
        let height = rows.len();
        if height == 0 {
            Matrix { rows, height, width: 0 }
        } else {
            let width = rows[0].len();
            assert!(rows.iter().all(|row| row.len() == width));
            Matrix { rows, height, width }
        }
    }

    pub fn zero_matrix(&mut self) {
        // Form two vectors containing the row and column indices (respectively) of each 0-element.
        let (zero_cols, zero_rows): (HashSet<usize>, HashSet<usize>) = self.rows.iter()
            .flatten()
            .enumerate()
            .filter_map(|(index, val)| {
                if *val == 0 {
                    // map to (x, y) coords
                    Some((index % self.width, index / self.width))
                } else {
                    None
                }
            })
            .unzip();
        
        let nonzero_rows = (0..self.height)
            // convert range to set so we can compute the difference
            .collect::<HashSet<usize>>()
            .difference(&zero_rows)
            // convert iter over &usize to iter over usize
            .cloned()
            // convert to vec so we can take slices
            .collect::<Vec<usize>>();

        for row in zero_rows {
            self.rows[row] = vec![0; self.width];
        }
        
        for col in zero_cols {
            // Since we're inside a loop, we need a slice to avoid a move.
            for row in &nonzero_rows[..] {
                self.rows[*row][col] = 0;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_matrix() {
        struct TestCase {
            input: Matrix,
            expected: Matrix,
        }

        let cases = vec![
            TestCase {
                input: Matrix::from(vec![]),
                expected: Matrix::from(vec![]),
            },
            TestCase {
                input: Matrix::from(vec![vec![1]]),
                expected: Matrix::from(vec![vec![1]]),
            },
            TestCase {
                input: Matrix::from(vec![
                    vec![1, 2],
                    vec![4, 0],
                    vec![6, 7],
                ]),
                expected: Matrix::from(vec![
                    vec![1, 0],
                    vec![0, 0],
                    vec![6, 0],
                ]),
            },
            TestCase {
                input: Matrix::from(vec![
                    vec![1, 0, 2],
                    vec![3, 4, 5],
                    vec![0, 6, 7],
                ]),
                expected: Matrix::from(vec![
                    vec![0, 0, 0],
                    vec![0, 0, 5],
                    vec![0, 0, 0],
                ]),
            },
        ];

        for TestCase { mut input, expected } in cases {
            input.zero_matrix();
            assert_eq!(input, expected);
        }
    }
}
