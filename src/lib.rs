mod dive_computer;
mod tissue_compartment;
mod measures;

use std::cell::Cell;
use crate::measures::Feet;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::cell::RefCell;
use std::path::Component;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{window, Window, Document, HtmlInputElement, Element};
use wasm_bindgen::JsCast;
use crate::dive_computer::DiveComputer;
use measures::Minutes;
use crate::tissue_compartment::TissueCompartment;

thread_local! {
    static CURRENT_DEPTH: Cell<Feet> = Cell::new(Feet(0f32));
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

        CURRENT_DEPTH.set(Feet(depth));
        doc.borrow()
            .get_element_by_id("current_depth")
            .unwrap()
            .set_inner_html(&format!("{}", CURRENT_DEPTH.get().0));

        DIVE_COMPUTER.with(|dc| {
            dc.borrow_mut()
                .run_iteration(Minutes(time_at_depth), Feet(depth));

            for (i, compartment) in dc.borrow().compartments().iter().enumerate() {
                doc.borrow()
                    .get_element_by_id(&format!("compartment_display_{}", i))
                    .unwrap()
                    .set_inner_html(&format!("{}", compartment.nitrogen_concentration()));

                doc.borrow()
                    .get_element_by_id(&format!("compartment_m_val_{}", i))
                    .unwrap()
                    .set_inner_html(&format!("{}",
                        compartment.get_m_value_at_depth(CURRENT_DEPTH.get())));
            }
        });

    });
}

#[wasm_bindgen]
pub fn initialise() {
    let window: Window = window().unwrap();
    let document: Document = window
        .document()
        .unwrap();

    let mut compartment_list: Element = document.get_element_by_id("compartment_list")
        .unwrap();

    document.get_element_by_id("current_depth")
        .unwrap()
        .set_inner_html(&format!("{}", CURRENT_DEPTH.get().0));

    DIVE_COMPUTER.with(|dc| {
        for (i, compartment) in dc.borrow().compartments().iter().enumerate() {
            let mut compartment_display_row = document.create_element("p").unwrap();

            let mut compartment_display_label = document.create_element("span").unwrap();
            compartment_display_label.set_text_content(
                Some(&format!("Compartment {}min: ", compartment.half_time().0)));

            let mut compartment_display = document.create_element("b").unwrap();
            compartment_display.set_id(&format!("compartment_display_{}", i));
            compartment_display.set_text_content(Some("0.79ata"));

            let separator = document.create_text_node(" ; M-Val at Depth: ");
            let mut compartment_m_val_display = document.create_element("b").unwrap();
            compartment_m_val_display.set_id(&format!("compartment_m_val_{}", i));
            compartment_m_val_display.set_text_content(Some(
                &format!("{}", compartment.get_m_value_at_depth(Feet(0f32)))));

            compartment_display_row.append_child(&compartment_display_label);
            compartment_display_row.append_child(&compartment_display);
            compartment_display_row.append_child(&separator);
            compartment_display_row.append_child(&compartment_m_val_display);

            compartment_list.append_child(&compartment_display_row);
        }
    })
}

