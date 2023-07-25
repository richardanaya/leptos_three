use crate::providers;
use crate::three::*;
use leptos::*;
use std::rc::Rc;

#[component]
pub fn Material(cx: Scope, #[prop(default = [1.0,1.0,1.0])] color: [f64; 3]) -> impl IntoView {
    let object3d = use_context::<providers::Object3DContext>(cx).unwrap().0;
    let (mat, set_mat) = create_signal::<Option<Rc<MeshBasicMaterial>>>(cx, None);

    create_effect(cx, move |_| {
        if let Some(o) = object3d.get() {
            let mat = MeshBasicMaterial::new();
            mat.set_color(Color::new_with_rgb_components(color[0], color[1], color[2]));
            o.set_material(&mat);
            set_mat.set(Some(Rc::new(mat)));
        }
    });

    view! { cx,
        <></>
    }
}
