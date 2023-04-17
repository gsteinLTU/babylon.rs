use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use js_sys::Reflect;
use wasm_bindgen::prelude::Closure;

use crate::constants::*;
use crate::core::*;

pub trait BasicGame {
    fn get_scene(&self) -> Rc<RefCell<Scene>>;
    fn run(&self, _delta_time: f64) {}
    fn key_up(&self, _key_code: u8) {}
    fn key_down(&self, _key_code: u8) {}
    fn get_keys(&self) -> Rc<RefCell<HashSet<u8>>>;
}

pub fn run_basic_game<T>() -> Rc<RefCell<T>>
where
    T: 'static + BasicGame + Default,
{
    let t = Rc::new(RefCell::new(T::default()));
    let t1 = Rc::clone(&t);
    t.borrow().get_scene().borrow().add_before_render_observable(Closure::new(move || {
        let g = t1.borrow();
        let scene = g.get_scene();
        let delta_time = scene.borrow().get_delta_time() / 1000.0;
        t1.borrow().run(delta_time);
    }));

    let t2 = Rc::clone(&t);
    t.borrow().get_scene().borrow().add_keyboard_observable(Closure::new(move |e1, _e2| {
        let event_type = Reflect::get(&e1, &"type".into()).unwrap();
        let key_code = Reflect::get(&e1, &"event".into()).unwrap();
        let key_code = Reflect::get(&key_code, &"inputIndex".into()).unwrap_or_default();
        if event_type == KEYDOWN {
            t2.borrow().key_down(key_code.as_f64().unwrap() as u8);
            t2.borrow().get_keys().borrow_mut().insert(key_code.as_f64().unwrap() as u8);
        }
        if event_type == KEYUP {
            t2.borrow().key_up(key_code.as_f64().unwrap() as u8);
            t2.borrow().get_keys().borrow_mut().remove(&(key_code.as_f64().unwrap() as u8));
        }
    }));

    t
}
