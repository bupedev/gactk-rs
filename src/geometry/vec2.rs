use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Div, Mul, Sub}
};

use num_traits::{real::Real, Zero, Euclid};

use crate::numerics::RealConst;

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

    pub fn normalize_mut(&mut self) -> Self {
        *self = self.normalize();
        *self
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag.is_zero() {
            Self {
                x: self.x,
                y: self.y
            }
        } else {
            Self {
                x: self.x / mag,
                y: self.y / mag
            }
        }
    }

    pub fn rotate_mut(&mut self, radians: T) -> Self {
        *self = self.rotate(radians);
        *self
    }

    pub fn rotate(&self, radians: T) -> Self {
        let cos = radians.cos();
        let sin = radians.sin();

        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    pub fn reflect_mut(&mut self, axis: Vec2<T>) -> Self {
        *self = self.reflect(axis);
        *self
    }

    pub fn reflect(&self, axis: Vec2<T>) -> Self {
        if axis.magnitude().is_zero() {
            return self.clone();
        }

        let radians = axis.angle() + axis.angle();
        let cos = radians.cos();
        let sin = radians.sin();

        Self {
            x: self.x * cos + self.y * sin,
            y: self.x * sin - self.y * cos,
        }
    }

    pub fn project_mut(&mut self, basis: Vec2<T>) -> Self {
        *self = self.project(basis);
        *self
    }

    pub fn project(&self, basis: Vec2<T>) -> Self {
        basis * (self.dot(basis) / basis.dot(basis))
    }
}

impl<T: Real + RealConst + Euclid> Vec2<T> {
    pub fn angle_to(&self, other: Vec2<T>) -> T {
        let std_angle = (other.angle() - self.angle()).rem_euclid(&T::TAU);
        match std_angle {
            t if t > T::PI => -T::PI + t.rem_euclid(&T::PI),
            _ => std_angle
        }
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
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{
        FRAC_PI_2, 
        FRAC_PI_3, 
        FRAC_PI_4, 
        FRAC_PI_6, 
        PI
    };

    const EPSILON: f64 = 1.5e-15;

    #[test]
    fn display() {
        fn test(v: Vec2<f64>, string: &str) {
            assert_eq!(v.to_string(), string);
        }

        test(Vec2::new(-2., 3.), "[-2, 3]");
        test(Vec2::new(0.5, 0.), "[0.5, 0]");
    }

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

        #[test]
        fn unit() {            
            fn test(angle: f64, expected: Vec2<f64>) {
                let actual = Vec2::unit(angle);
                assert!((actual.x - expected.x).abs() < EPSILON, "actual x: {}, expected x: {}", actual.x, expected.x);
                assert!((actual.y - expected.y).abs() < EPSILON, "actual y: {}, expected y: {}", actual.y, expected.y);
            }

            test(-FRAC_PI_6, Vec2::new(0.75.sqrt(), -0.5));
            test(-5. * FRAC_PI_4, Vec2::new(-0.5.sqrt(), 0.5.sqrt()));
            test(0., Vec2::new(1., 0.));
            test(FRAC_PI_6, Vec2::new(0.75.sqrt(), 0.5));
            test(2. * FRAC_PI_3, Vec2::new(-0.5, 0.75.sqrt()));
            test(5. * FRAC_PI_4, Vec2::new(-0.5.sqrt(), -0.5.sqrt()));
            test(5. * FRAC_PI_3, Vec2::new(0.5, -0.75.sqrt()));
            test(2. * PI, Vec2::new(1., 0.));
            test(13. * FRAC_PI_6, Vec2::new(0.75.sqrt(), 0.5));
            test(13. * FRAC_PI_4, Vec2::new(-0.5.sqrt(), -0.5.sqrt()));
        }
    }

    mod methods {
        use super::*;

        #[test]
        fn magnitude() {
            fn test(vector: Vec2<f64>, expected: f64) {
                let actual = vector.magnitude();
                assert!((actual - expected).abs() < EPSILON, "actual: {}, expected: {}", actual, expected);
            }

            test(Vec2::new(3., 4.), 5.);
            test(Vec2::new(-5., 12.), 13.);
            test(Vec2::new(8., -15.), 17.);
            test(Vec2::new(-7., -24.), 25.);
        }
        
        #[test]
        fn angle() {
            fn test(vector: Vec2<f64>, expected: f64) {
                let actual = vector.angle();
                assert!((actual - expected).abs() < EPSILON, "actual: {}, expected: {}", actual, expected);
            }

            test(Vec2::unit(FRAC_PI_6), FRAC_PI_6);
            test(Vec2::unit(2. * FRAC_PI_3), 2. * FRAC_PI_3);
            test(Vec2::unit(5. * FRAC_PI_4), -3. * FRAC_PI_4);
            test(Vec2::unit(3. * FRAC_PI_2),  -FRAC_PI_2);
        }

        #[test]
        fn dot() {
            fn test(a: Vec2<f64>, b: Vec2<f64>, expected: f64) {
                let actual = a.dot(b);
                assert!((actual - expected).abs() < EPSILON, "actual: {}, expected: {}", actual, expected);
            }

            test(Vec2::new(1., 2.), Vec2::new(3., 1.), 5.);
            test(Vec2::new(-1., 2.), Vec2::new(3., 1.), -1.);
            test(Vec2::new(1., 2.), Vec2::new(3., -1.), 1.);
            test(Vec2::new(-1., 2.), Vec2::new(3., -1.), -5.);
            test(Vec2::new(-1., -2.), Vec2::new(-3., -1.), 5.);
        }

        #[test]
        fn cross() {
            fn test(a: Vec2<f64>, b: Vec2<f64>, expected: f64) {
                let actual = a.cross(b);
                assert!((actual - expected).abs() < EPSILON, "actual: {}, expected: {}", actual, expected);
            }

            test(Vec2::new(1., 2.), Vec2::new(3., 1.), -5.);
            test(Vec2::new(-1., 2.), Vec2::new(3., 1.), -7.);
            test(Vec2::new(1., -2.), Vec2::new(3., 1.), 7.);
            test(Vec2::new(-1., 2.), Vec2::new(-3., 1.), 5.);
            test(Vec2::new(-1., -2.), Vec2::new(-3., -1.), -5.);
        }

        #[test]
        fn angle_to() {
            fn test(a: Vec2<f64>, b: Vec2<f64>, expected: f64) {
                let actual = a.angle_to(b);
                assert!((actual - expected).abs() < EPSILON, "actual: {}, expected: {}", actual, expected);
            }

            test(Vec2::unit(FRAC_PI_6), Vec2::unit(5. * FRAC_PI_6), 2. * FRAC_PI_3);
            test(Vec2::unit(5. * FRAC_PI_6), Vec2::unit(FRAC_PI_6), -2. * FRAC_PI_3);
            test(Vec2::unit(FRAC_PI_4), Vec2::unit(5. * FRAC_PI_4), PI);
            test(Vec2::unit(5. * FRAC_PI_4), Vec2::unit(FRAC_PI_4), -PI);
            test(Vec2::unit(3. * FRAC_PI_2), Vec2::unit(PI), -FRAC_PI_2);
            test(Vec2::unit(PI), Vec2::unit(3. * FRAC_PI_2), FRAC_PI_2);
        }

        #[test]
        fn normalize_mut() {
            fn test(vector: &mut Vec2<f64>, expected: Vec2<f64>) {
                let actual = vector.normalize_mut();
                assert!((actual.x - expected.x).abs() < EPSILON, "actual x: {}, expected x: {}", actual.x, expected.x);
                assert!((actual.y - expected.y).abs() < EPSILON, "actual y: {}, expected y: {}", actual.y, expected.y);
                assert_eq!(actual.x, vector.x, "Expect x to mutate");
                assert_eq!(actual.y, vector.y, "Expect y to mutate");
            }

            test(&mut Vec2::new(0., 0.), Vec2::new(0., 0.));
            test(&mut Vec2::new(1., 0.), Vec2::new(1., 0.));
            test(&mut Vec2::new(0., 1.), Vec2::new(0., 1.));
            test(&mut Vec2::new(3., 4.), Vec2::new(3. / 5., 4. / 5.));
            test(&mut Vec2::new(-5., 12.), Vec2::new(-5. / 13., 12. / 13.));
            test(&mut Vec2::new(8., -15.), Vec2::new(8. / 17., -15. / 17.));
            test(&mut Vec2::new(-7., -24.), Vec2::new(-7. / 25., -24. / 25.));
        }

        
        #[test]
        fn normalize() {
            fn test(vector: Vec2<f64>, expected: Vec2<f64>) {
                let actual = vector.normalize();
                assert!((actual.x - expected.x).abs() < EPSILON, "actual x: {}, expected x: {}", actual.x, expected.x);
                assert!((actual.y - expected.y).abs() < EPSILON, "actual y: {}, expected y: {}", actual.y, expected.y);
            }

            test(Vec2::new(0., 0.), Vec2::new(0., 0.));
            test(Vec2::new(1., 0.), Vec2::new(1., 0.));
            test(Vec2::new(0., 1.), Vec2::new(0., 1.));
            test(Vec2::new(3., 4.), Vec2::new(3. / 5., 4. / 5.));
            test(Vec2::new(-5., 12.), Vec2::new(-5. / 13., 12. / 13.));
            test(Vec2::new(8., -15.), Vec2::new(8. / 17., -15. / 17.));
            test(Vec2::new(-7., -24.), Vec2::new(-7. / 25., -24. / 25.));
        }

        #[test]
        fn rotate_mut() {
            fn test(vector: &mut Vec2<f64>, radians : f64, expected: Vec2<f64>) {
                let actual = vector.rotate_mut(radians);
                assert!((actual.x - expected.x).abs() < EPSILON, "actual x: {}, expected x: {}", actual.x, expected.x);
                assert!((actual.y - expected.y).abs() < EPSILON, "actual y: {}, expected y: {}", actual.y, expected.y);
                assert_eq!(actual.x, vector.x, "Expect x to mutate");
                assert_eq!(actual.y, vector.y, "Expect y to mutate");
            }

            test(&mut Vec2::zero(), PI, Vec2::zero());
            test(&mut Vec2::unit(0.), FRAC_PI_3, Vec2::unit(FRAC_PI_3));
            test(&mut Vec2::unit(FRAC_PI_4), FRAC_PI_2, Vec2::unit(3. * FRAC_PI_4));
            test(&mut Vec2::unit(FRAC_PI_2), 11. * FRAC_PI_6, Vec2::unit(FRAC_PI_3));
            test(&mut Vec2::unit(0.), 7. * FRAC_PI_3, Vec2::unit(FRAC_PI_3));
            test(&mut Vec2::unit(FRAC_PI_4), 5. * FRAC_PI_2, Vec2::unit(3. * FRAC_PI_4));
            test(&mut Vec2::unit(FRAC_PI_2), 23. * FRAC_PI_6, Vec2::unit(FRAC_PI_3));
            test(&mut Vec2::unit(0.), -FRAC_PI_3, Vec2::unit(5. * FRAC_PI_3));
            test(&mut Vec2::unit(FRAC_PI_4), -FRAC_PI_2, Vec2::unit(7. * FRAC_PI_4));
            test(&mut Vec2::unit(FRAC_PI_2), -11. * FRAC_PI_6, Vec2::unit(2. * FRAC_PI_3));
        }
        
        #[test]
        fn rotate() {
            fn test(vector: Vec2<f64>, radians : f64, expected: Vec2<f64>) {
                let actual = vector.rotate(radians);
                assert!((actual.x - expected.x).abs() < EPSILON, "actual x: {}, expected x: {}", actual.x, expected.x);
                assert!((actual.y - expected.y).abs() < EPSILON, "actual y: {}, expected y: {}", actual.y, expected.y);
            }

            test(Vec2::zero(), PI, Vec2::zero());
            test(Vec2::unit(0.), FRAC_PI_3, Vec2::unit(FRAC_PI_3));
            test(Vec2::unit(FRAC_PI_4), FRAC_PI_2, Vec2::unit(3. * FRAC_PI_4));
            test(Vec2::unit(FRAC_PI_2), 11. * FRAC_PI_6, Vec2::unit(FRAC_PI_3));
            test(Vec2::unit(0.), 7. * FRAC_PI_3, Vec2::unit(FRAC_PI_3));
            test(Vec2::unit(FRAC_PI_4), 5. * FRAC_PI_2, Vec2::unit(3. * FRAC_PI_4));
            test(Vec2::unit(FRAC_PI_2), 23. * FRAC_PI_6, Vec2::unit(FRAC_PI_3));
            test(Vec2::unit(0.), -FRAC_PI_3, Vec2::unit(5. * FRAC_PI_3));
            test(Vec2::unit(FRAC_PI_4), -FRAC_PI_2, Vec2::unit(7. * FRAC_PI_4));
            test(Vec2::unit(FRAC_PI_2), -11. * FRAC_PI_6, Vec2::unit(2. * FRAC_PI_3));
        }

        #[test]
        fn reflect_mut() {
            fn test(vector: &mut Vec2<f64>, axis: Vec2<f64>, expected: Vec2<f64>) {
                let actual = vector.reflect_mut(axis);
                assert!((actual.x - expected.x).abs() < EPSILON, "actual x: {}, expected x: {}", actual.x, expected.x);
                assert!((actual.y - expected.y).abs() < EPSILON, "actual y: {}, expected y: {}", actual.y, expected.y);
                assert_eq!(actual.x, vector.x, "Expect x to mutate");
                assert_eq!(actual.y, vector.y, "Expect y to mutate");
            }

            test(&mut Vec2::zero(), Vec2::unit(FRAC_PI_2), Vec2::zero());
            test(&mut Vec2::unit(FRAC_PI_6), Vec2::zero(), Vec2::unit(FRAC_PI_6));
            test(&mut Vec2::unit(0.), Vec2::unit(FRAC_PI_2), Vec2::unit(PI));
            test(&mut Vec2::unit(0.), Vec2::unit(3. * FRAC_PI_2), Vec2::unit(PI));
            test(&mut Vec2::unit(FRAC_PI_2), Vec2::unit(0.), Vec2::unit(3. * FRAC_PI_2));
            test(&mut Vec2::unit(FRAC_PI_2), Vec2::unit(PI), Vec2::unit(3. * FRAC_PI_2));
            test(&mut Vec2::unit(FRAC_PI_6), Vec2::unit(FRAC_PI_4), Vec2::unit(FRAC_PI_3));
            test(&mut Vec2::unit(FRAC_PI_6), Vec2::unit(3. * FRAC_PI_4), Vec2::unit(8. * FRAC_PI_6));
        }

        #[test]
        fn reflect() {
            fn test(vector: Vec2<f64>, axis: Vec2<f64>, expected: Vec2<f64>) {
                let actual = vector.reflect(axis);
                assert!((actual.x - expected.x).abs() < EPSILON, "actual x: {}, expected x: {}", actual.x, expected.x);
                assert!((actual.y - expected.y).abs() < EPSILON, "actual y: {}, expected y: {}", actual.y, expected.y);
            }

            test(Vec2::zero(), Vec2::unit(FRAC_PI_2), Vec2::zero());
            test(Vec2::unit(FRAC_PI_6), Vec2::zero(), Vec2::unit(FRAC_PI_6));
            test(Vec2::unit(0.), Vec2::unit(FRAC_PI_2), Vec2::unit(PI));
            test(Vec2::unit(0.), Vec2::unit(3. * FRAC_PI_2), Vec2::unit(PI));
            test(Vec2::unit(FRAC_PI_2), Vec2::unit(0.), Vec2::unit(3. * FRAC_PI_2));
            test(Vec2::unit(FRAC_PI_2), Vec2::unit(PI), Vec2::unit(3. * FRAC_PI_2));
            test(Vec2::unit(FRAC_PI_6), Vec2::unit(FRAC_PI_4), Vec2::unit(FRAC_PI_3));
            test(Vec2::unit(FRAC_PI_6), Vec2::unit(3. * FRAC_PI_4), Vec2::unit(8. * FRAC_PI_6));
        }

        #[test]
        fn project_mut() {
            fn test(vector: &mut Vec2<f64>, basis: Vec2<f64>, expected: Vec2<f64>) {
                let actual = vector.project_mut(basis);
                assert!((actual.x - expected.x).abs() < EPSILON, "actual x: {}, expected x: {}", actual.x, expected.x);
                assert!((actual.y - expected.y).abs() < EPSILON, "actual y: {}, expected y: {}", actual.y, expected.y);
                assert_eq!(actual.x, vector.x, "Expect x to mutate");
                assert_eq!(actual.y, vector.y, "Expect y to mutate");
            }

            test(&mut Vec2::zero(), Vec2::unit(0.), Vec2::zero());
            test(&mut Vec2::zero(), Vec2::unit(FRAC_PI_2), Vec2::zero());
            test(&mut Vec2::new(1., 1.), Vec2::new(1., 0.), Vec2::new(1., 0.));
            test(&mut Vec2::new(1., 1.), Vec2::new(0., 1.), Vec2::new(0., 1.));
            test(&mut Vec2::new(-1., -1.), Vec2::new(1., 0.), Vec2::new(-1., 0.));
            test(&mut Vec2::new(-1., -1.), Vec2::new(0., 1.), Vec2::new(0., -1.));
            test(&mut Vec2::new(1., 1.), Vec2::new(-1., 0.), Vec2::new(1., 0.));
            test(&mut Vec2::new(1., 1.), Vec2::new(0., -1.), Vec2::new(0., 1.));
            test(&mut Vec2::new(-2., 2.), Vec2::new(4., -3.), Vec2::new(-56./25., 42./25.));
        }

        #[test]
        fn project() {
            fn test(vector: Vec2<f64>, basis: Vec2<f64>, expected: Vec2<f64>) {
                let actual = vector.project(basis);
                assert!((actual.x - expected.x).abs() < EPSILON, "actual x: {}, expected x: {}", actual.x, expected.x);
                assert!((actual.y - expected.y).abs() < EPSILON, "actual y: {}, expected y: {}", actual.y, expected.y);
            }

            test(Vec2::zero(), Vec2::unit(0.), Vec2::zero());
            test(Vec2::zero(), Vec2::unit(FRAC_PI_2), Vec2::zero());
            test(Vec2::new(1., 1.), Vec2::new(1., 0.), Vec2::new(1., 0.));
            test(Vec2::new(1., 1.), Vec2::new(0., 1.), Vec2::new(0., 1.));
            test(Vec2::new(-1., -1.), Vec2::new(1., 0.), Vec2::new(-1., 0.));
            test(Vec2::new(-1., -1.), Vec2::new(0., 1.), Vec2::new(0., -1.));
            test(Vec2::new(1., 1.), Vec2::new(-1., 0.), Vec2::new(1., 0.));
            test(Vec2::new(1., 1.), Vec2::new(0., -1.), Vec2::new(0., 1.));
            test(Vec2::new(-2., 2.), Vec2::new(4., -3.), Vec2::new(-56./25., 42./25.));
        }
    }

    mod ops {
        use super::*;
        
        #[test]
        fn add_real() {
            fn test(vector: Vec2<f64>, real: f64, expected: Vec2<f64>) {
                let actual = vector + real;
                assert_eq!(actual, expected);
            }

            test(Vec2::zero(), 0., Vec2::zero());
            test(Vec2::zero(), 1., Vec2::new(1., 1.));
            test(Vec2::zero(), 2., Vec2::new(2., 2.));
            test(Vec2::zero(), -1., Vec2::new(-1., -1.));
            test(Vec2::new(1., 2.), 1., Vec2::new(2., 3.));
            test(Vec2::new(1., 2.), -1., Vec2::new(0., 1.));
        }
        
        #[test]
        fn add_vec() {
            fn test(a: Vec2<f64>, b: Vec2<f64>, expected: Vec2<f64>) {
                let actual = a + b;
                assert_eq!(actual, expected);
            }

            test(Vec2::zero(), Vec2::zero(), Vec2::zero());
            test(Vec2::zero(), Vec2::new(1., 0.), Vec2::new(1., 0.));
            test(Vec2::zero(), Vec2::new(0., 1.), Vec2::new(0., 1.));
            test(Vec2::new(-2., 3.), Vec2::zero(), Vec2::new(-2., 3.));
            test(Vec2::new(3., 4.), Vec2::new(-2., -3.), Vec2::new(1., 1.));
        }
        
        #[test]
        fn sub_real() {
            fn test(vector: Vec2<f64>, real: f64, expected: Vec2<f64>) {
                let actual = vector - real;
                assert_eq!(actual, expected);
            }

            test(Vec2::zero(), 0., Vec2::zero());
            test(Vec2::zero(), 1., Vec2::new(-1., -1.));
            test(Vec2::zero(), 2., Vec2::new(-2., -2.));
            test(Vec2::zero(), -1., Vec2::new(1., 1.));
            test(Vec2::new(1., 2.), 1., Vec2::new(0., 1.));
            test(Vec2::new(1., 2.), -1., Vec2::new(2., 3.));
        }
        
        #[test]
        fn sub_vec() {
            fn test(a: Vec2<f64>, b: Vec2<f64>, expected: Vec2<f64>) {
                let actual = a - b;
                assert_eq!(actual, expected);
            }

            test(Vec2::zero(), Vec2::zero(), Vec2::zero());
            test(Vec2::zero(), Vec2::new(1., 0.), Vec2::new(-1., 0.));
            test(Vec2::zero(), Vec2::new(0., 1.), Vec2::new(0., -1.));
            test(Vec2::new(-2., 3.), Vec2::zero(), Vec2::new(-2., 3.));
            test(Vec2::new(3., 4.), Vec2::new(-2., -3.), Vec2::new(5., 7.));
        }
        
        #[test]
        fn mul_real() {
            fn test(vector: Vec2<f64>, real: f64, expected: Vec2<f64>) {
                let actual = vector * real;
                assert_eq!(actual, expected);
            }

            test(Vec2::zero(), 2., Vec2::zero());
            test(Vec2::new(1., 0.), 2., Vec2::new(2., 0.));
            test(Vec2::new(0., 1.), 2., Vec2::new(0., 2.));
            test(Vec2::new(1., 2.), 2., Vec2::new(2., 4.));
            test(Vec2::new(1., 2.), 0.5, Vec2::new(0.5, 1.));
            test(Vec2::new(1., 2.), 0., Vec2::new(0., 0.));
        }
        
        #[test]
        fn div_real() {
            fn test(vector: Vec2<f64>, real: f64, expected: Vec2<f64>) {
                let actual = vector / real;
                assert_eq!(actual, expected);
            }

            test(Vec2::zero(), 2., Vec2::zero());
            test(Vec2::new(1., 0.), 2., Vec2::new(0.5, 0.));
            test(Vec2::new(0., 1.), 2., Vec2::new(0., 0.5));
            test(Vec2::new(1., 2.), 2., Vec2::new(0.5, 1.));
            test(Vec2::new(1., 2.), 0.5, Vec2::new(2., 4.));
            test(Vec2::new(1., 2.), 0., Vec2::new(f64::INFINITY, f64::INFINITY));
        }
    }
}
