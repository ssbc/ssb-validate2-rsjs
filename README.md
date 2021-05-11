# ssb-validate2-rsjs

Cryptographic validation of Scuttlebutt messages in the form of Rust bindings for Node.js.

Perform batch verification and validation of SSB messages using [ssb-verify-signatures](https://crates.io/crates/ssb-verify-signatures) and [ssb-validate](https://github.com/mycognosist/ssb-validate) from the [Sunrise Choir](https://github.com/sunrise-choir).

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

// Validate a single message
// Note: assumes msg.sequence == 1 (ie. `previous` == null)
validate.validateSingle(msg);

// Validate a single message (includes `previous`)
validate.validateSingle(msg, previous);

// Validate an array of messages by a single author
// Note: assumes msgs[0].sequence == 1 (ie. `previous` == null)
validate.validateBatch(msgs);

// Validate an array of messages by a single author (includes `previous`)
validate.validateBatch(msgs, previous);

// Validate an array of out-of-order messages by a single author
validate.validateOOOBatch(msgs);

// Validate an array of out-of-order messages by multiple authors
validate.validateMultiAuthorBatch(msgs);
```

See `test/test.js` in this repo for in-context example usage (uses [ssb-fixtures](https://github.com/ssb-ngi-pointer/ssb-fixtures) and [jitdb](https://github.com/ssb-ngi-pointer/jitdb)).

## Behaviour

The `msg` and `previous` arguments must always be in the form of message objects.

The `msgs` argument must always be in the form of an array of message objects.

The `previous` argument for `validateSingle()` and `validateBatch()` is optional. Calling these functions without `previous` is the equivalent of passing `previous` as `null`.

All five functions (`verifySignatures()`, `validateSingle()`, `validateBatch()`, `validateOOOBatch()` and `validateMultiAuthorBatch()`) return `true` on success. In the case of a failure in verification or validation, these functions will return immediately with detailed information in the error message (currently includes the reason for failure and the full message which caused the failure).

Note that `validateSingle()`, `validateBatch()`, `validateOOOBatch()` and `validateMultiAuthorBatch()` perform signature verification _and_ full message validation.

The `validateOOOBatch()` function does not perform checks for ascending sequence number (ie. if `sequence` of the current message equals `sequence` of `previous` plus one), nor does it validate the hash of the `previous` message against the value stored in `previous` of the current message. However, validation of the `author` field is performed (ie. `author` of `previous` message must match the `author` of the current message).

The `validateMultiAuthorBatch()` function does not perform any checks of the `previous` message.

### Error Examples

Passing an incorrect type:

```javascript
const v = require('ssb-validate2-rsjs');

msgs = "this is a string"

try {
  v.verifySignatures(msgs)
} catch (err) {
  console.log(err.message)
}

// input must be an array of message objects
```

Failing signature verification:

```javascript
msg = {
  key: '%kmXb3MXtBJaNugcEL/Q7G40DgcAkMNTj3yhmxKHjfCM=.sha256',
  value: {
    previous: '%IIjwbJbV3WBE/SBLnXEv5XM3Pr+PnMkrAJ8F+7TsUVQ=.sha256',
    author: '@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519',
    sequence: 9,
    timestamp: 1470187438539,
    hash: 'sha256',
    content: {
      type: 'contact',
      contact: '@ye+QM09iPcDJD6YvQYjoQc7sLF/IFhmNbEqgdzQo3lQ=.ed25519',
      following: true,
      blocking: false
    },
    signature: 'PkZ34BRVSmGG51vMXo4GvaoS/2NBc0lzdFoVv4wkI8E8zXv4QYyE5o2mPACKOcrhrLJpymLzqpoE70q78INuBg==.sig.ed25519'
  },
  timestamp: 1571140551543
}

msgs = [msg]

try {
  v.verifySignatures(msgs)
} catch (err) {
  console.log(err.message)
}

//found invalid message: Signature was invalid: {
//  "key": "%kmXb3MXtBJaNugcEL/Q7G40DgcAkMNTj3yhmxKHjfCM=.sha256",
//  "value": {
//    "previous": "%IIjwbJbV3WBE/SBLnXEv5XM3Pr+PnMkrAJ8F+7TsUVQ=.sha256",
//    "author": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",
//    "sequence": 9,
//    "timestamp": 1470187438539,
//    "hash": "sha256",
//    "content": {
//      "type": "contact",
//      "contact": "@ye+QM09iPcDJD6YvQYjoQc7sLF/IFhmNbEqgdzQo3lQ=.ed25519",
//      "following": true,
//      "blocking": false
//    },
//    "signature": "PkZ34BRVSmGG51vMXo4GvaoS/2NBc0lzdFoVv4wkI8E8zXv4QYyE5o2mPACKOcrhrLJpymLzqpoE70q78INuBg==.sig.ed25519"
//  },
//  "timestamp": 1571140551543
//}
```

Failing validation due to incorrect sequence number:

```javascript
msg = {
  "key": "%lYAK7Lfigw00zMt/UtVg5Ol9XdR4BHWUCxq4r2Ops90=.sha256",
  "value": {
    "previous": "%yV9QaYDbkEHl4W8S8hVf/3TUuvs0JUrOP945jLLK/2c=.sha256",
    "author": "@vt8uK0++cpFioCCBeB3p3jdx4RIdQYJOL/imN1Hv0Wk=.ed25519",
    "sequence": 36,
    "timestamp": 1445502075082,
    "hash": "sha256",
    "content": {
      "type": "post",
      "text": "Web frameworks.\n\n    Much industrial production in the late nineteenth century depended on skilled workers, whose knowledge of the production process often far exceeded their employers’; Taylor saw that this gave laborers a tremendous advantage over their employer in the struggle over the pace of work.\n\n    Not only could capitalists not legislate techniques they were ignorant of, but they were also in no position to judge when workers told them the process simply couldn’t be driven any faster. Work had to be redesigned so that employers did not depend on their employees for knowledge of the production process.\n\nhttps://www.jacobinmag.com/2015/04/braverman-gramsci-marx-technology/"
    },
    "signature": "FbDXlQtC2FQukU8svM5dOALN6QpxFhUHZaC7jTSXdOH7yqDfUlaj8q97YLdo5YqknZ71b0Y59hlQkmfkbtv5DA==.sig.ed25519"
  },
  "timestamp": 1571140555382.0059
}

msgs = [msg]

try {
  v.validateBatch(msgs)
} catch (err) {
  console.log(err.message)
}

// found invalid message: The first message of a feed must have seq of 1: {
//  "key": "%lYAK7Lfigw00zMt/UtVg5Ol9XdR4BHWUCxq4r2Ops90=.sha256",
//  "value": {
//    "previous": "%yV9QaYDbkEHl4W8S8hVf/3TUuvs0JUrOP945jLLK/2c=.sha256",
//    "author": "@vt8uK0++cpFioCCBeB3p3jdx4RIdQYJOL/imN1Hv0Wk=.ed25519",
//    "sequence": 36,
//    "timestamp": 1445502075082,
//    "hash": "sha256",
//    "content": {
//      "type": "post",
//      "text": "Web frameworks.\n\n    Much industrial production in the late nineteenth century depended on skilled workers, whose knowledge of the production process often far exceeded their employers’; Taylor saw that this gave laborers a tremendous advantage over their employer in the struggle over the pace of work.\n\n    Not only could capitalists not legislate techniques they were ignorant of, but they were also in no position to judge when workers told them the process simply couldn’t be driven any faster. Work had to be redesigned so that employers did not depend on their employees for knowledge of the production process.\n\nhttps://www.jacobinmag.com/2015/04/braverman-gramsci-marx-technology/"
//    },
//    "signature": "FbDXlQtC2FQukU8svM5dOALN6QpxFhUHZaC7jTSXdOH7yqDfUlaj8q97YLdo5YqknZ71b0Y59hlQkmfkbtv5DA==.sig.ed25519"
//  },
//  "timestamp": 1571140555382.0059
//}
```

## Performance Benchmarks

After performing build instructions (see above):

```bash
cd ssb-validate2-rsjs
# Run benchmarks
node test/perf
node test/multiAuthorPerf
```

The default values for the performance benchmarks (`test/perf.js`) are 100 messages from 1 author, for a total of 10 iterations. These value constants can be changed in `test/perf.js`. Performance benchmarks for the multi-author method default to 100 messages from 5 authors, for a total of 10 iterations (`test/multiAuthorPerf.js`).

## License

AGPL 3.0.
