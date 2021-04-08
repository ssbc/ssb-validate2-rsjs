// SPDX-License-Identifier: AGPL-3.0-only

mod utils;

use arrayvec::ArrayVec;
use neon::prelude::*;
use ssb_validate;

fn initial(mut cx: FunctionContext) -> JsResult<JsObject> {
    // create an empty object
    let object = JsObject::new(&mut cx);

    // define the initial state values
    let validated = cx.number(0 as f64);
    let queued = cx.number(0 as f64);
    let queue = cx.empty_array();
    let feeds = JsObject::new(&mut cx);
    let error = cx.null();

    // assign the initial state values to the object
    object.set(&mut cx, "validated", validated).unwrap();
    object.set(&mut cx, "queued", queued).unwrap();
    object.set(&mut cx, "queue", queue).unwrap();
    object.set(&mut cx, "feeds", feeds).unwrap();
    object.set(&mut cx, "error", error).unwrap();

    Ok(object)
}

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
