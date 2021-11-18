use num_traits::{One, Zero};
use std::ops::{Add, Mul, Neg, Sub};

#[inline]
pub fn min<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a < b {
        a
    } else if b < a {
        b
    } else {
        a
    }
}

#[inline]
pub fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a > b {
        a
    } else if b > a {
        b
    } else {
        a
    }
}

#[inline]
pub fn median<T>(a: T, b: T, c: T) -> T
where
    T: PartialOrd + Copy,
{
    max(min(a, b), min(max(a, b), c))
}

#[inline]
pub fn mix<T, S>(a: T, b: T, weight: S) -> T
where
    T: Mul<S, Output = T> + Add<T, Output = T>,
    S: Copy + One + Sub<S, Output = S>,
{
    a * (S::one() - weight) + b * weight
}

#[inline]
pub fn clamp_0_1<T>(n: T) -> T
where
    T: PartialOrd + Zero + One,
{
    if n < T::zero() {
        T::zero()
    } else if n > T::one() {
        T::one()
    } else {
        n
    }
}

#[inline]
pub fn clamp_0_b<T>(n: T, b: T) -> T
where
    T: PartialOrd + Zero,
{
    if n < T::zero() {
        T::zero()
    } else if n > b {
        b
    } else {
        n
    }
}

#[inline]
pub fn clamp_a_b<T>(n: T, a: T, b: T) -> T
where
    T: PartialOrd,
{
    if n < a {
        a
    } else if n > b {
        b
    } else {
        n
    }
}

#[inline]
pub fn sign<T, R>(n: T) -> R
where
    T: Zero + PartialOrd,
    R: Zero + One + Neg<Output = R>,
{
    if n > T::zero() {
        R::one()
    } else if n < T::zero() {
        -R::one()
    } else {
        R::zero()
    }
}

#[inline]
pub fn non_zero_sign<T, R>(n: T) -> R
where
    T: Zero + PartialOrd,
    R: One + Neg<Output = R>,
{
    if n >= T::zero() {
        R::one()
    } else {
        -R::one()
    }
}

#[inline]
pub fn fabs(x: f64) -> f64 {
    x.abs()
}
