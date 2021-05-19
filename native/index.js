const v = require("./dist/index.node");

const verifySignatures = (msgs) => {
  if (!Array.isArray(msgs)) return ('input must be an array of message objects')
  const jsonMsgs = msgs.map((msg) => {
    return JSON.stringify(msg, null, 2);
  });
  return v.verifySignatures(jsonMsgs);
};

const validateSingle = (msg, previous) => {
  const jsonMsg = JSON.stringify(msg, null, 2);
  if (previous) {
    const jsonPrevious = JSON.stringify(previous, null, 2);
    return v.validateSingle(jsonMsg, jsonPrevious);
  }
  return v.validateSingle(jsonMsg);
}

const validateBatch = (msgs, previous) => {
  if (!Array.isArray(msgs)) return ('input must be an array of message objects')
  const jsonMsgs = msgs.map((msg) => {
    return JSON.stringify(msg, null, 2);
  });
  if (previous) {
    const jsonPrevious = JSON.stringify(previous, null, 2);
    return v.validateBatch(jsonMsgs, jsonPrevious);
  }
  return v.validateBatch(jsonMsgs);
};

const validateOOOBatch = (msgs) => {
  if (!Array.isArray(msgs)) return ('input must be an array of message objects')
  const jsonMsgs = msgs.map((msg) => {
    return JSON.stringify(msg, null, 2);
  });
  return v.validateOOOBatch(jsonMsgs);
};

const validateMultiAuthorBatch = (msgs) => {
  if (!Array.isArray(msgs)) throw new Error('input must be an array of message objects')
  const jsonMsgs = msgs.map((msg) => {
    return JSON.stringify(msg, null, 2);
  });
  return v.validateMultiAuthorBatch(jsonMsgs);
};

module.exports.verifySignatures = verifySignatures;
module.exports.validateSingle = validateSingle;
module.exports.validateBatch = validateBatch;
module.exports.validateOOOBatch = validateOOOBatch;
module.exports.validateMultiAuthorBatch = validateMultiAuthorBatch;
