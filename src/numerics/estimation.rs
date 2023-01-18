use std::ops::{Add, Mul, Sub};

pub fn lerp<X, T>(start: X, end: X, proportion: T) -> X
where
    X: Copy + Add<Output = X> + Sub<Output = X> + Mul<T, Output = X>,
{
    (start + (end - start)) * proportion
}
