use js_sys::*;
use leptos::*;
use leptos_three::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (r_pos, set_r_pos) = create_signal(cx, [1.0, 0.0, 0.0]);
    let (g_pos, set_g_pos) = create_signal(cx, [0.0, 1.0, 0.0]);
    let (b_pos, set_b_pos) = create_signal(cx, [0.0, 0.0, 1.0]);

    let a = Closure::<dyn Fn()>::new(move || {
        set_r_pos.set([Math::random(), Math::random(), Math::random()]);
        set_g_pos.set([Math::random(), Math::random(), Math::random()]);
        set_b_pos.set([Math::random(), Math::random(), Math::random()]);
    });

    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(a.as_ref().unchecked_ref(), 1000)
        .unwrap();
    a.forget();

    view! { cx,
            <Canvas>
                <AmbientLight color=[1.0,1.0,1.0] intensity={0.5}/>
                <Cube position={[3.0, 0.0, 0.0]} >
                    <Material color={r_pos}/>
                </Cube>
                <Cube position={[0.0, 0.0, 0.0]} >
                    <Material color={g_pos}/>
                </Cube>
                <Cube position={[-3.0, 0.0, 0.0]} >
                    <Material color={b_pos}/>
                </Cube>
            </Canvas>
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    mount_to_body(|cx| {
        view! { cx,  <div style="width: 100%; height:100%;">
            <App />
        </div> }
    });
    Ok(())
}
