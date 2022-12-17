use log::info;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn main(my_object: JsValue, my_other_object: JsValue) {
  use js_sys::Reflect;

  console_error_panic_hook::set_once();
  console_log::init().unwrap();

  info!("{:?}", my_object);

  // let reflection_result = Reflect::get(&my_object, &JsValue::from_str("my_property"))
  //   .expect("Failed to get property on object");
}
