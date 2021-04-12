// SPDX-License-Identifier: AGPL-3.0-only

mod utils;

use arrayvec::ArrayVec;
use neon::prelude::*;
use ssb_validate::{par_validate_message_hash_chain_of_feed, validate_message_hash_chain};
use ssb_verify_signatures::{par_verify_messages, verify_message};

/// Verify signatures for an array of messages.
///
/// Takes an array of messages as the first argument and an optional previous message as the second
/// argument. The previous message argument is expected when the array of messages does not start
/// from the beginning of the feed (ie. sequence number != 1 and previous != null).
fn verify_validate_message_array(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    // parse the msg array argument
    let js_msgs: Handle<JsArray> = cx.argument(0)?;
    // convert the msg array to a vector
    let msg_array: Vec<Handle<JsValue>> = js_msgs.to_vec(&mut cx)?;
    // check for the `previous` argument (set to `None` if not found)
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
    //  - allows us to query msg fields (i think...might need some serde magic)

    for msg in msg_array {
        let msg_bytes = utils::json_stringify(&mut cx, ArrayVec::from([msg]))?
            .value()
            .into_bytes();
        msgs.push(msg_bytes)
    }

    par_verify_messages(&msgs, None)
        .map_err(|e| {
            let invalid_message = &msgs
                .iter()
                .find(|msg| verify_message(msg).is_err())
                .unwrap();
            panic!(
                "found invalid message: {}: {}",
                e,
                // do we actually want to return the entire message on error?
                std::str::from_utf8(invalid_message).unwrap()
            )
        })
        .unwrap();

    par_validate_message_hash_chain_of_feed(&msgs, previous.as_ref())
        .map_err(|_| {
            let invalid_message = &msgs
                .iter()
                .find(|msg| validate_message_hash_chain(msg, previous.as_ref()).is_err())
                .unwrap();
            panic!(
                "found invalid message: {}",
                std::str::from_utf8(invalid_message).unwrap()
            )
        })
        .unwrap();

    Ok(cx.boolean(true))
}

register_module!(mut cx, {
    cx.export_function("validateBatch", verify_validate_message_array)?;
    Ok(())
});
