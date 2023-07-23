use leptos::*;
use wasm_bindgen::prelude::*;

mod canvas;
mod cube;
mod providers;
mod util;

use canvas::*;
use cube::*;

pub fn main() -> Result<(), JsValue> {
    mount_to_body(|cx| {
        view! { cx,  <div style="width: 100%; height:100%;">
            <Canvas>
                <Cube position={[3.0, 0.0, 0.0]} ></Cube>
                <Cube position={[0.0, 0.0, 0.0]} ></Cube>
                <Cube position={[-3.0, 0.0, 0.0]} ></Cube>
            </Canvas>
        </div> }
    });
    Ok(())
}
