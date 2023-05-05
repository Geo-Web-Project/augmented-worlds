wai_bindgen_rust::export!("aw-core.wai");

use std::cell::RefCell;
use std::collections::HashMap;

struct AwCore;
struct World {
    entities: Entities
}
struct Entities {
    component_managers: HashMap<String, ComponentManager>
}
struct ComponentManager {
    components: Vec<aw_core::Component>,
    entity_ids: Vec<u32>,
    entity_id_map: HashMap<u32, usize>,
}
struct System {
    update: fn(&Entities)
}

impl aw_core::AwCore for AwCore {}

impl aw_core::World for World {
    fn new() -> wai_bindgen_rust::Handle<World> {
        let mut component_managers = HashMap::new();
        component_managers.insert(String::from("position"), ComponentManager {
            components: Vec::new(),
            entity_ids: Vec::new(),
            entity_id_map: HashMap::new(),
        });
        component_managers.insert(String::from("scale"), ComponentManager {
            components: Vec::new(),
            entity_ids: Vec::new(),
            entity_id_map: HashMap::new(),
        });

       wai_bindgen_rust::Handle::new(World {
           entities: Entities{
               component_managers
           }
       })
    }

    fn add_component_to_entity(&self, entity_id: u32, component: aw_core::Component) {
        // Match each component type to a component manager
        match component {
            aw_core::Component::Position(position) => {
                let component_manager = self.entities.component_managers.get_mut("position").unwrap();
                component_manager.components.push(component);
                component_manager.entity_ids.push(entity_id);
                component_manager.entity_id_map.insert(entity_id, component_manager.components.len() - 1);
            },
            aw_core::Component::Scale(scale) => {
                let component_manager = self.entities.component_managers.get_mut("scale").unwrap();
                component_manager.components.push(component);
                component_manager.entity_ids.push(entity_id);
                component_manager.entity_id_map.insert(entity_id, component_manager.components.len() - 1);
            },
        }
    }

    fn update(&self) {}
}

impl aw_core::Entities for Entities {
    fn get_component(&self, component_key: String, entity_id: u32) -> aw_core::Component {
        let component_manager = self.component_managers.get(&component_key).unwrap();
        let component_index = component_manager.entity_id_map.get(&entity_id).unwrap();
        component_manager.components[*component_index]
    }
    fn get_components(&self, component_key: String) -> Vec<aw_core::Component> {
        let component_manager = self.component_managers.get(&component_key).unwrap();
        component_manager.components
    }
    // fn get_entity_ids_all(&self, component_keys: Vec<String>) -> Vec<u32>;
    // fn get_entity_ids_any(&self, component_keys: Vec<String>) -> Vec<u32>;
}
