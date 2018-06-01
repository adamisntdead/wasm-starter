const js = import('../pkg/wasm_starter.js')

js.then(js => {
  js.hello_world()
})