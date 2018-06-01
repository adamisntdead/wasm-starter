<h1 align="center">WebAssembly Starter</h1>
<p align="center">
  WebAssembly Module Starter Code, Powered by Rust, wasm-pack, wasm-bindgen and Webpack 🔥 😍
</p>

## Setup

First you are gonna have to get Rust at [rust-lang.org](https://www.rust-lang.org/).
Then, you will need to have [nodejs](https://nodejs.org/) installed.

For the WebAssembly side of things, you are going to have to get to Rust nightly,
and install the WebAssembly compiler.

```bash
rustup default nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

Then, you are going to need [wasm-pack](https://github.com/ashleygwilliams/wasm-pack)

```bash
cargo install wasm-pack
```

Finally, you can compile the WebAssembly module.

```bash
# Run wasm-pack
wasm-pack init .

# Install the example files
cd example
npm install
npm run serve
```

Note: It can take about 3 minutes just to do that initial `wasm-pack` so be ready!
You can then open the page at [localhost:8080](http://localhost:8080)

## Usage

The file `wasm_starter.js` exposes a single method `hello_world()`, which just displays a hello world input

If you setup [Webpack 4](https://webpack.js.org/), it supports webassembly out of the box, and the module can be used like so:

```javascript
const js = import('../pkg/wasm_starter.js')

js.then(js => {
  js.hello_world()
})
```

Which displays:

![Hello World!](https://i.imgur.com/OAHa44l.png)

## License

MIT