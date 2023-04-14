use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicU16, mem};

use js_sys::{Math, Reflect};
use wasm_bindgen::prelude::*; 
use babylon::{prelude::*, api};
use web_sys::console;

struct Game {
    scene: Rc<RefCell<Scene>>,
    shapes: Vec<BabylonMesh>,
}

impl Game {
    fn new() -> Self {
        Game {
            scene: api::create_basic_scene("#renderCanvas"),
            shapes: vec![],
        }
    }
}

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::new());
}


thread_local! {
    static count: AtomicU16 = AtomicU16::new(0);
}

#[wasm_bindgen(start)]
pub fn main() {
    let closure = Closure::new(move |e1: JsValue, e2: JsValue| {
        let event_type = Reflect::get(&e1, &"type".into()).unwrap();
        let key_code = Reflect::get(&e1, &"event".into()).unwrap();
        let key_code = Reflect::get(&key_code, &"inputIndex".into()).unwrap_or_default();
        if event_type == KEYUP {
            console::log_1(&format!("{} {:?}", event_type.as_f64().unwrap(), key_code).into());
            GAME.with(|game| {
                count.with(|i| {
                    console::log_1(&format!("{}", i.load(std::sync::atomic::Ordering::Relaxed)).into());
                    let sphere = BabylonMesh::create_sphere(&game.borrow().scene.borrow(), format!("sphere_{}", i.fetch_add(1, std::sync::atomic::Ordering::SeqCst)).as_str(), SphereOptions{ diameter: Some(Math::random() + 0.5), ..Default::default() });
                    sphere.set_position(Vector3::new(
                        Math::random() - 0.5,
                        Math::random() - 0.5,
                        Math::random() - 0.5,
                    ));     
                    game.borrow_mut().shapes.push(sphere);
                });
            }); 
        }
    });
    let game = GAME.with(|game| {
        game.borrow().scene.borrow().add_keyboard_observable(closure);
    });
}
