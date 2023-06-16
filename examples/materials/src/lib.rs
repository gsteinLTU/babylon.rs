use neo_babylon::{api, prelude::*};
use js_sys::Math;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

struct Game {
    scene: Rc<RefCell<Scene>>,
    shapes: Vec<BabylonMesh>,
}

impl Game {
    fn new() -> Self {
        let scene = api::create_scene("#renderCanvas");

        // Add a camera to the scene and attach it to the canvas
        let camera = UniversalCamera::new(
            "Camera",
            Vector3::new(0.0, 0.0, -5.0),
            Some(&scene.borrow())
        );

        camera.attachControl(get_element("#renderCanvas"), true);
    
        // Add lights to the scene
        HemisphericLight::new("light1", Vector3::new(1.0, 1.0, 0.0), &scene.borrow());
        PointLight::new("light2", Vector3::new(0.0, 1.0, -1.0), &scene.borrow());

        Game {
            scene,
            shapes: vec![],
        }
    }
}

thread_local! {
    static GAME: RefCell<Game> = RefCell::new(Game::new());
}

#[wasm_bindgen(start)]
pub fn main() {
    let num_cubes = 10;

    console::log_1(&"Starting demo...".into());

    GAME.with(|game| {
        for i in 0..num_cubes {
            // Create cube of random size at random position
            let cube = BabylonMesh::create_box(
                &game.borrow().scene.borrow(),
                format!("box_{}", i).as_str(),
                BoxOptions {
                    depth: Math::random().into(),
                    height: Math::random().into(),
                    width: Math::random().into(),
                    ..Default::default()
                },
            );
            cube.set_position(&Vector3::new(
                Math::random() - 0.5,
                Math::random() - 0.5,
                Math::random() - 0.5,
            ));

            // Create material with random color
            let mat =
                StandardMaterial::new(format!("mat_{}", i).as_str(), &game.borrow().scene.borrow());
            mat.set_diffuse_color(Color3::new(Math::random(), Math::random(), Math::random()));
            mat.set_alpha(Math::random());
            cube.set_material(&mat);

            game.borrow_mut().shapes.push(cube);
        }
    });
}
