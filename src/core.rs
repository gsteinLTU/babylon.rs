use js_sys::{Function, Reflect};
use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsValue, JsCast};
use web_sys::{Element};

use crate::prelude::Color3;

#[wasm_bindgen]
extern "C" {
    pub type Engine;
    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(canvas: &Element, b: bool) -> Engine;

    #[wasm_bindgen(method)]
    pub fn resize(this: &Engine, forceSetSize: Option<bool>);

    #[wasm_bindgen(method)]
    pub fn runRenderLoop(this: &Engine, renderFunction: &Function);


    #[wasm_bindgen(method)]
    pub fn getDeltaTime(this: &Engine) -> f64;
}

#[wasm_bindgen]
extern "C" {
    pub type Scene;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(engine: &Engine) -> Scene;

    #[wasm_bindgen(method)]
    pub fn createDefaultEnvironment(this: &Scene);

    #[wasm_bindgen(method)]
    pub fn render(this: &Scene, updateCameras: Option<bool>, ignoreAnimations: Option<bool>);

    #[wasm_bindgen(method)]
    pub fn getEngine(this: &Scene) -> Engine;
}

impl Scene {
    pub fn set_clear_color(&self, color: Color3) {
        Reflect::set(&self, &JsValue::from_str("clearColor"), &color).unwrap();
    }

    pub fn get_delta_time(&self) -> f64 {
        self.getEngine().getDeltaTime()
    }

    pub fn add_observable(&self, name: &str, cb: Closure<dyn FnMut()>) {
        let target = Reflect::get(self, &name.into()).expect("Observable not found");
        let target_add = Reflect::get(&target, &"add".into()).expect("Target not observable").unchecked_into::<Function>();
        target_add.call1(&target, &cb.into_js_value()).expect("Could not add observer callback");
    }

    pub fn add_keyboard_observable(&self, cb: Closure<dyn FnMut(JsValue, JsValue)>) {
        let target = Reflect::get(self, &"onKeyboardObservable".into()).expect("Observable not found");
        let target_add = Reflect::get(&target, &"add".into()).expect("Target not observable");
        let target_add = target_add.dyn_into::<Function>().unwrap();
        target_add.call1(&target, &cb.into_js_value()).expect("Could not add observer callback");
    }
}
