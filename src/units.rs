use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub(crate) struct Kilometers (f32);

impl From<f32> for Kilometers {
    fn from(kms: f32) -> Kilometers{
        return Kilometers(kms);
    }
}


impl From<Kilometers> for f32 {
    fn from(kms: Kilometers) -> f32{
        return kms.0;
    }
}


impl Mul<f32> for Kilometers {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        return Self::from(self.0 * rhs);
    }
}



impl Div<f32> for Kilometers {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        return Self::from(self.0 / rhs);
    }
}


impl Add<f32> for Kilometers {
    type Output = Self;
    fn add(self, rhs: f32) -> Self {
        return Self::from(self.0 + rhs);
    }
}


impl Sub<f32> for Kilometers {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self {
        return Self::from(self.0 - rhs);
    }
}

impl Kilometers {
    const fn new(val: f32) -> Self {
        Kilometers(val)
    }
    const fn mul(self, rhs: f32) -> Self {
        Kilometers(self.0 * rhs)
    }
}

// 1 AU in kilometers 
pub(crate) const ASTRONOMICAL_UNIT: Kilometers = Kilometers::new(149_597_870.700);
pub(crate) const INNER_SOLAR_SYSTEM_RADIUS: Kilometers = ASTRONOMICAL_UNIT.mul(10.); // 100?
const OORT_RADIUS: Kilometers = ASTRONOMICAL_UNIT.mul(200_000.);


