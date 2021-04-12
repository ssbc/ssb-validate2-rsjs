// SPDX-License-Identifier: AGPL-3.0-only

mod utils;

use arrayvec::ArrayVec;
use neon::prelude::*;

// Verify signatures for an array of messages.
// TODO: change output to JsResult<JsBoolean> and `throw` on error
fn verify_validate_message_array(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let js_msgs: Handle<JsArray> = cx.argument(0)?;
    let msg_array: Vec<Handle<JsValue>> = js_msgs.to_vec(&mut cx)?;

    let mut msgs = Vec::new();

    for msg in msg_array {
        let msg_bytes = utils::json_stringify(&mut cx, ArrayVec::from([msg]))?
            .value()
            .into_bytes();
        msgs.push(msg_bytes)
    }

    ssb_verify_signatures::par_verify_messages(&msgs, None).unwrap();

    ssb_validate::par_validate_message_hash_chain_of_feed::<_, &[u8]>(&msgs, None).unwrap();

    Ok(cx.boolean(true))
}

register_module!(mut cx, {
    cx.export_function("validateBatch", verify_validate_message_array)?;
    Ok(())
});
