use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use js_sys::Reflect;

use crate::{prelude::*, get_set_jsvalue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = BABYLON)]
    pub type Light;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Light, js_namespace = BABYLON)]
    pub type HemisphericLight;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(name: &str, direction: Vector3, scene: &Scene) -> HemisphericLight;
}

impl HemisphericLight {
    get_set_jsvalue!(get_direction, set_direction, "direction", Vector3);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Light, js_namespace = BABYLON)]
    pub type PointLight;
    
    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(name: &str, position: Vector3, scene: &Scene) -> PointLight;
}
