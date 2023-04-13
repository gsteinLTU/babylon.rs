use babylon::{prelude::*, api};
use js_sys::Math;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;
use std::{cell::RefCell, rc::Rc};

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


#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Starting demo...".into());

    GAME.with(|game| {
        let scene = api::create_basic_scene("#renderCanvas");
        for i in 0..10 {
            let mut cube = BabylonMesh::create_box(
                &scene.borrow(),
                format!("box_{}", i).as_str(),
                BoxOptions { depth: Math::random().into(), height: Math::random().into(), width: Math::random().into(), ..Default::default()}
            );
            cube.set_position(Vector3::new(
                Math::random() - 0.5,
                Math::random() - 0.5,
                Math::random() - 0.5,
            ));

            let mut mat = StandardMaterial::new(format!("mat_{}", i).as_str(),&scene.borrow());
            mat.set_diffuse_color(Color3::new(
                Math::random(),
                Math::random(),
                Math::random(),
            ));
            mat.set_alpha(Math::random());
            cube.set_material(&mat);
            
            game.borrow_mut().shapes.push(cube);
        }
    });
}
