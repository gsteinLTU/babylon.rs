use babylon::*;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

struct Game {
    scene: Scene,
    shape: Option<Sphere>,
}

impl Game {
    fn new() -> Game {
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
            shape: None,
        }
    }

    fn init(&mut self) {
        self.shape = Some(Sphere::create_sphere(&self.scene, 1.0));
        Sphere::create_sphere(&self.scene, 2.0);
    }
}

#[no_mangle]
pub fn main() {
    let mut game = GAME.lock().unwrap();
    game.init();
}
