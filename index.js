if (typeof localStorage === "undefined" || localStorage === null)
  module.exports = require("ssb-validate2-rsjs-node");
else module.exports = require("ssb-validate2-rsjs-wasm");
