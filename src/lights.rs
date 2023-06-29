use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use js_sys::Reflect;

use crate::{prelude::*, get_set_jsvalue};

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

impl HemisphericLight {
    get_set_jsvalue!(get_direction, set_direction, "direction", Vector3);
    get_set_jsvalue!(get_intensity, set_intensity, "intensity", f64);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Light, js_namespace = BABYLON)]
    pub type DirectionalLight;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(name: &str, direction: Vector3, scene: &Scene) -> DirectionalLight;
}

impl DirectionalLight {
    get_set_jsvalue!(get_direction, set_direction, "direction", Vector3);
    get_set_jsvalue!(get_intensity, set_intensity, "intensity", f64);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Light, js_namespace = BABYLON)]
    pub type PointLight;
    
    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(name: &str, position: Vector3, scene: &Scene) -> PointLight;
}

impl PointLight {
    get_set_jsvalue!(get_intensity, set_intensity, "intensity", f64);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = BABYLON)]
    pub type ShadowGenerator;

    #[wasm_bindgen(method)]
    pub(crate) fn addShadowCaster(this: &ShadowGenerator, mesh: &Mesh, includeDescendants: bool);
}

impl ShadowGenerator {
    pub fn add_shadow_caster(&self, mesh: &BabylonMesh, include_descendants: bool) {
        self.addShadowCaster(&mesh.mesh, include_descendants);
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = BABYLON, extends = ShadowGenerator)]
    pub type CascadedShadowGenerator;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(mapSize: f64, light: &Light) -> CascadedShadowGenerator;
}

impl CascadedShadowGenerator {
    get_set_jsvalue!(get_bias, set_bias, "bias", f64);
    get_set_jsvalue!(get_cascade_blend_percentage, set_cascade_blend_percentage, "cascadeBlendPercentage", f64);
    get_set_jsvalue!(get_lambda, set_lambda, "lambda", f64);
    get_set_jsvalue!(get_filtering_quality, set_filtering_quality, "filteringQuality", f64);
    get_set_jsvalue!(get_filter, set_filter, "filter", f64);
    get_set_jsvalue!(get_frustum_edge_falloff, set_frustum_edge_falloff, "frustumEdgeFalloff", f64);
    get_set_jsvalue!(get_shadow_max_z, set_shadow_max_z, "shadowMaxZ", f64);
    get_set_jsvalue!(get_stabilize_cascades, set_stabilize_cascades, "stabilizeCascades", bool);
}