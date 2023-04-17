use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    /// A three-dimensional vector
    pub type Vector3;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3;

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

impl From<(f64, f64, f64)> for Vector3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Vector3::new(value.0, value.1, value.2)
    }
}

#[wasm_bindgen]
extern "C" {
    /// A four-dimensional vector
    pub type Vector4;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector4;

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
    /// An RGB color
    pub type Color3;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(r: f64, g: f64, b: f64) -> Color3;
}

#[wasm_bindgen]
extern "C" {
    /// An RGBA color
    pub type Color4;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color4;
}

#[wasm_bindgen]
extern "C" {
    pub type Quaternion;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Quaternion;
}
