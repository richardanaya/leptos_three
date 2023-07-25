use crate::providers;
use crate::three::*;
use leptos::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn Material(cx: Scope, #[prop(default = [1.0,1.0,1.0])] color: [f64; 3]) -> impl IntoView {
    let object3d = use_context::<providers::Object3DContext>(cx).unwrap().0;

    create_effect(cx, move |_| {
        if let Some(o) = object3d.get() {
            let mat = MeshBasicMaterial::new();
            let c: u32 = (color[0] * 255.0) as u32 * 256 * 256
                + (color[1] * 255.0) as u32 * 256
                + (color[2] * 255.0) as u32;
            mat.set_color(Color::new_with_rgb(c));
            o.set_material(mat);
        }
    });

    view! { cx,
        <></>
    }
}
