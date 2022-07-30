use crate::context::Context;

pub(crate) async fn main(context: &Context) {
    match context.clean_auth().await {
        Ok(logout) => {
            if logout {
                context.report_info("logout success");
            } else {
                context.report_error("you are not logged in");
            }
        }
        Err(error) => {
            context.report_error(&format!("logout failed: {}", error));
        }
    }
}
