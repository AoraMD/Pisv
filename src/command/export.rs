use crate::context::Context;

pub(crate) fn main(context: &Context) {
    let auth_json = match context.export_auth() {
        Ok(Some(auth_json)) => auth_json,
        Ok(None) => {
            context.report_error("you are not logged in");
            return;
        }
        Err(error) => {
            context.report_error(&format!("failed to export token: {}", error));
            return;
        }
    };
    let encoded = base64::encode_config(&auth_json, base64::STANDARD_NO_PAD);
    println!("copy the following token to pisv on other device");
    println!("{}", encoded);
}
