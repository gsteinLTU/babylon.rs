use std::f64::consts::PI;

use crate::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    pub type Camera;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(
        name: &str,
        position: Vector3,
        scene: Option<&Scene>,
        setActiveOnSceneIfNoneActive: Option<bool>,
    ) -> Camera;
}

#[wasm_bindgen]
extern "C" {
    pub type ArcRotateCamera;
    #[wasm_bindgen(constructor, js_namespace = BABYLON, extends = Camera)]
    pub fn new(
        name: &str,
        alpha: f64,
        beta: f64,
        radius: f64,
        target: Vector3,
        scene: Option<&Scene>,
        setActiveOnSceneIfNoneActive: Option<bool>,
    ) -> ArcRotateCamera;

    #[wasm_bindgen(method)]
    pub fn attachControl(this: &ArcRotateCamera, canvas: Element, noPreventDefault: bool);
}

impl Default for ArcRotateCamera {
    fn default() -> Self {
        ArcRotateCamera::new(
            "Camera",
            PI / 2.0,
            PI / 2.0,
            2.0,
            Vector3::Zero(),
            None,
            Some(true),
        )
    }
}