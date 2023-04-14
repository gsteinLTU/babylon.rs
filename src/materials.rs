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

impl StandardMaterial {
    pub fn get_diffuse_color(&self) -> Color3 {
        Reflect::get(&self, &JsValue::from_str("diffuseColor")).expect("diffuseColor not found in StandardMaterial").into()
    }

    pub fn set_diffuse_color(&self, c: Color3) {
        Reflect::set(&self, &JsValue::from_str("diffuseColor"), &c).expect("diffuseColor not found in StandardMaterial");
    }

    pub fn get_alpha(&self) -> f64 {
        Reflect::get(&self, &JsValue::from_str("alpha")).expect("alpha not found in StandardMaterial").as_f64().unwrap()
    }

    pub fn set_alpha(&self, alpha: f64) {
        Reflect::set(&self, &JsValue::from_str("alpha"), &JsValue::from_f64(alpha)).expect("alpha not found in StandardMaterial");
    }
}
// pub struct StandardMaterial {
//     js_ref: ExternRef,
//     diffuse_color: Color,
//     specular_color: Color,
//     emmisive_color: Color,
//     ambient_color: Color,
//     alpha: f64,
// }

// impl StandardMaterial {
//     pub fn new(scene: &Scene) -> StandardMaterial {
//         StandardMaterial {
//             js_ref: BabylonApi::create_standard_material(scene.get_js_ref()),
//             diffuse_color: Color::new(0.0, 0.0, 0.0),
//             specular_color: Color::new(0.0, 0.0, 0.0),
//             emmisive_color: Color::new(0.0, 0.0, 0.0),
//             ambient_color: Color::new(0.0, 0.0, 0.0),
//             alpha: 1.0,
//         }
//     }

//     pub fn set_emmisive_color(&mut self, c: Color) {
//         self.emmisive_color = c;
//         BabylonApi::set_emmisive_color(self.get_js_ref(), c.x, c.y, c.z);
//     }

//     pub fn set_specular_color(&mut self, c: Color) {
//         self.specular_color = c;
//         BabylonApi::set_specular_color(self.get_js_ref(), c.x, c.y, c.z);
//     }

//     pub fn set_ambient_color(&mut self, c: Color) {
//         self.ambient_color = c;
//         BabylonApi::set_ambient_color(self.get_js_ref(), c.x, c.y, c.z);
//     }

//     pub fn set_alpha(&mut self, a: f64) {
//         self.alpha = a;
//         BabylonApi::set_alpha(self.get_js_ref(), a);
//     }
// }

// impl Material for StandardMaterial {
//     fn get_js_ref(&self) -> &ExternRef {
//         return &self.js_ref;
//     }
// }



/*
            fn_set_emmisive_color: jsfn!(
                r#"
                Function(mat,r,g,b){
                    mat.emissiveColor = new BABYLON.Color3(r, g, b);
                }
            "#
            ),
            fn_set_specular_color: jsfn!(
                r#"
                Function(mat,r,g,b){
                    mat.specularColor = new BABYLON.Color3(r, g, b);
                }
            "#
            ),
            fn_set_ambient_color: jsfn!(
                r#"
                Function(mat,r,g,b){
                    mat.ambientColor = new BABYLON.Color3(r, g, b);
                }
            "#
            ),
*/