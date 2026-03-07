use std::ops::Mul;


struct Kilometers (f64);

impl From<f64> for Kilometers {
    fn from(kms: f64) -> Kilometers{
        return Kilometers(kms);
    }
}

impl Mul<f64> for Kilometers {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        return Self::from(self.0 * rhs);
    }
}

impl Kilometers {
    const fn new(val: f64) -> Self {
        Kilometers(val)
    }
    const fn mul(self, rhs: f64) -> Self {
        Kilometers(self.0 * rhs)
    }
}

// 1 AU in kilometers 
const ASTRONOMICAL_UNIT: Kilometers = Kilometers::new(149_597_870.700);
const INNER_SOLAR_SYSTEM_RADIUS: Kilometers = ASTRONOMICAL_UNIT.mul(100.);
const OORT_RADIUS: Kilometers = ASTRONOMICAL_UNIT.mul(200_000.);


