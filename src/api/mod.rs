pub(crate) mod auth;
pub(crate) mod base;

use crate::{context::Context, util::extension::ResultExtension};
use chrono::Utc;
use futures::Future;
use reqwest::{
    header,
    header::{HeaderMap, HeaderValue},
    Client, Response, StatusCode,
};
use std::error::Error;

const USER_AGENT: &str = "PixivAndroidApp/5.0.234 (Android 12; Pixel 5)";
const PIXIV_SECRET: &str = "28c1fdd170a5204386cb1313c7077b34f83e4aaf4aa829ce78c231e05b0bae2c";

static mut CURRENT_ACCESS: String = String::new();
static mut CURRENT_REFRESH: String = String::new();

pub(self) fn base_client(headers: HeaderMap) -> Result<Client, Box<dyn Error>> {
    let client_time = Utc::now().to_rfc3339();
    let client_hash = format!(
        "{:x}",
        md5::compute(format!("{}{}", client_time, PIXIV_SECRET))
    );

    let values = {
        let mut values = HeaderMap::new();
        values.insert(header::ACCEPT_LANGUAGE, HeaderValue::from_static("zh-CN"));
        values.insert("x-client-time", HeaderValue::try_from(&client_time)?);
        values.insert("x-client-hash", HeaderValue::try_from(&client_hash)?);
        values.extend(headers);
        values
    };

    return Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(values)
        .build()?
        .into_ok();
}

pub(crate) async fn initialize_token(context: &mut Context) {
    context.report_debug("try to load tokens from local storage");
    let auth = match context.load_auth().await {
        Ok(auth) => auth,
        Err(_) => {
            return;
        }
    };
    context.report_debug(&format!(
        "loaded token: access {}, refresh {}",
        auth.access, auth.refresh
    ));
    context.update_id(&auth.id);
    unsafe {
        CURRENT_ACCESS = auth.access;
        CURRENT_REFRESH = auth.refresh;
    }
}

fn authorized_client() -> Result<Client, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    unsafe {
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", CURRENT_ACCESS))?,
        );
    }
    return base_client(headers);
}

pub(self) async fn authorized_request<R>(
    context: &mut Context,
    request: impl Fn(Client) -> R,
) -> Result<String, Box<dyn Error>>
where
    R: Future<Output = Result<Response, Box<dyn Error>>>,
{
    let mut continue_request = true;
    let mut result = "".to_string();
    while continue_request {
        let response = request(authorized_client()?).await?;
        let status = response.status();
        result = response.text().await?;
        if status == StatusCode::BAD_REQUEST {
            if result.contains("Error occurred at the OAuth process") {
                unsafe {
                    let auth = match auth::refresh(&CURRENT_REFRESH).await {
                        Ok(auth) => auth,
                        Err(_) => {
                            panic!("refresh token failed, please log in again");
                        }
                    };
                    CURRENT_ACCESS = auth.access.clone();
                    CURRENT_REFRESH = auth.refresh.clone();
                    context.save_auth(auth).await?;
                }
            }
        } else {
            continue_request = false;
        }
    }
    return result.into_ok();
}
