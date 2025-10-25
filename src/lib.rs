mod dive_computer;
mod tissue_compartment;
mod measures;

use crate::measures::Feet;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{window, Window, Document, HtmlInputElement};
use wasm_bindgen::JsCast;
use crate::dive_computer::DiveComputer;
use measures::Minutes;

thread_local! {
    static DIVE_COMPUTER: RefCell<DiveComputer> = RefCell::new(DiveComputer::new());
    static DOCUMENT: RefCell<Document> = RefCell::new(window().unwrap().document().unwrap());
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

