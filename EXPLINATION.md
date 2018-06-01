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

This tells the Rust compiler that we need to enable some extra features. 
The first is [proc_macro](https://github.com/rust-lang/rust/blob/master/src/doc/unstable-book/src/language-features/proc-macro.md). 
This allows Rust macros to operate in a slightly different way, which is needed for `wasm-bindgen`.
Next is `wasm_import_module` and `wasm_custom_section`. This is again needed for `wasm-bindgen` to function.