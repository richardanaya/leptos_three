use leptos::*;
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone)]
pub struct SceneContext(pub ReadSignal<Option<JsValue>>);

#[derive(Copy, Clone)]
pub struct Object3DContext(pub ReadSignal<Option<JsValue>>);
