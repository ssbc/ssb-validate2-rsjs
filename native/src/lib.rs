use neon::prelude::*;
use ssb_validate;

// Validate a single message value (must be first message of feed).
// Expects arguments in the form of JSON strings. This is obviously undesireable (we want to accept
// a JsObject instead, but it's a simple first step to us moving).
fn validate_single_message_value(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg0 = cx.argument::<JsValue>(0)?;
    let msg1 = arg0.downcast::<JsString>().or_throw(&mut cx)?.value();
    match ssb_validate::validate_message_value_hash_chain::<_, &[u8]>(msg1.as_bytes(), None) {
        Ok(_) => Ok(cx.string("validated")),
        Err(e) => panic!("{}", e),
    }
}

register_module!(mut cx, {
    cx.export_function("validateSingleMsgValue", validate_single_message_value)
});
