use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use rand::{Rng, thread_rng};

#[derive(Default,Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn random(min: f32, max: f32) -> Vec3 {
        let mut rng = thread_rng();
        Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.len_squared() >= 1.0 {continue;}
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        }else{
            -in_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-self).dot(n);
        let r_out_perp = (self + &(n * cos_theta)) * etai_over_etat;
        let r_out_paralell = n * -(((1.0 - r_out_perp.len_squared()).abs()).sqrt());
        r_out_perp + r_out_paralell
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.y * v2.z - v1.z * v2.y,
        v1.z * v2.x - v1.x * v2.z,
        v1.x * v2.y - v1.y * v2.x,
    )
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n*dot(v, n)*2.0
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.len()
}

pub type Point3 = Vec3;
pub type Color = Vec3;

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn neg_vec3() {
        let v = Vec3::new(12.0, 0.0, -5.0);
        assert_eq!(-v, Vec3::new(-12.0, 0.0, 5.0));
    }

    #[test]
    fn add_assign_vec3() {
        let mut v1 = Vec3::new(12.0, 6.12, -2.6);
        v1 += Vec3::new(17.0, 8.72, -7.6);
        assert_eq!(v1, Vec3::new(29.0, 14.84, -10.2))
    }

    #[test]
    fn mul_assign_vec3() {
        let mut v1 = Vec3::new(12.0, 6.12, -2.6);
        v1 *= Vec3::new(17.0, 8.72, -7.6);
        assert_eq!(v1, Vec3::new(12.0 * 17.0, 6.12 * 8.72, -7.6 * -2.6))
    }

    #[test]
    fn div_assign_vec3() {
        let mut v1 = Vec3::new(12.0, 6.12, -2.6);
        v1 /= Vec3::new(17.0, 8.72, -7.6);
        assert_eq!(v1, Vec3::new(12.0 / 17.0, 6.12 / 8.72, -2.6 / -7.6))
    }

    #[test]
    fn len_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.len(), 3.7416575)
    }

    #[test]
    fn add_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let v3 = Vec3::new(3.0, 5.0, 7.0);
        assert_eq!(v1 + v2, v3);
    }

    #[test]
    fn sub_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let v3 = Vec3::new(-1.0, -1.0, -1.0);
        assert_eq!(v1 - v2, v3);
    }

    #[test]
    fn div_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let v3 = Vec3::new(1.0 / 2.0, 2.0 / 3.0, 3.0 / 4.0);
        assert_eq!(v1 / v2, v3);
    }

    #[test]
    fn mul_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let v3 = Vec3::new(1.0 * 2.0, 2.0 * 3.0, 3.0 * 4.0);
        assert_eq!(v1 * v2, v3);
    }

    #[test]
    fn mul_scalar_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0 * 3.0, 2.0 * 3.0, 3.0 * 3.0);
        assert_eq!(v1 * 3.0, v2);
    }
}
