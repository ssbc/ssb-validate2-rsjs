const test = require("tape");
const validate = require("../");

test("validateSingle is available in node.js", (t) => {
  validate.validateSingle({foo: 32}, null, (err) => {
    t.ok(err);
    t.match(err, /found invalid message/);
    t.end();
  })
});


