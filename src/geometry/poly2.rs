use num_traits::{real::Real, PrimInt};

use crate::numerics::RealConst;

use super::Vec2;

pub struct Poly2<T: Real> {
    pub vertices: Vec<Vec2<T>>,
}

impl<T: Real> Poly2<T> {
    pub fn new(vertices: &[Vec2<T>]) -> Self {
        if vertices.len() <= 2 {
            panic!("polygons must have at least three vertices")
        }

        let mut filtered = vec![vertices[0]];
        for i in 1..vertices.len() {
            if vertices[i] != vertices[i - 1] {
                filtered.push(vertices[i]);
            }
        }        
        
        if filtered.len() <= 2 {
            panic!("polygons must have at least three distinct vertices")
        }

        Self { vertices: filtered }
    }
}

impl<T : Real + RealConst> Poly2<T> {
    pub fn regular<I : PrimInt>(vertex_count: I, side_length: T) -> Self {
        if vertex_count <= I::zero() {
            panic!("polygons cannot have a non-positive number of vertices");
        }

        if side_length <= T::zero() {
            panic!("polygon side lengths must be strictly positive");
        }
        
        let n = T::from(vertex_count).expect("cast failure");
        let radius = side_length * T::HALF / (T::PI / n).sin();
        let angle = T::TWO * T::PI / n;
        
        let mut vertices = vec![];
        let mut cum_angle = T::zero();
        while cum_angle < T::TAU {
            vertices.push(Vec2::unit(cum_angle) * radius);
            cum_angle = cum_angle + angle;
        }
        Self::new(&vertices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{
        FRAC_PI_2, 
        FRAC_PI_3, 
        FRAC_PI_4,
    };

    const EPSILON: f64 = 1.5e-15;

    mod constructors {
        use num_traits::Zero;

        use super::*;
        use std::panic::catch_unwind;

        #[test]
        fn new() {
            fn test(vertices: &[Vec2<f64>], expected: &[Vec2<f64>]) {

                let poly = Poly2::new(vertices);
                assert_eq!(poly.vertices.len(), expected.len());
                for i in 0..expected.len() {
                    assert_eq!(poly.vertices[i], expected[i]);
                }
            }

            let square = vec![
                Vec2::new(-0.5, -0.5),
                Vec2::new(-0.5, 0.5),
                Vec2::new(0.5, 0.5),
                Vec2::new(0.5, -0.5)
            ];
            test(&square, &square);
            
            let duplicates = vec![
                Vec2::new(-0.5, -0.5),
                Vec2::new(-0.5, -0.5),
                Vec2::new(0.5, 0.5),
                Vec2::new(0.5, -0.5)
            ];
            test(&duplicates, &duplicates[1..]);
        }

        #[test]
        fn new_panic() {
            fn test(vertices: Vec<Vec2<f64>>) {
                let construction = || Poly2::new(&vertices);
                let result = catch_unwind(construction);
                assert!(result.is_err());
            }

            test(vec![]);
            test(vec![Vec2::new(1., 0.)]);
            test(vec![Vec2::new(1., 0.), Vec2::new(-1., 0.)]);
            test(vec![Vec2::new(1., 0.), Vec2::new(-1., 0.), Vec2::new(-1., 0.)]);
        }
        
        #[test]
        fn regular() { 
            fn test(vertex_count: usize, side_length: f64, expected: Vec<Vec2<f64>>) {
                let poly = Poly2::regular(vertex_count, side_length);
                assert_eq!(expected.len(), poly.vertices.len());
                for i in 0..expected.len() {
                    assert!((expected[i].x - poly.vertices[i].x).abs() < EPSILON);
                    assert!((expected[i].y - poly.vertices[i].y).abs() < EPSILON);
                }
            }

            let tri_length = 0.5 / FRAC_PI_3.sin();
            test(3, 1., vec![
                Vec2::unit(0. * FRAC_PI_3) * tri_length,
                Vec2::unit(2. * FRAC_PI_3) * tri_length,
                Vec2::unit(4. * FRAC_PI_3) * tri_length,
            ]);
            test(3, 2., vec![
                Vec2::unit(0. * FRAC_PI_3) * 2. * tri_length,
                Vec2::unit(2. * FRAC_PI_3) * 2. * tri_length,
                Vec2::unit(4. * FRAC_PI_3) * 2. * tri_length,
            ]);

            
            let quad_length = 0.5 / FRAC_PI_4.sin();
            test(4, 1., vec![
                Vec2::unit(0. * FRAC_PI_2) * quad_length,
                Vec2::unit(1. * FRAC_PI_2) * quad_length,
                Vec2::unit(2. * FRAC_PI_2) * quad_length,
                Vec2::unit(3. * FRAC_PI_2) * quad_length
            ]);
            test(4, 2., vec![
                Vec2::unit(0. * FRAC_PI_2) * 2. * quad_length,
                Vec2::unit(1. * FRAC_PI_2) * 2. * quad_length,
                Vec2::unit(2. * FRAC_PI_2) * 2. * quad_length,
                Vec2::unit(3. * FRAC_PI_2) * 2. * quad_length
            ]);
        }
        
        #[test]
        fn regular_panic() {
            fn test(vertex_count: isize, side_length: f64) {
                let construction = || Poly2::regular(vertex_count, side_length);
                let result = catch_unwind(construction);
                assert!(result.is_err());
            }

            test(-1, 1.);
            test(0, 1.);
            test(1, 1.);
            test(2, 1.);
            test(3, -1.);
            test(3, 0.);
        }
    }
}

