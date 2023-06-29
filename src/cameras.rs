use std::f64::consts::PI;

use crate::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    pub type Camera;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(
        name: &str,
        position: Vector3,
        scene: Option<&Scene>,
        setActiveOnSceneIfNoneActive: Option<bool>,
    ) -> Camera;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Camera)]
    pub type ArcRotateCamera;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(
        name: &str,
        alpha: f64,
        beta: f64,
        radius: f64,
        target: Vector3,
        scene: Option<&Scene>,
        setActiveOnSceneIfNoneActive: Option<bool>,
    ) -> ArcRotateCamera;

    /// Attach the camera to the given canvas, necessary for user to manipulate the camera
    #[wasm_bindgen(method)]
    pub fn attachControl(this: &ArcRotateCamera, canvas: Element, noPreventDefault: bool);
}

impl Default for ArcRotateCamera {
    fn default() -> Self {
        ArcRotateCamera::new(
            "Camera",
            PI / 2.0,
            PI / 2.0,
            2.0,
            Vector3::Zero(),
            None,
            Some(true),
        )
    }
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Camera)]
    pub type UniversalCamera;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(
        name: &str,
        position: Vector3,
        scene: Option<&Scene>
    ) -> UniversalCamera;

    /// Attach the camera to the given canvas, necessary for user to manipulate the camera
    #[wasm_bindgen(method)]
    pub fn attachControl(this: &UniversalCamera, canvas: Element, noPreventDefault: bool);

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &UniversalCamera) -> Vector3;

    #[wasm_bindgen(method, setter)]
    pub fn set_position(this: &UniversalCamera, newPosition: &Vector3);

    #[wasm_bindgen(method, getter)]
    pub fn rotation(this: &UniversalCamera) -> Vector3;

    #[wasm_bindgen(method, setter)]
    pub fn set_rotation(this: &UniversalCamera, newRotation: &Vector3);

    #[wasm_bindgen(method, getter)]
    pub fn rotationQuaternion(this: &UniversalCamera) -> Quaternion;

    #[wasm_bindgen(method, setter)]
    pub fn set_rotationQuaternion(this: &UniversalCamera, quaternion: Option<&Quaternion>);
}

impl UniversalCamera {
    get_set_jsvalue!(get_min_z, set_min_z, "minZ", f64);
    get_set_jsvalue!(get_max_z, set_max_z, "maxZ", f64);
    get_set_jsvalue!(get_fov, set_fov, "fov", f64);
    get_set_jsvalue!(get_speed, set_speed, "speed", f64);
    get_set_jsvalue!(get_camera_rotation, set_camera_rotation, "cameraRotation", Vector3);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Camera)]
    pub type FollowCamera;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(
        name: &str,
        position: Vector3,
        scene: Option<&Scene>
    ) -> FollowCamera;

    /// Attach the camera to the given canvas, necessary for user to manipulate the camera
    #[wasm_bindgen(method)]
    pub fn attachControl(this: &FollowCamera, canvas: Element, noPreventDefault: bool);
}

impl FollowCamera {
    get_set_jsvalue!(get_min_z, set_min_z, "minZ", f64);
    get_set_jsvalue!(get_max_z, set_max_z, "maxZ", f64);
    get_set_jsvalue!(get_fov, set_fov, "fov", f64);
    get_set_jsvalue!(get_radius, set_radius, "radius", f64);
    get_set_jsvalue!(get_height_offset, set_height_offset, "heightOffset", f64);
    get_set_jsvalue!(get_rotation_offset, set_rotation_offset, "rotationOffset", f64);
    get_set_jsvalue!(get_camera_acceleration, set_camera_acceleration, "cameraAcceleration", f64);
    get_set_jsvalue!(get_max_camera_speed, set_max_camera_speed, "maxCameraSpeed", f64);
}
