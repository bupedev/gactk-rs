use std::{fmt::Display, ops::Rem};

use num_traits::{real::Real, Euclid, PrimInt, Zero};

use crate::numerics::RealConst;

use super::{LineSegment2, Vec2};

pub enum AngularDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Poly2<T: Real> {
    pub vertices: Vec<Vec2<T>>,
}

impl<T: Real + RealConst + Euclid> Poly2<T> {
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

impl<T: Real + RealConst + Euclid> Poly2<T> {
    pub fn regular<I: PrimInt>(vertex_count: I, side_length: T) -> Self {
        if vertex_count <= I::zero() {
            panic!("polygons cannot have a non-positive number of vertices");
        }

        if side_length <= T::zero() {
            panic!("polygon side lengths must be strictly positive");
        }

        let n = T::from(vertex_count).expect("cast failure");
        let radius = side_length * T::HALF / (T::PI / n).sin();
        let angle = T::TWO * T::PI / n;
        let limit = T::TAU * (T::one() - T::one() / n / T::TWO);

        let mut vertices = vec![];
        let mut cum_angle = T::zero();
        while cum_angle < limit {
            vertices.push(Vec2::unit(cum_angle) * radius);
            cum_angle = cum_angle + angle;
        }
        Self::new(&vertices)
    }
}

impl<T: Real + RealConst + Euclid> Poly2<T> {
    pub fn translate(&self, displacement: Vec2<T>) -> Self {
        let translated_vertices: Vec<Vec2<T>> =
            self.vertices.iter().map(|&x| x + displacement).collect();
        Self::new(&translated_vertices)
    }

    pub fn rotate(&self, radians: T) -> Self {
        let rotated_vertices: Vec<Vec2<T>> =
            self.vertices.iter().map(|x| x.rotate(radians)).collect();
        Self::new(&rotated_vertices)
    }

    pub fn reflect(&self, axis: Vec2<T>) -> Self {
        let reflected_vertices: Vec<Vec2<T>> =
            self.vertices.iter().map(|x| x.reflect(axis)).collect();
        Self::new(&reflected_vertices)
    }

    fn angular_sum(&self) -> T {
        let mut sum = T::zero();
        let vertices = &self.vertices;
        let count = vertices.len();

        if count < 1 {
            return sum;
        }

        let last_heading = vertices[0] - vertices[count - 1];
        let mut current_heading = last_heading;
        for i in 1..count {
            let next_heading = vertices[i] - vertices[i - 1];
            let incr = current_heading.angle_to(next_heading);
            sum = sum + incr;
            current_heading = next_heading;
        }

        let incr = current_heading.angle_to(last_heading);
        sum + incr
    }

    fn centroid(&self) -> Vec2<T>
    where
        T: Real + RealConst,
    {
        let v = &self.vertices;
        let n = v.len();

        match n {
            0 => return Vec2::<T>::zero(),
            1 => return v.first().unwrap().clone(),
            _ => (),
        }

        let cross: Vec<T> = (0..n)
            .map(|i| v[i.rem(n)].cross(v[(i + 1).rem(n)]))
            .collect();

        (0..n)
            .map(|i| (v[i.rem(n)] + v[(i + 1).rem(n)]) * cross[i])
            .fold(Vec2::zero(), |sum, product| sum + product)
            / cross.iter().fold(T::zero(), |sum, &product| sum + product)
            / T::THREE
    }

    pub fn edges(&self) -> Vec<LineSegment2<T>> {
        (0..self.vertices.len() - 1)
            .map(|i| LineSegment2::new(self.vertices[i], self.vertices[i + 1]))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6};

    const EPSILON: f64 = 1.5e-14;

    mod constructors {
        use super::*;
        use num_traits::Zero;
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

            let clockwise_square = vec![
                Vec2::new(-0.5, -0.5),
                Vec2::new(-0.5, 0.5),
                Vec2::new(0.5, 0.5),
                Vec2::new(0.5, -0.5),
            ];
            test(&clockwise_square, &clockwise_square);

            let duplicates = vec![
                Vec2::new(-0.5, -0.5),
                Vec2::new(-0.5, -0.5),
                Vec2::new(-0.5, 0.5),
                Vec2::new(0.5, 0.5),
                Vec2::new(0.5, -0.5),
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
            test(vec![
                Vec2::new(1., 0.),
                Vec2::new(-1., 0.),
                Vec2::new(-1., 0.),
            ]);
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
            test(
                3,
                1.,
                vec![
                    Vec2::unit(0. * FRAC_PI_3) * tri_length,
                    Vec2::unit(2. * FRAC_PI_3) * tri_length,
                    Vec2::unit(4. * FRAC_PI_3) * tri_length,
                ],
            );
            test(
                3,
                2.,
                vec![
                    Vec2::unit(0. * FRAC_PI_3) * 2. * tri_length,
                    Vec2::unit(2. * FRAC_PI_3) * 2. * tri_length,
                    Vec2::unit(4. * FRAC_PI_3) * 2. * tri_length,
                ],
            );

            let quad_length = 0.5 / FRAC_PI_4.sin();
            test(
                4,
                1.,
                vec![
                    Vec2::unit(0. * FRAC_PI_2) * quad_length,
                    Vec2::unit(1. * FRAC_PI_2) * quad_length,
                    Vec2::unit(2. * FRAC_PI_2) * quad_length,
                    Vec2::unit(3. * FRAC_PI_2) * quad_length,
                ],
            );
            test(
                4,
                2.,
                vec![
                    Vec2::unit(0. * FRAC_PI_2) * 2. * quad_length,
                    Vec2::unit(1. * FRAC_PI_2) * 2. * quad_length,
                    Vec2::unit(2. * FRAC_PI_2) * 2. * quad_length,
                    Vec2::unit(3. * FRAC_PI_2) * 2. * quad_length,
                ],
            );

            let hex_length = 0.5 / FRAC_PI_6.sin();
            test(
                6,
                1.,
                vec![
                    Vec2::unit(0. * FRAC_PI_3) * hex_length,
                    Vec2::unit(1. * FRAC_PI_3) * hex_length,
                    Vec2::unit(2. * FRAC_PI_3) * hex_length,
                    Vec2::unit(3. * FRAC_PI_3) * hex_length,
                    Vec2::unit(4. * FRAC_PI_3) * hex_length,
                    Vec2::unit(5. * FRAC_PI_3) * hex_length,
                ],
            );
            test(
                6,
                2.,
                vec![
                    Vec2::unit(0. * FRAC_PI_3) * 2. * hex_length,
                    Vec2::unit(1. * FRAC_PI_3) * 2. * hex_length,
                    Vec2::unit(2. * FRAC_PI_3) * 2. * hex_length,
                    Vec2::unit(3. * FRAC_PI_3) * 2. * hex_length,
                    Vec2::unit(4. * FRAC_PI_3) * 2. * hex_length,
                    Vec2::unit(5. * FRAC_PI_3) * 2. * hex_length,
                ],
            );
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

    mod methods {
        use super::*;
        use std::f64::consts::PI;

        #[test]
        fn angular_sum() {
            fn test(polygon: Poly2<f64>, expected: f64) {
                let actual = polygon.angular_sum();
                assert!((actual - expected).abs() < EPSILON);
            }

            let clockwise_square = vec![
                Vec2::new(0.5, 0.5),
                Vec2::new(-0.5, 0.5),
                Vec2::new(-0.5, -0.5),
                Vec2::new(0.5, -0.5),
            ];
            test(Poly2::new(&clockwise_square), 2. * PI);

            let counter_clockwise_square = vec![
                Vec2::new(0.5, 0.5),
                Vec2::new(0.5, -0.5),
                Vec2::new(-0.5, -0.5),
                Vec2::new(-0.5, 0.5),
            ];
            test(Poly2::new(&counter_clockwise_square), -2. * PI);

            let winding_heart = vec![
                Vec2::new(0.5, 0.5),
                Vec2::new(-0.5, 0.5),
                Vec2::new(-0.5, 0.0),
                Vec2::new(0.25, 0.0),
                Vec2::new(0.25, 0.25),
                Vec2::new(0.0, 0.25),
                Vec2::new(0.0, -0.5),
                Vec2::new(0.5, -0.5),
            ];
            test(Poly2::new(&winding_heart), 4. * PI);

            let figure_eight = vec![
                Vec2::new(0.5, 0.5),
                Vec2::new(0.0, 0.5),
                Vec2::new(0.0, -0.5),
                Vec2::new(-0.5, -0.5),
                Vec2::new(-0.5, 0.0),
                Vec2::new(0.5, 0.0),
            ];
            test(Poly2::new(&figure_eight), 0.);
        }

        #[test]
        fn centroid() {
            fn test(polygon: Poly2<f64>, expected: Vec2<f64>) {
                let actual = polygon.centroid();
                assert!((actual - expected).magnitude() < EPSILON);
            }

            let displacement_vectors = vec![
                Vec2::zero(),
                Vec2::unit(FRAC_PI_3),
                Vec2::unit(3. * FRAC_PI_4),
                Vec2::unit(7. * FRAC_PI_6),
                Vec2::unit(3. * FRAC_PI_2),
            ];

            for i in 3..10 {
                for &displacement_vector in &displacement_vectors {
                    test(
                        Poly2::regular(i, 1.).translate(displacement_vector),
                        displacement_vector,
                    );
                }
            }
        }
    }
}
