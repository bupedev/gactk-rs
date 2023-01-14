use num_traits::real::Real;

pub trait RealConst: Real {
    const FRAC_PI_2: Self;
    const FRAC_PI_3: Self;
    const FRAC_PI_4: Self;
    const FRAC_PI_6: Self;
    const PI: Self;
    const TAU: Self;
    const TWO: Self;
    const HALF: Self;
}

impl RealConst for f32 {
    const FRAC_PI_2: Self = std::f32::consts::FRAC_PI_2;
    const FRAC_PI_3: Self = std::f32::consts::FRAC_PI_3;
    const FRAC_PI_4: Self = std::f32::consts::FRAC_PI_4;
    const FRAC_PI_6: Self = std::f32::consts::FRAC_PI_6;
    const PI: Self = std::f32::consts::PI;
    const TAU: Self = std::f32::consts::TAU;
    const TWO: Self = 2.0;
    const HALF: Self = 0.5;
}

impl RealConst for f64 {
    const FRAC_PI_2: Self = std::f64::consts::FRAC_PI_2;
    const FRAC_PI_3: Self = std::f64::consts::FRAC_PI_3;
    const FRAC_PI_4: Self = std::f64::consts::FRAC_PI_4;
    const FRAC_PI_6: Self = std::f64::consts::FRAC_PI_6;
    const PI: Self = std::f64::consts::PI;
    const TAU: Self = std::f64::consts::TAU;
    const TWO: Self = 2.0;
    const HALF: Self = 0.5;
}