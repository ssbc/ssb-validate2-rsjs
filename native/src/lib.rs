// SPDX-License-Identifier: AGPL-3.0-only

mod utils;

use arrayvec::ArrayVec;
use neon::prelude::*;
use ssb_validate;

// Validate array of messages.
fn validate_message_array(mut cx: FunctionContext) -> JsResult<JsString> {
    let js_msgs: Handle<JsArray> = cx.argument(0)?;
    let msg_array: Vec<Handle<JsValue>> = js_msgs.to_vec(&mut cx)?;

    let mut msgs = Vec::new();

    for msg in msg_array {
        let msg_bytes = utils::json_stringify(&mut cx, ArrayVec::from([msg]))?
            .value()
            .into_bytes();
        msgs.push(msg_bytes)
    }

    match ssb_validate::par_validate_message_hash_chain_of_feed::<_, &[u8]>(&msgs, None) {
        Ok(_) => Ok(cx.string("validated")),
        Err(e) => panic!("{}", e),
    }
}

register_module!(mut cx, {
    cx.export_function("validateMsgArray", validate_message_array)?;
    Ok(())
});
