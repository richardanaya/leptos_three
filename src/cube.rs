use crate::providers;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/cube.js")]
extern "C" {
    fn createCube(scene: JsValue, x: f64, y: f64, z: f64) -> JsValue;
}

#[component]
pub fn Cube(cx: Scope, position: [f64; 3]) -> impl IntoView {
    let scene = use_context::<providers::SceneContext>(cx).unwrap().0;

    create_effect(cx, move |_| {
        if let Some(scene) = scene.get() {
            createCube(scene, position[0], position[1], position[2]);
        }
    });

    view! { cx,
        None
    }
}
