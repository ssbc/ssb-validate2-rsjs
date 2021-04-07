// SPDX-License-Identifier: LGPL-3.0

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

// Validate a chain of messages.
// Expects an argument in the form of an array of JSON objects.
fn validate_message_chain(mut cx: FunctionContext) -> JsResult<JsString> {
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

    let messages = [msg1, msg2];

    match ssb_validate::par_validate_message_hash_chain_of_feed::<_, &[u8]>(&messages, None) {
        Ok(_) => Ok(cx.string("validated")),
        Err(e) => panic!("{}", e),
    }
}

// Return number of items in array.
fn array_item_count(mut cx: FunctionContext) -> JsResult<JsString> {
    // take the first argument, which must be an array
    let js_msgs: Handle<JsArray> = cx.argument(0)?;
    // convert JsArray to Rust Vec
    let msgs: Vec<Handle<JsValue>> = js_msgs.to_vec(&mut cx)?;
    // check number of messages in vec
    let len = format!("{}", msgs.len());
    Ok(cx.string(len))
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
    cx.export_function("validateSingleMsgValue", validate_single_message_value)?;
    cx.export_function("validateMsgValueChain", validate_message_value)?;
    cx.export_function("validateMsgChain", validate_message_chain)?;
    cx.export_function("countArrayItems", array_item_count)?;
    cx.export_function("validateMsgArray", validate_message_array)?;
    Ok(())
});
