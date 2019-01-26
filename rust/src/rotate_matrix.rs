/**
 * 1.7 Rotate Matrix: Given an image represented by an NxN matrix, where each pixel in the image is 4 bytes, write a method to rotate the image by 90 degrees. Can you do this in place?
 */

#[derive(Debug, PartialEq)]
pub struct SquareMatrix<T: Copy> {
    rows: Vec<Vec<T>>,
    side: usize,
}

impl<T: Copy> SquareMatrix<T> {
    pub fn from(rows: Vec<Vec<T>>) -> Self {
        let side = rows.len();
        assert!(rows.iter().all(|row| row.len() == side), "can only construct a square matrix");
        SquareMatrix { rows, side }
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<T> {
        if x >= self.side || y >= self.side {
            None
        } else {
            Some(self.rows[y][x])
        }
    }

    pub fn set(&mut self, (x, y): (usize, usize), value: T) -> Result<(), &str> {
        if x >= self.side || y >= self.side {
            Err("index out of range")
        } else {
            self.rows[y][x] = value;
            Ok(())
        }
    }

    pub fn rotate_cw(&mut self) {
        for row in 0..(self.side / 2) {
            for col in row..(self.side - 1 - row) {
                let coords_a = (col, row);
                let coords_b = self.rotate_coords_90d_cw(coords_a);
                let coords_c = self.rotate_coords_90d_cw(coords_b);
                let coords_d = self.rotate_coords_90d_cw(coords_c);
                let a = self.get(coords_a).unwrap();
                let b = self.get(coords_b).unwrap();
                let c = self.get(coords_c).unwrap();
                let d = self.get(coords_d).unwrap();
                self.set(coords_b, a).unwrap();
                self.set(coords_c, b).unwrap();
                self.set(coords_d, c).unwrap();
                self.set(coords_a, d).unwrap();
            }
        }
    }

    /**
     * (0, 0)               -> (side - 1, 0)
     * (side - 1, 0)        -> (side - 1, side - 1)
     * (side - 1, side - 1) -> (0, side - 1)
     * (0, side - 1)        -> (0, 0)
     * 
     * (1, 2) -> (side - 2, 1)
     * (side - 2, 1) -> (side - 2, side - 2)
     */
    fn rotate_coords_90d_cw(&self, (x, y): (usize, usize)) -> (usize, usize) {
        (self.side - 1 - y, x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_cw() {
        struct TestCase {
            input: SquareMatrix<i32>,
            expected: SquareMatrix<i32>,
        }

        let cases = vec![
            TestCase {
                input: SquareMatrix::from(vec![]),
                expected: SquareMatrix::from(vec![]),
            },
            TestCase {
                input: SquareMatrix::from(vec![vec![1]]),
                expected: SquareMatrix::from(vec![vec![1]]),
            },
            TestCase {
                input: SquareMatrix::from(vec![
                    vec![0, 1],
                    vec![2, 3],
                ]),
                expected: SquareMatrix::from(vec![
                    vec![2, 0],
                    vec![3, 1],
                ]),
            },
            TestCase {
                input: SquareMatrix::from(vec![
                    vec![0, 1, 2],
                    vec![3, 4, 5],
                    vec![6, 7, 8],
                ]),
                expected: SquareMatrix::from(vec![
                    vec![6, 3, 0],
                    vec![7, 4, 1],
                    vec![8, 5, 2],
                ]),
            },
        ];

        for TestCase { mut input, expected } in cases {
            input.rotate_cw();
            assert_eq!(input, expected);
        }
    }
}
