use wasm_bindgen::prelude::wasm_bindgen;
use js_sys::Reflect;
use wasm_bindgen::JsValue;
use std::ops;

use impl_ops::*;

use crate::get_set_jsvalue;

#[wasm_bindgen]
extern "C" {
    /// A three-dimensional vector
    pub type Vector3;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3;

    #[wasm_bindgen(js_namespace = BABYLON, static_method_of = Vector3)]
    pub fn One() -> Vector3;

    #[wasm_bindgen(js_namespace = BABYLON, static_method_of = Vector3)]
    pub fn Zero() -> Vector3;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Vector3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &Vector3, value: f64);

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Vector3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &Vector3, value: f64);

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Vector3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &Vector3, value: f64);
}

impl<T> From<(T, T, T)> for Vector3 
where T: Into<f64> + Copy {
    fn from(value: (T, T, T)) -> Self {
        Vector3::new(value.0.into(), value.1.into(), value.2.into())
    }
}

impl Vector3 {
    pub fn set(&self, x: f64, y: f64, z: f64) {
        self.set_x(x);
        self.set_y(y);
        self.set_z(z);
    }
}

impl_op_ex!(+ |a: &Vector3, b: &Vector3| -> Vector3 { Vector3::new(a.x() + b.x(), a.y() + b.y(), a.z() + b.z()) });
impl_op_ex!(- |a: &Vector3, b: &Vector3| -> Vector3 { Vector3::new(a.x() - b.x(), a.y() - b.y(), a.z() - b.z()) });
impl_op_ex!(* |a: &Vector3, b: f64| -> Vector3 { Vector3::new(a.x() * b, a.y() * b, a.z() * b) });
impl_op_ex!(/ |a: &Vector3, b: f64| -> Vector3 { Vector3::new(a.x() / b, a.y() / b, a.z() / b) });

#[wasm_bindgen]
extern "C" {
    /// A four-dimensional vector
    pub type Vector4;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector4;

    #[wasm_bindgen(js_namespace = BABYLON, static_method_of = Vector4)]
    pub fn One() -> Vector4;

    #[wasm_bindgen(js_namespace = BABYLON, static_method_of = Vector4)]
    pub fn Zero() -> Vector4;

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Vector4) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &Vector4, value: f64);

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Vector4) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &Vector4, value: f64);

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Vector4) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &Vector4, value: f64);

    #[wasm_bindgen(method, getter)]
    pub fn w(this: &Vector4) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_w(this: &Vector4, value: f64);
}

impl_op_ex!(+ |a: &Vector4, b: &Vector4| -> Vector4 { Vector4::new(a.x() + b.x(), a.y() + b.y(), a.z() + b.z(), a.w() + b.w()) });
impl_op_ex!(- |a: &Vector4, b: &Vector4| -> Vector4 { Vector4::new(a.x() - b.x(), a.y() - b.y(), a.z() - b.z(), a.w() - b.w()) });
impl_op_ex!(* |a: &Vector4, b: f64| -> Vector4 { Vector4::new(a.x() * b, a.y() * b, a.z() * b, a.w() * b) });
impl_op_ex!(/ |a: &Vector4, b: f64| -> Vector4 { Vector4::new(a.x() / b, a.y() / b, a.z() / b, a.w() / b) });

#[wasm_bindgen]
extern "C" {
    /// An RGB color
    pub type Color3;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(r: f64, g: f64, b: f64) -> Color3;
}

impl Color3 {
    get_set_jsvalue!(r, set_r, "r", f64);
    get_set_jsvalue!(g, set_g, "g", f64);
    get_set_jsvalue!(b, set_b, "b", f64);
}

impl<T> From<(T, T, T)> for Color3 
where T: Into<f64> + Copy {
    fn from(value: (T, T, T)) -> Self {
        Color3::new(value.0.into(), value.1.into(), value.2.into())
    }
}

#[wasm_bindgen]
extern "C" {
    /// An RGBA color
    pub type Color4;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Color4;
}

impl Color4 {
    get_set_jsvalue!(r, set_r, "r", f64);
    get_set_jsvalue!(g, set_g, "g", f64);
    get_set_jsvalue!(b, set_b, "b", f64);
    get_set_jsvalue!(a, set_a, "a", f64);
}

impl From<Color3> for Color4 {
    fn from(value: Color3) -> Self {
        Color4::new(value.r().into(), value.g().into(), value.b().into(), 1.0)
    }
}

impl<T> From<(T, T, T, T)> for Color4
where T: Into<f64> + Copy {
    fn from(value: (T, T, T, T)) -> Self {
        Color4::new(value.0.into(), value.1.into(), value.2.into(), value.3.into())
    }
}

impl<T> From<(T, T, T, T)> for Vector4
where T: Into<f64> + Copy {
    fn from(value: (T, T, T, T)) -> Self {
        Vector4::new(value.0.into(), value.1.into(), value.2.into(), value.3.into())
    }
}

#[wasm_bindgen]
extern "C" {
    pub type Quaternion;

    #[wasm_bindgen(constructor, js_namespace = BABYLON)]
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Quaternion;
}