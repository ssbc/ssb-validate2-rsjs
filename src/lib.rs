use node_bindgen::core::NjError;
use node_bindgen::derive::node_bindgen;
use ssb_validate::{par_validate_message_hash_chain_of_feed, validate_message_hash_chain};
use ssb_verify_signatures::{par_verify_messages, verify_message};

/// Verify signatures for an array of messages.
///
/// Takes an array of messages as the only argument. If verification fails, the cause of the error
/// is returned along with the offending message. Note: this method only verifies message signatures;
/// it does not perform full message validation (use `verify_validate_message_array` for complete
/// verification and validation).
#[node_bindgen(name = "verifySignatures")]
fn verify_messages(array: Vec<String>) -> Result<bool, NjError> {
    let mut msgs = Vec::new();
    for msg in array {
        let msg_bytes = msg.into_bytes();
        msgs.push(msg_bytes)
    }

    // attempt batch verficiation and match on error to find invalid message
    match par_verify_messages(&msgs, None) {
        Ok(_) => Ok(true),
        Err(e) => {
            let invalid_msg = &msgs
                .iter()
                .find(|msg| verify_message(msg).is_err())
                .unwrap();
            let invalid_msg_str = std::str::from_utf8(invalid_msg).unwrap();
            let err_msg = format!("found invalid message: {}: {}", e, invalid_msg_str);
            Err(NjError::Other(err_msg))
        }
    }
}

/// Verify signatures and perform validation for an array of messages.
///
/// Takes an array of messages as the first argument and an optional previous message as the second
/// argument. The previous message argument is expected when the array of messages does not start
/// from the beginning of the feed (ie. sequence number != 1 and previous != null). If
/// verification or validation fails, the cause of the error is returned along with the offending
/// message.
#[node_bindgen(name = "validateBatch")]
fn verify_validate_messages(array: Vec<String>, previous: Option<String>) -> Result<bool, NjError> {
    let mut msgs = Vec::new();
    for msg in array {
        let msg_bytes = msg.into_bytes();
        msgs.push(msg_bytes)
    }

    let previous_msg = match previous {
        Some(msg) => Some(msg.into_bytes()),
        None => None,
    };

    // attempt batch verficiation and match on error to find invalid message
    match par_verify_messages(&msgs, None) {
        Ok(_) => (),
        Err(e) => {
            let invalid_msg = &msgs
                .iter()
                .find(|msg| verify_message(msg).is_err())
                .unwrap();
            let invalid_msg_str = std::str::from_utf8(invalid_msg).unwrap();
            let err_msg = format!("found invalid message: {}: {}", e, invalid_msg_str);
            return Err(NjError::Other(err_msg));
        }
    };

    // attempt batch validation and match on error to find invalid message
    match par_validate_message_hash_chain_of_feed(&msgs, previous_msg.as_ref()) {
        Ok(_) => Ok(true),
        Err(e) => {
            let invalid_message = &msgs
                .iter()
                .find(|msg| validate_message_hash_chain(msg, previous_msg.as_ref()).is_err())
                .unwrap();
            let invalid_msg_str = std::str::from_utf8(invalid_message).unwrap();
            let err_msg = format!("found invalid message: {}: {}", e, invalid_msg_str);
            Err(NjError::Other(err_msg))
        }
    }
}
