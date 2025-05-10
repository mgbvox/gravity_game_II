// CURRENTLY BROKEN
// #![cfg(target_arch = "wasm32")]
// 
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use web_sys::{Event, HtmlCanvasElement};
// 
// pub fn install() {
//     let document = web_sys::window()
//         .expect("no global `window`")
//         .document()
//         .expect("should have a `document`");
// 
//     // Prefer the user‑supplied canvas; fall back to the first <canvas> Bevy creates.
//     let canvas: HtmlCanvasElement = document
//         .get_element_by_id("bevy")
//         .or_else(|| document.query_selector("canvas").ok().flatten())
//         .expect("canvas element not found")
//         .dyn_into()
//         .unwrap();
// 
//     // One closure reused for all events.
//     let cb = Closure::<dyn FnMut(Event)>::wrap(Box::new(|e: Event| {
//         e.prevent_default();
//     }));
// 
//     for ev in ["touchstart", "touchmove", "touchend", "contextmenu"] {
//         canvas
//             .add_event_listener_with_callback(ev, cb.as_ref().unchecked_ref())
//             .unwrap();
//     }
// 
//     // Leak the closure so it lives for the life‑time of the app.
//     cb.forget();
// }