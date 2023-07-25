use crate::providers;
use crate::three::*;
use leptos::*;
use std::rc::Rc;

#[component]
pub fn Cube(
    cx: Scope,
    children: Children,
    #[prop(default = [0.0,0.0,0.0])] position: [f64; 3],
) -> impl IntoView {
    let scene = use_context::<providers::SceneContext>(cx).unwrap().0;

    let (object3d, set_object3d) = create_signal::<Option<Rc<Mesh>>>(cx, None);

    provide_context(cx, providers::Object3DContext(object3d));

    create_effect(cx, move |_| {
        if let Some(scene) = scene.get() {
            let mat = MeshBasicMaterial::new();
            let geo = BoxGeometry::new();
            let mesh = Mesh::new(&geo, &mat);
            mesh.position().set_x(position[0]);
            mesh.position().set_y(position[1]);
            mesh.position().set_z(position[2]);
            scene.add(&mesh);
            set_object3d.set(Some(Rc::new(mesh)));
        }
    });

    view! { cx,
            <>
            {children(cx)}
            </>
    }
}
