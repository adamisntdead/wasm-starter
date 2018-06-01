# Explination

This file is intended as an explination of all the parts of this starter code.

## `cargo.toml`

In the `cargo.toml` file, there are some things that may be different to how you normally write your rust crates.

First you will see there is a crate added already:

```toml
[dependencies]
wasm-bindgen = "0.2"
```

This adds the [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen/), which is a crate which makes it easier to do things between Rust and WebAssembly.

This includes working with things such as 

* Importing Javascript functionality
* Exporting Rust functionality
* working with richer types like strings, classes, closures, objects rather then just `u32` and floats


Next you will see another statement:

```toml
[lib]
crate-type = ["cdylib"]
```

What this does it is tells rust to compile a library using static linking, rather than dynamic linking. 
This is because you cannot dynamically link to other rust code through WebAssembly.

## `lib.rs`

The first line of code in `lib.rs` is 
`#![feature(proc_macro, wasm_import_module, wasm_custom_section)]`

This tells the nightly Rust compiler that we need to enable some extra features. 
The first is [proc_macro](https://github.com/rust-lang/rust/blob/master/src/doc/unstable-book/src/language-features/proc-macro.md). 
This allows Rust macros to operate in a slightly different way, which is needed for `wasm-bindgen`.
Next is `wasm_import_module` and `wasm_custom_section`. This is again needed for `wasm-bindgen` to function.

After this, there is some imports.
The `wasm-bindgen` crate is imported, then the prelude functions are used:

```rust
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
```

The `prelude` module contains all of the types and functions that `wasm-bindgen` uses
to run (when it is doing the code generation).

Then, `wasm-bindgen` is begin used to import a function from javascript, the `alert` function:

```rust
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}
```

Notice that we are using `extern`, which is usually used for external function signatures
from foreign libraries.
Normally, WebAssembly only supports Numbers, so `wasm-bindgen` will handle all of the
types such as `&str`.

Once this is done, the `alert` function is available for use, and another function `hello_world` is created, and again, `wasm-bindgen` is being used to handle all the
types and the javascript bindings.

```rust
#[wasm_bindgen]
pub fn hello_world() {
    alert("Hello World!");
}
```

## wasm-pack

[wasm-pack](https://github.com/ashleygwilliams/wasm-pack) is a tool to be able to publish compiled WebAssembly projects to the npm registry. 

It has 3 main commands, which are best described by the `wasm-pack` README:

![The wasm-pack Commands](https://i.imgur.com/CACWwL3.png)

The `wasm-pack init .` command can be used to create the wasm files and the javascript bindings (along with the TypeScript types!), which are then used by the final javascript app

## The `example` Folder

The `example` folder shows some demo usage for the library npm module which is being built.

It uses `webpack` and `webpack-dev-server`, as [WebAssembly is supported in Webpack 4](https://medium.com/webpack/webpack-4-released-today-6cdb994702d4).

This shows the usage of the library

```javascript
const js = import('../pkg/wasm_starter.js')

js.then(js => {
  js.hello_world()
})
```

First of all, the use of the `import` function is because (at this moment in time, it should be changing!) WebAssembly is imported asynchronously. This is also why there is a promise used, which then passes through the available module.