# ssb-validate2-rsjs

Cryptographic validation of Scuttlebutt messages in the form of Rust bindings for Node.js and WebAssembly.

Perform batch verification and validation of SSB messages using [ssb-verify-signatures](https://crates.io/crates/ssb-verify-signatures) and [ssb-validate](https://github.com/mycognosist/ssb-validate) from the [Sunrise Choir](https://github.com/sunrise-choir).

See the `README` in the `native` directory for build instructions and details concerning the Node.js bindings. Corresponding information for the WebAssembly (WASM) Rust code can be found in the `README` of the `web` directory.

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

**Arguments**

The `msg` and `previous` arguments must always be in the form of message objects.

The `msgs` argument must always be in the form of an array of message objects.

The `previous` argument for `validateSingle()` and `validateBatch()` is optional. Calling these functions without `previous` is the equivalent of passing `previous` as `null`.

**Return Expressions**

All five functions (`verifySignatures()`, `validateSingle()`, `validateBatch()`, `validateOOOBatch()` and `validateMultiAuthorBatch()`) return `null` on success. In the case of a failure in verification or validation, these functions will return immediately with detailed information in the error message (currently includes the reason for failure and the full message which caused the failure). The error message is returned as a `string` type.

**Validation Checks**

Note that `validateSingle()`, `validateBatch()`, `validateOOOBatch()` and `validateMultiAuthorBatch()` perform signature verification _and_ full message validation.

The `validateOOOBatch()` function does not perform checks for ascending sequence number (ie. if `sequence` of the current message equals `sequence` of `previous` plus one), nor does it validate the hash of the `previous` message against the value stored in `previous` of the current message. However, validation of the `author` field is performed (ie. `author` of `previous` message must match the `author` of the current message).

The `validateMultiAuthorBatch()` function does not perform any checks of the `previous` message.

### Error Examples

Passing an incorrect type:

```javascript
const v = require('ssb-validate2-rsjs');

msgs = "this is a string";

let err = v.verifySignatures(msgs);
if (err) {
  console.log(err)
}

// 'input must be an array of message objects'
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

let err = v.verifySignatures(msgs);
if (err) {
  console.log(err)
}

//'found invalid message: Signature was invalid: {\n' +
//  '  "key": "%kmXb3MXtBJaNugcEL/Q7G40DgcAkMNTj3yhmxKHjfCM=.sha256",\n' +
//  '  "value": {\n' +
//  '    "previous": "%IIjwbJbV3WBE/SBLnXEv5XM3Pr+PnMkrAJ8F+7TsUVQ=.sha256",\n' +
//  '    "author": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",\n' +
//  '    "sequence": 9,\n' +
//  '    "timestamp": 1470187438539,\n' +
//  '    "hash": "sha256",\n' +
//  '    "content": {\n' +
//  '      "type": "contact",\n' +
//  '      "contact": "@ye+QM09iPcDJD6YvQYjoQc7sLF/IFhmNbEqgdzQo3lQ=.ed25519",\n' +
//  '      "following": true,\n' +
//  '      "blocking": false\n' +
//  '    },\n' +
//  '    "signature": "PkZ34BRVSmGG51vMXo4GvaoS/2NBc0lzdFoVv4wkI8E8zXv4QYyE5o2mPACKOcrhrLJpymLzqpoE70q78INuBg==.sig.ed25519"\n' +
//  '  },\n' +
//  '  "timestamp": 1571140551543\n' +
//  '}'
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

let err = v.validateBatch(msgs);
if (err) {
  console.log(err)
}

//'found invalid message: The first message of a feed must have seq of 1: {\n' +
//  '  "key": "%lYAK7Lfigw00zMt/UtVg5Ol9XdR4BHWUCxq4r2Ops90=.sha256",\n' +
//  '  "value": {\n' +
//  '    "previous": "%yV9QaYDbkEHl4W8S8hVf/3TUuvs0JUrOP945jLLK/2c=.sha256",\n' +
//  '    "author": "@vt8uK0++cpFioCCBeB3p3jdx4RIdQYJOL/imN1Hv0Wk=.ed25519",\n' +
//  '    "sequence": 36,\n' +
//  '    "timestamp": 1445502075082,\n' +
//  '    "hash": "sha256",\n' +
//  '    "content": {\n' +
//  '      "type": "post",\n' +
//  '      "text": "Web frameworks.\\n\\n    Much industrial production in the late nineteenth century depended on skilled workers, whose knowledge of the production process often far exceeded their employers’; Taylor saw that this gave laborers a tremendous advantage over their employer in the struggle over the pace of work.\\n\\n    Not only could capitalists not legislate techniques they were ignorant of, but they were also in no position to judge when workers told them the process simply couldn’t be driven any faster. Work had to be redesigned so that employers did not depend on their employees for knowledge of the production process.\\n\\nhttps://www.jacobinmag.com/2015/04/braverman-gramsci-marx-technology/"\n' +
//  '    },\n' +
//  '    "signature": "FbDXlQtC2FQukU8svM5dOALN6QpxFhUHZaC7jTSXdOH7yqDfUlaj8q97YLdo5YqknZ71b0Y59hlQkmfkbtv5DA==.sig.ed25519"\n' +
//  '  },\n' +
//  '  "timestamp": 1571140555382.0059\n' +
//  '}'
```

## Developers

Technical information regarding the implementation of this library, including details about tests and performance benchmarks, can be found in `web/README.md` (WASM) and `native/README.md` (Node.js).

## License

AGPL 3.0.
