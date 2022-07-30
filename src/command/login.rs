use crate::{api, context::Context, util::pkce::Pkce};

fn create_login_url() -> (String, Pkce) {
    let pkce = Pkce::new();
    let url = format!("https://app-api.pixiv.net/web/v1/login?code_challenge={}&code_challenge_method=S256&client=pixiv-android", &pkce.challenge);
    return (url, pkce);
}

pub(crate) async fn main(context: &Context) {
    let (url, pkce) = create_login_url();

    println!("login url: {}", url);
    let code = rpassword::prompt_password("code: ").unwrap();

    let auth = match api::auth::login(&code, &pkce.verifier).await {
        Ok(auth) => auth,
        Err(error) => {
            context.report_error(&format!("login error: {}", error));
            return;
        }
    };

    if let Err(error) = context.save_auth(auth).await {
        context.report_error(&format!("failed to save authorization: {}", error));
    };

    println!("login success");
}
