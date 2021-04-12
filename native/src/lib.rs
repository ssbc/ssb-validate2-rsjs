// SPDX-License-Identifier: AGPL-3.0-only

mod utils;

use arrayvec::ArrayVec;
use neon::prelude::*;

// Implement multiple function variants to test overhead of neon.
fn do_nothing(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("validated"))
}

fn get_arg(mut cx: FunctionContext) -> JsResult<JsString> {
    let _js_msgs: Handle<JsArray> = cx.argument(0)?;
    Ok(cx.string("validated"))
}

fn arg_to_vec(mut cx: FunctionContext) -> JsResult<JsString> {
    let js_msgs: Handle<JsArray> = cx.argument(0)?;
    let _msg_array: Vec<Handle<JsValue>> = js_msgs.to_vec(&mut cx)?;
    Ok(cx.string("validated"))
}

fn vec_to_bytes(mut cx: FunctionContext) -> JsResult<JsString> {
    let js_msgs: Handle<JsArray> = cx.argument(0)?;
    let msg_array: Vec<Handle<JsValue>> = js_msgs.to_vec(&mut cx)?;

    let mut msgs = Vec::new();

    for msg in msg_array {
        let msg_bytes = utils::json_stringify(&mut cx, ArrayVec::from([msg]))?
            .value()
            .into_bytes();
        msgs.push(msg_bytes)
    }

    Ok(cx.string("validated"))
}

// Validate array of messages.
// TODO: change output to JsResult<JsBoolean> and `throw` on error
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

// Verify signatures and validate an array of messages.
// TODO: change output to JsResult<JsBoolean> and `throw` on error
fn verify_message_array(mut cx: FunctionContext) -> JsResult<JsString> {
    let js_msgs: Handle<JsArray> = cx.argument(0)?;
    let msg_array: Vec<Handle<JsValue>> = js_msgs.to_vec(&mut cx)?;

    let mut msgs = Vec::new();

    for msg in msg_array {
        let msg_bytes = utils::json_stringify(&mut cx, ArrayVec::from([msg]))?
            .value()
            .into_bytes();
        msgs.push(msg_bytes)
    }

    match ssb_verify_signatures::par_verify_messages(&msgs, None) {
        Ok(_) => Ok(cx.string("verified")),
        Err(e) => panic!("{}", e),
    }
}

// Verify signatures for an array of messages.
// TODO: change output to JsResult<JsBoolean> and `throw` on error
fn verify_validate_message_array(mut cx: FunctionContext) -> JsResult<JsString> {
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

    match ssb_validate::par_validate_message_hash_chain_of_feed::<_, &[u8]>(&msgs, None) {
        Ok(_) => Ok(cx.string("validated")),
        Err(e) => panic!("{}", e),
    }
}

register_module!(mut cx, {
    cx.export_function("doNothing", do_nothing)?;
    cx.export_function("getArg", get_arg)?;
    cx.export_function("argVec", arg_to_vec)?;
    cx.export_function("vecBytes", vec_to_bytes)?;
    cx.export_function("validateMsgArray", validate_message_array)?;
    cx.export_function("verifyMsgArray", verify_message_array)?;
    cx.export_function("verifyValidateMsgArray", verify_validate_message_array)?;
    Ok(())
});
