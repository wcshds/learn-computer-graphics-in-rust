use std::ops::Div;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn cross_product(&self, other: &Self) -> Self {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn dot_product(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn orthonormal_basis_simple(&self) -> Option<(Self, Self)> {
        let w_len = self.length();
        if w_len == 0.0 {
            return None;
        }
        let w = self / w_len;

        let mut t = w.clone();
        let (t_x, t_y, t_z) = (t.x.abs(), t.y.abs(), t.z.abs());
        if t_x <= t_y {
            if t_x <= t_z {
                t.x = 1.0;
            } else {
                t.z = 1.0;
            }
        } else {
            if t_y <= t_z {
                t.y = 1.0;
            } else {
                t.z = 1.0;
            }
        }

        let mut u = w.cross_product(&t);
        u.div_(u.length());

        let v = w.cross_product(&u);

        Some((u, v))
    }

    /// Get the orthonormal basis from current vector. The vector itself
    /// should be a unit vector.
    pub fn orthonormal_basis_frisvad_from_unit(&self) -> (Self, Self) {
        if self.z < -0.9999999 {
            return (Self::new(0.0, -1.0, 0.0), Self::new(-1.0, 0.0, 0.0));
        }

        let a = 1.0 / (1.0 + self.z);
        let b = -self.x * self.y * a;
        (
            Self::new(1.0 - self.x * self.x * a, b, -self.x),
            Self::new(b, 1.0 - self.y * self.y * a, -self.y),
        )
    }

    /// Get the orthonormal basis from current vector. This function will
    /// automatically normalize this vector to a unit vector, then invoke
    /// `orthonormal_basis_frisvad_from_unit` function.
    pub fn orthonormal_basis_frisvad(&self) -> Option<(Self, Self)> {
        let w_len = self.length();
        if w_len == 0.0 {
            return None;
        }
        let w = self / w_len;

        Some(w.orthonormal_basis_frisvad_from_unit())
    }

    /// Get the orthonormal basis from current vector. The vector itself
    /// should be a unit vector.
    #[inline]
    pub fn orthonormal_basis_revised_from_unit(&self) -> (Self, Self) {
        let sign = 1.0f32.copysign(self.z);
        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;
        (
            Self::new(1.0 + sign * self.x * self.x * a, sign * b, -sign * self.x),
            Self::new(b, sign + self.y * self.y * a, -self.y),
        )
    }

    /// Get the orthonormal basis from current vector. This function will
    /// automatically normalize this vector to a unit vector, then invoke
    /// `orthonormal_basis_frisvad_from_unit` function.
    #[inline]
    pub fn orthonormal_basis_revised(&self) -> Option<(Self, Self)> {
        let w_len = self.length();
        if w_len == 0.0 {
            return None;
        }
        let w = self / w_len;

        Some(w.orthonormal_basis_revised_from_unit())
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<f32> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Vector {
    fn div_(&mut self, rhs: f32) -> &Self {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
        self
    }
}

#[cfg(test)]
mod test {}
