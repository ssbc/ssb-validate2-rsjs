import init, { initThreadPool } from "./pkg/ssb_validate2_rsjs_wasm.js";
import {
  verifySignatures,
  validateSingle,
  validateBatch,
  validateOOOBatch,
  validateMultiAuthorBatch,
} from "./index.js";

const msg = { this: "errors" };
const msgs = [msg];

async function run() {
  await init();
  await initThreadPool(navigator.hardwareConcurrency);

  let err = validateSingle(msg);
  if (!err) {
    console.log("validateSingle works :)");
  } else {
    console.warn(err);
  }

  let err2 = validateBatch(msgs);
  if (!err2) {
    console.log("validateBatch works :)");
  }

  let err3 = verifySignatures(msgs);
  if (!err3) {
    console.log("verifySignatures works :)");
  }
}

run();
