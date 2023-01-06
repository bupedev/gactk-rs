use num_traits::real::Real;

use super::Vec2;

pub struct Poly2<T: Real> {
    vertices: Vec<Vec2<T>>,
}

impl<T: Real> Poly2<T> {
    pub fn new(vertices: Vec<Vec2<T>>) -> Self {
        Self { vertices }
    }
}


