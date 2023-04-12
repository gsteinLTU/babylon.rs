// pub use nalgebra::Rotation3;
// pub use nalgebra::Vector3;

// pub type Color = Vector3<f64>;
// pub type Vector = Vector3<f64>;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {    
    pub type Vector3;
    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(x: f64, y:f64, z:f64) -> Vector3;

    #[wasm_bindgen(js_namespace = BABYLON, static_method_of = Vector3)]
    pub fn Zero() -> Vector3;    
}

#[wasm_bindgen]
extern "C" {    
    pub type Color;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(r: f64, g:f64, b:f64) -> Color;
}