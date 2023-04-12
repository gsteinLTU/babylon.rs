use std::{f64::consts::PI, cell::RefCell, rc::Rc, mem};

use js_sys::{Function};
use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsCast};
use web_sys::Element;

use crate::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type BABYLON;
}

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

#[wasm_bindgen]
extern "C" {
    pub type Mesh;
    
    #[wasm_bindgen(method)]
    pub fn dispose(this: &Mesh, doNotRecurse: Option<bool>, disposeMaterialAndTextures: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &Mesh) -> Vector3;

    #[wasm_bindgen(method, setter)]
    pub fn set_position(this: &Mesh, newPosition: Vector3);
}


pub fn create_basic_scene(selector: &str) -> Rc<RefCell<Scene>> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let canvas = document.query_selector(selector).expect("Selector not found").expect("Selector not found");
    
    let engine = Rc::new(RefCell::new(Engine::new(&canvas, true))); 
    
    let scene = Rc::new(RefCell::new(Scene::new(&engine.borrow())));
    
    // Add a camera to the scene and attach it to the canvas
    let camera = ArcRotateCamera::new(
        "Camera",
        PI / 2.0,
        PI / 2.0,
        2.0,
        Vector3::Zero(),
        Some(&scene.borrow()),
        Some(true)
    );
    camera.attachControl(canvas, true);

    // Add lights to the scene
    HemisphericLight::new(
        "light1",
        Vector3::new(1.0, 1.0, 0.0),
        &scene.borrow()
    );
    PointLight::new(
        "light2",
        Vector3::new(0.0, 1.0, -1.0),
        &scene.borrow()
    );

    let closure_scene = scene.clone();
    let renderloop_closure = Closure::<dyn FnMut()>::new(move || {
        closure_scene.borrow().render(None, None);
    });
    engine.borrow().runRenderLoop(renderloop_closure.as_ref().unchecked_ref());
    mem::forget(renderloop_closure);

    let resize_closure = Closure::<dyn FnMut()>::new(move || {
        engine.borrow().resize(None);
    });
    window.set_onresize(Some(resize_closure.as_ref().unchecked_ref()));
    mem::forget(resize_closure);

    scene
}

pub fn create_scene(selector: &str) -> Rc<RefCell<Scene>> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let canvas = document.query_selector(selector).expect("Selector not found").expect("Selector not found");

    let engine = Rc::new(RefCell::new(Engine::new(&canvas, true))); 
    
    let scene = Rc::new(RefCell::new(Scene::new(&engine.borrow())));
    scene
}

/*
impl Default for BabylonApi {
    fn default() -> Self {
        BabylonApi {
 
            fn_create_sphere: jsfn!(
                r#"
                Function(scene,size){
                    return BABYLON.MeshBuilder.CreateSphere(
                        null,
                        { diameter: size },
                        scene);
                }
            "#
            ),
            fn_create_cube: jsfn!(
                r#"
                Function(scene,w,h,d){
                    return BABYLON.MeshBuilder.CreateBox(
                        null,
                        { height: h, width: w, depth: d },
                        scene);
                }
            "#
            ),
            fn_create_standard_material: jsfn!(
                r#"
                Function(scene){
                    return new BABYLON.StandardMaterial(null, scene);
                }
            "#
            ),
            fn_dispose_mesh: jsfn!(
                r#"
                Function(mesh){
                    mesh.dispose()
                }
            "#
            ),
            fn_log: jsfn!(
                r#"
                Function(msg){
                    console.log(msg);
                }
            "#
            ),
            fn_error: jsfn!(
                r#"
                Function(msg){
                    console.error(msg);
                }
            "#
            ),
            fn_debug: jsfn!(
                r#"
                Function(){
                    debugger;
                }
            "#
            ),
            fn_random: jsfn!(
                r#"
                Function(){
                    return Math.random();
                }
            "#
            ),
            fn_set_position: jsfn!(
                r#"
                Function(mesh,x,y,z){
                    mesh.position = new BABYLON.Vector3(x,y,z);
                }
            "#
            ),
            fn_set_scaling: jsfn!(
                r#"
                Function(mesh,x,y,z){
                    mesh.scaling = new BABYLON.Vector3(x,y,z);
                }
            "#
            ),
            fn_set_material: jsfn!(
                r#"
                Function(mesh,mat){
                    mesh.material = mat;
                }
            "#
            ),
            fn_set_emmisive_color: jsfn!(
                r#"
                Function(mat,r,g,b){
                    mat.emissiveColor = new BABYLON.Color3(r, g, b);
                }
            "#
            ),
            fn_set_diffuse_color: jsfn!(
                r#"
                Function(mat,r,g,b){
                    mat.diffuseColor = new BABYLON.Color3(r, g, b);
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
            fn_set_clear_color: jsfn!(
                r#"
                Function(scene,r,g,b){
                    scene.clearColor = new BABYLON.Color3(r, g, b);
                }
            "#
            ),
            fn_set_alpha: jsfn!(
                r#"
                Function(mat,a){
                    mat.alpha = a;
                }
            "#
            ),
            fn_add_keyboard_observable: jsfn!(
                r#"
                Function(scene,cb){
                    scene.onKeyboardObservable.add((kbInfo) => {
                        this.module.instance.exports[cb](kbInfo.type,kbInfo.event.keyCode)
                    });
                }
            "#
            ),
            fn_add_observable: jsfn!(
                r#"
                Function(scene,name,cb){
                    scene[name].add(() => {
                        this.module.instance.exports[cb]()
                    });
                }
            "#
            ),
            fn_get_delta_time: jsfn!(
                r#"
                Function(scene){
                    return scene.getEngine().getDeltaTime();
                }
            "#
            ),
            fn_create_arc_rotate_camera: jsfn!(
                r#"
                Function(scene){
                    var camera = new BABYLON.ArcRotateCamera(
                        "Camera",
                        Math.PI / 2,
                        Math.PI / 2,
                        2,
                        BABYLON.Vector3.Zero(),
                        scene
                    );
                    return camera;
                }
            "#
            ),
            fn_create_hemispheric_light: jsfn!(
                r#"
                Function(scene){
                    var light = new BABYLON.HemisphericLight(
                        null,
                        new BABYLON.Vector3(1, 1, 0),
                        scene
                    );
                    return light;
                }
            "#
            ),
            fn_create_point_light: jsfn!(
                r#"
                Function(scene){
                    var light = new BABYLON.PointLight(
                        null,
                        new BABYLON.Vector3(0, 1, -1),
                        scene
                    );
                    return light;
                }
            "#
            ),
            fn_create_gltf: jsfn!(
                r#"
                Function(scene,file){
                    var dummy = new BABYLON.Mesh(null, scene);
                    BABYLON.SceneLoader.ImportMesh(null, "", file, scene, Function (newMeshes, particleSystems, skeletons) {
                        for(let v in newMeshes){
                            var dude = newMeshes[v];
                            dude.parent = dummy;
                        }
                    });
                    return dummy;
                }
                "#
            ),
        }
    }
}

impl BabylonApi {
    pub fn create_basic_scene(selector: &str) -> Function {
        let api = globals::get::<BabylonApi>();
        api.fn_create_basic_scene.invoke_and_return_object(&[InvokeParam::String(selector)])
    }
    pub fn create_scene(selector: &str) -> Function {
        let api = globals::get::<BabylonApi>();
        api.fn_create_scene.invoke_and_return_object(&[InvokeParam::String(selector)])
    }
    pub fn create_sphere(scene_ref: &Function, size: f64) -> Function {
        let api = globals::get::<BabylonApi>();
        api.fn_create_sphere
            .invoke_and_return_object(&[InvokeParam::Function(scene_ref), InvokeParam::Float64(size)])
    }

    pub fn create_cube(scene_ref: &Function, width: f64, height: f64, depth: f64) -> Function {
        let api = globals::get::<BabylonApi>();
        api.fn_create_cube
            .invoke_and_return_object(&[InvokeParam::Function(scene_ref), InvokeParam::Float64(width), InvokeParam::Float64(height), InvokeParam::Float64(depth)])
    }

    pub fn create_standard_material(scene_ref: &Function) -> Function {
        let api = globals::get::<BabylonApi>();
        api.fn_create_standard_material
            .invoke_and_return_object(&[InvokeParam::Function(scene_ref)])
    }

    pub fn dispose_mesh(mesh: &Function) {
        let api = globals::get::<BabylonApi>();
        api.fn_dispose_mesh.invoke(&[InvokeParam::Function(mesh)]);
    }

    pub fn log(msg: &str) {
        let api = globals::get::<BabylonApi>();
        api.fn_log.invoke(&[InvokeParam::String(msg)]);
    }

    pub fn error(msg: &str) {
        let api = globals::get::<BabylonApi>();
        api.fn_error.invoke(&[InvokeParam::String(msg)]);
    }

    pub fn debugger() {
        let api = globals::get::<BabylonApi>();
        api.fn_debug.invoke(&[]);
    }

    pub fn random() -> f64 {
        let api = globals::get::<BabylonApi>();
        api.fn_random.invoke(&[])
    }

    pub fn set_position(mesh: &Function, x: f64, y: f64, z: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_position.invoke(&[InvokeParam::Function(mesh), InvokeParam::Float64(x), InvokeParam::Float64(y), InvokeParam::Float64(z)]);
    }

    pub fn set_scaling(mesh: &Function, x: f64, y: f64, z: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_scaling.invoke(&[InvokeParam::Function(mesh), InvokeParam::Float64(x), InvokeParam::Float64(y), InvokeParam::Float64(z)]);
    }

    pub fn set_material(mesh: &Function, mat: &Function) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_material.invoke(&[InvokeParam::Function(mesh), InvokeParam::Function(mat)]);
    }

    pub fn set_emmisive_color(mat: &Function, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_emmisive_color.invoke(&[InvokeParam::Function(mat), InvokeParam::Float64(r), InvokeParam::Float64(g), InvokeParam::Float64(b)]);
    }

    pub fn set_diffuse_color(mat: &Function, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_diffuse_color.invoke(&[InvokeParam::Function(mat), InvokeParam::Float64(r), InvokeParam::Float64(g), InvokeParam::Float64(b)]);
    }

    pub fn set_specular_color(mat: &Function, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_specular_color.invoke(&[InvokeParam::Function(mat), InvokeParam::Float64(r), InvokeParam::Float64(g), InvokeParam::Float64(b)]);
    }

    pub fn set_ambient_color(mat: &Function, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_ambient_color.invoke(&[InvokeParam::Function(mat), InvokeParam::Float64(r), InvokeParam::Float64(g), InvokeParam::Float64(b)]);
    }

    pub fn set_clear_color(mat: &Function, r: f64, g: f64, b: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_clear_color.invoke(&[InvokeParam::Function(mat), InvokeParam::Float64(r), InvokeParam::Float64(g), InvokeParam::Float64(b)]);
    }

    pub fn set_alpha(mat: &Function, a: f64) {
        let api = globals::get::<BabylonApi>();
        api.fn_set_alpha.invoke(&[InvokeParam::Function(mat), InvokeParam::Float64(a)]);
    }

    pub fn add_keyboard_observable(
        scene: &Function,
        callback: &str
    ) {
        let api = globals::get::<BabylonApi>();
        api.fn_add_keyboard_observable.invoke(&[InvokeParam::Function(scene), InvokeParam::String(callback)]);
    }

    pub fn add_observable(
        scene: &Function,
        observable_name: &str,
        callback: &str
    ) {
        let api = globals::get::<BabylonApi>();
        api.fn_add_observable.invoke(&[InvokeParam::Function(scene), InvokeParam::String(observable_name), InvokeParam::String(callback)]);
    }

    pub fn get_delta_time(scene: &Function) -> f64 {
        let api = globals::get::<BabylonApi>();
        api.fn_get_delta_time.invoke(&[InvokeParam::Function(scene)])
    }

    pub fn create_arc_rotate_camera(scene: &Function) -> Function {
        let api = globals::get::<BabylonApi>();
        api.fn_create_arc_rotate_camera
            .invoke_and_return_object(&[InvokeParam::Function(scene)])
    }

    pub fn create_hemispheric_light(scene: &Function) -> Function {
        let api = globals::get::<BabylonApi>();
        api.fn_create_hemispheric_light
            .invoke_and_return_object(&[InvokeParam::Function(scene)])
    }

    pub fn create_point_light(scene: &Function) -> Function {
        let api = globals::get::<BabylonApi>();
        api.fn_create_point_light.invoke_and_return_object(&[InvokeParam::Function(scene)])
    }

    pub fn create_gltf(scene: &Function, file: &str) -> Function {
        let api = globals::get::<BabylonApi>();
        api.fn_create_gltf
            .invoke_and_return_object(&[InvokeParam::Function(scene), InvokeParam::String(file)])
    }
}
*/