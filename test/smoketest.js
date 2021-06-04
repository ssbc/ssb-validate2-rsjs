const test = require("tape");
const validate = require("../");

test("validateSingle is available in node.js", (t) => {
  const out = validate.validateSingle({foo: 32})
  t.ok(out)
  t.equals(out, 1285)
  t.end()
});


