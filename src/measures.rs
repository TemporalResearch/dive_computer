use std::fmt::{Display, Formatter};
use std::ops::Add;

pub(crate) const AIR_NITROGEN_RATIO: f32 = 0.79;
pub(crate) const WATER_VAPOUR_PRESSURE: f32 = 0.063;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub(crate) struct Ata(pub f32);

impl Add for Ata {
    type Output = Ata;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Display for Ata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:.2}ata", self.0))
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub(crate) struct Minutes(pub f32);

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub(crate) struct Feet(pub f32);

impl Feet {
    pub(crate) fn depth_atmospheric_pressure(&self) -> Ata {
        Ata(1f32 + (self.0 / 33f32))
    }
}

impl Display for Feet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}ft", self.0))
    }
}
