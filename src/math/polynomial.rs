use num_traits::{One, Zero};
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    pub fn new(mut coefficients: Vec<f64>) -> Self {
        while let Some(true) = coefficients.last().map(|x| *x == 0.) {
            coefficients.pop();
        }

        Self { coefficients }
    }

    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }

    pub fn order(&self) -> usize {
        self.coefficients.len()
    }
}

impl Zero for Polynomial {
    fn zero() -> Self {
        Polynomial::new(vec![])
    }

    fn is_zero(&self) -> bool {
        self.coefficients.len() == 0
    }
}

impl One for Polynomial {
    fn one() -> Self {
        Polynomial::new(vec![1.])
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.coefficients.len() == 0 {
            return write!(f, "0");
        }

        let mut formatted = "".to_string();
        for (index, coefficient) in self.coefficients.iter().enumerate().rev() {
            if *coefficient == 0. && (index > 0 || (index == 0 && self.coefficients.len() > 1)) {
                continue;
            }

            if index == self.coefficients.len() - 1 {
                if *coefficient < 0. {
                    formatted.push_str("-");
                }
            } else {
                let sgn = if *coefficient < 0. { "-" } else { "+" };
                formatted.push_str(format!(" {} ", sgn).as_str());
            }

            let abs_coefficient = coefficient.abs();
            if abs_coefficient != 1. || index == 0 {
                formatted.push_str(format!("{}", abs_coefficient).as_str());
            }

            if index < 1 {
                continue;
            }

            formatted.push_str("x");

            if index < 2 {
                continue;
            }

            formatted.push_str(format!("^{}", index).as_str());
        }
        write!(f, "{}", formatted)
    }
}

macro_rules! binary_operation_all {
    (impl $imp:ident, $method:ident) => {
        binary_operation_value_value!(impl $imp, $method);
        binary_operation_value_reference!(impl $imp, $method);
        binary_operation_reference_value!(impl $imp, $method);
    };
}

macro_rules! binary_operation_value_value {
    (impl $imp:ident, $method:ident) => {
        impl $imp<Polynomial> for Polynomial {
            type Output = Polynomial;

            #[inline]
            fn $method(self, other: Polynomial) -> Polynomial {
                (&self).$method(&other)
            }
        }
    };
}

macro_rules! binary_operation_value_reference {
    (impl $imp:ident, $method:ident) => {
        impl<'rhs> $imp<&'rhs Polynomial> for Polynomial {
            type Output = Polynomial;

            #[inline]
            fn $method(self, other: &Polynomial) -> Polynomial {
                (&self).$method(other)
            }
        }
    };
}

macro_rules! binary_operation_reference_value {
    (impl $imp:ident, $method:ident) => {
        impl<'lhs> $imp<Polynomial> for &'lhs Polynomial {
            type Output = Polynomial;

            #[inline]
            fn $method(self, other: Polynomial) -> Polynomial {
                self.$method(&other)
            }
        }
    };
}

binary_operation_all!(impl Add, add);

impl<'lhs, 'rhs> Add<&'rhs Polynomial> for &'lhs Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Polynomial {
        fn ordered_add(long: &Polynomial, short: &Polynomial) -> Polynomial {
            let mut new_coefficients = long.coefficients.clone();

            for (index, coefficient) in short.coefficients.iter().enumerate() {
                new_coefficients[index] += *coefficient;
            }

            Polynomial::new(new_coefficients)
        }
        
        if self.order() > rhs.order() {
            ordered_add(self, rhs)
        } else {
            ordered_add(rhs, self)
        }
    }
}

binary_operation_all!(impl Mul, mul);

impl<'lhs, 'rhs> Mul<&'rhs Polynomial> for &'lhs Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: &Polynomial) -> Polynomial {
        if self.coefficients.len() == 0 && rhs.coefficients.len() == 0 {
            return Polynomial::new(vec![]);
        }

        let size = self.coefficients.len() + rhs.coefficients.len() - 1;
        let mut product = vec![0.; size];

        for (lhs_index, lhs_coefficient) in self.coefficients.iter().enumerate().rev() {
            for (rhs_index, rhs_coefficient) in rhs.coefficients.iter().enumerate() {
                product[lhs_index + rhs_index] += lhs_coefficient * rhs_coefficient;
            }
        }

        Polynomial::new(product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        mod new {
            use super::*;

            fn test(coefficients: &[f64], expected: &[f64]) {
                let polynomial = Polynomial::new(coefficients.to_vec());
                let actual = polynomial.coefficients();
                assert_eq!(actual, expected);
            }

            #[test]
            fn singular_zero_coefficients() {
                test(&[0.], &[])
            }

            #[test]
            fn multiple_zero_coefficients() {
                test(&[0., 0., 0.], &[])
            }

            #[test]
            fn trailing_zero_coefficients() {
                test(&[1., 2., 0.], &[1., 2.])
            }
        }

        #[test]
        fn zero() {
            let polynomial = Polynomial::zero();
            let actual = polynomial.coefficients();
            assert_eq!(actual, &[]);
        }

        #[test]
        fn one() {
            let polynomial = Polynomial::one();
            let actual = polynomial.coefficients();
            assert_eq!(actual, &[1.]);
        }
    }

    mod display {
        use super::*;

        fn test(coefficients: &[f64], expected: &str) {
            let polynomial = Polynomial::new(coefficients.to_vec());
            let actual = polynomial.to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn zero() {
            test(&[], "0");
        }

        #[test]
        fn constant() {
            test(&[3.], "3");
        }

        #[test]
        fn linear() {
            test(&[1., 9.], "9x + 1");
        }

        #[test]
        fn quadratic() {
            test(&[0., 2., -3.], "-3x^2 + 2x");
        }

        #[test]
        fn cubic() {
            test(&[-4., 0., -7., 1.], "x^3 - 7x^2 - 4");
        }
    }

    mod operations {
        use super::*;

        fn test_operation(
            lhs: Polynomial,
            rhs: Polynomial,
            expected: Polynomial,
            operation: fn(&Polynomial, &Polynomial) -> Polynomial,
        ) {
            let forward = operation(&lhs, &rhs);
            assert_eq!(forward, expected);

            let backward = operation(&rhs, &lhs);
            assert_eq!(backward, expected);
        }

        mod addition {
            use super::*;

            fn test(lhs: Polynomial, rhs: Polynomial, expected: Polynomial) {
                test_operation(lhs, rhs, expected, |a, b| a + b);
            }

            #[test]
            fn zero_plus_zero() {
                test(
                    Polynomial::zero(),
                    Polynomial::zero(),
                    Polynomial::zero(),
                );
            }

            #[test]
            fn zero_plus_constant() {
                test(
                    Polynomial::zero(),
                    Polynomial::new(vec![5.]),
                    Polynomial::new(vec![5.]),
                );
            }

            #[test]
            fn zero_plus_linear() {
                test(
                    Polynomial::zero(),
                    Polynomial::new(vec![5., 7.]),
                    Polynomial::new(vec![5., 7.]),
                );
            }

            #[test]
            fn one_plus_constant() {
                test(
                    Polynomial::one(),
                    Polynomial::new(vec![3.]),
                    Polynomial::new(vec![4.]),
                );
            }

            #[test]
            fn one_plus_linear() {
                test(
                    Polynomial::one(),
                    Polynomial::new(vec![2., 3.]),
                    Polynomial::new(vec![3., 3.]),
                );
            }

            #[test]
            fn constant_plus_constant() {
                test(
                    Polynomial::new(vec![2.]),
                    Polynomial::new(vec![3.]),
                    Polynomial::new(vec![5.]),
                );
            }

            #[test]
            fn constant_plus_linear() {
                test(
                    Polynomial::new(vec![2.]),
                    Polynomial::new(vec![2., 3.]),
                    Polynomial::new(vec![4., 3.]),
                );
            }

            #[test]
            fn linear_plus_linear() {
                test(
                    Polynomial::new(vec![2., 1.]),
                    Polynomial::new(vec![5., 3.]),
                    Polynomial::new(vec![7., 4.]),
                );
            }

            #[test]
            fn quadratic_plus_quadratic() {
                test(
                    Polynomial::new(vec![6., 2., 4.]),
                    Polynomial::new(vec![3., 1., 5.]),
                    Polynomial::new(vec![9., 3., 9.]),
                );
            }
        }

        mod multiplication {
            use super::*;

            fn test(lhs: Polynomial, rhs: Polynomial, expected: Polynomial) {
                test_operation(lhs, rhs, expected, |a, b| a * b);
            }

            #[test]
            fn zero_times_zero() {
                test(
                    Polynomial::zero(),
                    Polynomial::zero(),
                    Polynomial::zero(),
                );
            }

            #[test]
            fn zero_times_constant() {
                test(
                    Polynomial::zero(),
                    Polynomial::new(vec![5.]),
                    Polynomial::zero(),
                );
            }

            #[test]
            fn zero_times_linear() {
                test(
                    Polynomial::zero(),
                    Polynomial::new(vec![5., 7.]),
                    Polynomial::zero(),
                );
            }

            #[test]
            fn one_times_constant() {
                test(
                    Polynomial::one(),
                    Polynomial::new(vec![3.]),
                    Polynomial::new(vec![3.]),
                );
            }

            #[test]
            fn one_times_linear() {
                test(
                    Polynomial::one(),
                    Polynomial::new(vec![2., 3.]),
                    Polynomial::new(vec![2., 3.]),
                );
            }

            #[test]
            fn constant_times_constant() {
                test(
                    Polynomial::new(vec![2.]),
                    Polynomial::new(vec![3.]),
                    Polynomial::new(vec![6.]),
                );
            }

            #[test]
            fn constant_times_linear() {
                test(
                    Polynomial::new(vec![2.]),
                    Polynomial::new(vec![2., 3.]),
                    Polynomial::new(vec![4., 6.]),
                );
            }

            #[test]
            fn linear_times_linear() {
                test(
                    Polynomial::new(vec![2., 1.]),
                    Polynomial::new(vec![5., 3.]),
                    Polynomial::new(vec![10., 11., 3.]),
                );
            }

            #[test]
            fn quadratic_times_quadratic() {
                test(
                    Polynomial::new(vec![6., 2., 4.]),
                    Polynomial::new(vec![3., 1., 5.]),
                    Polynomial::new(vec![18., 12., 44., 14., 20.]),
                );
            }
        }
    }
}
