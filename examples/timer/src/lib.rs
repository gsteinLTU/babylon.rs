use babylon::{prelude::*, api};
use js_sys::Math;
use std::borrow::BorrowMut;
use std::rc::Rc;
use std::sync::atomic::AtomicU16;
use std::cell::RefCell;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};

struct Game {
    time: RefCell<f64>,
    scene: Rc<RefCell<Scene>>,
    shapes: RefCell<Vec<Rc<BabylonMesh>>>,
}

impl Game {
    fn new() -> Self {
        Game {
            time: RefCell::new(0.0),
            scene: api::create_basic_scene("#renderCanvas"),
            shapes: RefCell::new(vec![]),
        }
    }
}


thread_local! {
    static GAME: Rc<RefCell<Game>> = Rc::new(RefCell::new(Game::new()));
}

thread_local! {
    static COUNT: AtomicU16 = AtomicU16::new(0);
}

#[wasm_bindgen(start)]
pub fn main() {
    GAME.with(|game| {
        game.borrow().scene.borrow().add_observable("onBeforeRenderObservable", Closure::new(|| {
            GAME.with(|game| {
                let delta_time = game.borrow().scene.borrow().get_delta_time();
                *game.borrow().time.borrow_mut() += delta_time;

                if *game.borrow().time.borrow() > 1000.0 {
                    *game.borrow().time.borrow_mut() -= 1000.0;
                    // add sphere every second
                    let i = COUNT.with(|count| { count.fetch_add(1, std::sync::atomic::Ordering::Relaxed) });

                    let sphere = BabylonMesh::create_sphere(
                        &game.borrow().scene.borrow(), 
                        &format!("sphere_{}", i), 
                        SphereOptions { diameter: Math::random().into(), ..Default::default()});
                    sphere.set_position(Vector3::new(
                        Math::random() - 0.5,
                        Math::random() - 0.5,
                        Math::random() - 0.5,
                    ));

                    game.borrow().shapes.borrow_mut().push(Rc::new(sphere));
                }
            });
        }));
    });
}
