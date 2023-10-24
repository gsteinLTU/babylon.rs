use std::{cell::RefCell, rc::Rc, f64::consts::PI};

use neo_babylon::prelude::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

struct Game {
    scene: Rc<RefCell<Scene>>,
    models: RefCell<Vec<Rc<BabylonMesh>>>,
    shadow_generator: Rc<CascadedShadowGenerator>,
}

impl Game {
    fn new() -> Self {
        let scene = neo_babylon::api::create_scene("#renderCanvas");
        
        // Add a camera to the scene and attach it to the canvas
        let camera = UniversalCamera::new(
            "Camera",
            Vector3::new(0.0, 1.0, 5.0),
            Some(&scene.borrow())
        );
        camera.set_rotation(&Vector3::new(0.0, PI, 0.0));
        camera.attachControl(neo_babylon::api::get_element("#renderCanvas"), true);
        camera.set_min_z(0.01);
        camera.set_max_z(300.0);
        camera.set_speed(0.35);

        // For the current version, lights are added here, later they will be requested as part of scenario to allow for other lighting conditions
        // Add lights to the scene
        let sun = DirectionalLight::new("light", Vector3::new(-0.25, -1.0, 0.0), &scene.borrow());
        HemisphericLight::new("light1", Vector3::new(1.0, 1.0, 0.0), &scene.borrow());

        let shadow_generator = CascadedShadowGenerator::new(2048.0, &sun);

        Game {
            scene,
            models: RefCell::new(vec![]),
            shadow_generator: Rc::new(shadow_generator),
        }
    }

    async fn load_model(&self, name: &str, url: &str) -> Result<Rc<BabylonMesh>, JsValue> {
        let model = BabylonMesh::create_gltf(&self.scene.borrow(), name, url).await.expect("Failed to load model");

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

    GAME.with(|game| {
        // Create ground to show shadows on
        let box_model = Rc::new(BabylonMesh::create_box(&game.borrow().scene.borrow(), "ground", BoxOptions { depth: Some(10.0), height: Some(0.05), width: Some(10.0), ..Default::default() }));
        box_model.set_position_y(-1.0);
        box_model.set_receive_shadows(true);
        game.borrow().models.borrow_mut().push(box_model);
    });

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

            game_rc.borrow().shadow_generator.add_shadow_caster(&gltf, true);
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

            game_rc.borrow().shadow_generator.add_shadow_caster(&gltf, true);
        }
    }));
}
