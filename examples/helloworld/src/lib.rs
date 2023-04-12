use wasm_bindgen::prelude::*;
use babylon::prelude::*;
use web_sys::console;

/* 
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;
lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

struct Game {
    scene: Scene,
    shape: Vec<Sphere>,
}

impl Game {
    fn new() -> Self {
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
            shape: vec![],
        }
    }
}*/

#[wasm_bindgen(start)]
pub fn main() {
    babylon::api::create_basic_scene("#renderCanvas");
    
    console::log_1(&"Started".into());

    /*babylon::js::log("Starting demo...");
    let mut game = GAME.lock().unwrap();
    for _ in 0..10 {
        let mut sphere = Sphere::new(&game.scene, babylon::js::random());
        sphere.set_position(Vector::new(
            babylon::js::random() - 0.5,
            babylon::js::random() - 0.5,
            babylon::js::random() - 0.5,
        ));
        game.shape.push(sphere);
    }*/
}
