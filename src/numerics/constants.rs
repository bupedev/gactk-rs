use num_traits::real::Real;

pub trait RealConst: Real {
    const PI: Self;
    const TAU: Self;
}

impl RealConst for f32 {
    const PI: Self = std::f32::consts::PI;
    const TAU: Self = std::f32::consts::TAU;
}

impl RealConst for f64 {
    const PI: Self = std::f64::consts::PI;
    const TAU: Self = std::f64::consts::TAU;
}