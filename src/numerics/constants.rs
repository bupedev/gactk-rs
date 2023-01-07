use num_traits::real::Real;

pub trait RealConst: Real {
    const PI: Self;
    const TAU: Self;
    const TWO: Self;
    const HALF: Self;
}

impl RealConst for f32 {
    const PI: Self = std::f32::consts::PI;
    const TAU: Self = std::f32::consts::TAU;
    const TWO: Self = 2.0;
    const HALF: Self = 0.5;
}

impl RealConst for f64 {
    const PI: Self = std::f64::consts::PI;
    const TAU: Self = std::f64::consts::TAU;
    const TWO: Self = 2.0;
    const HALF: Self = 0.5;
}