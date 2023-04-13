use js_sys::Function;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    pub type Engine;
    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(canvas: &Element, b: bool) -> Engine;

    #[wasm_bindgen(method)]
    pub fn resize(this: &Engine, forceSetSize: Option<bool>);

    #[wasm_bindgen(method)]
    pub fn runRenderLoop(this: &Engine, renderFunction: &Function);
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
}

/*
use crate::api::BabylonApi;
use crate::math::*;
use web::*;
pub struct Scene {
    ambient_color: Color,
    clear_color: Color,
    scene_ref: ExternRef,
}

impl Scene {
    pub fn new(selector: &str) -> Scene {
        let scene_ref = BabylonApi::create_scene(selector);
        Scene {
            clear_color: Color::new(0.0, 0.0, 0.0),
            ambient_color: Color::new(0.0, 0.0, 0.0),
            scene_ref,
        }
    }

    pub fn create_from_basic_engine(selector: &str) -> Scene {
        let scene_ref = BabylonApi::create_basic_scene(selector);
        Scene {
            clear_color: Color::new(0.0, 0.0, 0.0),
            ambient_color: Color::new(0.0, 0.0, 0.0),
            scene_ref,
        }
    }

    pub fn get_js_ref(&self) -> &ExternRef {
        &self.scene_ref
    }

    pub fn add_keyboard_observable(&self, callback: &str)
    {
        BabylonApi::add_keyboard_observable(&self.scene_ref, callback);
    }

    pub fn add_before_render_observable(&self, callback: &str)
    {
        BabylonApi::add_observable(
            &self.scene_ref,
            "onBeforeRenderObservable",
            callback,
        );
    }

    pub fn get_delta_time(&self) -> f64 {
        BabylonApi::get_delta_time(&self.scene_ref)
    }

    pub fn set_ambient_color(&mut self, c: Color) {
        self.ambient_color = c;
        BabylonApi::set_ambient_color(self.get_js_ref(), c.x, c.y, c.z);
    }

    pub fn set_clear_color(&mut self, c: Color) {
        self.clear_color = c;
        BabylonApi::set_clear_color(self.get_js_ref(), c.x, c.y, c.z);
    }
}*/
