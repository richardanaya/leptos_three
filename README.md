# leptos_fiber

A prototype of creating a React Three Fiber like experience with Leptos and ThreeJS

I wanted to show how i'd create an experience like

See the demo working [here](https://richardanaya.github.io/leptos_fiber/dist/index.html)

```rust
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
```

<img width="1224" alt="Screenshot 2023-07-22 at 3 11 09 PM" src="https://github.com/richardanaya/leptos_fiber/assets/294042/c00fb781-06d6-485b-9b1e-c5f41b0456bc">
