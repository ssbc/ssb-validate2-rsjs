// SPDX-FileCopyrightText: 2021 Andrew 'glyph' Reid
//
// SPDX-License-Identifier: Unlicense

const test = require("tape");
const validate = require("../");

test("validateSingle is available in node.js", (t) => {
  validate.validateSingle(null, {foo: 32}, null, (err) => {
    t.ok(err);
    t.match(err.message, /found invalid message/);
    t.end();
  })
});


