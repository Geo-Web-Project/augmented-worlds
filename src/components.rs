use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    pub type Component;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Component)]
    #[derive(Clone)]
    pub type Position;
    #[wasm_bindgen(method, getter)]
    fn x(this: &Position) -> f32;
    #[wasm_bindgen(method, getter)]
    fn y(this: &Position) -> f32;
    #[wasm_bindgen(method, getter)]
    fn z(this: &Position) -> f32;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Component)]
    #[derive(Clone)]
    pub type Scale;
    #[wasm_bindgen(method, getter)]
    fn x(this: &Scale) -> f32;
    #[wasm_bindgen(method, getter)]
    fn y(this: &Scale) -> f32;
    #[wasm_bindgen(method, getter)]
    fn z(this: &Scale) -> f32;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Component)]
    #[derive(Clone)]
    pub type Rotation;
    #[wasm_bindgen(method, getter)]
    fn x(this: &Rotation) -> f32;
    #[wasm_bindgen(method, getter)]
    fn y(this: &Rotation) -> f32;
    #[wasm_bindgen(method, getter)]
    fn z(this: &Rotation) -> f32;
    #[wasm_bindgen(method, getter)]
    fn w(this: &Rotation) -> f32;
}

impl ecs_rust::component::Component for Component {}
impl ecs_rust::component::Component for Position {}
impl ecs_rust::component::Component for Scale {}
impl ecs_rust::component::Component for Rotation {}

#[wasm_bindgen]
pub enum ComponentType {
    Position,
    Scale,
    Rotation,
}
