# ssb-validate2-neon

Cryptographic validation of Scuttlebutt messages in the form of Neon bindings (Rust) for Node.js.

Validate SSB messages and message values using the [ssb-validate crate](https://crates.io/crates/ssb-validate) ([repo](https://github.com/sunrise-choir/ssb-validate) & [docs](https://docs.rs/ssb-validate/1.0.1/ssb_validate/index.html)). Generate bindings to expose this functionality as a Node module.

## Usage

Validate an array of messages by a single author:

```javascript
const validate = require('.');

validate.validateMsgArray(msgs)
```

See `test.js` in this repo for in-context example usage (using [ssb-fixtures](https://github.com/ssb-ngi-pointer/ssb-fixtures) and [jitdb](https://github.com/ssb-ngi-pointer/jitdb)).

## Links

Neon: [repo](https://github.com/neon-bindings/neon) & [docs](https://neon-bindings.com/docs/intro).

## License

AGPL 3.0.
