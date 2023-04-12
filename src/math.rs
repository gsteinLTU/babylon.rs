use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {    
    pub type Vector3;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(x: f64, y:f64, z:f64) -> Vector3;

    #[wasm_bindgen(js_namespace = BABYLON, static_method_of = Vector3)]
    pub fn One() -> Vector3;    

    #[wasm_bindgen(js_namespace = BABYLON, static_method_of = Vector3)]
    pub fn Zero() -> Vector3;    

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Vector3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &Vector3, value: f64);

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Vector3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &Vector3, value: f64);

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Vector3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &Vector3, value: f64);
}

#[wasm_bindgen]
extern "C" {
    pub type Vector4;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(x: f64, y:f64, z:f64, w:f64) -> Vector4;

    #[wasm_bindgen(js_namespace = BABYLON, static_method_of = Vector4)]
    pub fn One() -> Vector4; 

    #[wasm_bindgen(js_namespace = BABYLON, static_method_of = Vector4)]
    pub fn Zero() -> Vector4;    

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Vector4) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &Vector4, value: f64);

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Vector4) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &Vector4, value: f64);

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Vector4) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &Vector4, value: f64);

    #[wasm_bindgen(method, getter)]
    pub fn w(this: &Vector4) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_w(this: &Vector4, value: f64);
}

#[wasm_bindgen]
extern "C" {    
    pub type Color;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(r: f64, g:f64, b:f64) -> Color;
}