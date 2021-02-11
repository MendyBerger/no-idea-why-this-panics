extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use crate::dom::Component;

mod dom;


#[wasm_bindgen(start)]
pub async fn main_js() {
    console_error_panic_hook::set_once();

    dominator::append_dom(&dominator::get_id("app"), Component::render(Component::new()));
}
