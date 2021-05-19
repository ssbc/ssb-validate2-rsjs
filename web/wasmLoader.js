import init, {initThreadPool, validateSingle, validateBatch, verifySignatures} from "./pkg/ssb_validate2_rsjs_wasm.js";

const msg = '{\n' +
'  "key": "%/v5mCnV/kmnVtnF3zXtD4tbzoEQo4kRq/0d/bgxP1WI=.sha256",\n' +
'  "value": {\n' +
'    "previous": null,\n' +
'    "author": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",\n' +
'    "sequence": 1,\n' +
'    "timestamp": 1470186877575,\n' +
'    "hash": "sha256",\n' +
'    "content": {\n' +
'      "type": "about",\n' +
'      "about": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",\n' +
'      "name": "Piet"\n' +
'    },\n' +
'    "signature": "QJKWui3oyK6r5dH13xHkEVFhfMZDTXfK2tW21nyfheFClSf69yYK77Itj1BGcOimZ16pj9u3tMArLUCGSscqCQ==.sig.ed25519"\n' +
'  },\n' +
'  "timestamp": 1571140551481\n' +
'}';

const msg2 = '{\n' +
'  "key": "%kLWDux4wCG+OdQWAHnpBGzGlCehqMLfgLbzlKCvgesU=.sha256",\n' +
'  "value": {\n' +
'    "previous": "%/v5mCnV/kmnVtnF3zXtD4tbzoEQo4kRq/0d/bgxP1WI=.sha256",\n' +
'    "author": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",\n' +
'    "sequence": 2,\n' +
'    "timestamp": 1470187292812,\n' +
'    "hash": "sha256",\n' +
'    "content": {\n' +
'      "type": "about",\n' +
'      "about": "@U5GvOKP/YUza9k53DSXxT0mk3PIrnyAmessvNfZl5E0=.ed25519",\n' +
'      "image": {\n' +
'        "link": "&MxwsfZoq7X6oqnEX/TWIlAqd6S+jsUA6T1hqZYdl7RM=.sha256",\n' +
'        "size": 642763,\n' +
'        "type": "image/png",\n' +
'        "width": 512,\n' +
'        "height": 512\n' +
'      }\n' +
'    },\n' +
'    "signature": "j3C7Us3JDnSUseF4ycRB0dTMs0xC6NAriAFtJWvx2uyz0K4zSj6XL8YA4BVqv+AHgo08+HxXGrpJlZ3ADwNnDw==.sig.ed25519"\n' +
'  },\n' +
'  "timestamp": 1571140551485\n' +
'}';

const msgs = [msg, msg2];

async function run() {
  await init();
  await initThreadPool(navigator.hardwareConcurrency);

  let err = validateSingle(msg);
  if (!err) {
    console.log("validateSingle works :)")
  };

  let err2 = validateBatch(msgs);
  if (!err2) {
    console.log("validateBatch works :)")
  };

  let err3 = verifySignatures(msgs);
  if (!err3) {
    console.log("verifySignatures works :)")
  };
};

run();
