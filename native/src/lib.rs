// SPDX-License-Identifier: AGPL-3.0-only

mod utils;

use arrayvec::ArrayVec;
use neon::prelude::*;
use ssb_validate;

// Validate a single message value (must be first message of feed).
// Expects an argument in the form of a JSON object.
fn validate_single_message_value(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg1 = cx.argument::<JsValue>(0)?;

    if arg1.is_a::<JsUndefined>() || arg1.is_a::<JsNull>() {
        panic!("undefined argument(s)")
    };

    let msg1 = utils::json_stringify(&mut cx, ArrayVec::from([arg1]))?
        .value()
        .into_bytes();

    match ssb_validate::validate_message_value_hash_chain::<_, &[u8]>(msg1, None) {
        Ok(_) => Ok(cx.string("validated")),
        Err(e) => panic!("{}", e),
    }
}

// Validate a pair of message values.
// Expects two arguments (msg1, msg2) in the form of JSON objects.
fn validate_message_value(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg1 = cx.argument::<JsValue>(0)?;
    let arg2 = cx.argument::<JsValue>(1)?;

    // TODO: improve type checking and error-handling
    if arg1.is_a::<JsUndefined>() || arg1.is_a::<JsNull>() || arg2.is_a::<JsUndefined>() {
        panic!("undefined argument(s)")
    };

    // previous message
    let msg1 = utils::json_stringify(&mut cx, ArrayVec::from([arg1]))?
        .value()
        .into_bytes();

    // current message
    let msg2 = utils::json_stringify(&mut cx, ArrayVec::from([arg2]))?
        .value()
        .into_bytes();

    match ssb_validate::validate_message_value_hash_chain(msg2, Some(msg1)) {
        Ok(_) => Ok(cx.string("validated")),
        Err(e) => panic!("{}", e),
    }
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
    cx.export_function("validateFirstMsgValue", validate_single_message_value)?;
    cx.export_function("validateMsgValue", validate_message_value)?;
    cx.export_function("validateMsgArray", validate_message_array)?;
    Ok(())
});
