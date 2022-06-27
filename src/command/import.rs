use crate::context::{Context, Authorization};

pub(crate) fn main(token: String, context: &Context) {
    let bytes = match base64::decode_config(token, base64::STANDARD_NO_PAD) {
        Ok(decoded) => decoded,
        Err(error) => {
            context.report_error(&format!("failed to decode code: {}", error));
            return;
        }
    };
    let decoded = match String::from_utf8(bytes) {
        Ok(decoded) => decoded,
        Err(error) => {
            context.report_error(&format!("failed to decode code: {}", error));
            return;
        }
    };
    let auth = match serde_json::from_str::<Authorization>(&decoded) {
        Ok(auth) => auth,
        Err(error) => {
            context.report_error(&format!("failed to decode code: {}", error));
            return;
        }
    };
    if let Err(error) = context.save_auth(auth) {
        context.report_error(&format!("failed to save authorization: {}", error));
    };
}
