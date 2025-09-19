use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

pub type Direction2D = Vec2;
pub type Point2D = Vec2;

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub const fn zeroes() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub const fn ones() -> Vec2 {
        Vec2 { x: 1.0, y: 1.0 }
    }

    pub fn length_squared(self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn unit(self) -> Vec2 {
        self / self.length()
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other
        }
    }
} 

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Self::Output {
        other * self
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x / other.x,
            y: self.y / other.y
        }       
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        (1.0 / other) * self
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, other: f32) {
        *self = *self / other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neg() {
        let v = Vec2::new(1.0, -1.0);
        let nv = -v;
        assert_eq!(nv, Vec2::new(-1.0, 1.0));
    }

    #[test]
    fn add() {
        let mut v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        v1 += v2;
        assert_eq!(v1, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn sub() {
        let mut v1 = Vec2::new(5.0, 4.0);
        let v2 = Vec2::new(3.0, 2.0);
        v1 -= v2;
        assert_eq!(v1, Vec2::new(2.0, 2.0));
    }

    #[test]
    fn mul_vec2() {
        let mut v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(2.0, 1.0);
        v1 *= v2;
        assert_eq!(v1, Vec2::new(2.0, 2.0));
    }

    #[test]
    fn mul_f32() {
        let mut v = Vec2::new(3.0, 3.0);
        let s = 2.0;
        v *= s;
        assert_eq!(v, Vec2::new(6.0, 6.0));
    }

    #[test]
    fn f32_mul_vec2() {
        let f = 2.0;
        let v = Vec2::new(3.0, 3.0);
        let product = f * v;
        assert_eq!(product, Vec2::new(6.0, 6.0));
    } 

    #[test]
    fn div_vec2() {
        let mut v1 = Vec2::new(9.0, 4.0);
        let v2 = Vec2::new(3.0, 2.0);
        v1 /= v2;
        assert_eq!(v1, Vec2::new(3.0, 2.0));
    }

    #[test]
    fn div_f32() {
        let mut v = Vec2::new(9.0, 6.0);
        let d = 3.0;
        v /= d;
        assert_eq!(v, Vec2::new(3.0, 2.0));
    }

    #[test]
    fn length() {
        let v = Vec2::new(3.0, 4.0);
        let len_squared = v.length_squared();
        assert_eq!(len_squared, 25.0);
        let len = v.length();
        assert_eq!(len, 5.0);
        let unit = v.unit();
        assert_eq!(unit, Vec2::new(0.6, 0.8));
    }
}