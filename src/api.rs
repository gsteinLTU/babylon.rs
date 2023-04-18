use std::{cell::RefCell, f64::consts::PI, rc::Rc};

use wasm_bindgen::{
    prelude::Closure,
    JsCast,
};

use crate::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type BABYLON;
}

/// Create a simple Scene in the canvas referred to by the selector.
/// 
/// The scene will have an ArcRotateCamera, a HemisphericLight, and a PointLight.
pub fn create_basic_scene(selector: &str) -> Rc<RefCell<Scene>> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let canvas = document
        .query_selector(selector)
        .expect("Selector not found")
        .expect("Selector not found");

    let scene = create_scene(selector);

    // Add a camera to the scene and attach it to the canvas
    let camera = ArcRotateCamera::new(
        "Camera",
        PI / 2.0,
        PI / 2.0,
        2.0,
        Vector3::Zero(),
        Some(&scene.borrow()),
        Some(true),
    );
    camera.attachControl(canvas, true);

    // Add lights to the scene
    HemisphericLight::new("light1", Vector3::new(1.0, 1.0, 0.0), &scene.borrow());
    PointLight::new("light2", Vector3::new(0.0, 1.0, -1.0), &scene.borrow());

    scene
}

/// Create a barebones Scene in the canvas referred to by the selector.
pub fn create_scene(selector: &str) -> Rc<RefCell<Scene>> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let canvas = document
        .query_selector(selector)
        .expect("Selector not found")
        .expect("Selector not found");

    let engine = Rc::new(RefCell::new(Engine::new(&canvas, true)));

    let scene = Rc::new(RefCell::new(Scene::new(&engine.borrow())));

    let closure_scene = scene.clone();
    let renderloop_closure = Closure::<dyn FnMut()>::new(move || {
        closure_scene.borrow().render(None, None);
    });
    engine
        .borrow()
        .runRenderLoop(renderloop_closure.into_js_value().unchecked_ref());

    let resize_closure = Closure::<dyn FnMut()>::new(move || {
        engine.borrow().resize(None);
    });
    window.set_onresize(Some(resize_closure.into_js_value().unchecked_ref()));

    scene
}

