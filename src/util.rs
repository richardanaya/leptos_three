use wasm_bindgen::prelude::*;

pub fn on_load_threejs(f: impl FnMut(web_sys::Event) + 'static) -> Result<(), JsValue> {
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
