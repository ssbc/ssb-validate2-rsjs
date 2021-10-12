// SPDX-FileCopyrightText: 2021 Andrew 'glyph' Reid
//
// SPDX-License-Identifier: LGPL-3.0-only

if (typeof localStorage === "undefined" || localStorage === null)
  module.exports = require("ssb-validate2-rsjs-node");
else module.exports = require("ssb-validate2-rsjs-wasm");
