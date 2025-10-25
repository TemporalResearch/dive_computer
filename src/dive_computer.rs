use crate::measures::{Feet, Minutes};
use crate::tissue_compartment::TissueCompartment;

pub(crate) struct DiveComputer {
    c_20: TissueCompartment,
}

impl DiveComputer {
    pub(crate) fn new() -> Self {
        Self {
            c_20: TissueCompartment::new(20f32, 3f32, 0.3f32),
        }
    }

    pub(crate) fn run_iteration(&mut self, time_at_depth: Minutes, depth: Feet) {
        self.c_20.simulate(time_at_depth, depth, 0.79);
    }

    pub(crate) fn c_20(&self) -> &TissueCompartment {
        &self.c_20
    }
}
