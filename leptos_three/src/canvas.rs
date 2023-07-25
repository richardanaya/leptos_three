use crate::providers;
use crate::three::*;
use leptos::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/canvas.js")]
extern "C" {
    fn startCanvas(canvas: JsValue, camera: PerspectiveCamera, scene: Scene) -> Scene;
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
        let client_width = as_html_element.client_width() as f64;
        let client_height = as_html_element.client_height() as f64;
        let camera = PerspectiveCamera::new(75.0, client_width / client_height, 0.1, 1000.0);
        camera.position().set_z(5.0);
        let s = startCanvas(as_html_element.into(), camera.into(), scene);
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
