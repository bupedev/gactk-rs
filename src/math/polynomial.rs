use num_traits::{One, Zero};
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul, Neg},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Polynomial<T> {
    coefficients: Vec<T>,
}

impl<T> Polynomial<T> {
    pub fn coefficients(&self) -> &[T] {
        &self.coefficients
    }

    pub fn order(&self) -> usize {
        self.coefficients.len()
    }
}

impl<T: Zero> Polynomial<T> {
    pub fn new(mut coefficients: Vec<T>) -> Self {
        while let Some(true) = coefficients.last().map(|x| (*x).is_zero()) {
            coefficients.pop();
        }

        Self { coefficients }
    }
}

trait Eval<E, T>
where
    T: Mul<E>,
{
    fn eval(&self, value: E) -> <T as Mul<E>>::Output;
}

impl<T, E> Eval<E, T> for Polynomial<T>
where
    T: Zero + One + Mul<E> + Clone,
    E: One + Clone,
    <T as Mul<E>>::Output: Zero + Clone,
{
    fn eval(&self, value: E) -> <T as Mul<E>>::Output {
        let mut result: <T as Mul<E>>::Output = <T as Mul<E>>::Output::zero();
        let mut power: E = E::one();

        for coefficient in self.coefficients.iter() {
            result = result + (*coefficient).clone() * power.clone();
            power = power.clone() * value.clone();
        }

        result
    }
}

impl<T: Zero + Clone> Zero for Polynomial<T> {
    fn zero() -> Self {
        Polynomial::new(vec![])
    }

    fn is_zero(&self) -> bool {
        self.coefficients.len() == 0
    }
}

impl<T: Zero + One + Clone> One for Polynomial<T> {
    fn one() -> Self {
        Polynomial::new(vec![T::one()])
    }
}

impl<T: Zero + One + Display + PartialOrd + Clone + Neg<Output = T>> Display for Polynomial<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.coefficients.len() == 0 {
            return write!(f, "{}", T::zero());
        }

        let mut formatted = "".to_string();
        for (index, coefficient) in self.coefficients.iter().enumerate().rev() {
            let blank_element = (*coefficient).is_zero()
                && (index > 0 || (index == 0 && self.coefficients.len() > 1));

            if blank_element {
                continue;
            }

            if index == self.coefficients.len() - 1 {
                if *coefficient < T::zero() {
                    formatted.push_str("-");
                }
            } else {
                let sgn = if *coefficient < T::zero() { "-" } else { "+" };
                formatted.push_str(format!(" {} ", sgn).as_str());
            }

            let abs_coefficient = if *coefficient < T::zero() {
                -(*coefficient).clone()
            } else {
                (*coefficient).clone()
            };

            if !abs_coefficient.is_one() || index == 0 {
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
        impl<TL, TR> $imp<Polynomial<TR>> for Polynomial<TL>
        where
            TL: Zero + $imp<TR> + Clone,
            TR: Zero + Clone,
            <TL as $imp<TR>>::Output: Zero + Clone,
        {
            type Output = Polynomial<<TL as $imp<TR>>::Output>;

            #[inline]
            fn $method(self, other: Polynomial<TR>) -> Polynomial<<TL as $imp<TR>>::Output> {
                (&self).$method(&other)
            }
        }
    };
}

macro_rules! binary_operation_value_reference {
    (impl $imp:ident, $method:ident) => {
        impl<'rhs, TL, TR> $imp<&'rhs Polynomial<TR>> for Polynomial<TL>
        where
            TL: Zero + $imp<TR> + Clone,
            TR: Zero + Clone,
            <TL as $imp<TR>>::Output: Zero + Clone,
        {
            type Output = Polynomial<<TL as $imp<TR>>::Output>;

            #[inline]
            fn $method(self, other: &Polynomial<TR>) -> Polynomial<<TL as $imp<TR>>::Output> {
                (&self).$method(other)
            }
        }
    };
}

macro_rules! binary_operation_reference_value {
    (impl $imp:ident, $method:ident) => {
        impl<'lhs, TL, TR> $imp<Polynomial<TR>> for &'lhs Polynomial<TL>
        where
            TL: Zero + $imp<TR> + Clone,
            TR: Zero + Clone,
            <TL as $imp<TR>>::Output: Zero + Clone,
        {
            type Output = Polynomial<<TL as $imp<TR>>::Output>;

            #[inline]
            fn $method(self, other: Polynomial<TR>) -> Polynomial<<TL as $imp<TR>>::Output> {
                self.$method(&other)
            }
        }
    };
}

binary_operation_all!(impl Add, add);

impl<'lhs, 'rhs, TL, TR> Add<&'rhs Polynomial<TR>> for &'lhs Polynomial<TL>
where
    TL: Zero + Add<TR> + Clone,
    TR: Zero + Clone,
    <TL as Add<TR>>::Output: Zero + Clone,
{
    type Output = Polynomial<<TL as Add<TR>>::Output>;

    fn add(self, rhs: &Polynomial<TR>) -> Polynomial<<TL as Add<TR>>::Output> {
        let resultant_size: usize = self.order().max(rhs.order());
        let mut base_coefficients = vec![TL::zero(); resultant_size];

        for (index, coefficient) in self.coefficients.iter().enumerate() {
            base_coefficients[index] = (*coefficient).clone();
        }

        let mut new_coefficients = vec![<TL as Add<TR>>::Output::zero(); resultant_size];

        for i in 0..base_coefficients.len() {
            if i < rhs.coefficients.len() {
                new_coefficients[i] = base_coefficients[i].clone() + rhs.coefficients[i].clone();
            } else {
                new_coefficients[i] = base_coefficients[i].clone() + TR::zero();
            }
        }

        Polynomial::new(new_coefficients)
    }
}

binary_operation_all!(impl Mul, mul);

impl<'lhs, 'rhs, TL, TR> Mul<&'rhs Polynomial<TR>> for &'lhs Polynomial<TL>
where
    TL: Zero + Mul<TR> + Clone,
    TR: Zero + Clone,
    <TL as Mul<TR>>::Output: Zero + Clone,
{
    type Output = Polynomial<<TL as Mul<TR>>::Output>;

    fn mul(self, rhs: &Polynomial<TR>) -> Polynomial<<TL as Mul<TR>>::Output> {
        if self.coefficients.len() == 0 && rhs.coefficients.len() == 0 {
            return Polynomial::<<TL as Mul<TR>>::Output>::new(vec![]);
        }

        let size = self.coefficients.len() + rhs.coefficients.len() - 1;
        let mut product = vec![<TL as Mul<TR>>::Output::zero(); size];

        for (lhs_index, lhs_coefficient) in self.coefficients.iter().enumerate().rev() {
            for (rhs_index, rhs_coefficient) in rhs.coefficients.iter().enumerate() {
                product[lhs_index + rhs_index] = product[lhs_index + rhs_index].clone()
                    + (*lhs_coefficient).clone() * (*rhs_coefficient).clone();
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
            let polynomial = Polynomial::<f64>::zero();
            let actual = polynomial.coefficients();
            assert_eq!(actual, &[]);
        }

        #[test]
        fn one() {
            let polynomial = Polynomial::<f64>::one();
            let actual = polynomial.coefficients();
            assert_eq!(actual, &[f64::one()]);
        }
    }

    mod evaluation {
        use super::*;

        fn test(polynomial: Polynomial<f64>, value: f64, expected: f64) {
            let actual = polynomial.eval(value);
            assert_eq!(actual, expected);
        }

        #[test]
        fn zero() {
            test(Polynomial::<f64>::zero(), f64::one(), f64::zero());
        }

        #[test]
        fn one() {
            test(Polynomial::<f64>::one(), f64::zero(), f64::one());
        }

        #[test]
        fn constant() {
            test(Polynomial::new(vec![5.]), f64::zero(), 5.);
        }

        #[test]
        fn linear_at_zero() {
            test(Polynomial::new(vec![2., 3.]), f64::zero(), 2.);
        }

        #[test]
        fn linear_at_one() {
            test(Polynomial::new(vec![2., 3.]), f64::one(), 5.);
        }

        #[test]
        fn linear_at_constant() {
            test(Polynomial::new(vec![2., 3.]), 2., 8.);
        }

        #[test]
        fn quadratic_at_zero() {
            test(Polynomial::new(vec![3., 6., 9.]), f64::zero(), 3.);
        }

        #[test]
        fn quadratic_at_one() {
            test(Polynomial::new(vec![3., 6., 9.]), f64::one(), 18.);
        }

        #[test]
        fn quadratic_at_constant() {
            test(Polynomial::new(vec![3., 6., 9.]), 2., 51.);
        }
    }

    mod display {
        use super::*;

        fn test(polynomial: Polynomial<f64>, expected: &str) {
            let actual = polynomial.to_string();
            assert_eq!(actual, expected);
        }

        #[test]
        fn zero() {
            test(Polynomial::<f64>::zero(), "0");
        }

        #[test]
        fn one() {
            test(Polynomial::<f64>::one(), "1");
        }

        #[test]
        fn constant() {
            test(Polynomial::new(vec![3.]), "3");
        }

        #[test]
        fn linear() {
            test(Polynomial::new(vec![1., 9.]), "9x + 1");
        }

        #[test]
        fn quadratic() {
            test(Polynomial::new(vec![0., 2., -3.]), "-3x^2 + 2x");
        }

        #[test]
        fn cubic() {
            test(Polynomial::new(vec![-4., 0., -7., 1.]), "x^3 - 7x^2 - 4");
        }
    }

    mod operations {
        use super::*;

        fn test_operation(
            lhs: Polynomial<f64>,
            rhs: Polynomial<f64>,
            expected: Polynomial<f64>,
            operation: fn(&Polynomial<f64>, &Polynomial<f64>) -> Polynomial<f64>,
        ) {
            let forward = operation(&lhs, &rhs);
            assert_eq!(forward, expected);

            let backward = operation(&rhs, &lhs);
            assert_eq!(backward, expected);
        }

        mod addition {
            use super::*;

            fn test(lhs: Polynomial<f64>, rhs: Polynomial<f64>, expected: Polynomial<f64>) {
                test_operation(lhs, rhs, expected, |a, b| a + b);
            }

            #[test]
            fn zero_plus_zero() {
                test(Polynomial::zero(), Polynomial::zero(), Polynomial::zero());
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

            fn test(lhs: Polynomial<f64>, rhs: Polynomial<f64>, expected: Polynomial<f64>) {
                test_operation(lhs, rhs, expected, |a, b| a * b);
            }

            #[test]
            fn zero_times_zero() {
                test(Polynomial::zero(), Polynomial::zero(), Polynomial::zero());
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
