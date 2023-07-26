use crate::providers;
use crate::three::*;
use leptos::*;

#[component]
pub fn AmbientLight(
    cx: Scope,
    #[prop(optional, into)] color: Option<MaybeSignal<[f64; 3]>>,
    #[prop(optional, into)] intensity: Option<MaybeSignal<f64>>,
) -> impl IntoView {
    let scene = use_context::<providers::SceneContext>(cx).unwrap().0;

    create_effect(cx, move |_| {
        if let Some(scene) = scene.get() {
            let color = if let Some(color) = color {
                color.get()
            } else {
                [1.0, 1.0, 1.0]
            };
            let intensity = if let Some(intensity) = intensity {
                intensity.get()
            } else {
                1.0
            };
            let light = AmbientLight::new(
                Color::new_with_rgb_components(color[0], color[1], color[2]),
                intensity,
            );
            scene.add_with_light(&light);
        }
    });

    view! { cx,
            <>
            </>
    }
}
