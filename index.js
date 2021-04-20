const v = require("./dist/index.node");

const verifySignatures = (msgs) => {
  if (!Array.isArray(msgs)) throw new Error('input must be an array of message objects')
  const jsonMsgs = msgs.map((msg) => {
    return JSON.stringify(msg, null, 2);
  });
  return v.verifySignatures(jsonMsgs);
};

const validateBatch = (msgs, previous) => {
  if (!Array.isArray(msgs)) throw new Error('input must be an array of message objects')
  const jsonMsgs = msgs.map((msg) => {
    return JSON.stringify(msg, null, 2);
  });
  if (previous !== undefined) {
    const jsonPrevious = JSON.stringify(previous, null, 2);
    return v.validateBatch(jsonMsgs, jsonPrevious);
  }
  return v.validateBatch(jsonMsgs);
};

const validateOOOBatch = (msgs, previous) => {
  if (!Array.isArray(msgs)) throw new Error('input must be an array of message objects')
  const jsonMsgs = msgs.map((msg) => {
    return JSON.stringify(msg, null, 2);
  });
  if (previous !== undefined) {
    const jsonPrevious = JSON.stringify(previous, null, 2);
    return v.validateOOOBatch(jsonMsgs, jsonPrevious);
  } else {
    return v.validateOOOBatch(jsonMsgs);
  }
};

module.exports.verifySignatures = verifySignatures;
module.exports.validateBatch = validateBatch;
module.exports.validateOOOBatch = validateOOOBatch;
