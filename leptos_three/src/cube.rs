use crate::providers;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/cube.js")]
extern "C" {
    fn createCube(scene: JsValue, x: f64, y: f64, z: f64) -> JsValue;
}

#[component]
pub fn Cube(
    cx: Scope,
    children: Children,
    #[prop(default = [0.0,0.0,0.0])] position: [f64; 3],
) -> impl IntoView {
    let scene = use_context::<providers::SceneContext>(cx).unwrap().0;

    let (object3d, set_object3d) = create_signal::<Option<JsValue>>(cx, None);

    provide_context(cx, providers::Object3DContext(object3d));

    create_effect(cx, move |_| {
        if let Some(scene) = scene.get() {
            let o = createCube(scene, position[0], position[1], position[2]);
            set_object3d.set(Some(o));
        }
    });

    view! { cx,
            <>
            {children(cx)}
            </>
    }
}
