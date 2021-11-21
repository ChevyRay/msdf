use num_traits::Zero;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    #[inline]
    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    #[inline]
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline]
    pub fn direction(&self) -> f64 {
        self.y.atan2(self.x)
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::new(0.0, 1.0)
        } else {
            Self::new(self.x / len, self.y / len)
        }
    }

    #[inline]
    pub fn normalize_allow_zero(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::new(0.0, 0.0)
        } else {
            Self::new(self.x / len, self.y / len)
        }
    }

    #[inline]
    pub fn get_orthogonal(&self, polarity: bool) -> Self {
        if polarity {
            Self::new(-self.y, self.x)
        } else {
            Self::new(self.y, -self.x)
        }
    }

    #[inline]
    pub fn get_orthonormal(&self, polarity: bool) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::new(0.0, -1.0)
        } else {
            if polarity {
                Self::new(-self.y / len, self.x / len)
            } else {
                Self::new(self.y / len, -self.x / len)
            }
        }
    }

    #[inline]
    pub fn get_orthonormal_allow_zero(&self, polarity: bool) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::new(0.0, 0.0)
        } else {
            if polarity {
                Self::new(-self.y / len, self.x / len)
            } else {
                Self::new(self.y / len, -self.x / len)
            }
        }
    }

    #[inline]
    pub fn project(&self, vector: Self) -> Self {
        let n = self.normalize_allow_zero();
        let t = dot_product(vector, n);
        n * t
    }

    #[inline]
    pub fn project_positive(&self, vector: Self) -> Self {
        let n = self.normalize_allow_zero();
        let t = dot_product(vector, n);
        if t <= 0.0 {
            Vector2::zero()
        } else {
            n * t
        }
    }
}

impl From<f64> for Vector2 {
    #[inline]
    fn from(val: f64) -> Self {
        Self::new(val, val)
    }
}

impl Zero for Vector2 {
    #[inline]
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

#[inline]
pub fn dot_product(a: Vector2, b: Vector2) -> f64 {
    a.x * b.x + a.y * b.y
}

#[inline]
pub fn cross_product(a: Vector2, b: Vector2) -> f64 {
    a.x * b.y - a.y * b.x
}

impl Add<Vector2> for Vector2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Vector2> for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Vector2> for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign<Vector2> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl DivAssign<Vector2> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f64> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f64> for Vector2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f64> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vector2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}
