use crate::utils::format_duration;
use wasm_bindgen::prelude::*;
use web_sys::Document;

fn document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

fn element(document: &Document, element_id: &str) -> web_sys::HtmlElement {
    document
        .get_element_by_id(element_id)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
}

pub fn update_text_display(score: i32, lines: i32, level: i32) {
    let document = document();
    element(&document, "score").set_text_content(Some(&score.to_string()));
    element(&document, "lines").set_text_content(Some(&lines.to_string()));
    element(&document, "level").set_text_content(Some(&level.to_string()));
}

pub fn update_duration_display(duration: u32) {
    let document = document();
    element(&document, "duration").set_text_content(Some(&format_duration(duration)));
}

pub fn set_background_colour(colour: &str) {
    let document = document();
    let element = element(&document, "body");
    let style = element.style();
    style.set_css_text(&("background-color: ".to_owned() + colour + ";"));
}
