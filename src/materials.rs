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
    get_set_jsvalue!(get_diffuse_color, set_diffuse_color, "diffuseColor", Color3);
    get_set_jsvalue!(get_specular_color, set_specular_color, "specularColor", Color3);
    get_set_jsvalue!(get_emmisive_color, set_emmisive_color, "emmisiveColor", Color3);
    get_set_jsvalue!(get_ambient_color, set_ambient_color, "ambientColor", Color3);
    get_set_jsvalue!(get_alpha, set_alpha, "alpha", f64);
}