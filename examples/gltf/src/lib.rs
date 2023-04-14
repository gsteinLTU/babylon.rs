use std::{cell::RefCell, rc::Rc};

use babylon::{api, prelude::*};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

struct Game {
    scene: Rc<RefCell<Scene>>,
    models: Vec<Rc<BabylonMesh>>,
}

impl Game {
    fn new() -> Self {
        Game {
            scene: api::create_basic_scene("#renderCanvas"),
            models: vec![],
        }
    }

    async fn load_model(&self, name: &str, url: &str) -> Result<BabylonMesh, JsValue> {
        let model = BabylonMesh::create_gltf(&self.scene.borrow(), name, url).await;
        Ok(model)
    }
}

thread_local! {
    static GAME: Rc<RefCell<Game>> = Rc::new(RefCell::new(Game::new()));
}

#[wasm_bindgen(start)]
pub fn main() {
    init_panic_hook();

    let future = GAME.with(|game| {
        let game_rc = Rc::clone(&game);
        async move {
            let gltf = game_rc
                .borrow()
                .load_model("boombox", "BoomBox.gltf")
                .await
                .unwrap();

            // Example model has odd scaling
            gltf.set_scaling((-50.0, 50.0, 50.0).into());
            game_rc.borrow_mut().models.push(Rc::new(gltf));
        }
    });

    wasm_bindgen_futures::spawn_local(future);
}
