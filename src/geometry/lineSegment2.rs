use num_traits::{real::Real, Euclid};
use crate::numerics::{lerp, RealConst};
use super::Vec2;

#[derive(Debug, PartialEq, Clone)]
pub struct LineSegment2<T : Real> {
    pub start: Vec2<T>,
    pub end: Vec2<T>
}

impl<T: Real + RealConst + Euclid> LineSegment2<T> {
    pub fn new(start: Vec2<T>, end: Vec2<T>) -> Self {
        Self { start, end }
    }

    pub fn centre(&self) -> Vec2<T> {
        lerp(self.start, self.end, T::HALF)
    }
}
