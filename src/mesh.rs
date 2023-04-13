use crate::prelude::*;
use js_sys::Reflect;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    pub(crate) type Mesh;

    #[wasm_bindgen(method)]
    pub(crate) fn dispose(
        this: &Mesh,
        doNotRecurse: Option<bool>,
        disposeMaterialAndTextures: Option<bool>,
    );

    #[wasm_bindgen(method, getter)]
    pub(crate) fn position(this: &Mesh) -> Vector3;

    #[wasm_bindgen(method, setter)]
    pub(crate) fn set_position(this: &Mesh, newPosition: Vector3);

    #[wasm_bindgen(method, getter)]
    pub(crate) fn rotation(this: &Mesh) -> Vector3;

    #[wasm_bindgen(method, setter)]
    pub(crate) fn set_rotation(this: &Mesh, newRotation: Vector3);

    #[wasm_bindgen(method, getter)]
    pub(crate) fn rotationQuaternion(this: &Mesh) -> Quaternion;

    #[wasm_bindgen(method, setter)]
    pub(crate) fn set_rotationQuaternion(this: &Mesh, quaternion: Option<Quaternion>);

    #[wasm_bindgen(method, getter)]
    pub(crate) fn scaling(this: &Mesh) -> Vector3;

    #[wasm_bindgen(method, setter)]
    pub(crate) fn set_scaling(this: &Mesh, newScaling: Vector3);

    #[wasm_bindgen(method)]
    pub(crate) fn addRotation(this: &Mesh, x: f64, y: f64, z: f64);
}

impl Mesh {
    pub(crate) fn set_material(&self, material: &Material){
        Reflect::set(&self, &JsValue::from_str("material"), &material).expect("material not found in Mesh");
    }
}

pub struct BabylonMesh {
    pub(crate) mesh: Mesh,
}

impl BabylonMesh {
    pub fn position(&self) -> Vector3 {
        self.mesh.position()
    }

    pub fn set_position(&self, pos: Vector3) {
        self.mesh.set_position(pos);
    }

    pub fn set_position_x(&self, x: f64) {
        let pos = self.mesh.position();
        self.mesh.set_position((x, pos.y(), pos.z()).into());
    }

    pub fn set_position_y(&self, y: f64) {
        let pos = self.mesh.position();
        self.mesh.set_position((pos.x(), y, pos.z()).into());
    }

    pub fn set_position_z(&self, z: f64) {
        let pos = self.mesh.position();
        self.mesh.set_position((pos.x(), pos.y(), z).into());
    }

    pub fn scaling(&self) -> Vector3 {
        self.mesh.scaling()
    }

    pub fn set_scaling_uniform(&self, scale: f64) {
        self.mesh.set_scaling((scale, scale, scale).into());
    }

    pub fn set_scaling(&self, scale: Vector3) {
        self.mesh.set_scaling(scale);
    }

    pub fn rotation(&self) -> Vector3 {
        self.mesh.rotation()
    }

    pub fn set_rotation(&self, rotation: Vector3) {
        self.mesh.set_rotation(rotation);
    }

    
    pub fn set_material(&self, material: &Material) {
        self.mesh.set_material(material);
    }

    //     pub fn set_material<T>(&mut self, mat: &T)
    //     where
    //         T: Material,
    //     {
    //         BabylonApi::set_material(&mut self.js_ref, mat.get_js_ref());
    //     }
}

impl Drop for BabylonMesh {
    fn drop(&mut self) {
        self.mesh.dispose(None, None);
    }
}
