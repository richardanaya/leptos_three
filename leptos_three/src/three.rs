use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/three.js")]
extern "C" {
    pub type BoxGeometry;

    #[wasm_bindgen(constructor)]
    pub fn new() -> BoxGeometry;
}

#[wasm_bindgen(module = "/src/three.js")]
extern "C" {
    pub type MeshBasicMaterial;

    #[wasm_bindgen(constructor)]
    pub fn new() -> MeshBasicMaterial;

    // color
    #[wasm_bindgen(method, setter, js_name = "color")]
    pub fn set_color(this: &MeshBasicMaterial, color: Color);
}

#[wasm_bindgen(module = "/src/three.js")]
extern "C" {
    pub type Mesh;

    #[wasm_bindgen(constructor, js_name = "new")]
    fn new_raw(geo: JsValue, mat: JsValue) -> Mesh;

    // position

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &Mesh) -> Vector3;

    // set_material_raw

    #[wasm_bindgen(method, setter, js_name = "material")]
    fn set_material_raw(this: &Mesh, mat: JsValue);
}

pub trait Geometry {
    fn into_wasm_abi(self) -> JsValue;
}

impl Geometry for BoxGeometry {
    fn into_wasm_abi(self) -> JsValue {
        self.into()
    }
}

pub trait Material {
    fn into_wasm_abi(self) -> JsValue;
}

impl Material for MeshBasicMaterial {
    fn into_wasm_abi(self) -> JsValue {
        self.into()
    }
}

impl Mesh {
    pub fn new(geo: impl Geometry, mat: impl Material) -> Mesh {
        let geo_js_value = geo.into_wasm_abi();
        let mat_js_value = mat.into_wasm_abi();
        Mesh::new_raw(geo_js_value, mat_js_value)
    }

    pub fn set_material(&self, mat: impl Material) {
        let mat_js_value = mat.into_wasm_abi();
        self.set_material_raw(mat_js_value);
    }
}

#[wasm_bindgen(module = "/src/three.js")]
extern "C" {
    pub type Vector3;

    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Vector3;

    // x, y ,z setters and getters

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &Vector3) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &Vector3) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn z(this: &Vector3) -> f64;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &Vector3, x: f64);

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &Vector3, y: f64);

    #[wasm_bindgen(method, setter)]
    pub fn set_z(this: &Vector3, z: f64);
}

#[wasm_bindgen(module = "/src/three.js")]
extern "C" {
    pub type Scene;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Scene;

    // add

    #[wasm_bindgen(method)]
    pub fn add(this: &Scene, object: &Mesh);
}

#[wasm_bindgen(module = "/src/three.js")]
extern "C" {
    pub type Color;

    #[wasm_bindgen(constructor, js_name = "new")]
    pub fn new_with_rgb(rgb: u32) -> Color;

    #[wasm_bindgen(constructor, js_name = "new")]
    pub fn new_with_str(s: &str) -> Color;
}
