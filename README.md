# ssb-validate2-neon

Cryptographic validation of Scuttlebutt messages in the form of Neon bindings (Rust) for Node.js.

Validate SSB messages and message values using the [ssb-validate crate](https://crates.io/crates/ssb-validate) ([repo](https://github.com/sunrise-choir/ssb-validate) & [docs](https://docs.rs/ssb-validate/1.0.1/ssb_validate/index.html)). Generate bindings to expose this functionality as a Node module.

## Usage

Validate the value of a single message:

```javascript
const validate = require('.');

msg = {
  previous: null,
  sequence: 1,
  author: '@ZByBkJp2fBgfw0ql3wbQFOWa9Fphzv0T0pq6FEvizuc=.ed25519',
  timestamp: 1597344109162,
  hash: 'sha256',
  content: { type: 'example' },
  signature: 'TMxuh2fAkPs9/6MNcyasYx+7m/kzD992zxLaBb4cnDDGnlYyplMhgpeA6uTtn7oQLJd3bY0XBw8mUu/9vnd3Bg==.sig.ed25519'
}

console.log(validate.validateSingleMsgValue(msg))
// validated
```

## Links

Neon: [repo](https://github.com/neon-bindings/neon) & [docs](https://neon-bindings.com/docs/intro).

## License

LGPL 3.0.
