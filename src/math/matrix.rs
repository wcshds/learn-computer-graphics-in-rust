use std::{fmt::Display, ops::Mul};

use super::Vector;

/// Just for test purposes, the performance of this `Matrix` struct
/// may be extremely low.
#[derive(Debug)]
pub struct Matrix {
    row1: Vector,
    row2: Vector,
    row3: Vector,
}

impl Matrix {
    pub fn new(
        x11: f32,
        x12: f32,
        x13: f32,
        x21: f32,
        x22: f32,
        x23: f32,
        x31: f32,
        x32: f32,
        x33: f32,
    ) -> Self {
        Self::from_vectors(
            Vector::new(x11, x12, x13),
            Vector::new(x21, x22, x23),
            Vector::new(x31, x32, x33),
        )
    }

    pub fn from_vectors(row1: Vector, row2: Vector, row3: Vector) -> Self {
        Self { row1, row2, row3 }
    }

    pub fn transpose(&self) -> Self {
        Self::new(
            self.row1.x,
            self.row2.x,
            self.row3.x,
            self.row1.y,
            self.row2.y,
            self.row3.y,
            self.row1.z,
            self.row2.z,
            self.row3.z,
        )
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    /// Matrix Multiplication
    fn mul(self, rhs: &Matrix) -> Matrix {
        Matrix {
            row1: Vector::new(
                self.row1.x * rhs.row1.x + self.row1.y * rhs.row2.x + self.row1.z * rhs.row3.x,
                self.row1.x * rhs.row1.y + self.row1.y * rhs.row2.y + self.row1.z * rhs.row3.y,
                self.row1.x * rhs.row1.z + self.row1.y * rhs.row2.z + self.row1.z * rhs.row3.z,
            ),
            row2: Vector::new(
                self.row2.x * rhs.row1.x + self.row2.y * rhs.row2.x + self.row2.z * rhs.row3.x,
                self.row2.x * rhs.row1.y + self.row2.y * rhs.row2.y + self.row2.z * rhs.row3.y,
                self.row2.x * rhs.row1.z + self.row2.y * rhs.row2.z + self.row2.z * rhs.row3.z,
            ),
            row3: Vector::new(
                self.row3.x * rhs.row1.x + self.row3.y * rhs.row2.x + self.row3.z * rhs.row3.x,
                self.row3.x * rhs.row1.y + self.row3.y * rhs.row2.y + self.row3.z * rhs.row3.y,
                self.row3.x * rhs.row1.z + self.row3.y * rhs.row2.z + self.row3.z * rhs.row3.z,
            ),
        }
    }
}
impl Mul<Matrix> for &Matrix {
    type Output = Matrix;

    /// Matrix Multiplication
    fn mul(self, rhs: Matrix) -> Matrix {
        self * &rhs
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements = [
            self.row1.x,
            self.row1.y,
            self.row1.z,
            self.row2.x,
            self.row2.y,
            self.row2.z,
            self.row3.x,
            self.row3.y,
            self.row3.z,
        ];

        let mut int_len = 0;
        let mut float_len = 0;
        let mut sign = 0;
        let mut point = 0;
        for each in elements {
            let f = format!("{}", each);
            let mut tmp = f.split(".");

            if let Some(int) = tmp.next() {
                if int.contains('-') {
                    sign = 1;
                    if int.len() - 1 > int_len {
                        int_len = int.len() - 1;
                    }
                } else if int.len() > int_len {
                    int_len = int.len();
                }
            }
            if let Some(float) = tmp.next() {
                if float.len() > float_len {
                    point = 1;
                    float_len = float.len();
                }
            }
        }

        let total_len = int_len + float_len + point + sign;
        f.write_fmt(format_args!(
            "⎡{number:>total$.float$}  ",
            number = self.row1.x,
            total = total_len,
            float = float_len
        ))?;
        f.write_fmt(format_args!(
            "{number:>total$.float$}  ",
            number = self.row1.y,
            total = total_len,
            float = float_len
        ))?;
        f.write_fmt(format_args!(
            "{number:>total$.float$}⎤\n",
            number = self.row1.z,
            total = total_len,
            float = float_len
        ))?;
        f.write_fmt(format_args!(
            "⎢{number:>total$.float$}  ",
            number = self.row2.x,
            total = total_len,
            float = float_len
        ))?;
        f.write_fmt(format_args!(
            "{number:>total$.float$}  ",
            number = self.row2.y,
            total = total_len,
            float = float_len
        ))?;
        f.write_fmt(format_args!(
            "{number:>total$.float$}⎥\n",
            number = self.row2.z,
            total = total_len,
            float = float_len
        ))?;
        f.write_fmt(format_args!(
            "⎣{number:>total$.float$}  ",
            number = self.row3.x,
            total = total_len,
            float = float_len
        ))?;
        f.write_fmt(format_args!(
            "{number:>total$.float$}  ",
            number = self.row3.y,
            total = total_len,
            float = float_len
        ))?;
        f.write_fmt(format_args!(
            "{number:>total$.float$}⎦",
            number = self.row3.z,
            total = total_len,
            float = float_len
        ))
    }
}

#[cfg(test)]
mod test {
    use super::Matrix;

    #[test]
    fn test_matrix() {
        let mat = Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        println!("{}", &mat * mat.transpose());
    }
}
