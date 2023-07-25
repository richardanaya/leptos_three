use crate::three::*;
use leptos::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[derive(Copy, Clone)]
pub struct SceneContext(pub ReadSignal<Option<Rc<Scene>>>);

#[derive(Copy, Clone)]
pub struct Object3DContext(pub ReadSignal<Option<Rc<Mesh>>>);
