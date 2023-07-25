use crate::{providers, Scene};
use leptos::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/canvas.js")]
extern "C" {
    fn startCanvas(canvas: JsValue, scene: Scene) -> Scene;
}

#[component]
pub fn Canvas(cx: Scope, children: Children) -> impl IntoView {
    let input_ref = create_node_ref(cx);
    let (scene, set_scene) = create_signal::<Option<Rc<Scene>>>(cx, None);

    provide_context(cx, providers::SceneContext(scene));

    input_ref.on_load(cx, move |element| {
        let element: &web_sys::HtmlCanvasElement = &element;
        let as_html_element = element.unchecked_ref::<web_sys::HtmlElement>();
        let scene = Scene::new();
        let s = startCanvas(as_html_element.into(), scene);
        set_scene.set(Some(Rc::new(s)));
    });

    view! { cx,
        <canvas
            _ref=input_ref
            style="width: 100%; height: 100%;"
        >
            {children(cx)}
        </canvas>
    }
}
