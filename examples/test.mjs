import { World, ComponentType } from "../pkg/augmented_worlds.js"

const world = new World()
const e = world.create_entity()
world.add_component_to_entity(e, ComponentType.Position, { type: ComponentType.Position, x: 1, y: 2, z: 3 })
world.add_component_to_entity(e, ComponentType.GLTFModel, { type: ComponentType.GLTFModel, glTFModel: { "/": "QmdPXtkGThsWvR1YKg4QVSR9n8oHMPmpBEnyyV8Tk638o9" } })

class TestSystem {
  update(get_component, get_components) {
    console.log("Hello from TestSystem")
    const c = get_components(ComponentType.Position)
    console.log(c)
    c[0].x += 1
  }
}

world.add_system(new TestSystem())

for (let i = 0; i < 10; i++) {
  world.update()
}
