use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlButtonElement;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlParagraphElement;

use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[wasm_bindgen]
pub async fn js_sleep(time_ms: i32) -> Result<(), JsValue> {
    // Create a JavaScript Promise that resolves after 2 seconds
    let promise = Promise::new(&mut |resolve, _| {
        let window = window().unwrap();
        let closure = Closure::once_into_js(move || {
            resolve.call0(&JsValue::null()).unwrap();
        });
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                time_ms, // 2000 milliseconds = 2 seconds
            )
            .unwrap();
    });

    // Convert the Promise to a Future and await it
    JsFuture::from(promise).await.map(|_| ())
}

pub fn get_canvas_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    return ctx;
}

pub fn get_canvas(id: &str) -> HtmlCanvasElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    return canvas;
}

pub fn get_paragraph(id: &str) -> HtmlParagraphElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let paragraph = document.get_element_by_id(id).unwrap();
    let paragraph: web_sys::HtmlParagraphElement = paragraph
        .dyn_into::<web_sys::HtmlParagraphElement>()
        .map_err(|_| ())
        .unwrap();

    return paragraph;
}

pub fn get_button(id: &str) -> HtmlButtonElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let button = document.get_element_by_id(id).unwrap();
    let button: web_sys::HtmlButtonElement = button
        .dyn_into::<web_sys::HtmlButtonElement>()
        .map_err(|_| ())
        .unwrap();

    return button;
}
