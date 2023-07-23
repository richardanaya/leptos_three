use leptos::*;
use leptos_three::*;
use wasm_bindgen::prelude::*;

pub fn main() -> Result<(), JsValue> {
    mount_to_body(|cx| {
        view! { cx,  <div style="width: 100%; height:100%;">
            <Canvas>
                <Cube position={[3.0, 0.0, 0.0]} >
                    <Material color=[1.0, 0.0, 0.0]/>
                </Cube>
                <Cube position={[0.0, 0.0, 0.0]} >
                    <Material color=[0.0, 1.0, 0.0]/>
                </Cube>
                <Cube position={[-3.0, 0.0, 0.0]} >
                    <Material color=[0.0, 0.0, 1.0]/>
                </Cube>
            </Canvas>
        </div> }
    });
    Ok(())
}
