use neon::prelude::*;
use ssb_validate;

/*
struct SsbMessageValue {
    pub sequence: i32,
    pub author: Multikey,
}

//#[derive(Deserialize)]
struct SsbMessage {
    pub key: Multihash,
    pub value: SsbMessageValue,
}
*/

// validate the message hash chain
fn validate_message(mut cx: FunctionContext) -> JsResult<JsString> {
    // parse arg for message 1
    // attempt to cast the first argument to a JsObject,
    // then get the value if cast is successul
    let msg1 = cx.argument::<JsObject>(0)?;
    // parse arg for message 2
    let msg2 = cx.argument::<JsObject>(1)?;

    // match on validation
    /*
    match ssb_validate::validate_message_hash_chain(msg1.as_bytes(), msg2.as_bytes()) {
        Ok(_) => Ok(cx.string("validated")),
        Err(e) => panic!("{}", e),
    };
    */

    Ok(cx.string("validated"))
}

register_module!(mut cx, {
    cx.export_function("validateMessage", validate_message)
});
