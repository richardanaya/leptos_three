# leptos_fiber
A prototype of creating a React Fiber like experience with Leptos and ThreeJS

I wanted to show how i'd create an experience like

See the demo working [here](https://richardanaya.github.io/leptos_fiber/dist/index.html)

```rust
pub fn main() -> Result<(), JsValue> {
    on_load_threejs(|_| {
        mount_to_body(|cx| {
            view! { cx,  <div style="width: 100%; height:100%;">
                <Canvas>
                    <Cube position={[3.0, 0.0, 0.0]} ></Cube>
                    <Cube position={[0.0, 0.0, 0.0]} ></Cube>
                    <Cube position={[-3.0, 0.0, 0.0]} ></Cube>
                </Canvas>
            </div> }
        });
    })?;
    Ok(())
}
```
