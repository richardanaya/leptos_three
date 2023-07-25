use crate::providers;
use crate::three::*;
use leptos::*;
use std::rc::Rc;

#[component]
pub fn Material(
    cx: Scope,
    #[prop(optional, into)] color: Option<MaybeSignal<[f64; 3]>>,
) -> impl IntoView {
    let object3d = use_context::<providers::Object3DContext>(cx).unwrap().0;
    let (mat, set_mat) = create_signal::<Option<Rc<MeshBasicMaterial>>>(cx, None);

    create_effect(cx, move |_| {
        if let Some(o) = object3d.get() {
            let mat = MeshBasicMaterial::new();
            o.set_material(&mat);
            set_mat.set(Some(Rc::new(mat)));
        }
    });

    create_effect(cx, move |_| {
        // set the the material to the watchable color
        if let Some(mat) = mat.get() {
            let color = if let Some(color) = color {
                color.get()
            } else {
                [1.0, 1.0, 1.0]
            };
            mat.set_color(Color::new_with_rgb_components(color[0], color[1], color[2]));
        }
    });

    view! { cx,
        <></>
    }
}
