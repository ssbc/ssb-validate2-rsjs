// SPDX-License-Identifier: AGPL-3.0-only

use arrayvec::ArrayVec;
use neon::prelude::*;
use ssb_validate::{par_validate_message_hash_chain_of_feed, validate_message_hash_chain};
use ssb_verify_signatures::{par_verify_messages, verify_message};

mod utils;

/// Verify signatures for an array of messages.
///
/// Takes an array of messages as the only argument. If verification fails, the cause of the error is returned along with the offending
/// message. Note: this method only verifies message signatures; it does not perform full
/// message validation (use `verify_validate_message_array` for complete verification and
/// validation).
fn verify_message_array(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // parse the msg array argument
    let js_msgs: Handle<JsArray> = cx.argument(0)?;
    // convert the msg array to a vector
    let msg_array: Vec<Handle<JsValue>> = js_msgs.to_vec(&mut cx)?;

    // create a vector to collect the bytes of each stringified msg
    let mut msgs = Vec::new();

    // stringify each msg and collect the bytes
    for msg in msg_array {
        let msg_bytes = utils::json_stringify(&mut cx, ArrayVec::from([msg]))?
            .value()
            .into_bytes();
        msgs.push(msg_bytes)
    }

    // attempt batch verficiation and match on error to find invalid message
    match par_verify_messages(&msgs, None) {
        Ok(_) => (),
        Err(e) => {
            let invalid_message = &msgs
                .iter()
                .find(|msg| verify_message(msg).is_err())
                .unwrap();
            let invalid_msg_str = std::str::from_utf8(invalid_message).unwrap();
            let err_msg = format!("found invalid message: {}: {}", e, invalid_msg_str);
            return cx.throw_error(err_msg);
        }
    }

    Ok(cx.boolean(true))
}

/// Verify signatures and perform validation for an array of messages.
///
/// Takes an array of messages as the first argument and an optional previous message as the second
/// argument. The previous message argument is expected when the array of messages does not start
/// from the beginning of the feed (ie. sequence number != 1 and previous != null). If
/// verification or validation fails, the cause of the error is returned along with the offending
/// message.
fn verify_validate_message_array(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // parse the msg array argument
    let js_msgs: Handle<JsArray> = cx.argument(0)?;
    // convert the msg array to a vector
    let msg_array: Vec<Handle<JsValue>> = js_msgs.to_vec(&mut cx)?;
    // check for the `previous` argument (set to `None` if not found)
    // if found, stringify and then convert to a byte array
    let previous = match cx.argument_opt(1) {
        Some(msg) => {
            let msg_bytes = utils::json_stringify(&mut cx, ArrayVec::from([msg]))?
                .value()
                .into_bytes();
            Some(msg_bytes)
        }
        None => None,
    };

    // create a vector to collect the bytes of each stringified msg
    let mut msgs = Vec::new();

    // TODO: maybe we do a separate allocation for stringified and bytes?
    //  - allows us to query msg fields by index (i think...might need some serde magic)
    //  - this could be useful for returning the sequence number or key of the last successfully
    //  validated msg

    // stringify each msg and collect the bytes
    for msg in msg_array {
        let msg_bytes = utils::json_stringify(&mut cx, ArrayVec::from([msg]))?
            .value()
            .into_bytes();
        msgs.push(msg_bytes)
    }

    // attempt batch verficiation and match on error to find invalid message
    match par_verify_messages(&msgs, None) {
        Ok(_) => (),
        Err(e) => {
            let invalid_message = &msgs
                .iter()
                .find(|msg| verify_message(msg).is_err())
                .unwrap();
            let invalid_msg_str = std::str::from_utf8(invalid_message).unwrap();
            let err_msg = format!("found invalid message: {}: {}", e, invalid_msg_str);
            return cx.throw_error(err_msg);
        }
    }

    // attempt batch validation and match on error to find invalid message
    match par_validate_message_hash_chain_of_feed(&msgs, previous.as_ref()) {
        Ok(_) => (),
        Err(e) => {
            let invalid_message = &msgs
                .iter()
                .find(|msg| validate_message_hash_chain(msg, previous.as_ref()).is_err())
                .unwrap();
            let invalid_msg_str = std::str::from_utf8(invalid_message).unwrap();
            let err_msg = format!("found invalid message: {}: {}", e, invalid_msg_str);
            return cx.throw_error(err_msg);
        }
    }

    Ok(cx.boolean(true))
}

register_module!(mut cx, {
    cx.export_function("verifySignatures", verify_message_array)?;
    cx.export_function("validateBatch", verify_validate_message_array)?;
    Ok(())
});
