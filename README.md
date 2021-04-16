# ssb-validate2-rsjs

Cryptographic validation of Scuttlebutt messages in the form of Rust bindings for Node.js.

Perform batch verification and validation of SSB messages using [ssb-verify-signatures](https://crates.io/crates/ssb-verify-signatures) and [ssb-validate](https://crates.io/crates/ssb-validate) from the [Sunrise Choir](https://github.com/sunrise-choir).

The [node-bindgen](https://github.com/infinyon/node-bindgen) crate is currently used to generate the bindings from Rust code.

## Build

Rust first needs to be installed in order to build the bindings ([installation instructions](https://rustup.rs/)).

```bash
git clone git@github.com:ssb-ngi-pointer/ssb-validate2-rsjs.git
cd ssb-validate2-rsjs
cargo install nj-cli
# generate release build of ssb-validate2-rsjs and run tests
npm it
```

The build process creates bindings in `./dist/index.node`.

If you wish to rebuild the bindings after making changes to the code, use the `nj-cli` tool:

`nj-cli build --release`

## Usage

```javascript
const validate = require('ssb-validate2-rsjs');

// Verify signatures for an array of messages
validate.verifySignatures(msgs);

// Validate an array of messages by a single author
// Note: assumes msgs[0].sequence == 1 (ie. `previous` == null)
validate.validateBatch(msgs);

// Validate an array of messages by a single author (includes `previous`)
validate.validateBatch(msgs, previous);

// Validate an array of out-of-order messages by a single author
validate.validateOooBatch(msgs);
```

See `test/test.js` in this repo for in-context example usage (uses [ssb-fixtures](https://github.com/ssb-ngi-pointer/ssb-fixtures) and [jitdb](https://github.com/ssb-ngi-pointer/jitdb)).

## Behaviour

The `msgs` argument must be in the form of an array of stringified message objects. The exact command used to stringify each message is:

```javascript
JSON.stringify(msg, null, 2);
```

The `previous` argument for `validateBatch()` is optional. Calling the function without `previous` is the equivalent of passing `previous` as `null`.

Both `verifySignatures()` and `validateBatch()` return `true` on success. In the case of a failure in verification or validation, both functions will return immediately with detailed information in the error message (currently includes the reason for failure and the full message which caused the failure).

Note that `validateBatch()` performs signature verification _and_ full message validation.

## Performance Benchmarks

After performing build instructions (see above):

```bash
cd ssb-validate2-rsjs
# Run benchmarks
node test/perf
```

The default values for the performance benchmarks are 100 messages from 1 author, for a total of 10 iterations. These value constants can be changed in `test/perf.js`.

## License

AGPL 3.0.
