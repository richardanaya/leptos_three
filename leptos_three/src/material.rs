use crate::providers;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/material.js")]
extern "C" {
    fn attachMaterial(object3d: JsValue, r: f64, g: f64, b: f64) -> JsValue;
}

#[component]
pub fn Material(cx: Scope, #[prop(default = [1.0,1.0,1.0])] color: [f64; 3]) -> impl IntoView {
    let object3d = use_context::<providers::Object3DContext>(cx).unwrap().0;

    create_effect(cx, move |_| {
        if let Some(o) = object3d.get() {
            attachMaterial(o, color[0], color[1], color[2]);
        }
    });

    view! { cx,
        <></>
    }
}
