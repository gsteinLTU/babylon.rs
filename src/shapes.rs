// use crate::api::BabylonApi;
// use crate::core::*;
// use crate::materials::*;
// use crate::math::*;
// use web::*;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::prelude::{BabylonMesh, Color4, Mesh, Scene, Vector3, Vector4};

#[wasm_bindgen]
extern "C" {
    type MeshBuilder;

    #[wasm_bindgen(static_method_of = MeshBuilder, js_namespace = BABYLON)]
    fn CreateSphere(name: &str, options: SphereOptions, scene: &Scene) -> Mesh;

    #[wasm_bindgen(static_method_of = MeshBuilder, js_namespace = BABYLON)]
    fn CreateBox(name: &str, options: BoxOptions, scene: &Scene) -> Mesh;
}

#[wasm_bindgen]
#[derive(Default)]
pub struct SphereOptions {
    pub arc: Option<f64>,
    pub diameter: Option<f64>,
    pub diameterX: Option<f64>,
    pub diameterY: Option<f64>,
    pub diameterZ: Option<f64>,
    pub segments: Option<f64>,
    pub sideOrientation: Option<f64>,
    pub slice: Option<f64>,
    pub updatable: Option<bool>,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct BoxOptions {
    pub bottomBaseAt: Option<f64>,
    pub depth: Option<f64>,
    pub height: Option<f64>,
    pub sideOrientation: Option<f64>,
    pub size: Option<f64>,
    pub topBaseAt: Option<f64>,
    pub updatable: Option<bool>,
    pub width: Option<f64>,
    pub wrap: Option<bool>,
}

impl BabylonMesh {
    pub fn create_sphere(scene: &Scene, name: &str, options: SphereOptions) -> BabylonMesh {
        BabylonMesh {
            mesh: MeshBuilder::CreateSphere(name, options, scene),
        }
    }

    pub fn create_box(scene: &Scene, name: &str, options: BoxOptions) -> BabylonMesh {
        BabylonMesh {
            mesh: MeshBuilder::CreateBox(name, options, scene),
        }
    }
}

// pub struct GLTF {
//     position: Vector3<f64>,
//     js_ref: ExternRef,
// }

// impl GLTF {
//     pub fn new(scene: &Scene, file: &str) -> GLTF {
//         GLTF {
//             position: Vector3::new(0.0, 0.0, 0.0),
//             js_ref: BabylonApi::create_gltf(scene.get_js_ref(), file),
//         }
//     }

//     pub fn get_position(&self) -> &Vector {
//         &self.position
//     }

//     pub fn set_position(&mut self, p: Vector) {
//         self.position = p;
//         BabylonApi::set_position(&mut self.js_ref, p.x, p.y, p.z);
//     }

//     pub fn set_position_x(&mut self, v: f64) {
//         self.position.x = v;
//         BabylonApi::set_position(
//             &mut self.js_ref,
//             self.position.x,
//             self.position.y,
//             self.position.z,
//         );
//     }

//     pub fn set_position_y(&mut self, v: f64) {
//         self.position.y = v;
//         BabylonApi::set_position(
//             &mut self.js_ref,
//             self.position.x,
//             self.position.y,
//             self.position.z,
//         );
//     }

//     pub fn set_position_z(&mut self, v: f64) {
//         self.position.z = v;
//         BabylonApi::set_position(
//             &mut self.js_ref,
//             self.position.x,
//             self.position.y,
//             self.position.z,
//         );
//     }

//     pub fn set_scaling(&mut self, p: Vector) {
//         self.position = p;
//         BabylonApi::set_scaling(&mut self.js_ref, p.x, p.y, p.z);
//     }
// }

// impl Drop for GLTF {
//     fn drop(&mut self) {
//         BabylonApi::dispose_mesh(&mut self.js_ref);
//     }
// }
