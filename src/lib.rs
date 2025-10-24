use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{window, Window, Document, HtmlInputElement};
use wasm_bindgen::JsCast;

thread_local! {
    static DIVE_COMPUTER: RefCell<DiveComputer> = RefCell::new(DiveComputer::new());
    static DOCUMENT: RefCell<Document> = RefCell::new(window().unwrap().document().unwrap());
}

struct DiveComputer {
    c_20: TissueCompartment,
}

impl DiveComputer {
    fn new() -> Self {
        Self {
            c_20: TissueCompartment::new(20f32, 3f32, 0.3f32),
        }
    }

    fn run_iteration(&mut self, time_at_depth: Minutes, depth: Feet) {
        self.c_20.simulate(time_at_depth, depth, 0.79);
    }

    fn c_20(&self) -> &TissueCompartment {
        &self.c_20
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn console_log(s: &str);
}

#[wasm_bindgen]
pub fn run_iteration() {
    DOCUMENT.with(|doc| {
        let depth: f32 = doc.borrow()
            .get_element_by_id("depth")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse()
            .unwrap();
        let time_at_depth: f32 = doc.borrow()
            .get_element_by_id("time_at_depth")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse()
            .unwrap();

        DIVE_COMPUTER.with(|dc| {
            dc.borrow_mut()
                .run_iteration(Minutes(time_at_depth), Feet(depth));

            doc.borrow()
                .get_element_by_id("comp20_nitro")
                .unwrap()
                .set_inner_html(&format!("{}", dc.borrow().c_20().nitrogen_concentration()));
        });

    });
}

#[wasm_bindgen]
pub fn initialise() {
    let window: Window = window().unwrap();
    let document: Document = window
        .document()
        .unwrap();

    document.get_element_by_id("comp20_nitro")
        .unwrap()
        .set_inner_html("0.79");
}

const AIR_NITROGEN_RATIO: f32 = 0.79;
const WATER_VAPOUR_PRESSURE: f32 = 0.063;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Ata(f32);

impl Add for Ata {
    type Output = Ata;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Display for Ata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}ata", self.0))
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Minutes(f32);

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Feet(f32);

impl Feet {
    fn depth_atmospheric_pressure(&self) -> Ata {
        Ata(1f32 + (self.0 / 33f32))
    }
}

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

    fn nitrogen_concentration(&self) -> Ata {
        self.nitrogen_concentration
    }
}