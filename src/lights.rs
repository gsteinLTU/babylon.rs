use wasm_bindgen::prelude::wasm_bindgen;

use crate::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type HemisphericLight;
    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(name: &str, direction: Vector3, scene: &Scene) -> HemisphericLight;
}

#[wasm_bindgen]
extern "C" {
    pub type PointLight;
    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(name: &str, position: Vector3, scene: &Scene) -> PointLight;
}
