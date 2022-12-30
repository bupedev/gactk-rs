use std::{
    fmt::{Display, Formatter, Result},
};

/// Describes a single-variable symbolic polynomial expression. A polynomial
/// expression is one that strictly contains a linear combination of positive-
/// integer powers of it's variables.
#[derive(Debug)]
pub struct Polynomial {
    coefficients: Vec<f64>,
    variable: String,
}

impl Polynomial {
    fn new(coefficients: &[f64], variable: &str) -> Polynomial {
        Polynomial {
            coefficients: coefficients.to_vec(),
            variable: variable.to_string(),
        }
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut formatted = "".to_string();
        for (index, coefficient) in self.coefficients.iter().enumerate().rev() {
            if *coefficient == 0. && index > 0 {
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

            formatted.push_str(self.variable.as_str());

            if index < 2 {
                continue;
            }

            formatted.push_str(format!("^{}", index).as_str());
        }
        write!(f, "{}", formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fmt {
        use super::*;

        struct DisplayTestCase<'a>{polynomial: Polynomial, expected: &'a str}

        #[test]
        fn polynomial_to_string() {
            let test_cases: [DisplayTestCase; 4] = [
                DisplayTestCase{
                    polynomial: Polynomial::new(&[0.], "x"), 
                    expected: "0"
                },
                DisplayTestCase{
                    polynomial: Polynomial::new(&[1., 9.], "y"), 
                    expected: "9y + 1"
                },
                DisplayTestCase{
                    polynomial: Polynomial::new(&[-3., 0., 1., -2.], "b"), 
                    expected: "-2b^3 + b^2 - 3"
                },
                DisplayTestCase{
                    polynomial: Polynomial::new(&[2., 4., 5., 1.], "x"), 
                    expected: "x^3 + 5x^2 + 4x + 2"
                },
            ];

            for test_case in test_cases {
                let actual = test_case.polynomial.to_string();
                assert_eq!(actual, test_case.expected);
            }
        }
    }
}