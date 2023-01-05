use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Div, Mul, Sub},
};

use num_traits::{real::Real, Zero};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2<T: Real> {
    pub x: T,
    pub y: T,
}

impl<T: Real> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn unit(radians: T) -> Self {
        Self {
            x: radians.cos(),
            y: radians.sin(),
        }
    }

    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn angle(&self) -> T {
        self.y.atan2(self.x)
    }

    pub fn dot(&self, other: Vec2<T>) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: Vec2<T>) -> T {
        self.x * other.y - self.y * other.x
    }

    pub fn angleTo(&self, other: Vec2<T>) -> T {
        other.angle() - self.angle()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn rotate(&self, radians: T) -> Self {
        let cos = radians.cos();
        let sin = radians.sin();

        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    pub fn reflect(&self, axis: Vec2<T>) -> Self {
        let radians = axis.angle() + axis.angle();
        let cos = radians.cos();
        let sin = radians.sin();

        Self {
            x: self.x * cos + self.y * sin,
            y: self.x * sin - self.y * cos,
        }
    }

    pub fn project(&self, basis: Vec2<T>) -> Self {
        basis * (self.dot(basis) / basis.dot(basis))
    }
}

impl<T: Real + Zero> Zero for Vec2<T> {
    fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl<T: Real + Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T: Real> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Vec2<T> {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Real> Div<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Vec2<T> {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Real> Add<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T: Real> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Real> Sub<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T: Real> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        #[test]
        fn new() {
            let x = -2.;
            let y = 3.;
            let v = Vec2::new(x, y);
            assert_eq!(v.x, x);
            assert_eq!(v.y, y);
        }

        mod unit {
            use super::*;
            use std::f64::consts::{FRAC_PI_2, PI};

            fn test(angle: f64, expected: Vec2<f64>) {
                let actual = Vec2::unit(angle);
                assert!((actual.x - expected.x).abs() < 1e-15);
                assert!((actual.y - expected.y).abs() < 1e-15);
            }

            #[test]
            fn zero_rads() {
                test(0., Vec2::new(1., 0.))
            }

            #[test]
            fn half_pi_rads() {
                test(FRAC_PI_2, Vec2::new(0., 1.))
            }

            #[test]
            fn pi_rads() {
                test(PI, Vec2::new(-1., 0.))
            }

            #[test]
            fn three_half_pi_rads() {
                test(3. * FRAC_PI_2, Vec2::new(0., -1.))
            }

            #[test]
            fn two_pi_rads() {
                test(2. * PI, Vec2::new(1., 0.))
            }

            #[test]
            fn four_pi_rads() {
                test(4. * PI, Vec2::new(1., 0.))
            }

            #[test]
            fn negative_half_pi_rads() {
                test(-FRAC_PI_2, Vec2::new(0., -1.))
            }
        }
    }
}
