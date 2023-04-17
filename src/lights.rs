use wasm_bindgen::prelude::wasm_bindgen;

use crate::prelude::*;

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Light, js_namespace = BABYLON)]
    pub type PointLight;
    
    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(name: &str, position: Vector3, scene: &Scene) -> PointLight;
}
