use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
    svg.set_attribute("width", "100")?;
    svg.set_attribute("hight", "100")?;
    svg.set_attribute("viewBox", "0 0 100 100")?;

    let circle = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    circle.set_attribute("cx", "50")?;
    circle.set_attribute("cy", "50")?;
    circle.set_attribute("r", "20")?;
    circle.set_attribute("stroke", "black")?;
    circle.set_attribute("fill", "red")?;
    svg.append_child(&circle)?;
    body.append_child(&svg)?;

    Ok(())
}
