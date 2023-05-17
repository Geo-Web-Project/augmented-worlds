use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const SYSTEM: &'static str = r#"
export interface Component {}

export type Vector = {
    x: number;
    y: number;
    z: number;
}

export type Quaternion = {
    x: number;
    y: number;
    z: number;
    w: number;
}

export type VectorAnchor = {
    x?: number;
    y?: number;
    z?: number;
};

export type QuaternionAnchor = {
    x?: number;
    y?: number;
    z?: number;
    w?: number;
};

export type Anchor = {
    anchor?: number;
    position?: number | VectorAnchor;
    orientation?: number | QuaternionAnchor;
};

export interface Position extends Component {
    startPosition: Vector;
    position: Vector;
}

export interface Scale extends Component {
    startScale: Vector;
    scale: Vector;
}

export interface Orientation extends Component {
    startOrientation: Quaternion;
    orientation: Quaternion;
}

export interface GLTFModel extends Component {
    glTFModel: any;
}

export interface IsAnchor extends Component {
    isAnchor: boolean;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Component")]
    #[derive(Clone)]
    pub type Component;

    #[wasm_bindgen(typescript_type = "Vector")]
    #[derive(Clone)]
    pub type Vector;
    #[wasm_bindgen(method, getter)]
    fn x(this: &Vector) -> f32;
    #[wasm_bindgen(method, getter)]
    fn y(this: &Vector) -> f32;
    #[wasm_bindgen(method, getter)]
    fn z(this: &Vector) -> f32;

    #[wasm_bindgen(typescript_type = "Quaternion")]
    #[derive(Clone)]
    pub type Quaternion;
    #[wasm_bindgen(method, getter)]
    fn x(this: &Quaternion) -> f32;
    #[wasm_bindgen(method, getter)]
    fn y(this: &Quaternion) -> f32;
    #[wasm_bindgen(method, getter)]
    fn z(this: &Quaternion) -> f32;
    #[wasm_bindgen(method, getter)]
    fn w(this: &Quaternion) -> f32;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Component, typescript_type = "Position")]
    #[derive(Clone)]
    pub type Position;
    #[wasm_bindgen(method, getter)]
    fn startPosition(this: &Position) -> Vector;
    #[wasm_bindgen(method, getter)]
    fn position(this: &Position) -> Vector;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Component, typescript_type = "Scale")]
    #[derive(Clone)]
    pub type Scale;
    #[wasm_bindgen(method, getter)]
    fn startScale(this: &Scale) -> Vector;
    #[wasm_bindgen(method, getter)]
    fn scale(this: &Scale) -> Vector;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Component, typescript_type = "Orientation")]
    #[derive(Clone)]
    pub type Orientation;
    #[wasm_bindgen(method, getter)]
    fn startOrientation(this: &Orientation) -> Quaternion;
    #[wasm_bindgen(method, getter)]
    fn orientation(this: &Orientation) -> Quaternion;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Component, typescript_type = "GLTFModel")]
    #[derive(Clone)]
    pub type GLTFModel;
    #[wasm_bindgen(method, getter)]
    fn glTFModel(this: &GLTFModel) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Component, typescript_type = "IsAnchor")]
    #[derive(Clone)]
    pub type IsAnchor;
    #[wasm_bindgen(method, getter)]
    fn isAnchor(this: &IsAnchor) -> bool;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Component, typescript_type = "Anchor")]
    #[derive(Clone)]
    pub type Anchor;
    #[wasm_bindgen(method, getter)]
    fn anchor(this: &Anchor) -> Option<usize>;
    #[wasm_bindgen(method, getter)]
    fn position(this: &Anchor) -> JsValue;
    #[wasm_bindgen(method, getter)]
    fn orientation(this: &Anchor) -> JsValue;
}

impl ecs_rust::component::Component for Component {}
impl ecs_rust::component::Component for Position {}
impl ecs_rust::component::Component for Scale {}
impl ecs_rust::component::Component for Orientation {}
impl ecs_rust::component::Component for GLTFModel {}
impl ecs_rust::component::Component for IsAnchor {}
impl ecs_rust::component::Component for Anchor {}

#[wasm_bindgen]
pub enum ComponentType {
    Component,
    Position,
    Scale,
    Orientation,
    GLTFModel,
    IsAnchor,
    Anchor,
}
