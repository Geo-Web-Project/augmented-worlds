use wasm_bindgen::prelude::*;

use ecs_rust::world::World;
use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "system")]
extern "C" {
    type System;

    #[wasm_bindgen(method)]
    fn update(this: &System, get_component: &mut dyn FnMut(ComponentType, usize) -> Option<Component>);
}

#[wasm_bindgen(module = "component")]
extern "C" {
    #[derive(Clone)]
    type Component;
}

#[wasm_bindgen(module = "position")]
extern "C" {
    #[wasm_bindgen(extends = Component)]
    #[derive(Clone)]
    type Position;
    #[wasm_bindgen(method, getter)]
    fn x(this: &Position) -> f32;
    #[wasm_bindgen(method, getter)]
    fn y(this: &Position) -> f32;
    #[wasm_bindgen(method, getter)]
    fn z(this: &Position) -> f32;
}

#[wasm_bindgen(module = "scale")]
extern "C" {
    #[wasm_bindgen(extends = Component)]
    #[derive(Clone)]
    type Scale;
    #[wasm_bindgen(method, getter)]
    fn x(this: &Scale) -> f32;
    #[wasm_bindgen(method, getter)]
    fn y(this: &Scale) -> f32;
    #[wasm_bindgen(method, getter)]
    fn z(this: &Scale) -> f32;
}

impl ecs_rust::component::Component for Component {}
impl ecs_rust::component::Component for Position {}
impl ecs_rust::component::Component for Scale {}

#[wasm_bindgen]
pub enum ComponentType {
    Position,
    Scale
}

#[wasm_bindgen]
struct AwWorld {
    ecs_world: World
}

impl ecs_rust::system::System for System {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        System::update(self, &mut |component_type: ComponentType, entity_id: usize| -> Option<Component> {
            match component_type {
                ComponentType::Position => {
                    // Handle Option
                    match manager.borrow_component::<Position>(entity_id) {
                        Some(component) => Some(component.clone().into()),
                        None => None
                    }
                },
                ComponentType::Scale => {
                    // Handle Option
                    match manager.borrow_component::<Scale>(entity_id) {
                        Some(component) => Some(component.clone().into()),
                        None => None
                    }
                }
            }
        });
    }
}

#[wasm_bindgen]
impl AwWorld {
    #[wasm_bindgen(constructor)]
    pub fn new() -> AwWorld {
        let mut world = World::new();

        // Register included components
        world.register_component::<Component>();
        world.register_component::<Position>();
        world.register_component::<Scale>();

        AwWorld { ecs_world: world }
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

	pub fn add_component_to_entity(&mut self, entity_id: usize, component_type: ComponentType, component: JsValue) {
	   match component_type {
            ComponentType::Position => {
                self.ecs_world.add_component_to_entity(entity_id, Position::from(component));
            },
            ComponentType::Scale => {
                self.ecs_world.add_component_to_entity(entity_id, Scale::from(component));
            }
        }
    }

    pub fn update(&mut self) {
		self.ecs_world.update()
	}
}

// impl aw_core::World for World {
//     fn new() -> Handle<World> {
//         let mut component_managers = HashMap::new();
//         component_managers.insert(String::from("position"), ComponentManager {
//             components: Vec::new(),
//             entity_ids: Vec::new(),
//             entity_id_map: HashMap::new(),
//         });
//         component_managers.insert(String::from("scale"), ComponentManager {
//             components: Vec::new(),
//             entity_ids: Vec::new(),
//             entity_id_map: HashMap::new(),
//         });

//        Handle::new(World {
//            entities: RefCell::new(Entities{
//                component_managers
//            }),
//            systems: RefCell::new(Vec::new())
//        })
//     }

//     fn add_component_to_entity(&self, entity_id: u32, component: aw_core::Component) {
//         // Match each component type to a component manager
//         match component {
//             aw_core::Component::Position(position) => {
//                 let mut entities = self.entities.borrow_mut();
//                 let component_manager = entities.component_managers.get_mut("position").unwrap();
//                 component_manager.components.push(component);
//                 component_manager.entity_ids.push(entity_id);
//                 component_manager.entity_id_map.insert(entity_id, component_manager.components.len() - 1);
//             },
//             aw_core::Component::Scale(scale) => {
//                 let mut entities = self.entities.borrow_mut();
//                 let component_manager = entities.component_managers.get_mut("scale").unwrap();
//                 component_manager.components.push(component);
//                 component_manager.entity_ids.push(entity_id);
//                 component_manager.entity_id_map.insert(entity_id, component_manager.components.len() - 1);
//             },
//         }
//     }

//     fn register_system(&self, system: Handle<dyn System>) {
//         let mut systems = self.systems.borrow_mut();
//         systems.push(system);
//     }

//     fn update(&self) {}
// }

// impl aw_core::Entities for Entities {
//     fn get_component(&self, component_key: String, entity_id: u32) -> aw_core::Component {
//         let component_manager = self.component_managers.get(&component_key).unwrap();
//         let component_index = component_manager.entity_id_map.get(&entity_id).unwrap();
//         component_manager.components[*component_index]
//     }
//     // fn get_components(&self, component_key: String) -> Vec<aw_core::Component> {
//     //     let component_manager = self.component_managers.get(&component_key).unwrap();
//     //     component_manager.components
//     // }
//     // fn get_entity_ids_all(&self, component_keys: Vec<String>) -> Vec<u32>;
//     // fn get_entity_ids_any(&self, component_keys: Vec<String>) -> Vec<u32>;
// }
