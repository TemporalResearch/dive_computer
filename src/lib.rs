mod dive_computer;
mod tissue_compartment;
mod measures;

use std::cell::Cell;
use crate::measures::Feet;
use std::cell::RefCell;
use std::fmt::format;
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

            let mut chosen_compartment = 0;
            let mut chosen_compartment_min_safe_depth = Feet(0f32);

            for (i, compartment) in dc.borrow().compartments().iter().enumerate() {
                let min_safe_depth = compartment.get_min_safe_depth();
                if min_safe_depth > chosen_compartment_min_safe_depth {
                    chosen_compartment = i;
                    chosen_compartment_min_safe_depth = min_safe_depth;
                }

                update_compartment_graph(&doc.borrow(), i, compartment);


                // TODO: incorporate this into our graph.
                // doc.borrow()
                //     .get_element_by_id(&format!("compartment_min_depth_{}", i))
                //     .unwrap()
                //     .set_inner_html(&format!("{}", min_safe_depth));
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

struct SvgRenderer {
    document: Document,
    svg_element: Element,
}

const SVG_NS: &str = "http://www.w3.org/2000/svg";

impl SvgRenderer {
    fn new(width: u32, height: u32) -> Self {
        let document = window().unwrap()
            .document().unwrap();
        let svg_element = document.create_element_ns(Some(SVG_NS), "svg").unwrap();
        svg_element.set_attribute("width", &format!("{}", width));
        svg_element.set_attribute("height", &format!("{}", height));
        svg_element.set_attribute("xmlns", "http://www.w3.org/2000/svg");

        Self {
            document,
            svg_element,
        }
    }

    fn get_element(&self) -> &Element {
        &self.svg_element
    }

    fn rect(&mut self, x: i32, y: i32, width: i32, height: i32, fill_color: &str, id: &str) -> &mut Self {
        let rect = self.document.create_element_ns(Some(SVG_NS), "rect").unwrap();

        rect.set_attribute("x", &format!("{}", x)).unwrap();
        rect.set_attribute("y", &format!("{}", y)).unwrap();
        rect.set_attribute("width", &format!("{}", width)).unwrap();
        rect.set_attribute("height", &format!("{}", height)).unwrap();
        rect.set_attribute("fill", fill_color).unwrap();
        rect.set_attribute("id", id).unwrap();

        self.svg_element.append_child(&rect);

        self
    }

    fn text(&mut self, x: i32, y: i32, fill_color: &str, font_size: i32, id: &str, value: &str) -> &mut Self {
        let text = self.document.create_element_ns(Some(SVG_NS), "text").unwrap();
        text.set_attribute("x", &format!("{}", x));
        text.set_attribute("y", &format!("{}", y));
        text.set_attribute("fill", fill_color);
        text.set_attribute("font-size", &format!("{}", font_size));
        text.set_attribute("id", id);

        text.set_text_content(Some(value));

        self.svg_element.append_child(&text);

        self
    }

    // TODO: text
}

const GRAPH_Y_OFFSET: i32 = 50;
const GRAPH_WIDTH: i32 = 200;
const GRAPH_HEIGHT: i32 = 200;

fn create_compartment_svg(compartment_index: usize) -> Element {
    SvgRenderer::new(200, 250)
        .rect(0, GRAPH_Y_OFFSET, GRAPH_WIDTH, GRAPH_HEIGHT, "beige", &format!("m_val_at_depth_{}", compartment_index))
        .text(6, GRAPH_Y_OFFSET - 4, "black", 18, &format!("m_val_at_depth_display_{}", compartment_index), "3.45ata")
        .rect(0, GRAPH_Y_OFFSET + 60, GRAPH_WIDTH, 140, "lightgreen", &format!("surface_m_val_{}", compartment_index))
        .text(6, GRAPH_Y_OFFSET + 78, "black", 18, &format!("surface_m_val_display_{}", compartment_index), "2.0ata")
        .rect(100, GRAPH_Y_OFFSET + 100, GRAPH_WIDTH / 2, 100, "goldenrod", &format!("current_sat_{}", compartment_index))
        .text(106, GRAPH_Y_OFFSET + 118, "black", 18, &format!("current_sat_display_{}", compartment_index), "0.79ata")
        .text(6, GRAPH_Y_OFFSET + 220, "black", 20, "compartment_label", &format!("Compartment {}mins", "5"))
        .get_element()
        .clone()
}

fn update_compartment_graph(doc: &Document, compartment_index: usize, compartment: &TissueCompartment) {
    let current_sat_graph_size =
        ((compartment.nitrogen_concentration().0 / compartment.get_m_value_at_depth(CURRENT_DEPTH.get()).0)
            * GRAPH_HEIGHT as f32) as i32;
    let current_sat_rect = doc
        .get_element_by_id(&format!("current_sat_{}", compartment_index))
        .unwrap();
    let current_sat_display = doc
        .get_element_by_id(&format!("current_sat_display_{}", compartment_index))
        .unwrap();
    current_sat_display.set_text_content(Some(&format!("{}", compartment.nitrogen_concentration())));
    current_sat_display.set_attribute(
        "y", &format!("{}", GRAPH_Y_OFFSET + (GRAPH_HEIGHT - current_sat_graph_size) + 18));
    current_sat_rect.set_attribute(
        "y", &format!("{}", GRAPH_Y_OFFSET + GRAPH_HEIGHT - current_sat_graph_size));
    current_sat_rect.set_attribute(
        "height", &format!("{}", current_sat_graph_size));

    let current_surface_m_value_size =
        ((compartment.get_m_value_at_depth(Feet(0f32)).0 / compartment.get_m_value_at_depth(CURRENT_DEPTH.get()).0)
            * GRAPH_HEIGHT as f32) as i32;
    let surface_m_value_rect = doc
        .get_element_by_id(&format!("surface_m_val_{}", compartment_index))
        .unwrap();
    let surface_m_value_display = doc
        .get_element_by_id(&format!("surface_m_val_display_{}", compartment_index))
        .unwrap();
    surface_m_value_display.set_attribute(
        "y", &format!("{}", GRAPH_Y_OFFSET + (GRAPH_HEIGHT - current_surface_m_value_size) + 18));
    surface_m_value_rect.set_attribute(
        "y", &format!("{}", GRAPH_Y_OFFSET + GRAPH_HEIGHT - current_surface_m_value_size));
    surface_m_value_rect.set_attribute(
        "height", &format!("{}", current_surface_m_value_size));

        // .get_element_by_id(&format!("surface_m_val_{}", GRAPH_Y_OFFSET + (GRAPH_HEIGHT - current_sat_graph_size)))

    let m_val_at_depth_display = doc
        .get_element_by_id(&format!("m_val_at_depth_display_{}", compartment_index))
        .unwrap();
    m_val_at_depth_display.set_text_content(Some(&format!("{}", compartment.get_m_value_at_depth(CURRENT_DEPTH.get()))));

    let surface_m_val_display = doc
        .get_element_by_id(&format!("surface_m_val_display_{}", compartment_index))
        .unwrap();
    surface_m_val_display.set_text_content(Some(&format!("{}", compartment.get_m_value_at_depth(Feet(0f32)))));
}

#[wasm_bindgen]
pub fn initialise() {
    let window: Window = window().unwrap();
    let document: Document = window
        .document()
        .unwrap();

    let compartment_graph: Element = document.get_element_by_id("compartment_graph")
        .unwrap();

    document.get_element_by_id("current_depth")
        .unwrap()
        .set_inner_html(&format!("{}", CURRENT_DEPTH.get().0));

    DIVE_COMPUTER.with(|dc| {
        for (i, compartment) in dc.borrow().compartments().iter().enumerate() {
            compartment_graph.append_child(&create_compartment_svg(i));

            update_compartment_graph(&document, i, compartment);
        }
    });
}

