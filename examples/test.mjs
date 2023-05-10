import { AwWorld, ComponentType } from "../pkg/wai_augmented_worlds.js"

const world = new AwWorld()
const e = world.create_entity()
world.add_component_to_entity(e, ComponentType.Position, { type: ComponentType.Position, x: 1, y: 2, z: 3 })
world.add_component_to_entity(e, ComponentType.Scale, { type: ComponentType.Position, x: 4, y: 5, z: 6 })
world.add_component_to_entity(e, ComponentType.Rotation, { type: ComponentType.Position, x: 4, y: 5, z: 6, w: 0 })
world.add_component_to_entity(e, ComponentType.GLTFModel, { type: ComponentType.GLTFModel, glTFModel: { "/": "QmdPXtkGThsWvR1YKg4QVSR9n8oHMPmpBEnyyV8Tk638o9" } })

class TestSystem {
  update(get_component) {
    console.log("Hello from TestSystem")
    console.log(get_component(ComponentType.Position, e))
    console.log(get_component(ComponentType.Scale, e))
    console.log(get_component(ComponentType.Rotation, e))
    console.log(get_component(ComponentType.GLTFModel, e))
  }
}

world.add_system(new TestSystem())
world.update()
