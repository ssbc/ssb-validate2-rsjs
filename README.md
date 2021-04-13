# ssb-validate2-neon

Cryptographic validation of Scuttlebutt messages in the form of [Neon bindings](https://github.com/neon-bindings/neon) (Rust) for Node.js.

Perform batch verification and validation of SSB messages using [ssb-verify-signatures](https://crates.io/crates/ssb-verify-signatures) and [ssb-validate](https://crates.io/crates/ssb-validate) from the [Sunrise Choir](https://github.com/sunrise-choir).

## Usage

```javascript
const validate = require('.');

// Verify signatures for an array of messages
validate.verifySignatures(msgs)

// Validate an array of messages by a single author
// Note: assumes msgs[0].sequence == 1 (ie. `previous` == null)
validate.validateBatch(msgs)

// Validate an array of messages by a single author (includes `previous`)
validate.validateBatch(msgs, previous)
```

See `test/test.js` in this repo for in-context example usage (uses [ssb-fixtures](https://github.com/ssb-ngi-pointer/ssb-fixtures) and [jitdb](https://github.com/ssb-ngi-pointer/jitdb)).

## Behaviour

Both `verifySignatures()` and `validateBatch()` return `true` on success. In the case of a failure in verification or validation, both functions will return immediately with detailed information in the error message (currently includes the reason for failure and the full message which caused the failure).

Note that `validateBatch()` performs signature verification _and_ full message validation.

## Tests

```bash
git clone git@github.com:ssb-ngi-pointer/ssb-validate2-neon.git
cd ssb-validate2-neon
npm install
# Run tests
node test/test
# Run benchmarks
node test/perf
```

# License

AGPL 3.0.
