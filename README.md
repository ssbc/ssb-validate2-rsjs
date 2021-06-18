# ssb-validate2-rsjs

Cryptographic validation of Scuttlebutt messages in the form of Rust bindings for Node.js and WebAssembly.

Perform batch verification and validation of SSB message values using [ssb-verify-signatures](https://crates.io/crates/ssb-verify-signatures) and [ssb-validate](https://github.com/mycognosist/ssb-validate) from the [Sunrise Choir](https://github.com/sunrise-choir).

This package wraps [ssb-validate2-rsjs-node](https://github.com/ssb-ngi-pointer/ssb-validate2-rsjs-node) and [ssb-validate2-rsjs-wasm](https://github.com/ssb-ngi-pointer/ssb-validate2-rsjs-wasm) so to support Node.js (by compiling a native add-on) and Browsers (by compiling to WASM).

## Usage

```javascript
const validate = require('ssb-validate2-rsjs');

// Initialize
validate.ready(() => {

  // Verify signatures for an array of messages
  validate.verifySignatures(msgs, cb);

  // Validate a single message
  // Note: assumes msg.sequence == 1 (ie. `previous` == null)
  validate.validateSingle(msg, null, cb);

  // Validate a single message (includes `previous`)
  validate.validateSingle(msg, previous, cb);

  // Validate an array of messages by a single author
  // Note: assumes msgs[0].sequence == 1 (ie. `previous` == null)
  validate.validateBatch(msgs, null, cb);

  // Validate an array of messages by a single author (includes `previous`)
  validate.validateBatch(msgs, previous, cb);

  // Validate an array of out-of-order messages by a single author
  validate.validateOOOBatch(msgs, cb);

  // Validate an array of out-of-order messages by multiple authors
  validate.validateMultiAuthorBatch(msgs, cb);

});
```

## Behaviour

**Arguments**

The `msg` and `previous` arguments must always be in the form of message value objects (_not_ `KVT` objects).

The `msgs` argument must always be in the form of an array of message value objects.

The `previous` argument for `validateSingle()` and `validateBatch()` can be skipped by passing `null`.

**Callbacks**

All five functions (`verifySignatures()`, `validateSingle()`, `validateBatch()`, `validateOOOBatch()` and `validateMultiAuthorBatch()`) require the last argument to be a callback function. On success, the callback is called with `null` as the first argument and an array of keys (as strings) as the second argument. The keys represent the hashes of the validated message values and are returned in order. The `validateSingle()` function returns the key of the validated message as a string and not an array of string.

In the case of a failure in verification or validation, the callback's first argument will be an error message with detailed information (currently includes the reason for failure and the full message which caused the failure). The error message is returned as a `string`. The value of the second argument will be `null`.

**Validation Checks**

Note that `validateSingle()`, `validateBatch()`, `validateOOOBatch()` and `validateMultiAuthorBatch()` perform signature verification _and_ full message value validation.

The `validateOOOBatch()` function does not perform checks for ascending sequence number (ie. if `sequence` of the current message equals `sequence` of `previous` plus one), nor does it validate the hash of the `previous` message against the value stored in `previous` of the current message. However, validation of the `author` field is performed (ie. `author` of `previous` message must match the `author` of the current message).

The `validateMultiAuthorBatch()` function does not perform any checks of the `previous` message.

### Success Examples

Validate a single message (where sequence is 1):

```javascript
msg = {
  previous: null,
  sequence: 1,
  author: '@AzvddyStfk/T95/3VuHxuJRwqqpBkCyoW7qHRCui2N4=.ed25519',
  timestamp: 1491901740000,
  hash: 'sha256',
  content: { type: 'TTT' },
  signature: '8XdA3TwXsWasY8PGo5zI/QJAi6XsyCklzQv8dVtgOEZk4jRCVFDLb4OCK7H/s+lxOcxjpKn4NGocbQ7Z5mF5CQ==.sig.ed25519'
}

v.validateSingle(msg, null, (err, res) => {
  if (err) console.log(err);
  else console.log(res);
});

// %ybJG6SQH63+71OtO9r7cnxeOgEZyZQdecsGaPQXo/CM=.sha256
```

Validate a batch of messages (array of `message.value` objects):

```javascript
v.validateBatch(msgs, null, (err, res) => {
  if (err) console.log(err);
  else console.log(res);
});

// [
//   '%txJjB9WAJRQL9DpZkTqq43FPrWY0Yhb2D9TTfkY1GfY=.sha256',
//   '%4aaJEz3+rWNgGKXqS4Pfce4BUo8Spov+bK/QNTXAAl0=.sha256',
//   '%Cf50qmc1NXoWiL+H+0fivcAAkxkbUKnED9gkUZoe028=.sha256'
// ]
```

### Error Examples

Passing an incorrect type:

```javascript
const v = require('ssb-validate2-rsjs');

msgs = "this is a string";

v.verifySignatures(msgs, (err, res) => {
  if (err) console.log(err);
});

// 'input must be an array of message objects'
```

Failing signature verification:

```javascript
msg = {
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
}

msgs = [msg]

v.verifySignatures(msgs, (err, res) => {
  if (err) console.log(err);
});

//'found invalid message: Signature was invalid: {\n' +
//  '  "previous": "%IIjwbJbV3WBE/SBLnXEv5XM3Pr+PnMkrAJ8F+7TsUVQ=.sha256",\n' +
//  '  "author": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",\n' +
//  '  "sequence": 9,\n' +
//  '  "timestamp": 1470187438539,\n' +
//  '  "hash": "sha256",\n' +
//  '  "content": {\n' +
//  '    "type": "contact",\n' +
//  '    "contact": "@ye+QM09iPcDJD6YvQYjoQc7sLF/IFhmNbEqgdzQo3lQ=.ed25519",\n' +
//  '    "following": true,\n' +
//  '    "blocking": false\n' +
//  '  },\n' +
//  '  "signature": "PkZ34BRVSmGG51vMXo4GvaoS/2NBc0lzdFoVv4wkI8E8zXv4QYyE5o2mPACKOcrhrLJpymLzqpoE70q78INuBg==.sig.ed25519"\n' +
//  '}'
```

Failing validation due to incorrect sequence number:

```javascript
msg = {
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
}

msgs = [msg]

v.validateBatch(msgs, null, (err, res) => {
  if (err) console.log(err);
});

//'found invalid message: The first message of a feed must have seq of 1: {\n' +
//  '  "previous": "%yV9QaYDbkEHl4W8S8hVf/3TUuvs0JUrOP945jLLK/2c=.sha256",\n' +
//  '  "author": "@vt8uK0++cpFioCCBeB3p3jdx4RIdQYJOL/imN1Hv0Wk=.ed25519",\n' +
//  '  "sequence": 36,\n' +
//  '  "timestamp": 1445502075082,\n' +
//  '  "hash": "sha256",\n' +
//  '  "content": {\n' +
//  '    "type": "post",\n' +
//  '    "text": "Web frameworks.\\n\\n    Much industrial production in the late nineteenth century depended on skilled workers, whose knowledge of the production process often far exceeded their employers’; Taylor saw that this gave laborers a tremendous advantage over their employer in the struggle over the pace of work.\\n\\n    Not only could capitalists not legislate techniques they were ignorant of, but they were also in no position to judge when workers told them the process simply couldn’t be driven any faster. Work had to be redesigned so that employers did not depend on their employees for knowledge of the production process.\\n\\nhttps://www.jacobinmag.com/2015/04/braverman-gramsci-marx-technology/"\n' +
//  '    },\n' +
//  '  "signature": "FbDXlQtC2FQukU8svM5dOALN6QpxFhUHZaC7jTSXdOH7yqDfUlaj8q97YLdo5YqknZ71b0Y59hlQkmfkbtv5DA==.sig.ed25519"\n' +
//  '}'
```

## License

AGPL 3.0.
