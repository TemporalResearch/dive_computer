mod dive_computer;
mod tissue_compartment;
mod measures;

use std::cell::Cell;
use crate::measures::Feet;
use std::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{window, Window, Document, HtmlInputElement, Element};
use wasm_bindgen::JsCast;
use crate::dive_computer::DiveComputer;
use measures::Minutes;

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

            let mut chosen_compartment = 0;
            let mut chosen_compartment_min_safe_depth = Feet(0f32);

            for (i, compartment) in dc.borrow().compartments().iter().enumerate() {
                let min_safe_depth = compartment.get_min_safe_depth();
                if min_safe_depth > chosen_compartment_min_safe_depth {
                    chosen_compartment = i;
                    chosen_compartment_min_safe_depth = min_safe_depth;
                }

                doc.borrow()
                    .get_element_by_id(&format!("compartment_display_{}", i))
                    .unwrap()
                    .set_inner_html(&format!("{}", compartment.nitrogen_concentration()));

                doc.borrow()
                    .get_element_by_id(&format!("compartment_m_val_{}", i))
                    .unwrap()
                    .set_inner_html(&format!("{}",
                        compartment.get_m_value_at_depth(CURRENT_DEPTH.get())));

                doc.borrow()
                    .get_element_by_id(&format!("compartment_min_depth_{}", i))
                    .unwrap()
                    .set_inner_html(&format!("{}", min_safe_depth));
            }

            doc.borrow()
                .get_element_by_id("chosen_compartment")
                .unwrap()
                .set_inner_html(
                    &format!("{}min", dc.borrow().compartments()[chosen_compartment].half_time().0));
            doc.borrow()
                .get_element_by_id("chosen_compartment_depth_ceiling")
                .unwrap()
                .set_inner_html(
                    &format!("{}", chosen_compartment_min_safe_depth)
                );
        });

    });
}

#[wasm_bindgen]
pub fn initialise() {
    let window: Window = window().unwrap();
    let document: Document = window
        .document()
        .unwrap();

    let compartment_list: Element = document.get_element_by_id("compartment_list")
        .unwrap();

    document.get_element_by_id("current_depth")
        .unwrap()
        .set_inner_html(&format!("{}", CURRENT_DEPTH.get().0));

    DIVE_COMPUTER.with(|dc| {
        for (i, compartment) in dc.borrow().compartments().iter().enumerate() {
            let compartment_display_row = document.create_element("p").unwrap();

            let compartment_display_label = document.create_element("span").unwrap();
            compartment_display_label.set_text_content(
                Some(&format!("Compartment {}min: ", compartment.half_time().0)));

            let compartment_display = document.create_element("b").unwrap();
            compartment_display.set_id(&format!("compartment_display_{}", i));
            compartment_display.set_text_content(Some("0.79ata"));

            let m_val_label = document.create_text_node(" ; M-Val at Depth: ");
            let compartment_m_val_display = document.create_element("b").unwrap();
            compartment_m_val_display.set_id(&format!("compartment_m_val_{}", i));
            compartment_m_val_display.set_text_content(Some(
                &format!("{}", compartment.get_m_value_at_depth(Feet(0f32)))));

            let max_safe_depth_label = document.create_text_node(" ; Min safe depth: ");
            let max_safe_depth_display = document.create_element("b").unwrap();
            max_safe_depth_display.set_id(&format!("compartment_min_depth_{}", i));
            max_safe_depth_display.set_text_content(Some(
                &format!("{}", compartment.get_min_safe_depth())
            ));

            let _ = compartment_display_row.append_child(&compartment_display_label);
            let _ = compartment_display_row.append_child(&compartment_display);
            let _ = compartment_display_row.append_child(&m_val_label);
            let _ = compartment_display_row.append_child(&compartment_m_val_display);
            let _ = compartment_display_row.append_child(&max_safe_depth_label);
            let _ = compartment_display_row.append_child(&max_safe_depth_display);

            let _ = compartment_list.append_child(&compartment_display_row);
        }
    });
}

