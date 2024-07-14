use super::Vector;

pub fn root_mean_square(w: &Vector, u: &Vector, v: &Vector) -> f32 {
    (((w.length() - 1.0).powi(2)
        + (u.length() - 1.0).powi(2)
        + (v.length() - 1.0).powi(2)
        + w.dot_product(&u).powi(2)
        + w.dot_product(&v).powi(2)
        + u.dot_product(v).powi(2))
        / 6.0)
        .sqrt()
}
