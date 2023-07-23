use crate::providers;
use crate::util;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/canvas.js")]
extern "C" {
    fn startCanvas(canvas: JsValue) -> JsValue;
}

#[component]
pub fn Canvas(cx: Scope, children: Children) -> impl IntoView {
    let input_ref = create_node_ref(cx);
    let (scene, set_scene) = create_signal::<Option<JsValue>>(cx, None);

    provide_context(cx, providers::SceneContext(scene));

    input_ref.on_load(cx, move |element| {
        util::on_load_threejs(move |_| {
            let element: &web_sys::HtmlCanvasElement = &element;
            let as_html_element = element.unchecked_ref::<web_sys::HtmlElement>();
            let s = startCanvas(as_html_element.into());
            set_scene.set(Some(s));
        })
        .unwrap();
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
