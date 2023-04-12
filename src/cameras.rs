use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;
use crate::prelude::*;

#[wasm_bindgen]
extern "C" {    
    pub type ArcRotateCamera;
    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(name: &str, alpha: f64, beta: f64, radius: f64, target: Vector3, scene: Option<&Scene>, setActiveOnSceneIfNoneActive: Option<bool>) -> ArcRotateCamera;

    #[wasm_bindgen(method)]
    pub fn attachControl(this: &ArcRotateCamera, canvas: Element, noPreventDefault: bool);
}
