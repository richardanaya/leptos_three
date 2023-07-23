# leptos_three

A prototype of creating a React Three Fiber like experience with Leptos and ThreeJS

I wanted to show how i'd create an experience like

See the demo working [here](https://richardanaya.github.io/leptos_three/example/dist/index.html)

```rust
use leptos::*;
use leptos_three::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
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
```

# How to run example?

```
cd example
cargo install trunk # if you don't have it
trunk serve --open
```

# Screenshot

<img width="1224" alt="Screenshot 2023-07-22 at 3 11 09 PM" src="https://github.com/richardanaya/leptos_fiber/assets/294042/c00fb781-06d6-485b-9b1e-c5f41b0456bc">
