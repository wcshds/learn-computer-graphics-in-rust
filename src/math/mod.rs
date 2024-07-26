mod matrix;
mod vector;
pub mod utils;
pub mod rand;

pub use matrix::*;
pub use vector::*;

#[derive(Debug)]
pub enum QuadraticResult {
    Roots(f32, f32),
    DoubleRoot(f32),
    NoRealSolution,
}

pub fn quadratic_equation(a: f32, b: f32, c: f32) -> QuadraticResult {
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        return QuadraticResult::NoRealSolution;
    }

    let discriminant_sqrt = discriminant.sqrt();
    if discriminant - 0.0 < f32::EPSILON {
        let double_root = -b / 2.0 / a;
        QuadraticResult::DoubleRoot(double_root)
    } else {
        let [x1, x2] = if b >= 0.0 {
            let x1 = 2.0 * c / (-b - discriminant_sqrt); // numerical stability
            let x2 = (-b - discriminant_sqrt) / 2.0 / a;
            [x1, x2]
        } else {
            let x1 = (-b + discriminant_sqrt) / 2.0 / a;
            let x2 = 2.0 * c / (-b + discriminant_sqrt); // numerical stability
            [x1, x2]
        };

        QuadraticResult::Roots(x1, x2)
    }
}
