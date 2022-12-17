import * as wasm from "bindgen-reproduction";

let my_object = {
  my_property: 1,
};

wasm.main(my_object, 2);
