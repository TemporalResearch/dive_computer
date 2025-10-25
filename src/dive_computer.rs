use crate::measures::{Feet, Minutes};
use crate::tissue_compartment::TissueCompartment;

pub(crate) struct DiveComputer {
    compartments: Vec<TissueCompartment>,
}

impl DiveComputer {
    pub(crate) fn new() -> Self {
        Self {
            compartments: vec![
                TissueCompartment::new(5f32, 4.15f32, 0.0479f32),
                TissueCompartment::new(10f32, 3.67f32, 0.0479f32),
                TissueCompartment::new(20f32, 3.27f32, 0.0585f32),
                TissueCompartment::new(40f32, 2.87f32, 0.0479f32),
                TissueCompartment::new(80f32, 2.67f32, 0.0479f32),
                TissueCompartment::new(120f32, 2.52f32, 0.0479f32),
            ],
        }
    }

    pub(crate) fn run_iteration(&mut self, time_at_depth: Minutes, depth: Feet) {
        for compartment in &mut self.compartments {
            compartment.simulate(time_at_depth, depth, 0.79)
        }
    }

    pub(crate) fn compartments(&self) -> &Vec<TissueCompartment> {
        &self.compartments
    }
}
