use leptos::*;
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone)]
pub struct SceneContext(pub ReadSignal<Option<JsValue>>);
