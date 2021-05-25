# ssb-validate2-rsjs-wasm

Cryptographic validation of Scuttlebutt messages using WebAssembly.

Perform batch verification and validation of SSB messages using [ssb-verify-signatures](https://crates.io/crates/ssb-verify-signatures) and [ssb-validate](https://github.com/mycognosist/ssb-validate) from the [Sunrise Choir](https://github.com/sunrise-choir) in the browser.

The [wasm-bindgen](https://crates.io/crates/wasm-bindgen) and [wasm-bindgen-rayon](https://crates.io/crates/wasm-bindgen-rayon) crates are currently used to generate WASM from Rust code.

## Build

Rust first needs to be installed in order to compile to WASM ([installation instructions](https://rustup.rs/)).

```bash
git clone git@github.com:ssb-ngi-pointer/ssb-validate2-rsjs.git
cd ssb-validate2-rsjs/web
cargo install wasm-pack
# generate release build of ssb-validate2-rsjs
npm run build:web
# run the tests
npm run test:web
```

The build process creates JavaScript and WASM artifacts in `./pkgs/`. This includes automatically-generated JavaScript code to initialize and handle web workers when running the WASM module in the browser (required for threading support).

If you wish to rebuild the WASM module after making changes to the code, use the `wasm-pack` tool:

`wasm-pack build --target web`

The tool can also compile for alternative target environments. See the [deployment guide](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html) for more information.

## Tests

Tests for single-author and multi-author messages are included. These tests are defined using [jasmine](https://jasmine.github.io/index.html) and are executed with [karma](http://karma-runner.github.io/6.3/index.html). The tests and related artifacts, such as JSON messages, can be found in the `web/test` directory. Test configuration for `karma` can be found in `karma.conf.js` in the root of this repo.

## Useful Documentation

The [wasm-bindgen book](https://rustwasm.github.io/docs/wasm-bindgen/introduction.html) provides detailed information about WebAssembly in the context of Rust.

## License

AGPL 3.0.
