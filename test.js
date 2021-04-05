const validateMessage = require('.');
//const ssbValidate = require("ssb-validate");

/*
const message = ssbValidate.create(
  state.feeds[alice.public],
  alice,
  null,
  exampleObject,
  Date.now()
);
*/

msg1 = {"hello": "there"}
msg2 = {"what's": "up"}

console.log(validateMessage(msg1, msg2))
