use crate::measures::{AIR_NITROGEN_RATIO, Ata, Feet, Minutes, WATER_VAPOUR_PRESSURE};

pub struct TissueCompartment {
    // constants
    surface_m_value: Ata,
    m_slope: f32,
    half_time: Minutes,

    nitrogen_concentration: Ata,
}

impl TissueCompartment {
    pub fn new(half_time: f32, surface_m_value: f32, m_slope: f32) -> TissueCompartment {
        TissueCompartment {
            surface_m_value: Ata(surface_m_value),
            m_slope,
            half_time: Minutes(half_time),

            nitrogen_concentration: Ata(AIR_NITROGEN_RATIO), // Start at sea level nitrogen pressure
        }
    }

    pub fn simulate(&mut self, time_at_depth: Minutes, depth: Feet, nitrogen_ratio: f32) {
        let inspired_pressure = (depth.depth_atmospheric_pressure().0 - WATER_VAPOUR_PRESSURE) * nitrogen_ratio;
        let k = 2f32.ln() / self.half_time.0;

        self.nitrogen_concentration = Ata(
            self.nitrogen_concentration.0
                + (
                (inspired_pressure - self.nitrogen_concentration.0)
                    * (1f32 - std::f32::consts::E.powf(-(k * time_at_depth.0)))
            ));
    }

    pub(crate) fn nitrogen_concentration(&self) -> Ata {
        self.nitrogen_concentration
    }

    pub(crate) fn half_time(&self) -> Minutes {
        self.half_time
    }
}
