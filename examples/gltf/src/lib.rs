use std::{cell::RefCell, rc::Rc};

use neo_babylon::{api, prelude::*};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

struct Game {
    scene: Rc<RefCell<Scene>>,
    models: RefCell<Vec<Rc<BabylonMesh>>>,
}

impl Game {
    fn new() -> Self {
        Game {
            scene: api::create_basic_scene("#renderCanvas"),
            models: RefCell::new(vec![]),
        }
    }

    async fn load_model(&self, name: &str, url: &str) -> Result<Rc<BabylonMesh>, JsValue> {
        let model = BabylonMesh::create_gltf(&self.scene.borrow(), name, url).await;

        let model = Rc::new(model);
        self.models.borrow_mut().push(model.clone());

        Ok(model)
    }
}

thread_local! {
    static GAME: Rc<RefCell<Game>> = Rc::new(RefCell::new(Game::new()));
}

#[wasm_bindgen(start)]
pub fn main() {
    init_panic_hook();

    wasm_bindgen_futures::spawn_local(GAME.with(|game| {
        let game_rc = Rc::clone(&game);
        async move {
            let gltf = game_rc
            .borrow()
            .load_model("boombox", "BoomBox.gltf")
                .await
                .unwrap();
            
            // Example model has odd scaling
            gltf.set_scaling(&(-50.0, 50.0, 50.0).into());            
        }
    }));

    // Spawn another model
    wasm_bindgen_futures::spawn_local(GAME.with(|game| {
        let game_rc = Rc::clone(&game);
        async move {
            let gltf = game_rc
            .borrow()
            .load_model("boombox2", "BoomBox.gltf")
                .await
                .unwrap();

            gltf.set_scaling(&(-70.0, 70.0, 70.0).into());
            gltf.set_position_x(2.0);
        }
    }));
}
