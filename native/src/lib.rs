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

register_module!(mut cx, {
    cx.export_function("validateSingleMsgValue", validate_single_message_value)?;
    cx.export_function("validateMsgValueChain", validate_message_value)?;
    Ok(())
});
