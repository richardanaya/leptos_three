use js_sys::Function;
use leptos::*;
use wasm_bindgen::prelude::*;

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

#[derive(Copy, Clone)]
struct SceneContext(ReadSignal<Option<JsValue>>);

#[component]
fn Canvas(cx: Scope, children: Children) -> impl IntoView {
    let input_ref = create_node_ref(cx);
    let (scene, set_scene) = create_signal::<Option<JsValue>>(cx, None);

    provide_context(cx, SceneContext(scene));

    input_ref.on_load(cx, move |element| {
        let element: &web_sys::HtmlCanvasElement = &element;
        let as_html_element = element.unchecked_ref::<web_sys::HtmlElement>();
        let f: Function = js_sys::eval(
            "(renderCanvas)=>{
                const scene = new THREE.Scene();
                const camera = new THREE.PerspectiveCamera(75, renderCanvas.clientWidth/renderCanvas.clientHeight, 0.1, 1000);
                const renderer = new THREE.WebGLRenderer({canvas: renderCanvas});
                renderer.setSize(renderCanvas.clientWidt,renderCanvas.clientHeight);
                // move camera back
                camera.position.z = 5;
                // render scene
                renderer.render(scene, camera);

                const animate = () => {
                    requestAnimationFrame(animate);
                    renderer.render(scene, camera);
                }
                animate();

                // add resize watch of renderCanvas
                const resizeObserver = new ResizeObserver(entries => {
                    for (let entry of entries) {
                        const { width, height } = entry.contentRect;
                        renderer.setSize(width, height);
                        camera.aspect = width / height;
                        camera.updateProjectionMatrix();
                    }
                });
                resizeObserver.observe(renderCanvas);
                return scene;
        }",
        )
        .unwrap()
        .into();
        let s = f
            .call1(
                // get window
                &js_sys::global().unchecked_into::<web_sys::Window>(),
                &as_html_element,
            )
            .unwrap();
        set_scene.set(Some(s));
    });

    view! { cx,
        <canvas
            _ref=input_ref
            style="width: 100%; height: 100%;"
        >
            {children(cx)}
        </canvas>
    }
}

#[component]
fn Cube(cx: Scope, position: [f64; 3]) -> impl IntoView {
    let scene = use_context::<SceneContext>(cx).unwrap().0;

    create_effect(cx, move |_| {
        if let Some(scene) = scene.get() {
            let f: Function = js_sys::eval(
                "(scene,x,y)=>{
                // add cube to scene
                const geometry = new THREE.BoxGeometry();
                const color = Math.random() * 0xffffff;
                const material = new THREE.MeshBasicMaterial({color});
                const cube = new THREE.Mesh(geometry, material);
                cube.position.x = x;
                cube.position.y = y;
                scene.add(cube);

                 // animate
                const animate = () => {
                    requestAnimationFrame(animate);
                    cube.rotation.x += 0.01;
                    cube.rotation.y += 0.01;
                }
                animate();
        }",
            )
            .unwrap()
            .into();
            f.call3(
                // get window
                &js_sys::global().unchecked_into::<web_sys::Window>(),
                &scene,
                &position[0].into(),
                &position[1].into(),
            )
            .unwrap();
        }
    });

    view! { cx,
        None
    }
}

fn on_load_threejs(f: impl FnMut(web_sys::Event) + 'static) -> Result<(), JsValue> {
    let cb = Closure::wrap(Box::new(f) as Box<dyn FnMut(_)>);

    // add threejs to head
    let head = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .head()
        .unwrap();
    let script = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("script")?;
    script.set_attribute(
        "src",
        "https://cdnjs.cloudflare.com/ajax/libs/three.js/0.154.0/three.min.js",
    )?;
    head.append_child(&script)?;

    script.add_event_listener_with_callback("load", &cb.as_ref().unchecked_ref())?;

    cb.forget();
    Ok(())
}
