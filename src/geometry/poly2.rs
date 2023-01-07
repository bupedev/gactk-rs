use num_traits::{real::Real, PrimInt};

use crate::numerics::RealConst;

use super::Vec2;

pub struct Poly2<T: Real> {
    pub vertices: Vec<Vec2<T>>,
}

impl<T: Real> Poly2<T> {
    pub fn new(vertices: &Vec<Vec2<T>>) -> Self {
        // TODO: Panic if less than 3...
        Self { vertices: vertices.clone() }
    }
}

impl<T : Real + RealConst> Poly2<T> {
    pub fn regular<I : PrimInt>(vertex_count: I, side_length: T) -> Self {
        // TODO: Panic if less than 3...
        let n = T::from(vertex_count).expect("shits fucked");
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
        // FRAC_PI_6, 
        // PI
    };

    const EPSILON: f64 = 1.5e-15;

    // #[test]
    // fn display() {
    //     fn test(v: Vec2<f64>, string: &str) {
    //         assert_eq!(v.to_string(), string);
    //     }

    //     test(Vec2::new(-2., 3.), "[-2, 3]");
    //     test(Vec2::new(0.5, 0.), "[0.5, 0]");
    // }

    mod constructors {
        use super::*;

        #[test]
        fn new() {
            let vertices = vec![
                Vec2::new(-0.5, -0.5),
                Vec2::new(-0.5, 0.5),
                Vec2::new(0.5, 0.5),
                Vec2::new(0.5, -0.5)
            ];
            let poly = Poly2::new(&vertices);
            for i in 0..4 {
                assert_eq!(poly.vertices[i], vertices[i]);
            }
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
    }
}

