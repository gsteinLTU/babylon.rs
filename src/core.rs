use js_sys::{Function, Reflect};
use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsValue, JsCast, closure::WasmClosure};
use web_sys::Element;

use crate::prelude::{Color4, Camera};

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

    #[wasm_bindgen(method)]
    pub fn createDefaultVRExperience(this: &Scene);
}


impl Scene {
    /// Set the background color for rendering the Scene
    pub fn set_clear_color(&self, color: Color4) {
        Reflect::set(&self, &JsValue::from_str("clearColor"), &color).unwrap();
    }

    /// Gets the time spent between current and previous frame, in ms
    pub fn get_delta_time(&self) -> f64 {
        self.getEngine().getDeltaTime()
    }

    /// Add a Closure to run when an observable on the Scene changes
    /// 
    /// See "Properties" list on https://doc.babylonjs.com/typedoc/classes/BABYLON.Scene for names of observables
    pub fn add_observable<F>(&self, name: &str, cb: Closure<F>)
            where F: WasmClosure + ?Sized
    {
        let target = Reflect::get(self, &name.into()).expect("Observable not found");
        let target_add = Reflect::get(&target, &"add".into()).expect("Target not observable").unchecked_into::<Function>();
        target_add.call1(&target, &cb.into_js_value()).expect("Could not add observer callback");
    }

    /// add_observable special case for onKeyboardObservable
    pub fn add_keyboard_observable(&self, cb: Closure<dyn FnMut(JsValue, JsValue)>) {
        self.add_observable("onKeyboardObservable", cb);
    }

    /// add_observable special case for onBeforeRenderObservable
    pub fn add_before_render_observable(&self,  cb: Closure<dyn FnMut()>) {
        self.add_observable("onBeforeRenderObservable", cb);
    }

    pub fn set_active_camera(&self,val: &Camera){
        Reflect::set(&self, &JsValue::from_str("activeCamera"), &val).expect(&format!("{} not found","activeCamera"));
    }
}
