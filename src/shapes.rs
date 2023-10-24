use js_sys::{Promise, Array};
use crate::prelude::*;

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

#[wasm_bindgen]
extern "C" {
    pub(crate) type SceneLoader;

    #[wasm_bindgen(static_method_of = SceneLoader, js_namespace = BABYLON)]
    pub(crate) fn ImportMeshAsync(meshNames: Option<&str>, rootUrl: &str, sceneFilename: &str, scene: Option<&Scene>) -> Promise;
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

    pub async fn create_gltf(scene: &Scene, name: &str, file: &str) -> Result<BabylonMesh, JsValue> {
        let dummy = Mesh::new(name, scene.into());
        let promise = SceneLoader::ImportMeshAsync(None, "", file, scene.into());
        let import = wasm_bindgen_futures::JsFuture::from(promise);

        let import_result = import.await?;

        let imported_meshes = &Reflect::get(&import_result, &JsValue::from_str("meshes"))?;
        let imported_meshes_array = imported_meshes.unchecked_ref::<Array>();

        for val in imported_meshes_array.iter() { 
            let mesh: &Mesh = val.unchecked_ref::<Mesh>();
            mesh.set_parent(&dummy);
        }

        Ok(BabylonMesh {
            mesh: dummy,
        })
    }
}
