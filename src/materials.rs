// use crate::api::BabylonApi;
// use crate::core::*;
// use crate::math::*;
// use web::*;

// pub trait Material {
//     fn get_js_ref(&self) -> &ExternRef;
// }

use js_sys::Reflect;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = BABYLON)]
    pub type Material;
    
    #[wasm_bindgen(extends = Material, js_namespace = BABYLON)]
    pub type StandardMaterial;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(name: &str, scene: &Scene) -> StandardMaterial;
}

macro_rules! get_set_jsvalue {
    ($getter_name:ident, $setter_name:ident, $jsprop:expr, $type:ty) => {
        pub fn $getter_name(&self) -> $type {
            Reflect::get(&self, &JsValue::from_str($jsprop)).expect(&format!("{} not found", $jsprop)).into()
        }
    
        pub fn $setter_name(&self, val: $type) {
            Reflect::set(&self, &JsValue::from_str($jsprop), &JsValue::from(val)).expect(&format!("{} not found", $jsprop));
        }
    };
}

impl StandardMaterial {
    get_set_jsvalue!(get_diffuse_color, set_diffuse_color, "diffuseColor", Color3);
    get_set_jsvalue!(get_specular_color, set_specular_color, "specularColor", Color3);
    get_set_jsvalue!(get_emmisive_color, set_emmisive_color, "emmisiveColor", Color3);
    get_set_jsvalue!(get_ambient_color, set_ambient_color, "ambientColor", Color3);
    get_set_jsvalue!(get_alpha, set_alpha, "alpha", js_sys::Number);
}