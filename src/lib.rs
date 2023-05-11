use wasm_bindgen::prelude::*;

use crate::components::*;
use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};

mod components;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
extern "C" {
    pub type System;

    #[wasm_bindgen(method)]
    fn update(
        this: &System,
        get_component: &mut dyn FnMut(ComponentType, usize) -> Option<Component>,
    );
}

#[wasm_bindgen]
pub struct World {
    ecs_world: ecs_rust::world::World,
}

impl ecs_rust::system::System for System {
    fn update(&mut self, manager: &mut EntityManager, _accessor: &mut EntityIdAccessor) {
        // Passing Rust closures to JS, seems to be the simplest way
        let get_component =
            &mut |component_type: ComponentType, entity_id: usize| -> Option<Component> {
                match component_type {
                    ComponentType::Position => manager
                        .borrow_component::<Position>(entity_id)
                        .map(|v| v.clone().into()),
                    ComponentType::Scale => manager
                        .borrow_component::<Scale>(entity_id)
                        .map(|v| v.clone().into()),
                    ComponentType::Orientation => manager
                        .borrow_component::<Orientation>(entity_id)
                        .map(|v| v.clone().into()),
                    ComponentType::GLTFModel => manager
                        .borrow_component::<GLTFModel>(entity_id)
                        .map(|v| v.clone().into()),
                }
            };

        System::update(self, get_component);
    }
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new() -> World {
        let mut world = ecs_rust::world::World::new();

        // Register included components
        world.register_component::<Component>();
        world.register_component::<Position>();
        world.register_component::<Scale>();
        world.register_component::<Orientation>();
        world.register_component::<GLTFModel>();

        World { ecs_world: world }
    }

    pub fn create_entity(&mut self) -> usize {
        self.ecs_world.create_entity()
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        self.ecs_world.remove_entity(entity_id);
    }

    pub fn add_system(&mut self, system: System) {
        self.ecs_world.add_system(system);
    }

    pub fn add_component_to_entity(
        &mut self,
        entity_id: usize,
        component_type: ComponentType,
        component: JsValue,
    ) {
        match component_type {
            // Position
            ComponentType::Position => {
                self.ecs_world
                    .add_component_to_entity(entity_id, Position::from(component));
            }
            // Scale
            ComponentType::Scale => {
                self.ecs_world
                    .add_component_to_entity(entity_id, Scale::from(component));
            }
            // Rotation
            ComponentType::Orientation => {
                self.ecs_world
                    .add_component_to_entity(entity_id, Orientation::from(component));
            }
            // GLTFModel
            ComponentType::GLTFModel => {
                self.ecs_world
                    .add_component_to_entity(entity_id, GLTFModel::from(component));
            }
        }
    }

    pub fn update(&mut self) {
        self.ecs_world.update()
    }
}
